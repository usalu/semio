/** 🧭️ The quiz client session: local-first, event-driven state of one learner on one device, and the controller that
 * turns learner intent into proctor commands and folds what comes back into that state.
 *
 * State only changes through {@link QuizClientEvent}s folded by the pure {@link evolveQuizState}. Slices are kept apart
 * by lifetime: persisted local-only (introduction seen, learner id, cached catalog, learner view, one record per run
 * view, the answer outbox), persisted shared (the proctor's events, reached only through commands), ephemeral shared
 * (the leaderboard, polled) and ephemeral local-only (the current step and the trail of steps around it, notices, newly
 * earned badges, the quizzes whose crowd the learner asked to see before submitting). The site's tabs share the persisted local-only slices: what another tab stores is adopted without
 * being written back — a run record by {@link mergeRunViews} — so tabs never undo each other. Answers apply locally at
 * once and travel through the {@link Outbox}; interactive commands retry transient failures with jittered backoff under
 * one command id and are cancellable through their `AbortSignal`. A learner is registered by a command and recalled by
 * a read: asking who holds a pseudonym or name writes nothing, and whoever asks for a claimed one continues as its
 * holder. The learner finds the way with the trail: back and forward along the steps that still stand, up to the place
 * above, and a new trail with every new learner.
 *
 * A run starts at a challenge; starting a quiz at another challenge than its open run's voids that run. The device
 * keeps the time: an opening of a task and an answer carry the instant the learner acted by the session clock
 * ({@link QuizSession.now}), so a connection shortage never makes a timely answer late. An easy run's hints follow
 * every answer — at once from the deputy, else with the proctor's view of the run once its answers are delivered.
 *
 * With a {@link Deputy} the device also decides by itself. A command goes straight to the proctor unless the proctor
 * is known not to answer or something the deputy decided still waits in the outbox; then the deputy decides it, its events
 * fold into the held views at once and the command waits in the outbox for the proctor. What the deputy decided is
 * provisional: while such a command waits, the proctor's views are behind and are not adopted; once none waits they
 * are read again and replace what the device made up. A learner the deputy registered under a pseudonym or name the
 * proctor already knows continues as its holder, like anyone who enters a claimed handle.
 *
 * @see ../../../../🧬️schema/🔣️.json — commands, events, views
 * @see ../🫡️deputy/🟦️.ts — the deciders on the device
 */

import { latestWins } from "@semio-tech/framework";
import {
  challengeRules,
  emptyLearnerState,
  evolveLearner,
  type Answer,
  type BadgeAwardedEvent,
  type Challenge,
  type Command,
  type Event,
  type HandleView,
  type Id,
  type Identity,
  type IdentityClaim,
  type IdentifyLearnerCommand,
  type LearnerState,
  type LearnerView,
  type OpenTaskCommand,
  type RunSubmittedEvent,
  type RunView,
  type Slug,
  type StartRunCommand,
  type SubmitRunCommand,
} from "@semio-tech/quiz";
import type { Deputy } from "../🫡️deputy/🟦️.ts";
import { type LocalChange, type LocalStore } from "../💾️persistence/🟦️.ts";
import { Outbox, coalescingKey, commandRun } from "../📮️outbox/🟦️.ts";
import {
  ProctorThrottled,
  ProctorUnavailable,
  RETRY_AFTER_MAX_MS,
  RETRY_TIMING,
  SIGN_UP_ALLOWANCE,
  errorRejection,
  isNotFound,
  isTransient,
  newId,
  retryTransient,
  signUpsSpent,
  type CommandVerdict,
  type ProctorClient,
  type RetryTiming,
} from "../🛂️proctor/🟦️.ts";
import {
  type BoardChoice,
  boardKey,
  DEFAULT_BOARD,
  evolveQuizState,
  HOME_PAGES,
  keptFresh,
  mergeRunViews,
  openRunOf,
  overallLeaderboard,
  overdue,
  type QuizClientEvent,
  type QuizState,
  type QuizStep,
  type QuizTrail,
  stepAbove,
  unhinted,
} from "./🧮️state/🟦️.ts";
import {
  type Decided,
  decision,
  DEPUTY_PATIENCE_MS,
  describe,
  failureOf,
  IDENTIFY_ATTEMPTS,
  PROJECTION_GRACE_MS,
  type QuizSessionOptions,
  type QuizSnapshot,
  type RefreshOutcome,
  registration,
  type SessionFailure,
  stoppedLifetime,
  type SubmissionPhase,
} from "./📋️session-model/🟦️.ts";
import {
  persistQuizState,
  restoredCatalog,
  restoredLearner,
  restoredLearnerView,
  restoredRun,
  restoredRuns,
  restoreQuizState,
} from "./💾️restore/🟦️.ts";

//#region 🎮️Session
/** 🎮️ One learner session against one proctor tenant — an external store for the UI and the controller of its intents. */
export class QuizSession {
  readonly proctor: ProctorClient;
  readonly outbox: Outbox;
  private readonly store: LocalStore;
  private readonly deputy: Deputy | undefined;
  private readonly patienceMs: number;
  private readonly timing: RetryTiming;
  private readonly clock: () => number;
  private state: QuizState;
  private online = true;
  private signUpsFrom = 0;
  private readonly unconfirmed = new Set<Id>();
  private edits = 0;
  private readonly edited = new Map<string, number>();
  private reads = 0;
  private readonly adopted = new Map<Id, number>();
  private snapshot: QuizSnapshot;
  private lifetime = stoppedLifetime();
  private readonly listeners = new Set<() => void>();
  private readonly teardown: (() => void)[] = [];

  /** 📚️ Reloads the catalog, retrying through connection shortages; concurrent calls collapse into one. */
  readonly refreshCatalog = latestWins(async (): Promise<void> => {
    const catalog = await this.background((signal) => this.proctor.catalog(this.state.learner?.id, signal));
    if (catalog !== undefined) this.dispatch({ type: "catalog-loaded", catalog });
  });

  /** 🔃️ Reloads the learner view, retrying through connection shortages and projection lag; concurrent calls collapse
   * into one. A learner the proctor still does not know afterwards is forgotten on this device and asked to identify.
   * While the device is ahead of the proctor nothing is read: the proctor has not heard everything yet. */
  readonly refreshLearner = latestWins(async (): Promise<void> => {
    const learner = this.state.learner?.id;
    if (learner === undefined || this.ahead()) return;
    const signal = this.lifetime.signal;
    try {
      const view = await this.projected((lifetime) => this.proctor.learner(learner, lifetime), signal);
      if (!this.ahead()) this.adoptLearnerView(view);
    } catch (error) {
      if (signal.aborted || this.state.learner?.id !== learner || this.ahead()) return;
      if (!isNotFound(error)) return this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      this.outbox.forget(learner);
      this.dispatch({ type: "learner-forgotten" });
      this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: "unknown-learner" } });
    }
  });

  /** 👥️ Asks for the submitted crowd of `quiz` once; a failure keeps the last known crowd. */
  async refreshCrowd(quiz: Slug): Promise<void> {
    try {
      this.dispatch({ type: "crowd-loaded", crowd: await this.proctor.crowd(quiz, this.state.learner?.id, this.lifetime.signal) });
    } catch {
      return;
    }
  }

  /** 👀️ The learner asks to see what the others answered in `quiz` now, before submitting: the crowd held shows at
   * once — also without a connection — and is asked for again. The ask ends with the run. */
  askCrowd(quiz: Slug): void {
    this.dispatch({ type: "crowd-asked", quiz });
    void this.refreshCrowd(quiz);
  }

  /** 🫣️ The learner hides the others of `quiz` again until submitting. */
  unaskCrowd(quiz: Slug): void {
    this.dispatch({ type: "crowd-unasked", quiz });
  }

  /** 👆️ The learner chooses the leaderboard to look at — a period, of every quiz or of one: what is held of it shows at
   * once, and it is asked for again. A quiz the catalog does not list is no choice. */
  chooseBoard(board: BoardChoice): void {
    const before = this.state.board;
    this.dispatch({ type: "board-chosen", board });
    if (this.state.board !== before) void this.refreshLeaderboard();
  }

  /** 📈️ Asks for the leaderboard the learner looks at once; polling repeats it, so a failure only keeps the last known
   * standings and tells the poller to slow down. While the learner looks at another leaderboard than the overall one,
   * that one — which says the learner's rank on the profile — is asked for too whenever the answer says that runs were
   * submitted since it was held. When the proctor does not answer — or not within the learner's patience — a deputy
   * fills in the device's own standing for every leaderboard the proctor never answered here. */
  readonly refreshLeaderboard = latestWins(async (): Promise<RefreshOutcome> => {
    const spent = setTimeout(() => this.ownStanding(), this.patienceMs);
    try {
      const { board, learner } = this.state;
      const leaderboard = await this.proctor.leaderboard(board, learner?.id, this.lifetime.signal);
      this.dispatch({ type: "leaderboard-loaded", leaderboard, at: this.now() });
      if (boardKey(board) !== boardKey(DEFAULT_BOARD) && overallLeaderboard(this.state)?.board.submissions !== leaderboard.submissions)
        this.dispatch({ type: "leaderboard-loaded", leaderboard: await this.proctor.leaderboard(DEFAULT_BOARD, learner?.id, this.lifetime.signal), at: this.now() });
      return { answered: true };
    } catch (error) {
      this.ownStanding();
      return error instanceof ProctorThrottled && error.retryAfterMs !== undefined ? { answered: false, retryAfterMs: error.retryAfterMs } : { answered: false };
    } finally {
      clearTimeout(spent);
    }
  });

  constructor(options: QuizSessionOptions) {
    this.proctor = options.proctor;
    this.store = options.store;
    this.deputy = options.deputy;
    this.patienceMs = options.patienceMs ?? DEPUTY_PATIENCE_MS;
    this.timing = options.timing ?? RETRY_TIMING;
    this.clock = options.now ?? Date.now;
    this.outbox = new Outbox({ send: (command, signal) => this.deliver(command, signal), store: options.store, timing: this.timing, now: this.clock, onSettled: (command, verdict) => this.settled(command, verdict) });
    this.state = this.restored();
    this.rehintAll();
    this.snapshot = this.freeze();
  }

  /** 🕰️ The session's clock in milliseconds since the Unix epoch: the instant an answer or an opening is stamped with
   * and the one every countdown is measured against. */
  now(): number {
    return this.clock();
  }

  /** 🎞️ The current snapshot; the same object until something changes. */
  getSnapshot = (): QuizSnapshot => this.snapshot;

  /** 🔔️ Calls `listener` on every change; returns the unsubscribe function. */
  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  /** ▶️ Starts delivering answers, loading the catalog and the learner, and following the browser's connectivity and
   * what the site's other tabs store. */
  start(): void {
    this.lifetime = new AbortController();
    this.teardown.push(
      this.outbox.subscribe(() => {
        this.overlayPending();
        this.notify();
      }),
    );
    this.teardown.push(this.store.watch((change) => this.remote(change)));
    let reached = this.proctor.reachability();
    this.teardown.push(
      this.proctor.subscribe(() => {
        const before = reached;
        reached = this.proctor.reachability();
        if (reached === "reachable" && before !== "throttled") this.outbox.wake();
        this.notify();
      }),
    );
    if (typeof window !== "undefined") {
      const online = (): void => this.connectivity(true);
      const offline = (): void => this.connectivity(false);
      window.addEventListener("online", online);
      window.addEventListener("offline", offline);
      this.teardown.push(
        () => window.removeEventListener("online", online),
        () => window.removeEventListener("offline", offline),
      );
      this.online = typeof navigator === "undefined" || navigator.onLine !== false;
    }
    this.outbox.start();
    void this.refreshCatalog();
    void this.refreshLearner();
    this.notify();
  }

  /** ⏹️ Stops every background activity; persisted slices stay for the next {@link start}. */
  stop(): void {
    this.lifetime.abort();
    this.outbox.stop();
    for (const release of this.teardown.splice(0)) release();
  }

  /** 🔌️ Asks the proctor again now instead of waiting out the current backoff: restarts the catalog and learner loads
   * and wakes the outbox. Does nothing while the session is stopped. */
  reconnect(): void {
    if (this.lifetime.signal.aborted) return;
    this.lifetime.abort();
    this.lifetime = new AbortController();
    this.outbox.wake();
    void this.refreshCatalog();
    void this.refreshLearner();
  }

  /** 👋️ The learner has read the introduction. */
  readIntroduction(): void {
    this.dispatch({ type: "introduction-read" });
  }

  /** 🚪️ Opens a screen and refreshes what it shows from the proctor; a cached run shows at once, also offline. The
   * step left stays on the trail behind — unless the new one shows `instead` of it, as the page the address named on
   * arrival does. */
  open(step: QuizStep, instead = false): void {
    this.dispatch({ type: "step-opened", step, instead });
    this.refresh(step);
  }

  /** ◀️ Goes back along the trail to the nearest step that still stands. */
  back(): void {
    this.retrace("back");
  }

  /** ⏭️ Goes forward again along the trail to the nearest step that still stands. */
  forward(): void {
    this.retrace("forward");
  }

  /** 🔼️ Opens the place above the step in front, if there is one. */
  up(): void {
    const above = stepAbove(this.state);
    if (above !== undefined) this.open(above);
  }

  private retrace(to: keyof QuizTrail): void {
    const left = this.state.step;
    this.dispatch({ type: "step-retraced", to });
    if (this.state.step !== left) this.refresh(this.state.step);
  }

  private refresh(step: QuizStep): void {
    if (step.screen === "home" && (step.page === undefined || step.page === HOME_PAGES.learner || step.page === HOME_PAGES.badges)) void this.refreshLearner();
    if (step.screen === "home" && step.page === HOME_PAGES.leaderboard) void this.refreshLeaderboard();
    if (step.screen === "run" || step.screen === "results")
      void this.loadRun(step.run).then((view) => {
        if (view !== undefined) void this.refreshCrowd(view.quiz);
      });
  }

  /** 🙈️ Dismisses the current notice. */
  dismissNotice(): void {
    this.dispatch({ type: "notice-dismissed" });
  }

  /** 🧽️ Forgets the learner on this device; their progress stays with the proctor. */
  forgetLearner(): void {
    this.dispatch({ type: "learner-forgotten" });
  }

  /** 🙋️ Makes this device a learner. An anonymous learner is registered under a fresh id. For a pseudonym or name the
   * proctor is asked who holds it: a claimed one is recalled — this device continues as its holder, nothing is written
   * — and a free one is registered; when someone else claims it in between, the proctor is asked again. When the
   * proctor says that the sign-up allowance of this address is spent, the sign-up is not sent again: the answer is the
   * `waiting` failure, and until the wait the proctor named is over no further sign-up leaves this device — a recall,
   * which is a read, still does. While the proctor is away a deputy registers the learner on the device: it knows no
   * holder of any handle, so the proctor settles later whose the handle is. */
  async identify(identity: IdentityClaim, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const register = async (learner: Id): Promise<Decided | Extract<SessionFailure, { readonly kind: "waiting" }>> => {
      const left = this.signUpsFrom - this.now();
      if (left > 0) return { kind: "waiting", allowance: SIGN_UP_ALLOWANCE, retryAfterMs: left };
      const command: IdentifyLearnerCommand = { type: "identify-learner", id: newId(), learner, identity };
      try {
        return await this.decided(command, signal);
      } catch (error) {
        if (!signUpsSpent(error)) throw error;
        this.signUpsFrom = this.now() + (error.retryAfterMs ?? 0);
        return error.retryAfterMs === undefined ? { kind: "waiting", allowance: SIGN_UP_ALLOWANCE } : { kind: "waiting", allowance: SIGN_UP_ALLOWANCE, retryAfterMs: error.retryAfterMs };
      }
    };
    const adopt = (learner: Id, registered: Identity, by: Decided["by"] = "proctor"): undefined => {
      this.dispatch({ type: "learner-identified", learner, identity: registered });
      if (by === "deputy") this.fold({ ...emptyLearnerState(learner), identity: registered }, []);
      void this.refreshLearner();
      return undefined;
    };
    for (let attempt = 0; attempt < IDENTIFY_ATTEMPTS; attempt += 1) {
      let display: string | undefined;
      if (identity.kind !== "anonymous") {
        let held: HandleView;
        try {
          held = await this.holding(identity.handle, signal);
        } catch (error) {
          const rejection = errorRejection(error);
          if (rejection === undefined) throw error;
          return { kind: "rejected", rejection };
        }
        if (held.holder !== undefined) return adopt(held.holder.learner, held.holder.identity);
        display = held.display;
      }
      const learner = newId();
      const outcome = await register(learner);
      if ("kind" in outcome) return outcome;
      const { verdict, by } = outcome;
      if (verdict.kind === "accepted") {
        const registered = verdict.events.find(registration);
        return adopt(registered?.learner ?? learner, registered?.identity ?? (identity.kind === "anonymous" ? identity : { kind: identity.kind, handle: display ?? identity.handle }), by);
      }
      if (verdict.kind !== "rejected" || verdict.rejection !== (identity.kind === "anonymous" ? "learner-exists" : "handle-claimed")) return failureOf(verdict);
    }
    return { kind: "rejected", rejection: identity.kind === "anonymous" ? "learner-exists" : "handle-claimed" };
  }

  /** 🚀️ Starts a run of `quiz` at `challenge` at the instant the learner started it by the session clock — the run's
   * start wherever and whenever it is decided — and opens it; an already open run of the quiz at that challenge is
   * resumed instead, and one at another challenge is voided by the start — the learner asked for that, so no notice says
   * it. */
  async startRun(quiz: Slug, challenge: Challenge, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    const command: StartRunCommand = { type: "start-run", id: newId(), learner, run: newId(), quiz, challenge, at: this.now() };
    const { verdict, by } = await this.decided(command, signal);
    if (verdict.kind === "rejected" && verdict.rejection === "run-open") {
      if (by === "proctor") this.adoptLearnerView(await this.projected((current) => this.proctor.learner(learner, current), signal));
      const open = openRunOf(this.state, quiz);
      return open === undefined ? failureOf(verdict) : this.resumeRun(open, signal);
    }
    if (verdict.kind !== "accepted") return failureOf(verdict);
    for (const event of verdict.events) {
      if (event.type !== "run-voided") continue;
      this.outbox.discard(event.run);
      this.dispatch({ type: "run-voided", run: event.run });
    }
    return this.resumeRun(command.run, signal);
  }

  /** ⏯️ Opens a run, loading it first when this device has not seen it yet. */
  async resumeRun(run: Id, signal: AbortSignal): Promise<undefined> {
    if (this.state.runs[run] === undefined) await this.loadRun(run, signal);
    else void this.loadRun(run);
    this.dispatch({ type: "step-opened", step: { screen: "run", run } });
    void this.refreshLearner();
    return undefined;
  }

  /** ⏱️ Opens a task of a timed run: its clock starts at the instant the learner acted by the session clock. Decided
   * like a start — by the proctor, or by the deputy while the proctor is away, but only once the run's view is held: it
   * is loaded first when the device knows the run from its listing alone — and a task the held view already holds as
   * opened needs no command. Once it resolves without a failure, the held view holds the opening. `already-opened` is
   * success — the task is open elsewhere, so the run is read again when the held view lacks the opening. A revised quiz
   * voids the run; a run that closed is read again. */
  async openTask(run: Id, task: Slug, signal: AbortSignal): Promise<SessionFailure | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    if (this.state.runs[run] === undefined) await this.loadRun(run, signal);
    const opened = this.state.runs[run]?.opened;
    if (opened !== undefined && Object.hasOwn(opened, task)) return undefined;
    const command: OpenTaskCommand = { type: "open-task", id: newId(), learner, run, task, at: this.now() };
    const { verdict, by } = await this.decided(command, signal);
    if (verdict.kind === "accepted") {
      if (by === "deputy") return undefined;
      const event = verdict.events.find((candidate) => candidate.type === "task-opened" && candidate.run === run && candidate.task === task);
      if (event === undefined) await this.loadRun(run, signal);
      else this.dispatch({ type: "task-opened", run, task, at: event.at });
      return undefined;
    }
    if (verdict.kind === "rejected" && verdict.rejection === "already-opened") {
      const held = this.state.runs[run]?.opened;
      if (held === undefined || !Object.hasOwn(held, task)) await this.loadRun(run, signal);
      return undefined;
    }
    if (verdict.kind === "rejected" && verdict.rejection === "quiz-revised") this.voided(run);
    if (verdict.kind === "rejected" && (verdict.rejection === "run-closed" || verdict.rejection === "unknown-run")) void this.loadRun(run);
    return failureOf(verdict);
  }

  /** ✍️ Records an answer given now by the session clock: applied locally at once, delivered through the outbox — its
   * instant is the one the deciders judge, also once a later edit of the same task replaced it in the queue. An answer
   * to a task whose time is up by the session clock is refused here as the deciders would refuse it: nothing changes and
   * nothing is sent, and the learner is told. The hints of the task's former answer go at once; on a run that hints, a
   * deputy tells those of the new answer right away, else they arrive with the proctor's view of the run once its
   * answers are delivered. */
  answer(run: Id, task: Slug, answer: Answer): void {
    const learner = this.state.learner?.id;
    const view = this.state.runs[run];
    if (learner === undefined || view === undefined || view.status !== "open") return;
    const at = this.now();
    if (overdue(view, task, at)) return this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: "time-up" } });
    this.dispatch({ type: "answer-given", run, task, answer });
    this.edited.set(coalescingKey({ run, task }), (this.edits += 1));
    this.outbox.enqueue({ type: "record-answer", id: newId(), learner, run, task, answer, at });
    this.rehint(run);
  }

  /** 📨️ Submits a run once all its answers are delivered, reporting each phase; `signal` cancels between and during
   * phases. A cancellation can race the proctor committing the submission, so every cancelled submission is
   * reconciled: the run view is read again and, if the proctor did submit it, adopted with its result. */
  async submit(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    try {
      return await this.submitting(run, signal, onPhase);
    } catch (error) {
      if (signal.aborted) void this.loadRun(run);
      throw error;
    }
  }

  private async submitting(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<SessionFailure | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return { kind: "rejected", rejection: "unknown-learner" };
    const delivered = this.direct() && (await this.saved(run, signal, onPhase));
    if (this.state.runs[run]?.status === "voided") return { kind: "rejected", rejection: "quiz-revised" };
    onPhase({ phase: "submitting" });
    const command: SubmitRunCommand = { type: "submit-run", id: newId(), learner, run };
    const verdict = delivered || !this.decidable(command) ? (await this.decided(command, signal)).verdict : this.deputise(command);
    if (verdict.kind === "refused" || (verdict.kind === "rejected" && verdict.rejection !== "run-closed")) return failureOf(verdict);
    onPhase({ phase: "results" });
    const events = verdict.kind === "accepted" ? verdict.events : [];
    const submitted = events.find((event): event is RunSubmittedEvent => event.type === "run-submitted" && event.run === run);
    if (events.some((event) => event.type === "run-voided" && event.run === run)) {
      this.voided(run);
      return { kind: "rejected", rejection: "quiz-revised" };
    }
    if (submitted !== undefined) {
      const badges = events.filter((event): event is BadgeAwardedEvent => event.type === "badge-awarded" && event.run === run).map((event) => event.badge);
      this.dispatch({ type: "run-submitted", run, result: submitted.result, badges, at: submitted.at });
    } else {
      const view = await this.loadRun(run, signal);
      if (view?.status !== "submitted") return { kind: "rejected", rejection: view?.status === "voided" ? "quiz-revised" : "run-closed" };
      this.dispatch({ type: "step-opened", step: { screen: "results", run } });
    }
    void this.refreshLearner();
    const quiz = this.state.runs[run]?.quiz;
    if (quiz !== undefined) void this.refreshCrowd(quiz);
    return undefined;
  }

  /** 📖️ Loads a run view, waiting out projection lag. The proctor's view wins — including answers another device gave
   * — except for the answers it cannot hold yet, with their hints: those still waiting in the outbox, and those this
   * device gave of a task that had one waiting when the read was asked for or that it answered since. A read overtaken
   * by a later one of the same run is dropped. A run that closed meanwhile is followed: to its results when submitted,
   * home with a notice when voided. `signal` makes it interactive, else it runs in the background. While the proctor's
   * view of the run lacks a decision of the deputy ({@link behind}) the held view is the answer. */
  async loadRun(run: Id, signal?: AbortSignal): Promise<RunView | undefined> {
    const learner = this.state.learner?.id;
    if (learner === undefined) return undefined;
    if (this.behind(run)) return this.state.runs[run];
    const ticket = (this.reads += 1);
    const edits = this.edits;
    const unsettled = Object.keys(this.outbox.pendingAnswers(run));
    const read = (current: AbortSignal): Promise<RunView> => this.proctor.run(run, learner, current);
    const loaded = signal === undefined ? await this.background(read) : await this.projected(read, signal);
    if (loaded === undefined || this.state.learner?.id !== learner) return undefined;
    if (this.behind(run) || (this.adopted.get(run) ?? 0) > ticket) return this.state.runs[run];
    this.adopted.set(run, ticket);
    const cached = this.state.runs[run];
    const fresh = (task: Slug): boolean => unsettled.includes(task) || (this.edited.get(coalescingKey({ run, task })) ?? 0) > edits;
    const view = this.withPending(cached === undefined ? loaded : keptFresh(loaded, cached, fresh));
    this.dispatch({ type: "run-loaded", view });
    this.rehint(run);
    this.followClosure(cached, view);
    return this.state.runs[run] ?? view;
  }

  /** 📝️ `view` with the answers still waiting in the outbox on top — they are newer than anything the proctor has — and
   * without the hints of the tasks whose answer that changes. */
  private withPending(view: RunView): RunView {
    if (view.status !== "open") return view;
    const pending = this.outbox.pendingAnswers(view.run);
    const changed = Object.keys(pending).filter((task) => JSON.stringify(view.answers[task]) !== JSON.stringify(pending[task]));
    return changed.length === 0 ? view : unhinted({ ...view, answers: { ...view.answers, ...pending } }, (task) => changed.includes(task));
  }

  private overlayPending(): void {
    for (const view of Object.values(this.state.runs)) {
      const overlaid = this.withPending(view);
      if (overlaid === view) continue;
      this.dispatch({ type: "run-loaded", view: overlaid }, false);
      this.rehint(view.run, false);
    }
  }

  private followClosure(before: RunView | undefined, after: RunView | undefined, refresh = true): void {
    if (before?.status !== "open" || after === undefined || after.status === "open") return;
    const onRun = this.state.step.screen === "run" && this.state.step.run === after.run;
    this.retire(after.run);
    if (after.status === "voided") {
      if (onRun) this.dispatch({ type: "step-opened", step: { screen: "home" } });
      this.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    } else if (onRun) this.dispatch({ type: "step-opened", step: { screen: "results", run: after.run } });
    if (refresh) void this.refreshLearner();
  }

  private adoptLearnerView(view: LearnerView, persist = true): void {
    const before = this.state.runs;
    this.dispatch({ type: "learner-loaded", view }, persist);
    for (const summary of view.runs) this.followClosure(before[summary.run], this.state.runs[summary.run], false);
  }

  private remote(change: LocalChange): void {
    if (change.kind === "cleared") return this.adoptStore();
    if (change.kind === "record") {
      if (change.collection === "runs") this.adoptRun(change.id);
      return;
    }
    switch (change.slice) {
      case "introduced":
        if (this.store.read("introduced") === true && !this.state.introduced) this.dispatch({ type: "introduction-read" }, false);
        return;
      case "learner":
        return this.adoptLearner();
      case "catalog": {
        const catalog = restoredCatalog(this.store.read("catalog"));
        if (catalog !== undefined) this.dispatch({ type: "catalog-loaded", catalog }, false);
        return;
      }
      case "learner-view": {
        const view = restoredLearnerView(this.store.read("learner-view"));
        if (view !== undefined) this.adoptLearnerView(view, false);
        return;
      }
      case "preferences":
        return;
    }
  }

  private adoptLearner(): void {
    const stored = restoredLearner(this.store.read("learner"));
    if (stored?.id === this.state.learner?.id) return;
    if (stored === undefined) return this.dispatch({ type: "learner-forgotten" }, false);
    this.dispatch({ type: "learner-identified", learner: stored.id, identity: stored.identity }, false);
    for (const view of Object.values(restoredRuns(this.store, stored.id))) {
      this.dispatch({ type: "run-loaded", view: this.withPending(view) }, false);
      this.rehint(view.run, false);
    }
    void this.refreshLearner();
  }

  private adoptRun(run: Id): void {
    const stored = restoredRun(this.store.record("runs", run), run);
    if (stored === undefined || stored.learner !== this.state.learner?.id) return;
    const cached = this.state.runs[run];
    const view = this.withPending(mergeRunViews(cached, stored));
    this.dispatch({ type: "run-loaded", view }, false);
    this.rehint(run, false);
    this.followClosure(cached, view);
  }

  private adoptStore(): void {
    this.state = this.restored();
    this.rehintAll();
    this.notify();
  }

  /** 💡️ Has the deputy tell the hints of `run` anew when it is an open run that hints and they differ from those held —
   * whenever the answers held may have changed. Without a deputy, or for a run whose held sheet the material does not
   * deal, the hints held stay as they are: whatever changed an answer already dropped the hints of its former one. */
  private rehint(run: Id, persist = true): void {
    const learner = this.state.learner?.id;
    const view = this.state.runs[run];
    if (this.deputy === undefined || learner === undefined || view?.status !== "open" || !challengeRules(view.sheet.challenge).hints) return;
    const hints = this.deputy.hints(this.held(learner), run);
    if (hints !== undefined && JSON.stringify(hints) !== JSON.stringify(view.hints ?? {})) this.dispatch({ type: "run-hinted", run, hints }, persist);
  }

  private rehintAll(): void {
    for (const run of Object.keys(this.state.runs)) this.rehint(run);
  }

  private restored(): QuizState {
    const held = restoreQuizState(this.store);
    const stored = held.catalog === undefined && this.deputy !== undefined ? { ...held, catalog: this.deputy.catalog } : held;
    const state = stored.learnerView === undefined ? stored : evolveQuizState(stored, { type: "learner-loaded", view: stored.learnerView });
    for (const summary of state.learnerView?.runs ?? []) if (summary.status !== "open") this.retire(summary.run);
    for (const [run, view] of Object.entries(state.runs)) if (stored.runs[run] !== view) this.store.put("runs", run, view);
    return { ...state, runs: Object.fromEntries(Object.entries(state.runs).map(([run, view]) => [run, this.withPending(view)])) };
  }

  /** 🗑️ Drops what is queued of a run that closed — unless a decision about that run still waits: then the proctor has
   * not heard of its closing yet and needs every command of it in order. */
  private retire(run: Id): void {
    if (!this.outbox.waiting((command) => decision(command) && commandRun(command) === run)) this.outbox.discard(run);
  }

  /** 🏎️ Whether the device is ahead of the proctor: a decision the deputy took still waits in the outbox, so the
   * proctor's views lack it. */
  private ahead(): boolean {
    return this.deputy !== undefined && this.outbox.waiting(decision);
  }

  /** 🐢️ Whether the proctor's view of `run` lacks a decision the deputy took that still waits in the outbox: one about
   * the run itself, a start of a run of its quiz — which may void it — or the learner's registration. Decisions about
   * other runs leave the proctor's view of this one as it is. */
  private behind(run: Id): boolean {
    if (this.deputy === undefined) return false;
    const quiz = this.state.runs[run]?.quiz ?? this.state.learnerView?.runs.find((summary) => summary.run === run)?.quiz;
    return this.outbox.waiting((command) => decision(command) && (command.type === "identify-learner" || commandRun(command) === run || (command.type === "start-run" && (quiz === undefined || command.quiz === quiz))));
  }

  /** 🛂️ Whether a command is the proctor's to decide right now: always without a deputy; with one unless the proctor
   * is known not to answer (it did not, it asked to slow down, it speaks another contract, or the browser is offline)
   * or the device is ahead —
   * then the command would overtake what waits. A proctor nobody has heard from yet is asked. */
  private direct(): boolean {
    const reached = this.proctor.reachability();
    return this.deputy === undefined || (this.online && (reached === "reachable" || reached === "unknown") && !this.ahead());
  }

  /** 🏛️ Has `command` decided. Without a deputy the proctor decides, however long a connection shortage lasts. With
   * one the proctor is asked once while it is the proctor's to decide, and the deputy decides whenever it is not or
   * the proctor did not answer; a sent command the proctor did decide is the same command when it arrives again. */
  private async decided(command: Command, signal: AbortSignal): Promise<Decided> {
    if (this.deputy === undefined || !this.decidable(command)) return { verdict: await retryTransient(() => this.proctor.command(command, signal), this.timing, signal), by: "proctor" };
    if (this.direct()) {
      try {
        return { verdict: await this.patient((waiting) => this.proctor.command(command, waiting), signal), by: "proctor" };
      } catch (error) {
        if (signal.aborted || !isTransient(error)) throw error;
      }
    }
    return { verdict: this.deputise(command), by: "deputy" };
  }

  /** 🔎️ Whether the deputy may decide `command` from what the device holds: an opening or a submission of an open run
   * only when its run view is held — a listing alone knows neither the run's answers nor when its tasks were opened, so
   * such a command waits for the proctor. */
  private decidable(command: Command): boolean {
    if (command.type !== "open-task" && command.type !== "submit-run") return true;
    return this.state.runs[command.run] !== undefined || this.state.learnerView?.runs.find((summary) => summary.run === command.run)?.status !== "open";
  }

  /** 🧘️ Asks the proctor through `call` for as long as the learner's patience lasts; after that the call is given up as
   * a connection shortage and the proctor counts as unreachable until it answers anything again, so the next command
   * does not wait for it once more. */
  private async patient<T>(call: (signal: AbortSignal) => Promise<T>, signal: AbortSignal): Promise<T> {
    const waiting = new AbortController();
    const cancel = (): void => waiting.abort(signal.reason);
    const spent = setTimeout(() => {
      this.proctor.silent();
      waiting.abort(new ProctorUnavailable("the proctor did not answer in time"));
    }, this.patienceMs);
    signal.addEventListener("abort", cancel, { once: true });
    try {
      return await call(waiting.signal);
    } finally {
      clearTimeout(spent);
      signal.removeEventListener("abort", cancel);
    }
  }

  /** 🫡️ The deputy's decision of `command`: rejected like the proctor would, or accepted — then the command waits in
   * the outbox and its events are folded into the held views (a registration is folded by whoever adopts the learner). */
  private deputise(command: Command): CommandVerdict {
    const before = command.type === "identify-learner" ? emptyLearnerState(command.learner) : this.held(command.learner);
    const taken = this.deputy!.decide(before, command, this.now());
    if ("rejection" in taken) return { kind: "rejected", rejection: taken.rejection };
    this.outbox.enqueue(command);
    if (command.type !== "identify-learner") this.fold(taken.events.reduce(evolveLearner, before), taken.events);
    return { kind: "accepted", events: taken.events };
  }

  /** 🗃️ The learner `learner` as the held views describe it; a learner this device does not act as has nothing held. */
  private held(learner: Id): LearnerState {
    const { learner: acting, learnerView, runs } = this.state;
    return acting?.id === learner ? this.deputy!.state({ learner, identity: acting.identity, view: learnerView, runs }) : emptyLearnerState(learner);
  }

  /** 📥️ Makes `after` — the learner once `events` happened — what the device holds: the view of every run the events
   * touch, with the sheet the learner already sees, and the learner view. */
  private fold(after: LearnerState, events: readonly Event[]): void {
    const deputy = this.deputy!;
    for (const run of new Set(events.flatMap((event) => (event.type === "learner-registered" ? [] : [event.run])))) {
      const view = deputy.runView(after, run);
      if (view !== undefined) this.dispatch({ type: "run-loaded", view: { ...view, sheet: this.state.runs[run]?.sheet ?? view.sheet } });
    }
    const view = deputy.learnerView(after);
    if (view !== undefined) this.dispatch({ type: "learner-loaded", view });
  }

  /** 🔦️ Who holds `handle`: the proctor's answer — or, when it is not the proctor's to say right now, the deputy's,
   * which knows no holder. */
  private async holding(handle: string, signal: AbortSignal): Promise<HandleView> {
    if (this.deputy === undefined) return retryTransient(() => this.proctor.handle(handle, signal), this.timing, signal);
    if (this.direct()) {
      try {
        return await this.patient((waiting) => this.proctor.handle(handle, waiting), signal);
      } catch (error) {
        if (signal.aborted || !isTransient(error)) throw error;
      }
    }
    return this.deputy.handle(handle);
  }

  /** 📤️ Waits until every answer of `run` reached the proctor, reporting the progress, and says whether they all did.
   * With a deputy the wait ends as soon as the run is no longer the proctor's to decide, and at the latest when the
   * learner's patience is spent: the answers go on waiting, and the submission must wait behind them. */
  private async saved(run: Id, signal: AbortSignal, onPhase: (phase: SubmissionPhase) => void): Promise<boolean> {
    const progress = (done: number, total: number): void => onPhase({ phase: "saving", done, total });
    if (this.deputy === undefined) return this.outbox.settled(run, signal, progress).then(() => true);
    const away = new AbortController();
    const leave = (): void => away.abort();
    const unsubscribe = this.proctor.subscribe(() => this.direct() || leave());
    const spent = setTimeout(leave, this.patienceMs);
    signal.addEventListener("abort", leave, { once: true });
    try {
      await this.outbox.settled(run, away.signal, progress);
      return true;
    } catch (error) {
      if (signal.aborted) throw signal.reason ?? error;
      return false;
    } finally {
      unsubscribe();
      clearTimeout(spent);
      signal.removeEventListener("abort", leave);
    }
  }

  /** 🥈️ Shows the device's own standing on the leaderboard the learner looks at; a standing the proctor sent stays. */
  private ownStanding(): void {
    const learner = this.state.learner?.id;
    if (this.deputy === undefined || learner === undefined || this.lifetime.signal.aborted) return;
    this.dispatch({ type: "leaderboard-loaded", leaderboard: this.deputy.leaderboard(this.held(learner), this.state.board, this.now()), at: this.now(), local: true });
  }

  /** 🚚️ Delivers one queued command to the proctor. A spent sign-up allowance and a full roster are waited out, never
   * given up on: the learner goes on playing on the device meanwhile. A registration is only delivered once the
   * proctor knows its learner ({@link enrolled}), so the commands behind it find that learner. */
  private async deliver(command: Command, signal: AbortSignal): Promise<CommandVerdict> {
    let verdict: CommandVerdict;
    try {
      verdict = await this.proctor.command(command, signal);
    } catch (error) {
      if (signUpsSpent(error)) throw new ProctorThrottled(error.retryAfterMs ?? RETRY_AFTER_MAX_MS);
      throw error;
    }
    return command.type === "identify-learner" ? this.enrolled(command, verdict, signal) : verdict;
  }

  /** 🎫️ The verdict of a delivered registration once the proctor knows the learner. A handle that turned out to be
   * claimed is recalled: the device continues as its holder ({@link recalled}) before the next command leaves. A
   * registration the proctor already holds is accepted. The proctor relays a registration under a handle to its learner
   * a moment later, so the learner view is awaited — when it stays missing, the delivery counts as not answered. */
  private async enrolled(command: IdentifyLearnerCommand, verdict: CommandVerdict, signal: AbortSignal): Promise<CommandVerdict> {
    let learner = command.learner;
    if (verdict.kind === "refused") return verdict;
    if (verdict.kind === "rejected") {
      if (verdict.rejection === "roster-full") throw new ProctorThrottled(RETRY_AFTER_MAX_MS);
      if (verdict.rejection === "handle-claimed" && command.identity.kind !== "anonymous") {
        const { holder } = await this.proctor.handle(command.identity.handle, signal);
        if (holder === undefined) throw new ProctorUnavailable("the holder of the handle is not known yet");
        if (holder.learner !== learner) this.recalled(learner, holder.learner, holder.identity);
        learner = holder.learner;
      } else if (verdict.rejection !== "learner-exists") return verdict;
    }
    try {
      await this.projected((current) => this.proctor.learner(learner, current), signal);
    } catch (error) {
      throw isNotFound(error) ? new ProctorUnavailable("the learner is not enrolled yet") : error;
    }
    return { kind: "accepted", events: verdict.kind === "accepted" ? verdict.events : [] };
  }

  /** 🪪️ The learner this device registered as `from` is the learner `to` of the proctor: every queued command and
   * everything held moves to that id, and the learner is told when it is the one on screen. */
  private recalled(from: Id, to: Id, identity: Identity): void {
    this.outbox.reassign(from, to);
    if (this.state.learner?.id !== from) return;
    this.dispatch({ type: "learner-recalled", from, to, identity });
    if (identity.kind !== "anonymous") this.dispatch({ type: "notice-raised", notice: { kind: "recalled", handle: identity.handle } });
  }

  /** 📭️ What the proctor decided about a command the deputy had decided. An accepted one needs nothing now — a run the
   * proctor voided with it is voided here too. A run the proctor would not start never existed for it, and a run whose
   * task the proctor would not open because the quiz was revised is void for it like one whose answer met the revision:
   * either is voided on the device. A task the proctor already holds as opened (`already-opened`) is open, as the
   * device wants it. Any other refusal is told. Once the device is no longer ahead, the proctor's views are
   * read again: the learner, and every run the proctor heard of meanwhile. */
  private decisionSettled(command: Exclude<Command, { readonly type: "record-answer" }>, verdict: CommandVerdict): void {
    const run = commandRun(command);
    if (run !== undefined) this.unconfirmed.add(run);
    if (verdict.kind === "accepted") {
      for (const event of verdict.events) {
        if (event.type !== "run-voided") continue;
        if (event.run === run) this.voided(event.run);
        else {
          this.outbox.discard(event.run);
          this.dispatch({ type: "run-voided", run: event.run });
        }
      }
    } else if (command.type === "start-run" || (command.type === "open-task" && verdict.kind === "rejected" && verdict.rejection === "quiz-revised")) {
      this.unconfirmed.delete(command.run);
      this.voided(command.run);
    } else if (verdict.kind === "refused") this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: verdict.detail } });
    else if ((command.type !== "submit-run" || verdict.rejection !== "run-closed") && (command.type !== "open-task" || verdict.rejection !== "already-opened")) this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: verdict.rejection } });
    if (this.ahead()) return;
    void this.refreshLearner();
    for (const confirmed of this.unconfirmed) void this.loadRun(confirmed);
    this.unconfirmed.clear();
  }

  private settled(command: Command, verdict: CommandVerdict): void {
    if (command.type === "record-answer") this.answerSettled(command.run, verdict);
    else this.decisionSettled(command, verdict);
  }

  /** 📬️ What the proctor decided about an answer. Without a deputy, the hints of a run that hints come with the
   * proctor's view of it: it is read again once none of its answers waits any more. An answer refused because its
   * task's time was up or its clock never started is told, and the run is read again, so the device shows what the
   * proctor recorded — at once, whatever the deputy decided about other runs, or, while the proctor's view of this run
   * lacks a decision of the deputy, once that decision is delivered. */
  private answerSettled(run: Id, verdict: CommandVerdict): void {
    if (verdict.kind === "accepted") {
      const view = this.state.runs[run];
      if (this.deputy === undefined && view?.status === "open" && challengeRules(view.sheet.challenge).hints && this.outbox.queued(run).length === 0) void this.loadRun(run);
      return;
    }
    if (verdict.kind === "refused") return this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: verdict.detail } });
    if (verdict.rejection === "quiz-revised") return this.voided(run);
    if (verdict.rejection === "run-closed" || verdict.rejection === "unknown-run") return void this.loadRun(run);
    this.dispatch({ type: "notice-raised", notice: { kind: "rejection", rejection: verdict.rejection } });
    if (verdict.rejection !== "time-up" && verdict.rejection !== "task-unopened") return;
    if (this.behind(run)) this.unconfirmed.add(run);
    else void this.loadRun(run);
  }

  private voided(run: Id): void {
    this.outbox.discard(run);
    this.dispatch({ type: "run-voided", run });
    this.dispatch({ type: "notice-raised", notice: { kind: "voided" } });
    void this.refreshLearner();
  }

  private projected<T>(call: (signal: AbortSignal) => Promise<T>, signal: AbortSignal): Promise<T> {
    const until = this.now() + PROJECTION_GRACE_MS;
    return retryTransient(
      async () => {
        try {
          return await call(signal);
        } catch (error) {
          if (isNotFound(error) && this.now() < until) throw new ProctorUnavailable("not projected yet");
          throw error;
        }
      },
      this.timing,
      signal,
    );
  }

  private async background<T>(call: (signal: AbortSignal) => Promise<T>): Promise<T | undefined> {
    const signal = this.lifetime.signal;
    try {
      return await this.projected(call, signal);
    } catch (error) {
      if (!signal.aborted) this.dispatch({ type: "notice-raised", notice: { kind: "refused", detail: describe(error) } });
      return undefined;
    }
  }

  private connectivity(online: boolean): void {
    this.online = online;
    if (online) this.outbox.wake();
    this.notify();
  }

  private dispatch(event: QuizClientEvent, persist = true): void {
    const before = this.state;
    this.state = evolveQuizState(before, event);
    if (this.state === before) return;
    if (persist) persistQuizState(this.store, before, this.state);
    this.notify();
  }

  private notify(): void {
    this.snapshot = this.freeze();
    for (const listener of [...this.listeners]) listener();
  }

  private freeze(): QuizSnapshot {
    const status = this.outbox.status();
    const reachability = this.proctor.reachability();
    const contract = reachability === "incompatible" ? this.proctor.contract() : undefined;
    return { state: this.state, connection: { reachability, online: this.online, pending: status.pending, activity: status.activity, deputy: this.deputy !== undefined, ...(contract === undefined ? {} : { contract }) } };
  }
}
//#endregion 🎮️Session

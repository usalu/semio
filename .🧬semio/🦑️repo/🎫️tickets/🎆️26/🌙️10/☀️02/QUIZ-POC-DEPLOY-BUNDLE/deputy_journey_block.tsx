
const MATERIAL: QuizMaterial = { catalog: CATALOG, quizzes: [QUIZ] };

/** 🫡️ A started session that holds the site's material, so its deputy decides while the proctor is away. */
function deputisedOn(proctor: FakeProctor, area: StorageArea): QuizSession {
  const session = new QuizSession({ proctor: new ProctorClient(() => proctor.transport, CATALOG.id), store: localStore(area, CATALOG.id), deputy: new Deputy(MATERIAL), timing: TIMING });
  session.start();
  return session;
}

/** 💯️ The answer that scores `task` of a sheet of the household quiz 1, from the quiz's own solutions. */
function perfectAnswer(task: SheetTask): Answer {
  const source = QUIZ.tasks.find((candidate) => candidate.id === task.id);
  if (task.kind === "classification" && source?.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(task.items.map((item) => [item.id, source.items.find((known) => known.id === item.id)!.category])) };
  if (task.kind === "sorting" && source?.kind === "sorting") {
    const value = (id: string): number => source.items.find((known) => known.id === id)!.value;
    return { kind: "sorting", order: task.items.map((item) => item.id).sort((left, right) => value(left) - value(right)) };
  }
  if (task.kind === "matching" && source?.kind === "matching") {
    const card = (dimension: (typeof task.dimensions)[number], id: string): number => dimension.cards.indexOf(source.items.find((known) => known.id === id)!.values[dimension.id]!);
    return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(task.items.map((item) => [item.id, card(dimension, item.id)]))])) };
  }
  throw new Error(`the household quiz has no ${task.kind} task ${task.id}`);
}

/** ▶️ Starts a run of the household quiz in `session` and answers every task perfectly. */
async function playedRun(session: QuizSession): Promise<string> {
  expect(await session.startRun(QUIZ.id, new AbortController().signal)).toBeUndefined();
  const { step, runs } = session.getSnapshot().state;
  if (step.screen !== "run") throw new Error("the run did not open");
  for (const task of runs[step.run]!.sheet.tasks) session.answer(step.run, task.id, perfectAnswer(task));
  return step.run;
}

const WHOLE_RUN = ["start-run", "record-answer", "record-answer", "record-answer", "submit-run"];

describe("🫡️ the deputy while the proctor is away", () => {
  it("registers, plays and scores a first visit on the device, keeps it over a reload and hands it to the proctor once it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const signal = new AbortController().signal;
    const session = deputisedOn(proctor, origin.tab());
    expect(session.getSnapshot().state.catalog).toEqual(proctor.view);
    expect(session.getSnapshot().state.step).toEqual({ screen: "introduction" });
    session.readIntroduction();

    expect(await session.identify({ kind: "pseudonym", handle: " Grace   Hopper " }, signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!;
    expect(learner.identity).toEqual({ kind: "pseudonym", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.learnerView).toMatchObject({ learner: learner.id, identity: learner.identity, runs: [], badges: [], total: 0 });

    const run = await playedRun(session);
    const phases: string[] = [];
    expect(await session.submit(run, signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases).toEqual(["submitting", "results"]);
    const played = session.getSnapshot().state;
    expect(played.step).toEqual({ screen: "results", run });
    expect(played.runs[run]).toMatchObject({ learner: learner.id, status: "submitted", result: { quiz: QUIZ.id, score: 1 } });
    expect(played.learnerView).toMatchObject({ total: 100, best: { household: 1 }, runs: [{ run, status: "submitted", score: 1 }] });
    expect(played.learnerView?.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(played.awards[run]).toEqual(["all-done", "flawless"]);
    expect(played.notice).toBeUndefined();
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    await waitFor(() => expect(session.getSnapshot().connection).toMatchObject({ reachability: "unreachable", deputy: true, pending: 6 }));

    await session.refreshLeaderboard();
    expect(shownLeaderboard(session.getSnapshot().state)).toMatchObject({ local: true, board: { learners: 1, rows: [{ rank: 1, tag: learnerTag(learner.id), total: 100, badges: ["all-done", "flawless"] }] } });
    expect(await session.startRun(QUIZ.id, signal)).toBeUndefined();
    const again = session.getSnapshot().state.step;
    expect(again.screen === "run" && again.run !== run).toBe(true);
    session.stop();

    const reloaded = deputisedOn(proctor, origin.tab());
    expect(reloaded.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(reloaded.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(reloaded.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN, "start-run"]);
    proctor.away = false;
    reloaded.reconnect();
    await waitFor(() => expect(reloaded.outbox.queued()).toEqual([]), { timeout: 10_000 });
    const known = proctor.runOf(run)!;
    expect(known).toMatchObject({ learner: learner.id, identity: learner.identity });
    expect(known.runs.map((candidate) => candidate.status)).toEqual(["submitted", "open"]);
    expect(known.runs[0]?.result?.score).toBe(1);
    expect(known.badges.map((award) => award.badge)).toEqual(["all-done", "flawless"]);
    expect(new Set(proctor.envelopes.map((envelope) => envelope.commandId)).size).toBe(7);
    await waitFor(() => expect(reloaded.getSnapshot().state.runs[run]?.submittedAt).toBe(known.runs[0]?.submittedAt));
    await waitFor(() => expect(reloaded.getSnapshot().state.learnerView?.badges.map((award) => award.at)).toEqual(known.badges.map((award) => award.at)));
    await reloaded.refreshLeaderboard();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.local).toBeUndefined();
    expect(shownLeaderboard(reloaded.getSnapshot().state)?.board.rows.map((row) => row.tag)).toEqual([learnerTag(learner.id)]);
    expect(reloaded.getSnapshot().state.notice).toBeUndefined();
    expect(reloaded.getSnapshot().connection).toMatchObject({ reachability: "reachable", pending: 0 });
    reloaded.stop();
  });

  it("leaves every decision to the proctor while it answers", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(session);
    expect(session.outbox.queued().every((command) => command.type === "record-answer")).toBe(true);
    const phases: string[] = [];
    expect(await session.submit(run, new AbortController().signal, (phase) => phases.push(phase.phase))).toBeUndefined();
    expect(phases[0]).toBe("saving");
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["quiz.identify-learner", ...WHOLE_RUN.map((type) => `quiz.${type}`)]);
    expect(session.outbox.queued()).toEqual([]);
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", submittedAt: proctor.runOf(run)?.runs[0]?.submittedAt });
    session.stop();
  });

  it("decides a submission on the device when the proctor goes away in the middle of a run", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    const { run, tasks } = await startedRun(session);
    session.answer(run, tasks[0]!.id, perfectAnswer(tasks[0]!));
    await session.outbox.settled(run);
    proctor.away = true;
    for (const task of tasks.slice(1)) session.answer(run, task.id, perfectAnswer(task));
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["record-answer", "record-answer", "submit-run"]);
    expect(proctor.runOf(run)?.runs[0]?.status).toBe("open");
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }));
    await waitFor(() => expect(session.getSnapshot().state.runs[run]?.submittedAt).toBe(proctor.runOf(run)?.runs[0]?.submittedAt));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("does not keep a learner waiting for a proctor that says nothing: the deputy decides once the patience is spent", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    let silent = false;
    const transport: HttpTransport = { send: (request) => (silent ? new Promise<never>((_, reject) => setTimeout(() => reject(new ProctorUnavailable("the request timed out")), 400)) : proctor.transport.send(request)) };
    const session = new QuizSession({ proctor: new ProctorClient(() => transport, CATALOG.id), store: localStore(memoryStorageOrigin().tab(), CATALOG.id), deputy: new Deputy(MATERIAL), patienceMs: 40, timing: TIMING });
    session.start();
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    silent = true;
    const began = Date.now();
    expect(await session.identify({ kind: "pseudonym", handle: "Patient Learner" }, new AbortController().signal)).toBeUndefined();
    expect(Date.now() - began).toBeLessThan(350);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ status: "submitted", result: { score: 1 } });
    expect(session.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    expect(proctor.envelopes).toEqual([]);
    silent = false;
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => expect(session.outbox.queued()).toEqual([]));
    expect(session.getSnapshot().state.notice).toBeUndefined();
    session.stop();
  });

  it("continues as the holder when the proctor already knows the pseudonym the device registered by itself", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const holder = "e".repeat(32);
    const earlier = "f".repeat(32);
    proctor.seed({ type: "identify-learner", id: newId(), learner: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    proctor.seed({ type: "start-run", id: newId(), learner: holder, run: earlier, quiz: QUIZ.id });
    for (const task of runView(proctor.runOf(earlier)!, earlier, proctor.quizzes)!.sheet.tasks) proctor.seed({ type: "record-answer", id: newId(), learner: holder, run: earlier, task: task.id, answer: sloppyAnswer(task) });
    proctor.seed({ type: "submit-run", id: newId(), learner: holder, run: earlier });
    proctor.away = true;
    const area = memoryStorageOrigin().tab();
    const session = deputisedOn(proctor, area);
    expect(await session.identify({ kind: "pseudonym", handle: "grace hopper" }, new AbortController().signal)).toBeUndefined();
    const provisional = session.getSnapshot().state.learner!.id;
    expect(provisional).not.toBe(holder);
    const run = await playedRun(session);
    expect(await session.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.learner).toEqual({ id: holder, identity: { kind: "pseudonym", handle: "Grace Hopper" } });
    expect(session.getSnapshot().state.notice).toEqual({ kind: "recalled", handle: "Grace Hopper" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "results", run });
    expect(localStore(area, CATALOG.id).read("learner")).toMatchObject({ id: holder });
    expect(proctor.runOf(run)?.learner).toBe(holder);
    expect(proctor.runOf(run)?.runs.map((candidate) => [candidate.run, candidate.status])).toEqual([
      [earlier, "submitted"],
      [run, "submitted"],
    ]);
    expect(proctor.envelopes.some((envelope) => envelope.target.id === provisional)).toBe(false);
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.runs.map((summary) => summary.run)).toEqual([run, earlier]));
    expect(session.getSnapshot().state.runs[run]).toMatchObject({ learner: holder, status: "submitted", result: { score: 1 } });
    session.stop();
  });

  it("voids a run of the device that the proctor would not start, and says so", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    const session = deputisedOn(proctor, memoryStorageOrigin().tab());
    await waitFor(() => expect(session.getSnapshot().connection.reachability).toBe("reachable"));
    expect(await session.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const learner = session.getSnapshot().state.learner!.id;
    await waitFor(() => expect(session.getSnapshot().state.learnerView?.learner).toBe(learner));
    proctor.away = true;
    const elsewhere = "9".repeat(32);
    proctor.seed({ type: "start-run", id: newId(), learner, run: elsewhere, quiz: QUIZ.id });
    const run = await playedRun(session);
    expect(session.outbox.queued(run).map((command) => command.type)).toEqual(WHOLE_RUN.slice(0, 4));
    proctor.away = false;
    session.reconnect();
    await waitFor(() => expect(session.outbox.queued()).toEqual([]), { timeout: 10_000 });
    expect(session.getSnapshot().state.runs[run]?.status).toBe("voided");
    expect(session.getSnapshot().state.notice).toEqual({ kind: "voided" });
    expect(session.getSnapshot().state.step).toEqual({ screen: "home" });
    expect(proctor.runOf(run)).toBeUndefined();
    expect(proctor.envelopes.filter((envelope) => envelope.kind === "quiz.record-answer")).toEqual([]);
    await waitFor(() => expect(openRunOf(session.getSnapshot().state, QUIZ.id)).toBe(elsewhere));
    session.stop();
  });

  it("keeps the queue of a run the deputy submitted when another tab adopts it, and both tabs deliver it once", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const origin = memoryStorageOrigin();
    const left = deputisedOn(proctor, origin.tab());
    const right = deputisedOn(proctor, origin.tab());
    expect(await left.identify({ kind: "anonymous" }, new AbortController().signal)).toBeUndefined();
    const run = await playedRun(left);
    expect(await left.submit(run, new AbortController().signal, () => undefined)).toBeUndefined();
    await waitFor(() => expect(right.getSnapshot().state.runs[run]?.status).toBe("submitted"));
    expect(right.getSnapshot().state.learner?.id).toBe(left.getSnapshot().state.learner?.id);
    for (const tab of [left, right]) expect(tab.outbox.queued().map((command) => command.type)).toEqual(["identify-learner", ...WHOLE_RUN]);
    proctor.away = false;
    left.reconnect();
    right.reconnect();
    await waitFor(() => expect(proctor.runOf(run)?.runs[0]).toMatchObject({ status: "submitted", result: { score: 1 } }), { timeout: 10_000 });
    await waitFor(() => {
      for (const tab of [left, right]) expect(tab.outbox.queued()).toEqual([]);
    });
    expect(proctor.runOf(run)?.runs).toHaveLength(1);
    for (const tab of [left, right]) expect(tab.getSnapshot().state.notice).toBeUndefined();
    left.stop();
    right.stop();
  });

  it("shows the catalog at once, says that everything is saved on the device and marks the leaderboard as this device's own", { timeout: 30_000 }, async () => {
    const proctor = new FakeProctor();
    proctor.away = true;
    const user = userEvent.setup();
    render(app(proctor, memoryStorageOrigin().tab(), MATERIAL));
    await screen.findByRole("heading", { level: 1, name: "Willkommen im Testkatalog" });
    expect((await screen.findAllByText("Quiz-Server nicht erreichbar – alles wird auf diesem Gerät gespeichert")).map((element) => element.closest("[role=status]") !== null)).toEqual([false, true]);
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    await screen.findByRole("heading", { level: 1, name: "Wie möchtest du erscheinen?" });
    await user.click(screen.getByRole("button", { name: "Weiter" }));
    const quizCard = await screen.findByRole("region", { name: "Haushaltsphysik" });
    await screen.findByText("Quiz-Server nicht erreichbar – auf diesem Gerät gespeichert, noch zu senden: 1");
    await waitFor(() => expect(document.querySelector("[data-board-local]")?.textContent).toBe("Nur dieses Gerät – die anderen erscheinen, sobald der Quiz-Server antwortet"));
    await user.click(within(quizCard).getByRole("button", { name: "Quiz starten" }));
    await screen.findByRole("heading", { level: 1, name: "Haushaltsphysik" });
    expect(screen.queryByRole("alert")).toBeNull();
    proctor.away = false;
    await screen.findByText("Alle Antworten gespeichert", {}, { timeout: 10_000 });
    expect(proctor.envelopes.map((envelope) => envelope.kind)).toEqual(["quiz.identify-learner", "quiz.start-run"]);
    await waitFor(() => expect(document.querySelector("[data-board-local]")).toBeNull());
  });
});

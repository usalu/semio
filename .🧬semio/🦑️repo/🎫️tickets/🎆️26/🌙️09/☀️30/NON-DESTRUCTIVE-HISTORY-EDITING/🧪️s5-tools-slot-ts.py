"""🟦️ Wave B TypeScript twin edit (design §22.10): host facts, the framework-owned persisted gesture and the per-window
ledger in `🟦️.ts`, plus their laws in `🧪️tests/🧪️gesture-drive-law/🟦️.ts`. Usage: python3 ts-edit.py <staged 🛠️tool-machine dir>"""

import pathlib
import sys

root = pathlib.Path(sys.argv[1])


def edit(relative, edits):
    path = root / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit(f"{relative}: anchor matched {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")
    print(f"edited {relative}")


TWIN = '''
/** 📡️ The host facts that may end a window's open gesture: the window lost focus or its pointer capture, its utility switched, it is closing, a history edit froze the document, a remote edit moved the base. */
export const GESTURE_HOST_EVENTS = ["blur", "captureLost", "utilityChanged", "retiring", "timeTravelFrozen", "baseMoved"] as const;
export type GestureHostEvent = (typeof GESTURE_HOST_EVENTS)[number];

/** 🧯️ The reason a host fact ends an open gesture with (law table `hostEvents`); `undefined` keeps the gesture: a moved base ends only one pinned to a base revision. */
export function gestureHostAbortReason(event: GestureHostEvent, baseBound: boolean): ToolAbortReason | undefined {
  switch (event) {
    case "blur":
      return "blur";
    case "captureLost":
      return "captureLost";
    case "utilityChanged":
    case "retiring":
      return "retired";
    case "timeTravelFrozen":
      return "frozen";
    case "baseMoved":
      return baseBound ? "baseMoved" : undefined;
  }
}

/** 💾️ One window's open gesture between dispatches — the framework-owned persisted form of a streamed statechart tool (twin of Rust `GestureState<M>`, `$defs/GestureState`): ephemeral, local-only, never history. */
export type GestureState<M> = { readonly states: readonly string[]; readonly verb: string; readonly authoringSeed: string; readonly baseRevision: string; readonly transaction: TransactionRef; readonly entries: ReadonlyArray<readonly [string, M]>; readonly context: unknown };

/** 🪦️ A gesture a host fact ended with zero trace. */
export type GestureEnd = { readonly window: string; readonly reason: ToolAbortReason };

/** 🧾️ What one ledger dispatch did: the transaction it committed (publish it as ONE edit), or the refusal that left the ledger exactly as it was. */
export type GestureLedgerResult<M> = { readonly ok: true; readonly committed: { readonly transaction: TransactionRef; readonly mutations: M[] } | undefined } | { readonly ok: false; readonly refusal: ToolRefusal };

/** 🗂️ Every window's open gesture, at most one per window: the ONE framework-owned window slot of a persisted gesture tool (twin of Rust `GestureLedger<M>`, law `slots`). Dispatches drive a window's slot through `driveGesture`; host facts end it with zero trace. */
export class GestureLedger<G, Tick, M> {
  readonly #kind: GestureToolKind<G, Tick, M>;
  readonly #windows = new Map<string, G>();

  constructor(kind: GestureToolKind<G, Tick, M>) {
    this.#kind = kind;
  }

  isEmpty(): boolean {
    return this.#windows.size === 0;
  }

  open(window: string): G | undefined {
    return this.#windows.get(window);
  }

  /** 🪟️ The windows holding an open gesture, in window id order. */
  windows(): string[] {
    return [...this.#windows.keys()].sort();
  }

  /** ✍️ Keeps the gesture a dispatch driven against a copy of the slot decided (`undefined` clears it). */
  settle(window: string, next: G | undefined): void {
    if (next === undefined) this.#windows.delete(window);
    else this.#windows.set(window, next);
  }

  /** 📨️ Drives `window`'s tool through ONE dispatch against its slot; the slot follows the drive, a refused dispatch leaves it as it was. */
  drive(window: string, verb: string, phase: GesturePhase, tick: Tick | undefined, authoringSeed: string, baseRevision: string): GestureLedgerResult<M> {
    const result = driveGesture(this.#kind, this.#windows.get(window), verb, phase, tick, authoringSeed, baseRevision);
    if (!result.ok) return result;
    const { committed, next } = result.drive;
    if (next.kind !== "unchanged") this.settle(window, next.kind === "persist" ? next.gesture : undefined);
    return { ok: true, committed };
  }

  /** 🧯️ Host cancel of `window`'s open gesture: zero trace. */
  abort(window: string, reason: ToolAbortReason): GestureEnd | undefined {
    return this.#windows.delete(window) ? { window, reason } : undefined;
  }

  /** 📡️ A host fact of `window`: its open gesture ends with the fact's reason, or stays (`gestureHostAbortReason`). */
  hostEvent(window: string, event: GestureHostEvent): GestureEnd | undefined {
    const gesture = this.#windows.get(window);
    const reason = gesture === undefined ? undefined : gestureHostAbortReason(event, this.#kind.baseRevision(gesture) !== "");
    return reason === undefined ? undefined : this.abort(window, reason);
  }

  /** 🌐️ A host fact of every window (a history edit freezing the document), in window id order. */
  hostEventAll(event: GestureHostEvent): GestureEnd[] {
    return this.windows().flatMap((window) => this.hostEvent(window, event) ?? []);
  }

  /** 🪦️ Host cancel (`retired`) of the open gesture of every window `keep` refuses. */
  retainWindows(keep: (window: string) => boolean): GestureEnd[] {
    return this.windows().flatMap((window) => (keep(window) ? [] : (this.abort(window, "retired") ?? [])));
  }
}
//#endregion 🌊️Gesture'''

edit(
    "🟦️.ts",
    [
        (
            "/** 🪪️ How a streamed tool starts at rest, resumes its window's persisted gesture and tells two persisted gestures apart. */\nexport interface GestureToolKind<G, Tick, M> {\n  start(verb: string, authoringSeed: string, baseRevision: string): GestureToolResult<G, Tick, M>;\n  resume(gesture: G): GestureToolResult<G, Tick, M>;\n  same(left: G, right: G): boolean;\n}",
            "/** 🪪️ How a streamed tool starts at rest, resumes its window's persisted gesture, tells two persisted gestures apart and reads the document revision one is pinned to (empty: none). */\nexport interface GestureToolKind<G, Tick, M> {\n  start(verb: string, authoringSeed: string, baseRevision: string): GestureToolResult<G, Tick, M>;\n  resume(gesture: G): GestureToolResult<G, Tick, M>;\n  same(left: G, right: G): boolean;\n  baseRevision(gesture: G): string;\n}",
        ),
        ("  return { ok: true, drive: { committed: sent.step.kind === \"committed\" ? { transaction: sent.step.transaction, mutations: sent.step.mutations } : undefined, next } };\n}\n//#endregion 🌊️Gesture", "  return { ok: true, drive: { committed: sent.step.kind === \"committed\" ? { transaction: sent.step.transaction, mutations: sent.step.mutations } : undefined, next } };\n}\n" + TWIN),
    ],
)

LAW_TYPES = '''type Row = { readonly name: string; readonly persisted: Gesture | null; readonly dispatch: Dispatch; readonly expected: Outcome };
type HostAbort = { readonly event: T.GestureHostEvent; readonly baseBound: boolean; readonly reason: T.ToolAbortReason | null };
type SlotOutcome = { readonly committed: readonly number[] | null; readonly refused: T.ToolRefusal | null; readonly aborted: readonly T.GestureEnd[]; readonly open: Readonly<Record<string, Gesture>> };
type SlotStep = { readonly expected: SlotOutcome } & ({ readonly drive: Dispatch & { readonly window: string } } | { readonly host: { readonly window: string; readonly event: T.GestureHostEvent } } | { readonly hostAll: { readonly event: T.GestureHostEvent } } | { readonly retain: readonly string[] } | { readonly corrupt: string });
type SlotScenario = { readonly name: string; readonly steps: readonly SlotStep[] };'''

LEDGER_LAWS = '''
//#region 🗂️Ledger
/** 👣️ One slot step through the ledger (`GestureLedger`), reported in the fixture's outcome shape. */
function ledgerStep(ledger: T.GestureLedger<Gesture, number, number>, step: SlotStep): SlotOutcome {
  let committed: readonly number[] | null = null;
  let refused: T.ToolRefusal | null = null;
  let aborted: T.GestureEnd[] = [];
  if ("drive" in step) {
    const phase = T.parseGesturePhase(step.drive.phase, step.drive.reason);
    if (!phase) throw new Error(`no gesture phase: ${JSON.stringify(step.drive)}`);
    const result = ledger.drive(step.drive.window, step.drive.verb, phase, step.drive.tick ?? undefined, "seed", step.drive.base);
    if (result.ok) committed = result.committed?.mutations ?? null;
    else refused = result.refusal;
  } else if ("host" in step) {
    const ended = ledger.hostEvent(step.host.window, step.host.event);
    aborted = ended ? [ended] : [];
  } else if ("hostAll" in step) aborted = ledger.hostEventAll(step.hostAll.event);
  else if ("retain" in step) aborted = ledger.retainWindows((window) => step.retain.includes(window));
  else ledger.settle(step.corrupt, { ...ledger.open(step.corrupt)!, corrupt: true });
  return { committed, refused, aborted, open: Object.fromEntries(ledger.windows().map((window) => [window, ledger.open(window)!])) };
}

describe("gesture ledger law (design §22.10)", () => {
  test("every host fact ends an open gesture with the fixture's reason", () => {
    expect(schema.$defs.GestureHostEvent.enum).toEqual([...T.GESTURE_HOST_EVENTS]);
    expect(law.hostEvents.map((row) => `${row.event}/${row.baseBound}`).sort()).toEqual(T.GESTURE_HOST_EVENTS.flatMap((event) => [`${event}/true`, `${event}/false`]).sort());
    for (const row of law.hostEvents) expect({ ...row, reason: T.gestureHostAbortReason(row.event, row.baseBound) ?? null }).toEqual(row);
  });

  test("every slot scenario steps the ledger to its expected outcome", () => {
    for (const scenario of law.slots) {
      const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));
      scenario.steps.forEach((step, index) => expect({ name: `${scenario.name}#${index}`, ...ledgerStep(ledger, step) }).toEqual({ name: `${scenario.name}#${index}`, ...step.expected }));
    }
  });

  test("the scenarios cover every host fact on an open gesture, the kept one and a frozen multi-window edit", () => {
    const facts = new Set(law.slots.flatMap((scenario) => scenario.steps.flatMap((step) => ("host" in step && step.expected.aborted.length > 0 ? [step.host.event] : []))));
    expect([...facts].sort()).toEqual([...T.GESTURE_HOST_EVENTS].sort());
    expect(law.slots.some((scenario) => scenario.steps.some((step) => "host" in step && step.host.event === "baseMoved" && step.expected.aborted.length === 0 && Object.keys(step.expected.open).length > 0))).toBe(true);
    expect(law.slots.some((scenario) => scenario.steps.some((step) => "hostAll" in step && step.hostAll.event === "timeTravelFrozen" && step.expected.aborted.length > 1 && step.expected.aborted.every((end) => end.reason === "frozen") && Object.keys(step.expected.open).length === 0))).toBe(true);
  });

  test("the xstate slot oracle judges every window of every scenario like the ledger", () => {
    for (const scenario of law.slots) {
      const windows = new Map<string, SlotSnapshot>();
      const slot = (window: string) => windows.get(window) ?? initialTransition(slotMachine)[0];
      const ended = (window: string, before: SlotSnapshot, after: SlotSnapshot): T.GestureEnd[] => (before.context.slot && !after.context.slot && after.context.last?.aborted ? [{ window, reason: after.context.last.aborted }] : []);
      scenario.steps.forEach((step, index) => {
        let aborted: T.GestureEnd[] = [];
        const host = (window: string, event: T.GestureHostEvent) => {
          const before = slot(window);
          const after = transition(slotMachine, before, { type: "host", event })[0];
          windows.set(window, after);
          aborted = [...aborted, ...ended(window, before, after)];
        };
        if ("drive" in step) windows.set(step.drive.window, judge(slot(step.drive.window), step.drive));
        else if ("host" in step) host(step.host.window, step.host.event);
        else if ("hostAll" in step) {
          for (const window of [...windows.keys()].sort()) host(window, step.hostAll.event);
        } else if ("retain" in step) {
          for (const window of [...windows.keys()].sort().filter((window) => !step.retain.includes(window))) host(window, "retiring");
        } else windows.set(step.corrupt, transition(slotMachine, slot(step.corrupt), { type: "corrupt" })[0]);
        const open = Object.fromEntries([...windows.entries()].filter(([, snapshot]) => snapshot.context.slot).map(([window, snapshot]) => [window, snapshot.context.slot!]).sort(([left], [right]) => (left < right ? -1 : 1)));
        expect({ name: `${scenario.name}#${index}`, aborted, open }).toEqual({ name: `${scenario.name}#${index}`, aborted: [...step.expected.aborted], open: step.expected.open });
      });
    }
  });
});
//#endregion 🗂️Ledger

describe("xstate oracle (fast-check)", () => {'''

HOST_ON = '''        host: [
          { guard: ({ context, event }) => event.type === "host" && T.gestureHostAbortReason(event.event, slotOf(context).base !== "") !== undefined, target: "idle", actions: record(({ context, event }) => ({ slot: undefined, last: settled({ aborted: event.type === "host" ? (T.gestureHostAbortReason(event.event, slotOf(context).base !== "") ?? null) : null, next: "cleared" }) })) },
          { actions: stay(() => settled({})) },
        ],
'''

edit(
    "🧪️tests/🧪️gesture-drive-law/🟦️.ts",
    [
        ("type Row = { readonly name: string; readonly persisted: Gesture | null; readonly dispatch: Dispatch; readonly expected: Outcome };", LAW_TYPES),
        ("  readonly rows: readonly Row[];\n};", "  readonly rows: readonly Row[];\n  readonly hostEvents: readonly HostAbort[];\n  readonly slots: readonly SlotScenario[];\n};"),
        (
            "    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),\n",
            "    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),\n    baseRevision: (gesture) => gesture.base,\n",
        ),
        ('type SlotEvent = Dispatched | { readonly type: "corrupt" };', 'type SlotEvent = Dispatched | { readonly type: "corrupt" } | { readonly type: "host"; readonly event: T.GestureHostEvent };'),
        (
            "    idle: {\n      on: {\n        corrupt: {},\n",
            "    idle: {\n      on: {\n        corrupt: {},\n        host: { actions: stay(() => settled({})) },\n",
        ),
        (
            "    open: {\n      on: {\n        corrupt: { target: \"unrestorable\",",
            "    open: {\n      on: {\n" + HOST_ON + "        corrupt: { target: \"unrestorable\",",
        ),
        (
            "    unrestorable: {\n      on: {\n        corrupt: {},\n",
            "    unrestorable: {\n      on: {\n        corrupt: {},\n" + HOST_ON,
        ),
        ("\ndescribe(\"xstate oracle (fast-check)\", () => {", LEDGER_LAWS),
        (
            "  const operation = fc.oneof({ weight: 14, arbitrary: dispatch.map((event) => ({ type: \"dispatch\" as const, ...event })) }, { weight: 3, arbitrary: fc.constant({ type: \"corrupt\" as const }) });",
            "  const operation = fc.oneof(\n    { weight: 14, arbitrary: dispatch.map((event) => ({ type: \"dispatch\" as const, ...event })) },\n    { weight: 3, arbitrary: fc.constant({ type: \"corrupt\" as const }) },\n    { weight: 3, arbitrary: fc.constantFrom(...T.GESTURE_HOST_EVENTS).map((event) => ({ type: \"host\" as const, event })) },\n  );",
        ),
        (
            "          if (operation.type === \"corrupt\") {\n            if (persisted) persisted = { ...persisted, corrupt: true };\n            snapshot = transition(slotMachine, snapshot, operation)[0];\n          } else {",
            "          if (operation.type === \"corrupt\") {\n            if (persisted) persisted = { ...persisted, corrupt: true };\n            snapshot = transition(slotMachine, snapshot, operation)[0];\n          } else if (operation.type === \"host\") {\n            const before = slotState(persisted);\n            const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));\n            if (persisted) ledger.settle(\"w\", persisted);\n            const ended = ledger.hostEvent(\"w\", operation.event);\n            persisted = ledger.open(\"w\") ?? null;\n            snapshot = transition(slotMachine, snapshot, operation)[0];\n            expect(snapshot.context.last?.aborted ?? null).toEqual(ended?.reason ?? null);\n            reached.add(`${before}/host/${ended?.reason ?? \"-\"}`);\n          } else {",
        ),
        (
            '"unrestorable/stream/-", "unrestorable/commit/commit", "unrestorable/once/commit", "unrestorable/abort/-"]) expect(reached).toContain(path);',
            '"unrestorable/stream/-", "unrestorable/commit/commit", "unrestorable/once/commit", "unrestorable/abort/-", "open/host/frozen", "open/host/blur", "open/host/retired", "open/host/baseMoved", "idle/host/-"]) expect(reached).toContain(path);',
        ),
    ],
)

"""🪪️ Gesture press identity (coordinator decision 2026-10-05, report `📓️s4-tools-a-report.md` § S5.11): the window slot owns a
gesture's host press and the press each window last closed. One wave over the pure tool-machine crate (schema, corpus, TS twin,
Rust, laws) and the plugin runtime (`GestureSlot`, laws), re-derived from the CURRENT tree by counted anchors (fail closed).

Usage (cwd: repo root):
  python3 🧪️s5-tools-press.py          stage under 🗑️generated/s5-tools/staged/press/ (copies + press.patch)
  python3 🧪️s5-tools-press.py --land   write the wave into the tree (hold `landing`, then `serve`: it saves 🟦️.ts and corpus JSON)
                                        and keep the pre-landing copies under 🗑️generated/s5-tools/pre-press/
  python3 🧪️s5-tools-press.py --restore  put the pre-landing copies back (the train's restore command)
"""

import difflib
import json
import pathlib
import shutil
import subprocess
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
GENERATED = TICKET / "🗑️generated/s5-tools"
TM = "🧰️framework/🔨️modules/🛠️tool-machine"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
FIXTURE = "🧫️fixtures/🧫️gesture-drive-law/🔣️.json"
SCHEMA = "🧬️schema/🔣️.json"


def schema(text):
    document = json.loads(text)
    if json.dumps(document, indent=2, ensure_ascii=False) + "\n" != text:
        raise SystemExit("the schema file no longer round-trips")
    defs = document["$defs"]
    if "press" in defs["GestureState"]["properties"]:
        raise SystemExit("schema: already applied")
    press = {"type": "string", "minLength": 1}
    defs["GestureDriveGesture"]["properties"]["press"] = {"description": "The host press that opened the gesture; absent = opened by a dispatch that named none.", **press}
    state = defs["GestureState"]
    state["description"] = state["description"].replace("the verb, the admission's", "the verb, the host press that opened it (empty = none named; set by the slot, never by the tool), the admission's")
    state["required"] = ["states", "verb", "press", *[name for name in state["required"] if name not in ("states", "verb")]]
    state["properties"] = {"states": state["properties"]["states"], "verb": state["properties"]["verb"], "press": {"type": "string"}, **{name: value for name, value in state["properties"].items() if name not in ("states", "verb")}}
    outcome = defs["GestureSlotOutcome"]
    outcome["description"] = outcome["description"].replace("and every window's persisted gesture after the step.", "every window's persisted gesture after the step and the press each window last closed.")
    outcome["required"] = [*outcome["required"], "closed"]
    outcome["properties"]["closed"] = {"type": "object", "additionalProperties": press}
    drive = defs["GestureSlotStep"]["oneOf"][0]["properties"]["drive"]
    drive["description"] = "One dispatch of a streamed gesture verb in `window`, optionally naming the host `press` it belongs to."
    drive["properties"] = {"window": drive["properties"]["window"], "press": press, **{name: value for name, value in drive["properties"].items() if name != "window"}}
    law = defs["GestureDriveLawFixture"]
    law["description"] = law["description"].replace("and step sequences over the ledger.", "and step sequences over the ledger, with the host press identity the slot owns.")
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def edits(*pairs):
    def apply(text):
        for old, new, *count in pairs:
            expected = count[0] if count else 1
            if text.count(old) != expected:
                raise SystemExit(f"anchor matched {text.count(old)} times, expected {expected}: {old[:100]!r}")
            text = text.replace(old, new)
        return text

    return apply


#region 🟦️Twin
TS_PRESS = '''
/** 🪪️ What one dispatch did to its window's slot: the transaction it committed, the slot's next gesture and the press the window closed with it. */
export type GesturePressDrive<G, M> = { readonly committed: { readonly transaction: TransactionRef; readonly mutations: M[] } | undefined; readonly next: GestureNext<G>; readonly closed: string | undefined };

export type GesturePressResult<G, M> = { readonly ok: true; readonly drive: GesturePressDrive<G, M> } | { readonly ok: false; readonly refusal: ToolRefusal };

/** 🎫️ Drives one window's slot through ONE dispatch that may name its host `press` (`drive_press` in `🦀️.rs`, law `slots`). A gesture belongs to the press that opened it. A dispatch of the press the window last `closed` is dropped with zero trace; a dispatch of another press interrupts the open gesture first; a named gesture that ends — by anything — closes its press, and so does a named one-shot, commit or abort. A refused dispatch changes nothing. */
export function drivePress<G, Tick, M>(kind: GestureToolKind<G, Tick, M>, held: G | undefined, closed: string | undefined, press: string | undefined, verb: string, phase: GesturePhase, tick: Tick | undefined, authoringSeed: string, baseRevision: string): GesturePressResult<G, M> {
  const named = press === "" ? undefined : press;
  if (named !== undefined && closed === named) return { ok: true, drive: { committed: undefined, next: { kind: "unchanged" }, closed: undefined } };
  const owner = held === undefined ? undefined : kind.press(held);
  const interrupts = named !== undefined && owner !== undefined && owner !== named;
  const result = driveGesture(kind, interrupts ? undefined : held, verb, phase, tick, authoringSeed, baseRevision);
  if (!result.ok) return result;
  const { committed, next: driven, continued } = result.drive;
  const after = driven.kind === "persist" ? kind.withPress(driven.gesture, continued ? owner : named) : driven.kind === "cleared" || interrupts ? undefined : held;
  const next: GestureNext<G> = after === undefined ? { kind: held === undefined ? "unchanged" : "cleared" } : held !== undefined && kind.same(after, held) ? { kind: "unchanged" } : { kind: "persist", gesture: after };
  const ended = owner !== undefined && (after === undefined || kind.press(after) !== owner) ? owner : undefined;
  return { ok: true, drive: { committed, next, closed: named !== undefined && phase.kind !== "stream" ? named : ended } };
}
'''

TWIN = edits(
    (
        "  same(left: G, right: G): boolean;\n  baseRevision(gesture: G): string;\n}",
        "  same(left: G, right: G): boolean;\n  baseRevision(gesture: G): string;\n  press(gesture: G): string | undefined;\n  withPress(gesture: G, press: string | undefined): G;\n}",
    ),
    (
        "/** 🪪️ How a streamed tool starts at rest, resumes its window's persisted gesture, tells two persisted gestures apart and reads the document revision one is pinned to (empty: none). */",
        "/** 🪪️ How a streamed tool starts at rest, resumes its window's persisted gesture, tells two persisted gestures apart, reads the document revision one is pinned to (empty: none) and reads or stamps the host press that owns one (the slot stamps it, never the tool). */",
    ),
    (
        "/** 📮️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit) and the window's next persisted gesture. */\nexport type GestureDrive<G, M> = { readonly committed: { readonly transaction: TransactionRef; readonly mutations: M[] } | undefined; readonly next: GestureNext<G> };",
        "/** 📮️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit), the window's next persisted gesture, and whether the dispatch `continued` the persisted gesture (resumed it, neither dropped nor interrupted). */\nexport type GestureDrive<G, M> = { readonly committed: { readonly transaction: TransactionRef; readonly mutations: M[] } | undefined; readonly next: GestureNext<G>; readonly continued: boolean };",
    ),
    ("  const dropped: GestureDriveResult<G, M> = { ok: true, drive: { committed: undefined, next: rest } };", "  const dropped: GestureDriveResult<G, M> = { ok: true, drive: { committed: undefined, next: rest, continued: false } };"),
    (
        '  return { ok: true, drive: { committed: sent.step.kind === "committed" ? { transaction: sent.step.transaction, mutations: sent.step.mutations } : undefined, next } };\n}\n',
        '  return { ok: true, drive: { committed: sent.step.kind === "committed" ? { transaction: sent.step.transaction, mutations: sent.step.mutations } : undefined, next, continued: resumed !== undefined && interrupted === undefined } };\n}\n' + TS_PRESS,
    ),
    (
        "export type GestureState<M> = { readonly states: readonly string[]; readonly verb: string; readonly authoringSeed: string;",
        "export type GestureState<M> = { readonly states: readonly string[]; readonly verb: string; readonly press: string; readonly authoringSeed: string;",
    ),
    (
        "/** 🗂️ Every window's open gesture, at most one per window: the ONE framework-owned window slot of a persisted gesture tool (twin of Rust `GestureLedger<M>`, law `slots`). Dispatches drive a window's slot through `driveGesture`; host facts end it with zero trace. */",
        "/** 🗂️ Every window's open gesture, at most one per window, and the press each window last closed: the ONE framework-owned window slot of a persisted gesture tool (twin of Rust `GestureLedger<M>`, law `slots`). Dispatches drive a window's slot through `drivePress`; host facts end it with zero trace and close its press. */",
    ),
    ("  readonly #windows = new Map<string, G>();\n\n  constructor(kind: GestureToolKind<G, Tick, M>) {", "  readonly #windows = new Map<string, G>();\n  readonly #closed = new Map<string, string>();\n\n  constructor(kind: GestureToolKind<G, Tick, M>) {"),
    (
        "  /** ✍️ Keeps the gesture a dispatch driven against a copy of the slot decided (`undefined` clears it). */",
        "  /** 🚪️ The press `window` last closed: its late dispatches leave zero trace. */\n  closed(window: string): string | undefined {\n    return this.#closed.get(window);\n  }\n\n  /** 📕️ Every window's last closed press, in window id order. */\n  closedPresses(): Record<string, string> {\n    return Object.fromEntries([...this.#closed.entries()].sort(([left], [right]) => (left < right ? -1 : 1)));\n  }\n\n  /** 🔏️ Records the press `window` closed. */\n  close(window: string, press: string): void {\n    this.#closed.set(window, press);\n  }\n\n  /** ✍️ Keeps the gesture a dispatch driven against a copy of the slot decided (`undefined` clears it). */",
    ),
    (
        "  /** 📨️ Drives `window`'s tool through ONE dispatch against its slot; the slot follows the drive, a refused dispatch leaves it as it was. */\n  drive(window: string, verb: string, phase: GesturePhase, tick: Tick | undefined, authoringSeed: string, baseRevision: string): GestureLedgerResult<M> {\n    const result = driveGesture(this.#kind, this.#windows.get(window), verb, phase, tick, authoringSeed, baseRevision);\n    if (!result.ok) return result;\n    const { committed, next } = result.drive;\n    if (next.kind !== \"unchanged\") this.settle(window, next.kind === \"persist\" ? next.gesture : undefined);\n    return { ok: true, committed };\n  }",
        "  /** 📨️ Drives `window`'s tool through ONE dispatch of `press` (`undefined`: the host named none) against its slot (`drivePress`); the slot and the closed press follow the drive, a refused dispatch leaves both as they were. */\n  drive(window: string, press: string | undefined, verb: string, phase: GesturePhase, tick: Tick | undefined, authoringSeed: string, baseRevision: string): GestureLedgerResult<M> {\n    const result = drivePress(this.#kind, this.#windows.get(window), this.#closed.get(window), press, verb, phase, tick, authoringSeed, baseRevision);\n    if (!result.ok) return result;\n    const { committed, next, closed } = result.drive;\n    if (next.kind !== \"unchanged\") this.settle(window, next.kind === \"persist\" ? next.gesture : undefined);\n    if (closed !== undefined) this.close(window, closed);\n    return { ok: true, committed };\n  }",
    ),
    (
        "  /** 🧯️ Host cancel of `window`'s open gesture: zero trace. */\n  abort(window: string, reason: ToolAbortReason): GestureEnd | undefined {\n    return this.#windows.delete(window) ? { window, reason } : undefined;\n  }",
        "  /** 🧯️ Host cancel of `window`'s open gesture: zero trace, and its press is closed. */\n  abort(window: string, reason: ToolAbortReason): GestureEnd | undefined {\n    const gesture = this.#windows.get(window);\n    if (gesture === undefined) return undefined;\n    this.#windows.delete(window);\n    const press = this.#kind.press(gesture);\n    if (press !== undefined) this.close(window, press);\n    return { window, reason };\n  }",
    ),
    (
        "  /** 🪦️ Host cancel (`retired`) of the open gesture of every window `keep` refuses. */\n  retainWindows(keep: (window: string) => boolean): GestureEnd[] {\n    return this.windows().flatMap((window) => (keep(window) ? [] : (this.abort(window, \"retired\") ?? [])));\n  }",
        "  /** 🪦️ Host cancel (`retired`) of the open gesture of every window `keep` refuses; their closed presses are forgotten. */\n  retainWindows(keep: (window: string) => boolean): GestureEnd[] {\n    const ended = this.windows().flatMap((window) => (keep(window) ? [] : (this.abort(window, \"retired\") ?? [])));\n    for (const window of [...this.#closed.keys()]) if (!keep(window)) this.#closed.delete(window);\n    return ended;\n  }",
    ),
)

TS_LAW_PRESS = '''
describe("gesture press identity law", () => {
  const named = law.slots.flatMap((scenario) => scenario.steps.flatMap((step, index) => ("drive" in step && step.drive.press !== undefined ? [{ scenario, step, before: index === 0 ? undefined : scenario.steps[index - 1]!.expected }] : [])));

  test("the scenarios cover a dropped late dispatch, an interrupting press, a refused one and a forgotten one", () => {
    const dropped = named.filter(({ step, before }) => before?.closed[step.drive.window] === step.drive.press);
    expect(dropped.length).toBeGreaterThan(3);
    expect(dropped.every(({ step, before }) => step.expected.committed === null && step.expected.refused === null && JSON.stringify(step.expected.open) === JSON.stringify(before!.open))).toBe(true);
    expect(dropped.some(({ step }) => step.drive.phase === "commit" && step.drive.tick !== null)).toBe(true);
    const interrupting = named.filter(({ step, before }) => before?.open[step.drive.window]?.press !== undefined && before.open[step.drive.window]!.press !== step.drive.press && before.closed[step.drive.window] !== step.drive.press);
    expect(interrupting.some(({ step, before }) => step.expected.refused === null && step.expected.closed[step.drive.window] === before!.open[step.drive.window]!.press && step.expected.open[step.drive.window]?.press === step.drive.press)).toBe(true);
    expect(interrupting.some(({ step, before }) => step.expected.refused !== null && JSON.stringify(step.expected.open) === JSON.stringify(before!.open) && JSON.stringify(step.expected.closed) === JSON.stringify(before!.closed))).toBe(true);
    expect(law.slots.some((scenario) => scenario.steps.some((step, index) => "retain" in step && index > 0 && Object.keys(scenario.steps[index - 1]!.expected.closed).length > 0 && Object.keys(step.expected.closed).length === 0))).toBe(true);
  });

  test("random named sequences keep the press invariants (fast-check)", () => {
    const phase = fc.oneof({ weight: 5, arbitrary: fc.constant<T.GesturePhase>({ kind: "stream" }) }, { weight: 2, arbitrary: fc.constant<T.GesturePhase>({ kind: "commit" }) }, { weight: 1, arbitrary: fc.constant<T.GesturePhase>({ kind: "once" }) }, { weight: 1, arbitrary: fc.constant<T.GesturePhase>({ kind: "abort", reason: "blur" }) });
    const dispatch = fc.record({ type: fc.constant("dispatch" as const), press: fc.constantFrom(undefined, "p1", "p1", "p2", "p3"), verb: fc.constantFrom("drag", "drag", "turn", law.tool.refuseStartVerb), phase, tick: fc.option(fc.integer({ min: -1, max: 9 }), { nil: undefined, freq: 5 }), base: fc.constantFrom("r1", "r1", "r1", "r2") });
    const operation = fc.oneof({ weight: 8, arbitrary: dispatch }, { weight: 1, arbitrary: fc.constantFrom(...T.GESTURE_HOST_EVENTS).map((event) => ({ type: "host" as const, event })) });
    const seen = new Set<string>();
    fc.assert(
      fc.property(fc.array(operation, { maxLength: 40 }), (operations) => {
        const ledger = new T.GestureLedger(counting({ aborted: [], minted: 0 }));
        for (const step of operations) {
          const before = { open: ledger.open("w"), closed: ledger.closed("w") };
          if (step.type === "host") {
            if (ledger.hostEvent("w", step.event) && before.open?.press !== undefined) expect(ledger.closed("w")).toBe(before.open.press);
            continue;
          }
          const result = ledger.drive("w", step.press, step.verb, step.phase, step.tick, "seed", step.base);
          const after = { open: ledger.open("w"), closed: ledger.closed("w") };
          if (!result.ok) {
            expect(after).toEqual(before);
            seen.add("refused");
          } else if (step.press !== undefined && before.closed === step.press) {
            expect({ ...after, committed: result.committed }).toEqual({ ...before, committed: undefined });
            seen.add("dropped");
          } else {
            const terminal = step.press !== undefined && step.phase.kind !== "stream";
            if (terminal) expect(after.closed).toBe(step.press);
            if (after.open?.press !== undefined) expect([before.open?.press, step.press]).toContain(after.open.press);
            if (before.open?.press !== undefined && after.open?.press !== before.open.press && !terminal) expect(after.closed).toBe(before.open.press);
            if (before.open?.press !== undefined && step.press !== undefined && before.open.press !== step.press) seen.add("interrupted");
          }
        }
      }),
      { numRuns: 400 },
    );
    for (const path of ["refused", "dropped", "interrupted"]) expect(seen).toContain(path);
  });
});

describe("xstate oracle (fast-check)", () => {'''

TS_LAW = edits(
    (
        "type Gesture = { readonly verb: string; readonly base: string; readonly ticks: readonly number[]; readonly corrupt?: true };",
        "type Gesture = { readonly verb: string; readonly base: string; readonly ticks: readonly number[]; readonly corrupt?: true; readonly press?: string };",
    ),
    (
        "readonly aborted: readonly T.GestureEnd[]; readonly open: Readonly<Record<string, Gesture>> };",
        "readonly aborted: readonly T.GestureEnd[]; readonly open: Readonly<Record<string, Gesture>>; readonly closed: Readonly<Record<string, string>> };",
    ),
    ("{ readonly drive: Dispatch & { readonly window: string } }", "{ readonly drive: Dispatch & { readonly window: string; readonly press?: string } }"),
    (
        "    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),\n    baseRevision: (gesture) => gesture.base,\n",
        "    same: (left, right) => left.verb === right.verb && left.base === right.base && left.corrupt === right.corrupt && left.press === right.press && left.ticks.length === right.ticks.length && left.ticks.every((tick, index) => tick === right.ticks[index]),\n    baseRevision: (gesture) => gesture.base,\n    press: (gesture) => gesture.press,\n    withPress: (gesture, press) => ({ verb: gesture.verb, base: gesture.base, ticks: gesture.ticks, ...(gesture.corrupt ? { corrupt: true as const } : {}), ...(press === undefined ? {} : { press }) }),\n",
    ),
    (
        '    const result = ledger.drive(step.drive.window, step.drive.verb, phase, step.drive.tick ?? undefined, "seed", step.drive.base);',
        '    const result = ledger.drive(step.drive.window, step.drive.press, step.drive.verb, phase, step.drive.tick ?? undefined, "seed", step.drive.base);',
    ),
    (
        "  return { committed, refused, aborted, open: Object.fromEntries(ledger.windows().map((window) => [window, ledger.open(window)!])) };",
        "  return { committed, refused, aborted, open: Object.fromEntries(ledger.windows().map((window) => [window, ledger.open(window)!])), closed: ledger.closedPresses() };",
    ),
    (
        '  test("the xstate slot oracle judges every window of every scenario like the ledger", () => {\n    for (const scenario of law.slots) {',
        '  test("the xstate slot oracle judges every window of every scenario without a named press like the ledger", () => {\n    for (const scenario of law.slots.filter((scenario) => scenario.steps.every((step) => !("drive" in step) || step.drive.press === undefined))) {',
    ),
    ('\ndescribe("xstate oracle (fast-check)", () => {', TS_LAW_PRESS),
)
#endregion 🟦️Twin

#region 🦀️Crate
RUST_PRESS = '''
/// 🎫️ What one dispatch did to its window's slot: the transaction it committed (publish it as ONE edit), the slot's next
/// gesture when it changed (`Some(None)` clears it) and the press the window closed with it.
pub struct PressDrive<M> {
    pub committed: Option<(TransactionRef, Vec<M>)>,
    pub next: Option<Option<GestureState<M>>>,
    pub closed: Option<String>,
}

/// 🪪️ Drives one window's slot through ONE dispatch that may name its host `press` (law `slots`): the slot owns the press
/// identity, never the tool. A gesture belongs to the press that opened it. A dispatch of the press the window last
/// `closed` is dropped with zero trace (a late release after a blur commits nothing); a dispatch of another press
/// interrupts the open gesture first; a named gesture that ends — by anything — closes its press, and so does a named
/// one-shot, commit or abort. A refused dispatch changes nothing.
#[expect(clippy::too_many_arguments, reason = "One dispatch against one slot: the slot's two halves, the press, and drive_gesture's own inputs.")]
pub fn drive_press<T: GestureTool<Gesture = GestureState<M>, Mutation = M>, M>(
    held: Option<&GestureState<M>>,
    closed: Option<&str>,
    press: Option<&str>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<PressDrive<M>, ToolRefusal> {
    let press = press.filter(|press| !press.is_empty());
    if press.is_some() && closed == press {
        return Ok(PressDrive { committed: None, next: None, closed: None });
    }
    let owner = held.map(|open| open.press.as_str()).filter(|owner| !owner.is_empty());
    let interrupts = matches!((press, owner), (Some(press), Some(owner)) if press != owner);
    let drive = drive_gesture::<T>(held.filter(|_| !interrupts), verb, phase, tick, authoring_seed, base_revision)?;
    let next = match drive.next {
        Some(Some(mut gesture)) => {
            gesture.press = if drive.continued { owner } else { press }.unwrap_or_default().to_string();
            (Some(&gesture) != held).then_some(Some(gesture))
        }
        Some(None) => Some(None),
        None => interrupts.then_some(None),
    };
    let after = match &next {
        Some(next) => next.as_ref(),
        None => held,
    };
    let ended = owner.filter(|owner| after.is_none_or(|after| after.press != *owner));
    let closed = press.filter(|_| phase != GesturePhase::Stream).or(ended).map(str::to_string);
    Ok(PressDrive { committed: drive.committed, next, closed })
}
'''

CRATE = edits(
    (
        "/// 📬️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit) and, when the window's\n/// persisted gesture changed, its next one (`Some(None)` clears it).\npub struct GestureDrive<G, M> {\n    pub committed: Option<(TransactionRef, Vec<M>)>,\n    pub next: Option<Option<G>>,\n}",
        "/// 📬️ What one gesture dispatch did: the transaction it committed (publish it as ONE edit), the window's next persisted\n/// gesture when it changed (`Some(None)` clears it), and whether the dispatch `continued` the persisted gesture (resumed\n/// it, neither dropped nor interrupted).\npub struct GestureDrive<G, M> {\n    pub committed: Option<(TransactionRef, Vec<M>)>,\n    pub next: Option<Option<G>>,\n    pub continued: bool,\n}",
    ),
    ("    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None) };", "    let dropped = || GestureDrive { committed: None, next: persisted.map(|_| None), continued: false };"),
    ("    let mut tool = match open {\n        Some(tool) => tool,\n        None => T::start(verb, authoring_seed, base_revision)?,\n    };", "    let continued = open.is_some();\n    let mut tool = match open {\n        Some(tool) => tool,\n        None => T::start(verb, authoring_seed, base_revision)?,\n    };"),
    ("    Ok(GestureDrive { committed, next })\n}", "    Ok(GestureDrive { committed, next, continued })\n}"),
    (
        "/// stable ids, the verb, the admission's authoring seed and the document revision it opened on (empty: pinned to none), the\n/// open transaction with its keyed provisional mutations, and the tool context its entries do not already say (`Null`: none).\n#[derive(Clone, Debug, PartialEq)]\npub struct GestureState<M> {\n    pub states: Vec<String>,\n    pub verb: String,\n",
        "/// stable ids, the verb, the host press that opened it (empty: none named; stamped by the slot — [`drive_press`] — never by\n/// the tool), the admission's authoring seed and the document revision it opened on (empty: pinned to none), the open\n/// transaction with its keyed provisional mutations, and the tool context its entries do not already say (`Null`: none).\n#[derive(Clone, Debug, PartialEq)]\npub struct GestureState<M> {\n    pub states: Vec<String>,\n    pub verb: String,\n    pub press: String,\n",
    ),
    ("            states: machine::persist(&snapshot).states,\n            verb: self.verb,\n            authoring_seed: self.authoring_seed,", "            states: machine::persist(&snapshot).states,\n            verb: self.verb,\n            press: String::new(),\n            authoring_seed: self.authoring_seed,"),
    ("\n/// 🗄️ Every window's open gesture, at most one per window — the ONE framework-owned window slot of a persisted", RUST_PRESS + "\n/// 🗄️ Every window's open gesture, at most one per window, and the press each window last closed — the ONE framework-owned window slot of a persisted"),
    (
        "pub struct GestureLedger<M> {\n    windows: std::collections::BTreeMap<String, GestureState<M>>,\n}\n\nimpl<M> Default for GestureLedger<M> {\n    fn default() -> Self {\n        Self { windows: std::collections::BTreeMap::new() }\n    }\n}",
        "pub struct GestureLedger<M> {\n    windows: std::collections::BTreeMap<String, GestureState<M>>,\n    closed: std::collections::BTreeMap<String, String>,\n}\n\nimpl<M> Default for GestureLedger<M> {\n    fn default() -> Self {\n        Self { windows: std::collections::BTreeMap::new(), closed: std::collections::BTreeMap::new() }\n    }\n}",
    ),
    (
        "    /// 🖋️ Keeps the gesture a dispatch decided for `window` (`None` clears the slot).",
        "    /// 🚪️ The press `window` last closed: its late dispatches leave zero trace.\n    pub fn closed(&self, window: &str) -> Option<&str> {\n        self.closed.get(window).map(String::as_str)\n    }\n\n    /// 📕️ Every window's last closed press, in window id order.\n    pub fn closed_presses(&self) -> impl Iterator<Item = (&str, &str)> {\n        self.closed.iter().map(|(window, press)| (window.as_str(), press.as_str()))\n    }\n\n    /// 🔏️ Records the press `window` closed.\n    pub fn close(&mut self, window: &str, press: String) {\n        self.closed.insert(window.to_string(), press);\n    }\n\n    /// 🖋️ Keeps the gesture a dispatch decided for `window` (`None` clears the slot).",
    ),
    (
        "    /// 🚃️ Drives `window`'s tool through ONE dispatch against its slot ([`drive_gesture`]): the slot follows the drive and\n    /// the committed transaction is answered; a refused dispatch leaves the ledger exactly as it was.\n    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(\n        &mut self,\n        window: &str,\n        verb: &str,",
        "    /// 🚃️ Drives `window`'s tool through ONE dispatch of `press` (`None`: the host named none) against its slot\n    /// ([`drive_press`]): the slot and the closed press follow the drive and the committed transaction is answered; a\n    /// refused dispatch leaves the ledger exactly as it was.\n    #[expect(clippy::too_many_arguments, reason = \"One dispatch of one window: the window, the press, and drive_gesture's own inputs.\")]\n    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(\n        &mut self,\n        window: &str,\n        press: Option<&str>,\n        verb: &str,",
    ),
    (
        "        let drive = drive_gesture::<T>(self.windows.get(window), verb, phase, tick, authoring_seed, base_revision)?;\n        if let Some(next) = drive.next {\n            self.settle(window, next);\n        }\n        Ok(drive.committed)",
        "        let drive = drive_press::<T, M>(self.windows.get(window), self.closed.get(window).map(String::as_str), press, verb, phase, tick, authoring_seed, base_revision)?;\n        if let Some(next) = drive.next {\n            self.settle(window, next);\n        }\n        if let Some(press) = drive.closed {\n            self.close(window, press);\n        }\n        Ok(drive.committed)",
    ),
    (
        "    /// 🧹️ Host cancel of `window`'s open gesture: zero trace. `Aborted(ref, reason)`, or `Idle` when none was open.\n    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {\n        self.windows.remove(window).map_or(ToolStep::Idle, |gesture| ToolStep::Aborted(gesture.transaction, reason))\n    }",
        "    /// 🧹️ Host cancel of `window`'s open gesture: zero trace, and its press is closed. `Aborted(ref, reason)`, or `Idle`\n    /// when none was open.\n    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {\n        let Some(gesture) = self.windows.remove(window) else { return ToolStep::Idle };\n        if !gesture.press.is_empty() {\n            self.closed.insert(window.to_string(), gesture.press);\n        }\n        ToolStep::Aborted(gesture.transaction, reason)\n    }",
    ),
    (
        "    /// ⚰️ Host cancel (`retired`) of the open gesture of every window `keep` refuses.\n    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<(String, ToolStep<M>)> {\n        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();\n        retired.into_iter().map(|window| (self.abort(&window, ToolAbortReason::Retired), window)).map(|(step, window)| (window, step)).collect()\n    }",
        "    /// ⚰️ Host cancel (`retired`) of the open gesture of every window `keep` refuses; their closed presses are forgotten.\n    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<(String, ToolStep<M>)> {\n        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();\n        let ended = retired.into_iter().map(|window| (self.abort(&window, ToolAbortReason::Retired), window)).map(|(step, window)| (window, step)).collect();\n        self.closed.retain(|window, _| keep(window));\n        ended\n    }",
    ),
)

CRATE_LAW = edits(
    (
        "    if gesture.states.iter().any(|state| state == \"tampered\") {\n        slot[\"corrupt\"] = json!(true);\n    }\n    slot",
        "    if gesture.states.iter().any(|state| state == \"tampered\") {\n        slot[\"corrupt\"] = json!(true);\n    }\n    if !gesture.press.is_empty() {\n        slot[\"press\"] = json!(gesture.press);\n    }\n    slot",
    ),
    (
        "        match ledger.drive::<Counting>(text(&drive[\"window\"]), text(&drive[\"verb\"]), phase,",
        "        match ledger.drive::<Counting>(text(&drive[\"window\"]), drive[\"press\"].as_str(), text(&drive[\"verb\"]), phase,",
    ),
    (
        "    json!({ \"committed\": committed, \"refused\": refused, \"aborted\": aborted, \"open\": open })",
        "    let closed: serde_json::Map<String, Value> = ledger.closed_presses().map(|(window, press)| (window.to_string(), json!(press))).collect();\n    json!({ \"committed\": committed, \"refused\": refused, \"aborted\": aborted, \"open\": open, \"closed\": closed })",
    ),
    (
        "/// history edit ends every window's gesture with zero trace, windows never share a slot, a refused tick keeps the slot, an\n/// unrestorable slot is dropped by the next dispatch.",
        "/// history edit ends every window's gesture with zero trace, windows never share a slot, a refused tick keeps the slot, an\n/// unrestorable slot is dropped by the next dispatch; and the slot owns the press identity: a dispatch of the press a window\n/// last closed is dropped with zero trace, a dispatch of another press interrupts the open gesture first.",
    ),
    ("    assert_eq!(ledger.drive::<Counting>(\"w1\", \"drag\", GesturePhase::Stream, Some(4), \"seed\", \"r1\"), Ok(None));", "    assert_eq!(ledger.drive::<Counting>(\"w1\", Some(\"p1\"), \"drag\", GesturePhase::Stream, Some(4), \"seed\", \"r1\"), Ok(None));"),
    (
        "    assert_eq!((gesture.verb.as_str(), gesture.authoring_seed.as_str(), gesture.base_revision.as_str()), (\"drag\", \"seed\", \"r1\"));",
        "    assert_eq!((gesture.verb.as_str(), gesture.press.as_str(), gesture.authoring_seed.as_str(), gesture.base_revision.as_str()), (\"drag\", \"p1\", \"seed\", \"r1\"));",
    ),
    (
        "    assert_eq!(Counting::resume(&gesture).expect(\"the gesture resumes\").persist(), Some(gesture.clone()));",
        "    assert_eq!(Counting::resume(&gesture).expect(\"the gesture resumes\").persist(), Some(GestureState { press: String::new(), ..gesture.clone() }), \"the tool persists no press: the slot stamps it\");",
    ),
    ("    assert_eq!(ledger.drive::<Counting>(\"w1\", \"drag\", GesturePhase::Stream, None, \"other\", \"r1\"), Ok(None));", "    assert_eq!(ledger.drive::<Counting>(\"w1\", Some(\"p1\"), \"drag\", GesturePhase::Stream, None, \"other\", \"r1\"), Ok(None));"),
    (
        "    assert_eq!(ledger.drive::<Counting>(\"w1\", \"drag\", GesturePhase::Commit, None, \"seed\", \"r1\"), Ok(None));\n    assert!(ledger.is_empty(), \"an unrestorable gesture is dropped with zero trace\");",
        "    assert_eq!(ledger.drive::<Counting>(\"w1\", Some(\"p1\"), \"drag\", GesturePhase::Commit, None, \"seed\", \"r1\"), Ok(None));\n    assert!(ledger.is_empty(), \"an unrestorable gesture is dropped with zero trace\");\n    assert_eq!(ledger.closed(\"w1\"), Some(\"p1\"), \"and its press is closed\");",
    ),
)
#endregion 🦀️Crate

#region 🔌️Runtime
RUNTIME_SLOT_OLD = '''struct GestureSlotState<M> {
    admitted: Option<GestureState<M>>,
    decided: Option<Option<GestureState<M>>>,
}

impl<M> GestureSlot<M> {
    /// 🕳️ The slot of a dispatch outside a live window (a preview, a view without command authority): it holds no gesture
    /// and nothing it decides is kept.
    pub fn detached() -> Self {
        Self::of(String::new(), String::new(), None)
    }

    fn of(window: String, base_revision: String, admitted: Option<GestureState<M>>) -> Self {
        Self { window, base_revision, state: std::sync::Mutex::new(GestureSlotState { admitted, decided: None }) }
    }
'''
RUNTIME_SLOT_NEW = '''struct GestureSlotState<M> {
    admitted: Option<GestureState<M>>,
    closed: Option<String>,
    decided: Option<Option<GestureState<M>>>,
    closing: Option<String>,
    driven: bool,
}

/// 📮️ What a driven slot hands the runtime at publication: the gesture it was admitted on, the gesture it decided (when it
/// changed) and the press it closed.
struct GestureSlotDecision<M> {
    admitted: Option<GestureState<M>>,
    decided: Option<Option<GestureState<M>>>,
    closing: Option<String>,
}

impl<M> GestureSlot<M> {
    /// 🕳️ The slot of a dispatch outside a live window (a preview, a view without command authority): it holds no gesture
    /// and nothing it decides is kept.
    pub fn detached() -> Self {
        Self::of(String::new(), String::new(), None, None)
    }

    fn of(window: String, base_revision: String, admitted: Option<GestureState<M>>, closed: Option<String>) -> Self {
        Self { window, base_revision, state: std::sync::Mutex::new(GestureSlotState { admitted, closed, decided: None, closing: None, driven: false }) }
    }
'''
RUNTIME_TAKE_OLD = '''    fn take(&self) -> Option<(Option<GestureState<M>>, Option<GestureState<M>>)> {
        let mut state = self.state.lock().ok()?;
        let decided = state.decided.take()?;
        Some((state.admitted.take(), decided))
    }
'''
RUNTIME_TAKE_NEW = '''    fn take(&self) -> Option<GestureSlotDecision<M>> {
        let mut state = self.state.lock().ok()?;
        std::mem::take(&mut state.driven).then(|| GestureSlotDecision { admitted: state.admitted.take(), decided: state.decided.take(), closing: state.closing.take() })
    }
'''
RUNTIME_DRIVE_OLD = '''    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(&self, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault> {
        let mut state = self.state.lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("toolGesture.slot-poisoned"), format!("the gesture slot of window {:?} is poisoned", self.window)))?;
        let persisted = match state.decided.as_ref() {
            Some(decided) => decided.as_ref(),
            None => state.admitted.as_ref(),
        };
        let drive = semio_framework_tool_machine::drive_gesture::<T>(persisted, verb, phase, tick, authoring_seed, if T::BASE_BOUND { self.base_revision.as_str() } else { "" })
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
        if let Some(decided) = drive.next {
            state.decided = Some(decided);
        }
        Ok(drive.committed)
    }'''
RUNTIME_DRIVE_NEW = '''    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(&self, press: Option<&str>, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault> {
        let mut state = self.state.lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("toolGesture.slot-poisoned"), format!("the gesture slot of window {:?} is poisoned", self.window)))?;
        let held = match state.decided.as_ref() {
            Some(decided) => decided.as_ref(),
            None => state.admitted.as_ref(),
        };
        let closed = state.closing.as_deref().or(state.closed.as_deref());
        let drive = semio_framework_tool_machine::drive_press::<T, M>(held, closed, press, verb, phase, tick, authoring_seed, if T::BASE_BOUND { self.base_revision.as_str() } else { "" })
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
        state.driven = true;
        if let Some(decided) = drive.next {
            state.decided = Some(decided);
        }
        if let Some(press) = drive.closed {
            state.closing = Some(press);
        }
        Ok(drive.committed)
    }'''

RUNTIME = edits(
    (RUNTIME_SLOT_OLD, RUNTIME_SLOT_NEW),
    (RUNTIME_TAKE_OLD, RUNTIME_TAKE_NEW),
    (RUNTIME_DRIVE_OLD, RUNTIME_DRIVE_NEW),
    (
        "    /// 🚃️ Drives the window's streamed tool `T` through ONE dispatch of `verb` ([`semio_framework_tool_machine::drive_gesture`])\n    /// against this slot — a second drive of the same dispatch continues from the first — and answers the transaction it\n    /// committed (publish it as ONE edit: [`gesture_emit`]).",
        "    /// 🚃️ Drives the window's streamed tool `T` through ONE dispatch of `verb` ([`semio_framework_tool_machine::drive_press`])\n    /// against this slot — a second drive of the same dispatch continues from the first — and answers the transaction it\n    /// committed (publish it as ONE edit: [`gesture_emit`]). `press` is the host press the dispatch names (the verb's\n    /// `gesture` argument; `None`: none): a dispatch of the press the window last closed is dropped with zero trace, a\n    /// dispatch of another press interrupts the open gesture first.",
    ),
    (
        "/// at admission, the document revision the dispatch runs on, and what the dispatch decided — kept by the runtime only when",
        "/// at admission with the press it last closed, the document revision the dispatch runs on, and what the dispatch decided — kept by the runtime only when",
    ),
    (
        "        let open = self.tool_machines.gestures.open(&window).cloned();\n        let slot = Arc::new(GestureSlot::of(window, base_revision, open));",
        "        let open = self.tool_machines.gestures.open(&window).cloned();\n        let closed = self.tool_machines.gestures.closed(&window).map(str::to_string);\n        let slot = Arc::new(GestureSlot::of(window, base_revision, open, closed));",
    ),
    (
        "        let Some((admitted, decided)) = slot.take() else { return false };\n        if publishes && !self.time_travel.freezes_local_emits() && self.tool_machines.gestures.open(slot.window()) == admitted.as_ref() {\n            self.tool_machines.gestures.settle(slot.window(), decided);\n            self.follow_tool_machines(true);\n        }\n        true",
        "        let Some(decision) = slot.take() else { return false };\n        if publishes && !self.time_travel.freezes_local_emits() && self.tool_machines.gestures.open(slot.window()) == decision.admitted.as_ref() {\n            if let Some(press) = decision.closing {\n                self.tool_machines.gestures.close(slot.window(), press);\n            }\n            if let Some(decided) = decision.decided {\n                self.tool_machines.gestures.settle(slot.window(), decided);\n                self.follow_tool_machines(true);\n            }\n        }\n        true",
    ),
)

RUNTIME_LAW_PRESS = '''
/// 🪪️ LAW (press identity): the slot owns a gesture's host press — a late release of the press a blur ended commits
/// nothing and opens nothing, although the tool would commit a release at rest; a tick of another press interrupts the open
/// gesture and takes the slot; and a dispatch the runtime dropped still logs as one that drove its slot.
#[semio_framework_async_macros::async_test]
async fn a_late_release_of_a_press_the_runtime_ended_leaves_zero_trace() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let base = app.store.content_revision_now();
    let dispatch = |app: &mut ToyApp, operation: u64, press: &str, phase: GesturePhase, value: i32| {
        let slot = app.admit_gesture_slot(operation, &base, &in_window(&actor, "pane-a"));
        let committed = slot.drive::<NudgeTool>(Some(press), "nudge", phase, Some(SetCount { value }.into()), "seed").expect("the dispatch is never refused");
        assert!(app.settle_gesture_slot(operation, true), "even a dropped dispatch drove its slot");
        committed
    };
    assert_eq!(dispatch(&mut app, 900, "p1", GesturePhase::Stream, 40), None);
    assert_eq!(app.tool_machines.gestures().open("pane-a").map(|gesture| gesture.press.as_str()), Some("p1"), "the gesture belongs to the press that opened it");
    let blur = DslValue::Object(vec![(semio_framework::HOST_EVENT_ARG_WINDOW_ID.to_string(), DslValue::String("pane-a".into())), (semio_framework::HOST_EVENT_ARG_KIND.to_string(), DslValue::String(semio_framework::HOST_EVENT_KIND_BLUR.into()))]);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&blur), &in_window(&actor, "pane-a")).await.expect("blur");
    assert_eq!(app.tool_machines.gestures().closed("pane-a"), Some("p1"), "the blur closed the press");
    assert_eq!(dispatch(&mut app, 901, "p1", GesturePhase::Commit, 41), None, "the late release commits nothing");
    assert_eq!(dispatch(&mut app, 902, "p1", GesturePhase::Stream, 42), None);
    assert!(app.tool_machines.gestures().is_empty(), "and a late tick opens nothing");
    assert_eq!(dispatch(&mut app, 903, "p2", GesturePhase::Stream, 50), None);
    assert_eq!(dispatch(&mut app, 904, "p3", GesturePhase::Stream, 60), None);
    assert_eq!(app.tool_machines.gestures().open("pane-a").map(|gesture| (gesture.press.as_str(), gesture.entries.len())), Some(("p3", 1)), "another press interrupted the open one and took the slot");
    assert_eq!(app.tool_machines.gestures().closed("pane-a"), Some("p2"));
    let (_, mutations) = dispatch(&mut app, 905, "p3", GesturePhase::Commit, 61).expect("the release of the open press commits");
    assert_eq!(mutations, vec![TestMutation::from(SetCount { value: 61 })]);
    assert!(app.tool_machines.gestures().is_empty());
    close(&mut app);
}
'''

RUNTIME_LAW = edits(
    ('slot.drive::<NudgeTool>("nudge", GesturePhase::Stream, Some(SetCount { value }.into()), "seed")', 'slot.drive::<NudgeTool>(None, "nudge", GesturePhase::Stream, Some(SetCount { value }.into()), "seed")'),
    ('faulted.drive::<NudgeTool>("nudge",', 'faulted.drive::<NudgeTool>(None, "nudge",'),
    ('first.drive::<NudgeTool>("nudge",', 'first.drive::<NudgeTool>(None, "nudge",'),
    ('second.drive::<NudgeTool>("nudge",', 'second.drive::<NudgeTool>(None, "nudge",'),
    ('release.drive::<NudgeTool>("nudge",', 'release.drive::<NudgeTool>(None, "nudge",'),
    (
        'Some(GestureState { states: vec!["streaming".to_string()], verb: self.verb, authoring_seed: self.authoring_seed, base_revision: self.base_revision, transaction, entries: vec![("leaf".to_string(), leaf)], context: DslValue::Null })',
        'Some(GestureState {\n            states: vec!["streaming".to_string()],\n            verb: self.verb,\n            press: String::new(),\n            authoring_seed: self.authoring_seed,\n            base_revision: self.base_revision,\n            transaction,\n            entries: vec![("leaf".to_string(), leaf)],\n            context: DslValue::Null,\n        })',
    ),
    (
        "//! answers no host event of its own (the toy history app). The drive itself is pinned by the tool-machine corpus",
        "//! answers no host event of its own (the toy history app); the slot owns the host press identity (a late release of an\n//! ended press leaves zero trace). The drive itself is pinned by the tool-machine corpus",
    ),
    ("\n/// 🖋️ LAW (design §22.10): the slot follows a dispatch only when it publishes", RUNTIME_LAW_PRESS + "\n/// 🖋️ LAW (design §22.10): the slot follows a dispatch only when it publishes"),
)
#endregion 🔌️Runtime

FILES = {
    f"{TM}/{SCHEMA}": schema,
    f"{TM}/🟦️.ts": TWIN,
    f"{TM}/🧪️tests/🧪️gesture-drive-law/🟦️.ts": TS_LAW,
    f"{TM}/🦀️.rs": CRATE,
    f"{TM}/🧪️tests/🧪️gesture-drive-law/🦀️.rs": CRATE_LAW,
    f"{PLUGIN}/🛠️tool-machine/🦀️.rs": RUNTIME,
    f"{PLUGIN}/🧪️tests/🧪️gesture/🦀️.rs": RUNTIME_LAW,
}


def main():
    before = GENERATED / "pre-press"
    if "--restore" in sys.argv:
        for path in [*FILES, f"{TM}/{FIXTURE}"]:
            if not (before / path).is_file():
                raise SystemExit(f"no pre-landing copy of {path}")
        for path in [*FILES, f"{TM}/{FIXTURE}"]:
            shutil.copyfile(before / path, REPO / path)
            print(f"restored {path}")
        return
    work = GENERATED / "staged/press"
    shutil.rmtree(work, ignore_errors=True)
    staged = []
    for path, apply in FILES.items():
        current = (REPO / path).read_text(encoding="utf-8")
        text = apply(current)
        (work / path).parent.mkdir(parents=True, exist_ok=True)
        (work / path).write_text(text, encoding="utf-8")
        staged.append((path, current, text))
    fixture = REPO / TM / FIXTURE
    (work / TM / FIXTURE).parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(fixture, work / TM / FIXTURE)
    subprocess.run([str(REPO / ".venv/bin/python"), str(TICKET / "🧪️s4-tools-a-gesture-drive-law.py"), "--module", str(work / TM)], check=True, cwd=REPO)
    staged.append((f"{TM}/{FIXTURE}", fixture.read_text(encoding="utf-8"), (work / TM / FIXTURE).read_text(encoding="utf-8")))
    if "--land" in sys.argv:
        for path, current, _ in staged:
            (before / path).parent.mkdir(parents=True, exist_ok=True)
            (before / path).write_text(current, encoding="utf-8")
        for path, _, text in staged:
            (REPO / path).write_text(text, encoding="utf-8")
            print(f"landed {path}")
        return
    patch = []
    for path, current, text in staged:
        patch += difflib.unified_diff(current.splitlines(keepends=True), text.splitlines(keepends=True), f"a/{path}", f"b/{path}")
    (GENERATED / "staged/press.patch").write_text("".join(patch), encoding="utf-8")
    print(f"staged press: {len(staged)} files, {sum(1 for line in patch if line.startswith(('+', '-')) and not line.startswith(('+++', '---')))} changed lines")


if __name__ == "__main__":
    main()

/** 🧫️ W3-T2-TEXT: authors the typing-law fixture by hand (expected steps, open runs, overlays, deadlines) and pins every
 * transaction id with an independent minting (first-party TS BLAKE3 + hand-written LEB128, never the implementation under test).
 * Usage: `bun 🧪️w3-t2-text-typing-law-fixture.ts <owner>/🧫️fixtures/🧫️typing-law/🔣️.json`. */
import { writeFileSync } from "node:fs";
import { blake3Hex } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts";

function varint(out: number[], value: bigint): void {
  let rest = value;
  for (;;) {
    const byte = Number(rest & 0x7fn);
    rest >>= 7n;
    if (rest === 0n) return void out.push(byte);
    out.push(byte | 0x80);
  }
}
function str(out: number[], text: string): void {
  const bytes = new TextEncoder().encode(text);
  varint(out, BigInt(bytes.length));
  out.push(...bytes);
}
function mint(actor: string, physical: number, tool: string): string {
  const out: number[] = [];
  str(out, actor);
  varint(out, 0n);
  varint(out, BigInt(physical));
  varint(out, 0n);
  str(out, tool);
  return `tx-${blake3Hex(Uint8Array.from(out)).slice(0, 16)}`;
}

const ACTOR = "actor-1";
const IDLE = 750;
const SPLICE = "demo#textSplice";
const EDIT = "demo#textEdit";
type Json = unknown;
const ref = (clock: number, tool = SPLICE) => ({ id: mint(ACTOR, clock, tool), tool });
const ins = (at: number, text: string) => ({ at, text });
const whole = (text: string) => ({ text });
const entries = (leaves: Json[]) => leaves.map((mutation, index) => ({ key: String(index), mutation }));
type Open = Record<string, { buffer: string; tool: string; deadline: number; transaction: Json; entries: Json[] }>;
const open = (window: string, buffer: string, transaction: Json, deadline: number, leaves: Json[], tool = SPLICE): Open => ({ [window]: { buffer, tool, deadline, transaction, entries: entries(leaves) } });

const edit = (window: string, clock: number, buffer: string, leaves: Json[], tool = SPLICE) => ({ window, tool, clock, input: { kind: "edit", buffer, leaves } });
const commit = (window: string, clock: number, reason: string) => ({ window, clock, commit: reason });
const idle = { kind: "idle" };
const opened = (transaction: Json) => ({ kind: "open", transaction });
const committed = (transaction: Json, mutations: Json[]) => ({ kind: "committed", transaction, mutations });
const empty = (transaction: Json) => ({ kind: "empty", transaction });
const aborted = (transaction: Json, reason: string) => ({ kind: "aborted", transaction, reason });
const row = (op: Json, outcome: Json, runs: Open, provisional: Json[]) => ({ ...(op as object), expect: { ...(outcome as object), open: runs, provisional } });

const scenarios = [
  {
    name: "one run is one transaction holding the net text and commits once idle",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "H")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1000 + IDLE, [ins(0, "H")]), [ins(0, "H")]),
      row(edit("w1", 1100, "s1", [ins(1, "i")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1100 + IDLE, [ins(0, "Hi")]), [ins(0, "Hi")]),
      row(edit("w1", 1200, "s1", [ins(2, "!")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1200 + IDLE, [ins(0, "Hi!")]), [ins(0, "Hi!")]),
      row({ lapse: 1200 + IDLE - 1 }, { lapsed: [] }, open("w1", "s1", ref(1000), 1200 + IDLE, [ins(0, "Hi!")]), [ins(0, "Hi!")]),
      row({ lapse: 1200 + IDLE }, { lapsed: [["w1", committed(ref(1000), [ins(0, "Hi!")])]] }, {}, []),
    ],
  },
  {
    name: "two runs separated by idle are two transactions",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row(edit("w1", 1750, "s1", [ins(1, "b")]), { steps: [committed(ref(1000), [ins(0, "a")]), opened(ref(1750))] }, open("w1", "s1", ref(1750), 2500, [ins(1, "b")]), [ins(1, "b")]),
      row(commit("w1", 1800, "blur"), { step: committed(ref(1750), [ins(1, "b")]) }, {}, []),
    ],
  },
  {
    name: "a caret jump splits the run",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "ab")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "ab")]), [ins(0, "ab")]),
      row(edit("w1", 1100, "s1", [ins(10, "x")]), { steps: [committed(ref(1000), [ins(0, "ab")]), opened(ref(1100))] }, open("w1", "s1", ref(1100), 1850, [ins(10, "x")]), [ins(10, "x")]),
      row(commit("w1", 1200, "selectionJump"), { step: committed(ref(1100), [ins(10, "x")]) }, {}, []),
    ],
  },
  {
    name: "a commit signal ends the run as one edit; without a run it is idle",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "q")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "q")]), [ins(0, "q")]),
      row(commit("w1", 1001, "enter"), { step: committed(ref(1000), [ins(0, "q")]) }, {}, []),
      row(commit("w1", 1002, "hidden"), { step: idle }, {}, []),
    ],
  },
  {
    name: "typing into another buffer of the window commits the open run first",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row(edit("w1", 1100, "s2", [ins(1, "b")]), { steps: [committed(ref(1000), [ins(0, "a")]), opened(ref(1100))] }, open("w1", "s2", ref(1100), 1850, [ins(1, "b")]), [ins(1, "b")]),
    ],
  },
  {
    name: "a host abort leaves zero trace; a frozen document aborts every run",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row({ window: "w1", abort: "baseMoved" }, { step: aborted(ref(1000), "baseMoved") }, {}, []),
      row(edit("w1", 1100, "s1", [ins(0, "b")]), { steps: [opened(ref(1100))] }, open("w1", "s1", ref(1100), 1850, [ins(0, "b")]), [ins(0, "b")]),
      row(edit("w2", 1101, "s9", [ins(5, "c")]), { steps: [opened(ref(1101))] }, { ...open("w1", "s1", ref(1100), 1850, [ins(0, "b")]), ...open("w2", "s9", ref(1101), 1851, [ins(5, "c")]) }, [ins(0, "b"), ins(5, "c")]),
      row({ abortAll: "frozen" }, { all: [["w1", aborted(ref(1100), "frozen")], ["w2", aborted(ref(1101), "frozen")]] }, {}, []),
      row({ window: "w1", abort: "frozen" }, { step: idle }, {}, []),
    ],
  },
  {
    name: "a run that erased everything it typed commits empty: no edit",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row(edit("w1", 1100, "s1", []), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1850, []), []),
      row(commit("w1", 1200, "blur"), { step: empty(ref(1000)) }, {}, []),
    ],
  },
  {
    name: "windows type independently; another verb commits every run",
    steps: [
      row(edit("w2", 1000, "s1", [ins(0, "x")]), { steps: [opened(ref(1000))] }, open("w2", "s1", ref(1000), 1750, [ins(0, "x")]), [ins(0, "x")]),
      row(edit("w1", 1001, "s1", [ins(3, "y")]), { steps: [opened(ref(1001))] }, { ...open("w1", "s1", ref(1001), 1751, [ins(3, "y")]), ...open("w2", "s1", ref(1000), 1750, [ins(0, "x")]) }, [ins(3, "y"), ins(0, "x")]),
      row({ commitAll: "otherVerb", clock: 1100 }, { all: [["w1", committed(ref(1001), [ins(3, "y")])], ["w2", committed(ref(1000), [ins(0, "x")])]] }, {}, []),
    ],
  },
  {
    name: "a run of another tool in the window commits before the new tool types",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row(edit("w1", 1100, "s1", [whole("query")], EDIT), { steps: [committed(ref(1000), [ins(0, "a")]), opened(ref(1100, EDIT))] }, open("w1", "s1", ref(1100, EDIT), 1850, [whole("query")], EDIT), [whole("query")]),
    ],
  },
  {
    name: "a window that left the roster commits its run like a blur",
    steps: [
      row(edit("w1", 1000, "s1", [ins(0, "a")]), { steps: [opened(ref(1000))] }, open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), [ins(0, "a")]),
      row(edit("w2", 1001, "s1", [ins(0, "b")]), { steps: [opened(ref(1001))] }, { ...open("w1", "s1", ref(1000), 1750, [ins(0, "a")]), ...open("w2", "s1", ref(1001), 1751, [ins(0, "b")]) }, [ins(0, "a"), ins(0, "b")]),
      row({ retain: ["w2"], clock: 1100 }, { all: [["w1", committed(ref(1000), [ins(0, "a")])]] }, open("w2", "s1", ref(1001), 1751, [ins(0, "b")]), [ins(0, "b")]),
    ],
  },
  {
    name: "a single buffer's run holds its final text",
    steps: [
      row(edit("w1", 1000, "q", [whole("m")], EDIT), { steps: [opened(ref(1000, EDIT))] }, open("w1", "q", ref(1000, EDIT), 1750, [whole("m")], EDIT), [whole("m")]),
      row(edit("w1", 1100, "q", [whole("ma")], EDIT), { steps: [opened(ref(1000, EDIT))] }, open("w1", "q", ref(1000, EDIT), 1850, [whole("ma")], EDIT), [whole("ma")]),
      row(edit("w1", 1200, "q", [whole("match")], EDIT), { steps: [opened(ref(1000, EDIT))] }, open("w1", "q", ref(1000, EDIT), 1950, [whole("match")], EDIT), [whole("match")]),
      row(commit("w1", 1300, "enter"), { step: committed(ref(1000, EDIT), [whole("match")]) }, {}, []),
    ],
  },
];

const fixture = {
  schema: "semio.framework.tool-machine.typing-law.v1",
  args: { buffer: "typing", commit: "typingCommit" },
  idleMs: IDLE,
  reasons: ["idle", "selectionJump", "blur", "enter", "hidden", "apply", "otherVerb"],
  chart: {
    id: "typing",
    fingerprint: "18324688487390181293",
    manifestJson: '{"id":"typing","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"typing","parent":0}],"events":["Edit","Commit"],"transitionCount":4}',
    states: ["root", "idle", "typing"],
    initial: "idle",
    transitions: [
      { from: "idle", trigger: { event: "Edit" }, guard: null, to: "typing", action: "follow" },
      { from: "typing", trigger: { afterMs: IDLE }, guard: null, to: "idle", action: "settle" },
      { from: "typing", trigger: { event: "Edit" }, guard: "sameBuffer", to: "typing", action: "follow" },
      { from: "typing", trigger: { event: "Commit" }, guard: null, to: "idle", action: "settle" },
    ],
  },
  phases: [
    { args: { typing: "surface" }, phase: { kind: "edit", buffer: "surface" } },
    { args: { typing: "surface", typingCommit: "idle" }, phase: { kind: "commit", buffer: "surface", reason: "idle" } },
    { args: { typing: "surface", typingCommit: "selectionJump" }, phase: { kind: "commit", buffer: "surface", reason: "selectionJump" } },
    { args: { typing: "surface", typingCommit: "hidden" }, phase: { kind: "commit", buffer: "surface", reason: "hidden" } },
    { args: { typing: "surface", typingCommit: "paste" }, phase: null },
    { args: { typing: "" }, phase: null },
    { args: { typingCommit: "blur" }, phase: null },
    { args: {}, phase: null },
  ],
  algebra: {
    note: "The fixture's typing algebra (both twins implement it in their tests): a leaf {at, text} inserts text at scalar `at`, a leaf {text} is a whole buffer. One insertion folds into an open one when it starts where the open one ends; a whole buffer replaces a whole buffer; an edit without leaves cancels the net; anything else splits.",
    folds: [
      { net: [ins(0, "Hi")], next: [ins(2, "!")], fold: { kind: "net", leaves: [ins(0, "Hi!")] } },
      { net: [ins(0, "Hi")], next: [ins(7, "!")], fold: { kind: "split" } },
      { net: [whole("ma")], next: [whole("mat")], fold: { kind: "net", leaves: [whole("mat")] } },
      { net: [ins(0, "a")], next: [], fold: { kind: "net", leaves: [] } },
      { net: [ins(0, "a")], next: [whole("x")], fold: { kind: "split" } },
    ],
  },
  actor: ACTOR,
  scenarios,
};

writeFileSync(process.argv[2]!, JSON.stringify(fixture, null, 2) + "\n");
console.log(`typing-law: ${scenarios.length} scenarios`);

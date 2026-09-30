/** 🧫️ W3-T2-CONTROLS: authors the scrub-law fixture by hand (expected steps, entries, overlays) and pins every transaction id with an
 * independent minting (first-party TS BLAKE3 + hand-written LEB128, never the implementation under test).
 * Usage: `bun 🧪️w3-t2-controls-scrub-law-fixture.ts <owner>/🧫️fixtures/🧫️scrub-law/🔣️.json`. */
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
const OPACITY = "demo#setOpacity";
const WIDTH = "demo#setWidth";
type Json = unknown;
const leaf = (target: string, value: number) => ({ set: "opacity", target, value });
const entries = (leaves: Json[]) => leaves.map((mutation, index) => ({ key: String(index), mutation }));
const ref = (clock: number, tool = OPACITY) => ({ id: mint(ACTOR, clock, tool), tool });

type Open = Record<string, { gesture: string; base: string; tool: string; transaction: Json; entries: Json[] }>;
const open = (window: string, gesture: string, transaction: Json, leaves: Json[], base = "r1", tool = OPACITY): Open => ({ [window]: { gesture, base, tool, transaction, entries: entries(leaves) } });

function input(window: string, clock: number, kind: "tick" | "commit", gesture: string, leaves: Json[], base = "r1", tool = OPACITY) {
  return { window, tool, base, clock, input: { kind, gesture, leaves } };
}
const idle = { kind: "idle" };
const opened = (transaction: Json) => ({ kind: "open", transaction });
const committed = (transaction: Json, mutations: Json[]) => ({ kind: "committed", transaction, mutations });
const empty = (transaction: Json) => ({ kind: "empty", transaction });
const aborted = (transaction: Json, reason: string) => ({ kind: "aborted", transaction, reason });
const step = (op: Json, outcome: Json, openScrubs: Open, provisional: Json[]) => ({ ...(op as object), expect: { ...(outcome as object), open: openScrubs, provisional } });

const a = (value: number) => leaf("a", value);
const b = (value: number) => leaf("b", value);

const scenarios = [
  {
    name: "one press is one transaction holding the net value; its late tick stays silent",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.2)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.2)]), [a(0.2)]),
      step(input("w1", 1002, "tick", "g1", [a(0.4)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.4)]), [a(0.4)]),
      step(input("w1", 1003, "tick", "g1", [a(0.6)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.6)]), [a(0.6)]),
      step(input("w1", 1004, "commit", "g1", [a(0.7)]), { step: committed(ref(1001), [a(0.7)]) }, {}, []),
      step(input("w1", 1005, "tick", "g1", [a(0.9)]), { step: idle }, {}, []),
    ],
  },
  {
    name: "a host abort leaves zero trace and closes the press",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.3)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.3)]), [a(0.3)]),
      step(input("w1", 1002, "tick", "g1", [a(0.5)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.5)]), [a(0.5)]),
      step({ window: "w1", abort: { gesture: "g1", reason: "blur" } }, { step: aborted(ref(1001), "blur") }, {}, []),
      step(input("w1", 1003, "commit", "g1", [a(0.8)]), { step: idle }, {}, []),
    ],
  },
  {
    name: "two presses are two transactions",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w1", 1002, "commit", "g1", [a(0.2)]), { step: committed(ref(1001), [a(0.2)]) }, {}, []),
      step(input("w1", 1003, "tick", "g2", [a(0.3)]), { step: opened(ref(1003)) }, open("w1", "g2", ref(1003), [a(0.3)]), [a(0.3)]),
      step(input("w1", 1004, "commit", "g2", [a(0.4)]), { step: committed(ref(1003), [a(0.4)]) }, {}, []),
    ],
  },
  {
    name: "a release without a tick is a one-shot press",
    steps: [step(input("w1", 1001, "commit", "g1", [a(0.5)]), { step: committed(ref(1001), [a(0.5)]) }, {}, [])],
  },
  {
    name: "a press whose leaves are empty publishes nothing",
    steps: [
      step(input("w1", 1001, "tick", "g1", []), { step: idle }, {}, []),
      step(input("w1", 1002, "commit", "g1", []), { step: idle }, {}, []),
      step(input("w1", 1003, "tick", "g2", [a(0.5)]), { step: opened(ref(1003)) }, open("w1", "g2", ref(1003), [a(0.5)]), [a(0.5)]),
      step(input("w1", 1004, "commit", "g2", []), { step: empty(ref(1003)) }, {}, []),
    ],
  },
  {
    name: "each tick replaces the leaves by position, shrinking and growing",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1), b(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1), b(0.1)]), [a(0.1), b(0.1)]),
      step(input("w1", 1002, "tick", "g1", [b(0.2)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [b(0.2)]), [b(0.2)]),
      step(input("w1", 1003, "tick", "g1", [b(0.3), a(0.3)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [b(0.3), a(0.3)]), [b(0.3), a(0.3)]),
      step(input("w1", 1004, "commit", "g1", [b(0.4), a(0.4)]), { step: committed(ref(1001), [b(0.4), a(0.4)]) }, {}, []),
    ],
  },
  {
    name: "another press captures the window: the open one leaves zero trace",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w1", 1002, "tick", "g2", [b(0.2)]), { step: opened(ref(1002)) }, open("w1", "g2", ref(1002), [b(0.2)]), [b(0.2)]),
      step(input("w1", 1003, "commit", "g2", [b(0.3)]), { step: committed(ref(1002), [b(0.3)]) }, {}, []),
    ],
  },
  {
    name: "a moved document reopens the press on the new revision",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)], "r1"), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)], "r1"), [a(0.1)]),
      step(input("w1", 1002, "tick", "g1", [a(0.2)], "r2"), { step: opened(ref(1002)) }, open("w1", "g1", ref(1002), [a(0.2)], "r2"), [a(0.2)]),
      step(input("w1", 1003, "commit", "g1", [a(0.3)], "r2"), { step: committed(ref(1002), [a(0.3)]) }, {}, []),
    ],
  },
  {
    name: "another tool captures the window",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w1", 1002, "tick", "g1", [b(2)], "r1", WIDTH), { step: opened(ref(1002, WIDTH)) }, open("w1", "g1", ref(1002, WIDTH), [b(2)], "r1", WIDTH), [b(2)]),
    ],
  },
  {
    name: "windows scrub independently and a freeze aborts them all",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w2", 1002, "tick", "g2", [b(0.2)]), { step: opened(ref(1002)) }, { ...open("w1", "g1", ref(1001), [a(0.1)]), ...open("w2", "g2", ref(1002), [b(0.2)]) }, [a(0.1), b(0.2)]),
      step({ abortAll: "frozen" }, { steps: [aborted(ref(1001), "frozen"), aborted(ref(1002), "frozen")] }, {}, []),
      step(input("w1", 1003, "tick", "g1", [a(0.3)]), { step: idle }, {}, []),
    ],
  },
  {
    name: "a window that left the roster retires its press",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w2", 1002, "tick", "g2", [b(0.2)]), { step: opened(ref(1002)) }, { ...open("w1", "g1", ref(1001), [a(0.1)]), ...open("w2", "g2", ref(1002), [b(0.2)]) }, [a(0.1), b(0.2)]),
      step({ retain: ["w1"] }, { steps: [aborted(ref(1002), "retired")] }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w1", 1003, "commit", "g1", [a(0.4)]), { step: committed(ref(1001), [a(0.4)]) }, {}, []),
    ],
  },
  {
    name: "a stale abort of another press keeps the open one",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step({ window: "w1", abort: { gesture: "g0", reason: "blur" } }, { step: idle }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step(input("w1", 1002, "commit", "g1", [a(0.2)]), { step: committed(ref(1001), [a(0.2)]) }, {}, []),
    ],
  },
  {
    name: "an abort input drops the open press",
    steps: [
      step(input("w1", 1001, "tick", "g1", [a(0.1)]), { step: opened(ref(1001)) }, open("w1", "g1", ref(1001), [a(0.1)]), [a(0.1)]),
      step({ window: "w1", tool: OPACITY, base: "r1", clock: 1002, input: { kind: "abort", reason: "captureLost" } }, { step: aborted(ref(1001), "captureLost") }, {}, []),
    ],
  },
];

const fixture = {
  schema: "semio.framework.tool-machine.scrub-law.v1",
  args: { gesture: "gesture", commit: "commit", abort: "abort" },
  phases: [
    { args: {}, phase: null },
    { args: { gesture: "" }, phase: null },
    { args: { commit: true }, phase: null },
    { args: { gesture: "slider:1" }, phase: { kind: "tick", gesture: "slider:1" } },
    { args: { gesture: "slider:1", commit: false }, phase: { kind: "tick", gesture: "slider:1" } },
    { args: { gesture: "slider:1", commit: true }, phase: { kind: "commit", gesture: "slider:1" } },
    { args: { gesture: "slider:1", abort: "blur" }, phase: { kind: "abort", gesture: "slider:1", reason: "blur" } },
    { args: { gesture: "slider:1", commit: true, abort: "retired" }, phase: { kind: "abort", gesture: "slider:1", reason: "retired" } },
    { args: { gesture: "slider:1", abort: "sideways" }, phase: null },
  ],
  chart: {
    id: "scrub",
    fingerprint: "16240238296638685209",
    manifestJson: '{"id":"scrub","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"scrubbing","parent":0}],"events":["Tick","Commit"],"transitionCount":4}',
    states: ["idle", "scrubbing"],
    initial: "idle",
    events: ["Tick", "Commit"],
    transitions: [
      { from: "idle", event: "Tick", guard: null, to: "scrubbing", action: "follow" },
      { from: "idle", event: "Commit", guard: null, to: "idle", action: "settle" },
      { from: "scrubbing", event: "Tick", guard: "sameGesture", to: "scrubbing", action: "follow" },
      { from: "scrubbing", event: "Commit", guard: "sameGesture", to: "idle", action: "settle" },
    ],
  },
  actor: ACTOR,
  scenarios,
};

writeFileSync(process.argv[2]!, `${JSON.stringify(fixture, null, 2)}\n`);

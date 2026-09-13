/**
 * @emoji 🧭️ The TypeScript half of the graph keyboard-navigation laws — a SECOND implementation of
 * the traversal, written from `🧫️fixtures/🧭️graph-keyboard-navigation.json` alone and never from the
 * Rust one, answering the very same walks `🔬️unit/🦀️.rs` answers through
 * `FlowFixture::keyboard_step`.
 *
 * The two halves are deliberately independent: Rust runs the shipped reducer against the shipped
 * example document (and a third law pins the fixture's graph EQUAL to that document, so neither half
 * can drift onto an invented graph); this half re-derives reading order, anchor reduction and wire
 * following from the declared nodes and wires. A traversal rule that only one implementation got
 * right fails here, and an expectation that quietly matches a bug in one implementation cannot match
 * the other.
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

type GraphNode = { readonly id: string; readonly x: number; readonly y: number };
type GraphWire = { readonly id: string; readonly from: string; readonly to: string };
type Step = "next" | "previous" | "upstream" | "downstream";
type WalkStep = { readonly step: Step; readonly expected: readonly string[] };
type NavigationFixture = {
  readonly exampleId: string;
  readonly verbs: Readonly<Record<string, string>>;
  readonly graph: { readonly nodes: readonly GraphNode[]; readonly wires: readonly GraphWire[] };
  readonly keyboardOrder: readonly string[];
  readonly anchors: readonly { readonly selection: readonly string[]; readonly anchor: string | null; readonly why: string }[];
  readonly walks: readonly { readonly name: string; readonly start: readonly string[]; readonly steps: readonly WalkStep[] }[];
};

/** 🧭️ Reading order: ascending x, then ascending y, then the document's own node order. */
function readingOrder(nodes: readonly GraphNode[]): string[] {
  return nodes
    .map((node, index) => ({ node, index }))
    .sort((left, right) => left.node.x - right.node.x || left.node.y - right.node.y || left.index - right.index)
    .map((entry) => entry.node.id);
}

/** 🧭️ A port handle steps from its owning node; a multi-selection from its first member in reading order; a stale id from nowhere. */
function anchorOf(order: readonly string[], selection: readonly string[]): string | null {
  const positions = selection.map((id) => order.indexOf(id.includes("@") ? id.slice(0, id.indexOf("@")) : id)).filter((at) => at >= 0);
  if (positions.length === 0) return null;
  return order[Math.min(...positions)]!;
}

/** 🧭️ Where one step lands, or `null` when it has nowhere to go. */
function stepTarget(fixture: NavigationFixture, order: readonly string[], selection: readonly string[], step: Step): string | null {
  if (order.length === 0) return null;
  const anchor = anchorOf(order, selection);
  if (anchor === null) return step === "next" || step === "downstream" ? (order[0] ?? null) : (order[order.length - 1] ?? null);
  const at = order.indexOf(anchor);
  const wired = (pick: (wire: GraphWire) => string, match: (wire: GraphWire) => string) =>
    fixture.graph.wires
      .filter((wire) => match(wire) === anchor)
      .map((wire) => order.indexOf(pick(wire)))
      .filter((position) => position >= 0)
      .sort((left, right) => left - right)
      .map((position) => order[position]!)[0] ?? null;
  const target =
    step === "next"
      ? order[(at + 1) % order.length]!
      : step === "previous"
        ? order[(at + order.length - 1) % order.length]!
        : step === "upstream"
          ? wired((wire) => wire.from, (wire) => wire.to)
          : wired((wire) => wire.to, (wire) => wire.from);
  return target === anchor ? null : target;
}

/** 🧭️ Answers every row of the shared fixture, returning how many assertions it carried. */
export function generation3dGraphKeyboardSelfTests(): number {
  const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("../../../../../🧫️fixtures/🧭️graph-keyboard-navigation.json", import.meta.url)), "utf8")) as NavigationFixture;
  let checks = 0;

  const order = readingOrder(fixture.graph.nodes);
  assert.deepEqual(order, [...fixture.keyboardOrder], "reading order — ascending x, then y, then document order");
  checks += 1;

  assert.equal(new Set(fixture.graph.nodes.map((node) => node.id)).size, fixture.graph.nodes.length, "no node id is declared twice");
  for (const wire of fixture.graph.wires) {
    assert(order.includes(wire.from), `${wire.id}: source ${wire.from} is a declared node`);
    assert(order.includes(wire.to), `${wire.id}: target ${wire.to} is a declared node`);
    checks += 2;
  }
  checks += 1;

  for (const row of fixture.anchors) {
    assert.equal(anchorOf(order, row.selection), row.anchor, `${JSON.stringify(row.selection)}: ${row.why}`);
    checks += 1;
  }

  for (const walk of fixture.walks) {
    let selection: readonly string[] = walk.start;
    for (const step of walk.steps) {
      const target = stepTarget(fixture, order, selection, step.step);
      if (target !== null) selection = [target];
      assert.deepEqual([...selection], [...step.expected], `${walk.name}: after ${step.step}`);
      checks += 1;
    }
  }

  for (const [step, verb] of Object.entries(fixture.verbs)) {
    assert.match(verb, /^[a-z][A-Za-z]+$/u, `${step}: the verb is a camelCase action id`);
    checks += 1;
  }

  /** 🧭️ Every node is reachable by repeating one key — the property the wrap exists for, and the one
   * a walk of fixed length cannot state. */
  let reached = new Set<string>();
  let selection: readonly string[] = [];
  for (let hop = 0; hop < order.length; hop += 1) {
    const target = stepTarget(fixture, order, selection, "next");
    assert(target !== null, "arrow-down always has somewhere to go");
    selection = [target];
    reached.add(target);
  }
  assert.equal(reached.size, order.length, "repeating arrow-down reaches every node of the graph");
  checks += 1;

  reached = new Set<string>();
  selection = [];
  for (let hop = 0; hop < order.length; hop += 1) {
    const target = stepTarget(fixture, order, selection, "previous");
    assert(target !== null, "arrow-up always has somewhere to go");
    selection = [target];
    reached.add(target);
  }
  assert.equal(reached.size, order.length, "repeating arrow-up reaches every node of the graph");
  checks += 1;

  return checks;
}

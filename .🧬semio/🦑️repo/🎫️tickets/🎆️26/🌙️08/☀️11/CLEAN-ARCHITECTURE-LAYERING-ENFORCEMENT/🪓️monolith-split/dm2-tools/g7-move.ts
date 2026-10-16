// 🚚️ Editor test harnesses: process3d, lowpoly and cad each have one owning editor crate; the sub-crate test targets that mount the harness move there.
import {log, propagateEdges} from "./lib.ts";
import {moveTests} from "./movetests.ts";
const P = "✏️s/🔌️plugins";
const sets = [
  {shared: `${P}/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️support/🦀️.rs`, owner: `${P}/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust`},
  {shared: `${P}/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️support/🦀️.rs`, owner: `${P}/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust`},
  {shared: `${P}/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️support/🦀️.rs`, owner: `${P}/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust`},
];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
for (const set of sets) {
  const result = moveTests(set);
  for (const m of result.moved) log(`  moved ${m}`);
  edges.push(...result.edges);
}
const locks = propagateEdges(edges, []);
log(`locks changed: ${locks.changed.length}; skipped: ${locks.skipped.length}`);

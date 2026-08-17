// 🔗️ W1-D parity reconciliation (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
// 📓️w1-d-report.md): the SAME fixture as the Rust twin
// (💻️os/🔌️plugin/🖥️host/🦀️component.rs, region 🔖️IoRouterW1d,
// `io_router_route_is_deterministic_across_load_order` /
// `io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops` /
// `io_router_route_respects_max_hops` / `io_router_run_io_reentrancy_guard_predicate`) — two mock
// plugins (`stdio` owns one Exact hop, `gif` owns a Canonical migration hop AND a competing Lossy
// direct shortcut) — run here through the TS `IoEntryGraph`/`ioRun`. Both sides must resolve
// `binary@raw/* -> gif@89a/*` to the identical 2-hop Canonical route regardless of registration
// order, and the reentrancy guard must refuse the same hops. Run once, ad hoc:
//   bun run 🧪️w1d-io-router-parity.ts
import { IoEntryGraph, ioRun, type ArtifactDialect, type IoEntryDescriptor, type IoEntryGraphPlugin } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts";

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) {
    failures += 1;
    console.error(`[FAIL] ${label}: got ${a}, expected ${e}`);
  } else {
    console.log(`[ok] ${label}`);
  }
}
async function checkThrows(label: string, fn: () => unknown): Promise<void> {
  try {
    await fn();
    failures += 1;
    console.error(`[FAIL] ${label}: expected throw, got none`);
  } catch {
    console.log(`[ok] ${label}`);
  }
}

const binaryRaw: ArtifactDialect = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" };
const gif87a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" };
const gif89a: ArtifactDialect = { artifactKind: "s.stdio.gif", standard: "89a", subset: "*" };

//#region 🧭️Fixture — same shape as the Rust twin's io_router_w1d_fixture_entries
const stdioEntries: IoEntryDescriptor[] = [{ from: binaryRaw, into: gif87a, fidelity: "Exact", sniffs: true }];
const gifEntries: IoEntryDescriptor[] = [
  { from: gif87a, into: gif89a, fidelity: "Canonical", sniffs: false },
  { from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true },
];
const forward: IoEntryGraphPlugin[] = [
  { pluginId: "stdio", entries: stdioEntries },
  { pluginId: "gif", entries: gifEntries },
];
const reversed: IoEntryGraphPlugin[] = [...forward].reverse();
//#endregion

//#region 🧭️Determinism across registration order
const graphForward = IoEntryGraph.build(forward);
const graphReversed = IoEntryGraph.build(reversed);
const routeForward = graphForward.route(binaryRaw, gif89a);
const routeReversed = graphReversed.route(binaryRaw, gif89a);
check("resolved route identical regardless of registration order", routeForward, routeReversed);
check("winning route is the 2-hop path, not the 1-hop lossy shortcut", routeForward.hops.length, 2);
//#endregion

//#region ⚖️Prefers higher minimum fidelity over fewer hops
check("route fidelity is Canonical (min of Exact,Canonical)", routeForward.fidelity, "Canonical");
check("first hop starts at the binary carrier", routeForward.hops[0]?.from, binaryRaw);
check("last hop ends at gif89a", routeForward.hops[1]?.into, gif89a);
//#endregion

//#region 🌉️max hops bound
const route1Hop = graphForward.route(binaryRaw, gif89a, 1);
check("bounded to 1 hop picks the direct lossy shortcut", route1Hop, { hops: [{ from: binaryRaw, into: gif89a, fidelity: "Lossy", sniffs: true }], fidelity: "Lossy" });
//#endregion

//#region 🔒️Reentrancy guard (ioRun refuses a self-owned hop, whole route, before running anything)
let ranHops: Array<{ pluginId: string; from: ArtifactDialect; into: ArtifactDialect }> = [];
const payload = new Uint8Array([1, 2, 3]);
const okResult = await ioRun(graphForward, "norm", binaryRaw, gif89a, payload, (pluginId, from, into, hopPayload) => {
  ranHops.push({ pluginId, from, into });
  return hopPayload;
});
check("a calling plugin owning neither hop runs both hops in order", ranHops, [
  { pluginId: "stdio", from: binaryRaw, into: gif87a },
  { pluginId: "gif", from: gif87a, into: gif89a },
]);
check("ioRun returns the final hop's payload", Array.from(okResult), Array.from(payload));

ranHops = [];
await checkThrows("ioRun refuses the WHOLE route when the caller owns the first hop (stdio)", () => ioRun(graphForward, "stdio", binaryRaw, gif89a, payload, (pluginId, from, into, hopPayload) => {
  ranHops.push({ pluginId, from, into });
  return hopPayload;
}));
check("no hop ran before the refusal (no partial execution)", ranHops, []);

await checkThrows("ioRun refuses the WHOLE route when the caller owns the second hop (gif)", () => ioRun(graphForward, "gif", binaryRaw, gif89a, payload, (_pluginId, _from, _into, hopPayload) => hopPayload));
//#endregion

if (failures > 0) {
  console.error(`\n${failures} check(s) FAILED`);
  process.exit(1);
}
console.log("\nAll checks passed");

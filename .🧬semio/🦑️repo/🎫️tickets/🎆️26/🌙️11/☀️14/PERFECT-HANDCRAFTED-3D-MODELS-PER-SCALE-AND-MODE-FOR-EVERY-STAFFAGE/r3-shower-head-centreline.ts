/** 🧶️ Evidence for the tool request of appliances/shower-head: prints the centre line the product finds for the hose loft of one scale (`centreLineOf`): cluster count, thickness, length, the cluster centres in world coordinates and the tree edges; the centres wander several millimetres off the true axis and the tree has side twigs.
 *
 * Usage: `bun r3-shower-head-centreline.ts <scale>`
 * @see ./tool-requests.md */
import { centreLineOf, clusterPoints } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts";
import { load, rigAt, queryOf, listsIn } from "./r3-lib.ts";
const { asset } = load("appliances/shower-head", "");
const scale = Number(process.argv[2]);
const rig = rigAt(asset, scale, queryOf(asset, undefined, undefined, undefined));
const sm = listsIn(asset, scale, rig.frames);
const m = sm.body.find((x) => x.id === "hose")!;
const line = centreLineOf(m, m.world, m.indices);
console.log("[DEBUG] ", line === undefined ? "none" : { clusters: line.clusters.length, edges: line.edges.length, thickness: line.thickness, length: line.length });
if (line) console.log("[DEBUG] ", clusterPoints(line, m.world, 0).map((p) => p.map((c) => c.toFixed(3)).join(",")).join(" | "), JSON.stringify(line.edges));

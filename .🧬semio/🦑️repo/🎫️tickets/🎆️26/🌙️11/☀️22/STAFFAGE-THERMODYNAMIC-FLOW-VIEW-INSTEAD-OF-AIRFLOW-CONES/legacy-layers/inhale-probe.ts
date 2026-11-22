/** 👃️ Probe: the streams of the flow field around an inhaling nose of the human transport scenario. */
import { SCENARIOS, scenario } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/💨️transport/🟦️.ts";
import { solveTransport } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🟦️.ts";

const s = scenario("human");
const bare = { ...s.asset, thermal: { ...s.asset.thermal, emitters: s.asset.thermal.emitters.filter((e) => e.kind === "breath") } };
const field = solveTransport(bare, { kind: "pose", pose: "rest", timeS: 0.8 }, SCENARIOS.environments["calm-cold"], { ...s.options, pathlines: false, flow: { obstacles: [], stepM: 0.015, domain: { min: { x: -3, y: 0, z: -3 }, max: { x: 3, y: 4, z: 3 } } } });
const nose = field.emitters.find((e) => e.emitter === "nose")!;
const d = (p: { x: number; y: number; z: number }) => Math.hypot(p.x - nose.at.x, p.y - nose.at.y, p.z - nose.at.z);
console.log(field.respiration, bare.thermal.emitters.map((e) => [e.id, e.diameterM]), field.parcels.length);
console.log(field.flow!.streams.map((l) => [l.role, d(l.seed).toFixed(3), l.points.length, d(l.points[0]!).toFixed(3), d(l.points[l.points.length - 1]!).toFixed(3)].join(" ")));

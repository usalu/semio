import { load, stateAt, boxes } from "./lib.ts";
const [id, den] = [process.argv[2]!, Number(process.argv[3])]; const { asset: a } = load(id);
const p = a.poses.find((q: any) => q.id === a.presentation.defaultPose);
console.log(JSON.stringify(Object.fromEntries(Object.entries(p.bones).filter(([, v]: any) => v.abductionDeg !== undefined).map(([k, v]: any) => [k, [v.angleDeg, v.abductionDeg]]))));
const bx = boxes(stateAt(a, den));
const rows = Object.entries(bx.front).map(([b, v]: any) => [b, v[0], v[1]] as any).sort((x: any, y: any) => Math.min(x[1], x[2]) - Math.min(y[1], y[2]));
console.log("front lateral extremes", rows.slice(0, 4).map((r: any) => `${r[0]} ${r[1].toFixed(2)}..${r[2].toFixed(2)}`).join(" | "), "||", rows.sort((x: any, y: any) => y[2] - x[2]).slice(0, 4).map((r: any) => `${r[0]} ${r[1].toFixed(2)}..${r[2].toFixed(2)}`).join(" | "));
const bp = Object.entries(bx.plan).map(([b, v]: any) => [b, v[2], v[3]] as any);
console.log("plan lateral", bp.sort((x: any, y: any) => x[1] - y[1]).slice(0, 3).map((r: any) => `${r[0]} ${r[1].toFixed(2)}`).join(" | "), "||", bp.sort((x: any, y: any) => y[2] - x[2]).slice(0, 3).map((r: any) => `${r[0]} ${r[2].toFixed(2)}`).join(" | "));

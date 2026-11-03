import { load, bands } from "./lib.ts";
const [id, den, view] = [process.argv[2]!, process.argv[3]!, process.argv[4]!];
const { asset } = load(id);
for (const s of asset.representations[den].geometry[view]) console.log(s.id, s.bone, s.role, s.kind, s.region ?? s.regions ?? "-", s.z, s.followTip ? "ft" : "", s.kind === "ellipse" ? `${s.cx},${s.cy} r${s.rx},${s.ry} rot${s.rotationDeg}` : s.kind === "capsule" ? `${s.x0},${s.y0}-${s.x1},${s.y1}` : (s.d ?? JSON.stringify(s.points ?? s)).slice(0, 90));

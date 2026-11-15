/** 🐦️ Fauna: 2D root repair of the herring gull against real dimensions (Olsen and Larsson 2003; BWP: exposed culmen 50-62 mm, bill depth at gonys 20-26 mm, bill width 15 mm, tarsus 60-72 mm and about 8 mm thick, webbed foot about 3 mm thick, tail 190-210 mm and about 40 mm deep at the base): thin bill, tibiotarsus, tarsus and foot at 1:50 in all three views, the bill at 1:100, and a thin rectrix wedge instead of the 125 mm deep tail at 1:50, 1:100 and 1:200.
 * Usage: bun fauna-gull2d.ts > 🗑️generated/fauna/gull2d-patch.json ; python3 fauna-shape.py <asset> <patch> --apply */
import { openCtx } from "./kit-lib.ts";
const A = "animals/herring-gull";
const out: any[] = [];
const r5 = (x: number) => Math.round(x * 1e5) / 1e5;
type P = [number, number];
const frameOf = (c: any, bone: string) => c.state.frames[bone] as { o: number[]; u: number[]; v: number[] };
const toLocal = (c: any, bone: string, pts: P[]): P[] => { const f = frameOf(c, bone); return pts.map(([x, y]) => [r5((x - f.o[0]!) * f.u[0]! + (y - f.o[1]!) * f.u[1]!), r5((x - f.o[0]!) * f.v[0]! + (y - f.o[1]!) * f.v[1]!)]); };
const capsule = (x0: number, x1: number, r0: number, r1: number, n = 4): P[] => {
  const top: P[] = []; const bot: P[] = [];
  const m = 5;
  for (let i = 0; i <= m; i++) { const x = x0 + ((x1 - x0) * i) / m; const r = r0 + ((r1 - r0) * i) / m; top.push([x, r]); bot.push([x, -r]); }
  const cap = (x: number, r: number, s: number): P[] => Array.from({ length: n }, (_, i) => { const a = (Math.PI / 2) * ((i + 1) / (n + 1)); return [x + s * r * Math.sin(a), r * Math.cos(a)] as P; });
  const tipEnd = cap(x1, r1, 1);
  const tipStart = cap(x0, r0, -1);
  return [...top, ...tipEnd.map(([x, y]) => [x, y] as P), ...tipEnd.reverse().map(([x, y]) => [x, -y] as P), ...bot.reverse(), ...tipStart.map(([x, y]) => [x, -y] as P), ...tipStart.reverse().map(([x, y]) => [x, y] as P)].map(([x, y]) => [r5(x), r5(y)] as P);
};
const FOOT: P[] = [[-0.012, -0.0055], [-0.008, -0.0095], [0, -0.01], [0.05, -0.01], [0.082, -0.0085], [0.088, -0.0055], [0.082, -0.002], [0.05, -0.001], [0, -0.001], [-0.008, -0.002]];
const BILL: P[] = [[0.166, 0.470], [0.185, 0.4718], [0.205, 0.4685], [0.225, 0.4630], [0.240, 0.4565], [0.2468, 0.4500], [0.2487, 0.4435], [0.2468, 0.4400], [0.2400, 0.4402], [0.2320, 0.4375], [0.2220, 0.4375], [0.2050, 0.4385], [0.1850, 0.4400], [0.1660, 0.4415]];
const TOPX: P[] = [[-0.150, 0.2465], [-0.197, 0.2500], [-0.222, 0.2440], [-0.266, 0.2330], [-0.307, 0.2200], [-0.324, 0.2070], [-0.3275, 0.1950]];
const topAt = (x: number): number => { for (let i = 1; i < TOPX.length; i++) if (x >= TOPX[i]![0]) { const a = TOPX[i - 1]!; const b = TOPX[i]!; return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]); } return TOPX[TOPX.length - 1]![1]; };
const tailPoly = (tipH: number): P[] => {
  const xs = [-0.150, -0.18, -0.21, -0.24, -0.27, -0.30, -0.3275];
  const h = (x: number) => 0.055 + (tipH - 0.055) * Math.pow((-0.150 - x) / 0.1775, 0.9);
  const top = xs.map((x) => [x, topAt(x)] as P);
  const bot = [...xs].reverse().map((x) => [x, topAt(x) - h(x)] as P);
  return [...top, ...bot];
};

for (const sc of [50, 100, 200]) {
  const c = openCtx(A, sc);
  const s = String(sc);
  out.push({ scale: s, view: "elevation", id: "tail", set: { points: toLocal(c, "tail", tailPoly(sc === 200 ? 0.03 : sc === 100 ? 0.018 : 0.016)) } });
  if (sc === 200) continue;
  out.push({ scale: s, view: "elevation", id: "bill", set: { points: toLocal(c, "head", BILL) } });
  out.push({ scale: s, view: "front", id: "bill", set: { rx: 0.0085, ry: 0.013 } });
  out.push({ scale: s, view: "plan", id: "bill", set: { r0: 0.0085, r1: 0.0035 } });
  if (sc !== 50) continue;
  for (const sd of ["near", "far"]) {
    const k = sd === "near" ? -1 : 1;
    out.push({ scale: s, view: "elevation", id: `${sd}-shank`, set: { points: capsule(-0.008, 0.108, 0.0075, 0.0052) } });
    out.push({ scale: s, view: "elevation", id: `${sd}-tarsus`, set: { kind: "poly", points: capsule(-0.006, 0.072, 0.0048, 0.0040), closed: true }, del: ["d"] });
    out.push({ scale: s, view: "elevation", id: `${sd}-foot`, set: { kind: "poly", points: FOOT, closed: true, filled: true }, del: ["d"] });
    out.push({ scale: s, view: "front", id: `${sd}-foot`, set: { points: FOOT } });
    const RAW: Record<string, P[]> = { "near-shank": [[-0.053, -0.102], [-0.049, -0.1959], [-0.031, -0.1959], [-0.031, -0.102]], "near-tarsus": [[-0.049, -0.1959], [-0.048, -0.2548], [-0.032, -0.2548], [-0.031, -0.1959]], "far-shank": [[0.031, -0.102], [0.031, -0.1959], [0.049, -0.1959], [0.053, -0.102]], "far-tarsus": [[0.031, -0.1959], [0.032, -0.2548], [0.048, -0.2548], [0.049, -0.1959]] };
    for (const [id, f] of [["shank", 0.36], ["tarsus", 0.5]] as const) out.push({ scale: s, view: "front", id: `${sd}-${id}`, set: { points: RAW[`${sd}-${id}`]!.map(([x, y]) => [r5(k * 0.04 + (x - k * 0.04) * f), y] as P) } });
    out.push({ scale: s, view: "plan", id: `${sd}-shank`, set: { r0: 0.0075, r1: 0.0058 } });
    out.push({ scale: s, view: "plan", id: `${sd}-tarsus`, set: { r0: 0.0048, r1: 0.0042 } });
  }
}
console.log(JSON.stringify(out));

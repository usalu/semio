/** 📐️ Prints the shapes reaching furthest along an axis in one view of a pose: bun shapex.ts <file> <pose> <scale> <view> <minX|maxX|minY|maxY> [n] */
import { evaluate, loadAsset } from "./lib.ts";
const [file, poseId, scale, view, which, n] = process.argv.slice(2);
const asset = await loadAsset(file!);
const s = evaluate(asset, { kind: "pose", pose: poseId! }, { kind: "scale", scale: { denominator: Number(scale) } } as any);
const rows = (s.shapes as any)[view!].map((e: any) => { const pts = e.rings.flat(); const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y); return { id: e.shape.id, bone: e.shape.bone, minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) }; });
rows.sort((a: any, b: any) => (which!.startsWith("max") ? b[which!] - a[which!] : a[which!] - b[which!]));
for (const r of rows.slice(0, Number(n ?? 6))) console.log(r.id, r.bone, r[which!].toFixed(4));

/** 📏️ World bounding box per solid at rest, sorted by the axis given. Usage: bun extent.ts <slug> <file.json> [x|y|z] */
import { readFileSync } from "node:fs";
import { meshes, loadAsset, resolveAsset } from "../volume-lib.ts";
const [slug, file, axis = "z"] = process.argv.slice(2);
const asset = loadAsset(resolveAsset(slug!).path);
const raw = JSON.parse(readFileSync(file!, "utf8"));
const vol = raw.volume ?? raw;
const k = "xyz".indexOf(axis);
const rows = meshes(asset, vol.solids, 2).map((m) => { const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9]; for (let i = 0; i < m.world.length; i += 3) for (let a = 0; a < 3; a++) { mn[a] = Math.min(mn[a]!, m.world[i + a]!); mx[a] = Math.max(mx[a]!, m.world[i + a]!); } return { id: m.id, mn, mx }; });
rows.sort((a, b) => Math.max(Math.abs(b.mn[k]!), Math.abs(b.mx[k]!)) - Math.max(Math.abs(a.mn[k]!), Math.abs(a.mx[k]!)));
for (const r of rows.slice(0, 12)) console.log(r.id.padEnd(20), r.mn.map((v) => v.toFixed(3)).join(" "), "|", r.mx.map((v) => v.toFixed(3)).join(" "));

import { readdirSync, readFileSync, statSync } from "node:fs"; import { brotliDecompressSync } from "node:zlib";
const dir = process.env.STORE, tuple = p => { try { const s = statSync(p); return s.size + ":" + s.mtimeMs + ":" + s.ctimeMs + ":" + s.ino + ":" + s.mode; } catch (e) { return "!" + (e?.code ?? "E"); } };
for (const name of readdirSync(dir + "/s")) {
  const w = JSON.parse(brotliDecompressSync(readFileSync(dir + "/s/" + name)).toString()).witness;
  const moved = []; w.paths.forEach((p, i) => { const now = tuple(p); if (now !== w.tuples[i]) moved.push(p.slice(-110) + " " + w.tuples[i] + " -> " + now); });
  console.log(name.slice(0, 10), "paths", w.paths.length, "live", w.live.length, "moved", moved.length, moved.slice(0, 3));
}

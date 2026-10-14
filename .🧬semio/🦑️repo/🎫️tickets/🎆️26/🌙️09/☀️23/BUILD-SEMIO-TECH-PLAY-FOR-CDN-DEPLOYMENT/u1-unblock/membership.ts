/** 🧭️ U1: each workspace member must resolve to exactly the workspace that lists it (package.workspace or nearest [workspace] ancestor). */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
const R = "/Users/ueli/Documents/semio";
const W = [R, R + "/🧰️framework", R + "/🌎️hub", R + "/✏️s"];
const load = (p: string): any => Bun.TOML.parse(readFileSync(p, "utf8"));
const wsOf = (d: string) => {
  const pw = load(d + "/Cargo.toml").package?.workspace;
  if (pw) return normalize(join(d, pw));
  for (let x = dirname(d); x.startsWith(R); x = dirname(x)) if (existsSync(x + "/Cargo.toml") && load(x + "/Cargo.toml").workspace) return x;
};
const owners = new Map<string, string[]>();
for (const w of W) for (const pat of load(w + "/Cargo.toml").workspace.members ?? []) {
  const hits = pat.includes("*") ? [...new Bun.Glob(pat).scanSync({ cwd: w, onlyFiles: false })] : [pat];
  for (const h of hits) { const d = normalize(join(w, h)); owners.set(d, [...(owners.get(d) ?? []), w]); }
}
let bad = 0;
const rel = (p?: string) => (p ? relative(R, p) || "." : "none");
for (const [d, ws] of [...owners].sort()) {
  if (!existsSync(d + "/Cargo.toml")) { console.log("MISSING", rel(d), "listed by", ws.map(rel).join(",")); bad++; continue; }
  const real = wsOf(d);
  if (ws.length > 1 || ws[0] !== real) { console.log("MISMATCH", rel(d), "listed by", ws.map(rel).join(","), "declares", rel(real)); bad++; }
}
console.log("members", owners.size, "problems", bad);
process.exit(bad ? 1 : 0);

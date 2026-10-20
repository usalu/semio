import { dirname } from "node:path";
import { loadPackage, Package } from "./cargo.ts";
const root = "/Users/ueli/Documents/semio";
function dirOf(name: string): string {
  const r = Bun.spawnSync(["rg", "-l", "--glob", "Cargo.toml", `^name = "${name}"$`, "✏️s", "🧰️framework", "🌎️hub", "🎓️teaching", "🏢️semio-tech", "👴️leutwiler", "♻️mit-bestand"], { cwd: root });
  const f = r.stdout.toString().split("\n").filter(Boolean)[0];
  if (!f) throw new Error("no package " + name);
  return dirname(`${root}/${f}`);
}
const [a, b, mode = "normal"] = process.argv.slice(2);
const A = loadPackage(dirOf(a))!, B = loadPackage(dirOf(b))!;
const sections = mode === "all" ? ["dependencies", "dev-dependencies", "build-dependencies"] : ["dependencies", "build-dependencies"];
const seen = new Set<string>();
function path(from: Package, to: Package, trail: string[]): string[] | null {
  if (from.dir === to.dir) return trail;
  if (seen.has(from.dir)) return null;
  seen.add(from.dir);
  for (const d of from.deps) {
    if (!d.path || !sections.includes(d.section)) continue;
    const p = loadPackage(d.path);
    if (!p) continue;
    const r = path(p, to, [...trail, p.name]);
    if (r) return r;
  }
  return null;
}
const r = path(A, B, [A.name]);
console.log(r ? `${a} REACHES ${b}: ${r.join(" -> ")}` : `${a} does not reach ${b}`);

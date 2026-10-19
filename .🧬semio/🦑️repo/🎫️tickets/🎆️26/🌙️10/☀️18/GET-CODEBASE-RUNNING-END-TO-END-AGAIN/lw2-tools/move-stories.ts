import { readFileSync, writeFileSync, mkdirSync, renameSync, readdirSync, existsSync, rmdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const P = "✏️s/🔌️plugins/", D = "✏️s/🧑‍💻dev/";
const RENDERER = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
const moves: [string, string][] = [
  [P + "🏗️fem/📖️stories", D + "🏗️fem/📖️stories"],
  [P + "🧱️block/📖️stories", D + "🧱️block/📖️stories"],
  [P + "📸️remodel/📖️stories", D + "📸️remodel/📖️stories"],
  [P + "🧩️puzzle/📖️stories/🎭️2d-board", D + "🧩️puzzle/📖️stories/🎭️2d-board"],
  [P + "🧩️puzzle/🗿️artifacts/🧊️3d/📖️stories/🎭️3d-world", D + "🧩️puzzle/📖️stories/🎭️3d-world"],
  [P + "🧩️puzzle/🗿️artifacts/◻️2d/📖️stories/🎭️2d-fixtures", D + "🧩️puzzle/📖️stories/🎭️2d-fixtures"],
  [P + "🧩️puzzle/🗿️artifacts/🖐️5d/📖️stories/🎭️5d-timeline", D + "🧩️puzzle/📖️stories/🎭️5d-timeline"],
];
const listFiles = (d: string): string[] => readdirSync(join(ROOT, d), { withFileTypes: true }).flatMap(e => e.isDirectory() ? (e.name === "node_modules" ? [] : listFiles(join(d, e.name))) : [join(d, e.name)]);
const mapping = new Map<string, string>();
for (const [from, to] of moves) {
  if (!existsSync(join(ROOT, from))) { console.error("missing", from); continue; }
  for (const f of listFiles(from)) {
    const rel = relative(from, f);
    const dest = join(to, rel);
    if (existsSync(join(ROOT, dest))) throw new Error("exists " + dest);
    mapping.set(f, dest);
  }
}
const newOf = (abs: string): string | undefined => {
  const rel = relative(ROOT, abs);
  for (const [o, n] of mapping) if (o === rel) return n;
  return undefined;
};
const EXT = ["", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx"];
const resolveTarget = (abs: string): string => { for (const e of EXT) { const c = abs + e; if (existsSync(c) && !(e === "" && (readdirSyncSafe(c)))) return c; } return abs; };
function readdirSyncSafe(p: string) { try { return readdirSync(p).length >= 0; } catch { return false; } }
const SPEC = /((?:from|import)\s*\(?\s*["'])(\.{1,2}\/[^"']*)(["'])/gu;
const plan: { src: string; dest: string; text: string }[] = [];
for (const [src, dest] of mapping) {
  let text = readFileSync(join(ROOT, src), "utf8");
  if (/\.(ts|tsx)$/u.test(src)) {
    text = text.replace(SPEC, (all, a, spec: string, z) => {
      const clean = spec.split("?")[0]!, query = spec.slice(clean.length);
      const oldAbs = resolve(ROOT, dirname(src), clean);
      const rel = relative(ROOT, oldAbs);
      if (rel.startsWith("🧰️framework/") && (oldAbs === resolve(ROOT, RENDERER) || resolve(oldAbs) === resolve(ROOT, RENDERER))) return a + "@semio-tech/framework-renderer-react" + z;
      const target = resolveTarget(oldAbs);
      const moved = mapping.get(relative(ROOT, target));
      const newTarget = moved ? resolve(ROOT, moved) : target === oldAbs ? oldAbs : target;
      let out = relative(resolve(ROOT, dirname(dest)), moved ? resolve(ROOT, moved) : oldAbs);
      if (moved) { out = relative(resolve(ROOT, dirname(dest)), resolve(ROOT, moved)); const keepExt = clean.endsWith(".ts") || clean.endsWith(".tsx"); if (!keepExt) out = out.replace(/\/?🟦️\.tsx?$/u, "") || "."; }
      if (!out.startsWith(".")) out = "./" + out;
      return a + out + query + z;
    });
    text = text.replaceAll(src, dest);
  }
  plan.push({ src, dest, text });
}
for (const { src, dest, text } of plan) {
  mkdirSync(dirname(join(ROOT, dest)), { recursive: true });
  renameSync(join(ROOT, src), join(ROOT, dest));
  writeFileSync(join(ROOT, dest), text);
}
const prune = (d: string) => { try { for (const e of readdirSync(join(ROOT, d), { withFileTypes: true })) if (e.isDirectory()) prune(join(d, e.name)); if (readdirSync(join(ROOT, d)).length === 0) rmdirSync(join(ROOT, d)); } catch {} };
for (const [from] of moves) prune(from);
console.log("moved", plan.length, "files");
for (const { dest } of plan) console.log(dest);

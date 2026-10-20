import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { Glob } from "bun";
import { loadTargets, ROOT } from "./targets.ts";
const WRITE = /\b(fs::write|std::fs::write|File::create|create_dir_all|OpenOptions::new|fs::remove_file|fs::remove_dir_all|fs::rename|fs::copy)\b/;
const MANIFEST = /CARGO_MANIFEST_DIR|env!\("CARGO_MANIFEST_DIR"\)|env::current_dir|current_dir\(\)/;
export type RustHit = { file: string; line: number; text: string };
export function scanRust(dirs: string[]): Map<string, RustHit[]> {
  const out = new Map<string, RustHit[]>();
  for (const dir of dirs) {
    const hits: RustHit[] = [];
    for (const f of new Glob("**/*.rs").scanSync({ cwd: join(ROOT, dir), onlyFiles: true, followSymlinks: false })) {
      if (f.includes("/target/") || f.includes("node_modules")) continue;
      const src = readFileSync(join(ROOT, dir, f), "utf8");
      if (!MANIFEST.test(src) || !WRITE.test(src)) continue;
      const lines = src.split("\n");
      lines.forEach((l, i) => { if (WRITE.test(l) && !/\/\//.test(l.split(WRITE)[0] ?? "")) hits.push({ file: join(dir, f), line: i + 1, text: l.trim().slice(0, 160) }); });
    }
    out.set(dir, hits);
  }
  return out;
}
if (import.meta.main) {
  const dirs = new Set<string>();
  for (const t of loadTargets()) { if (!t.def || !t.root.includes("🦀️rust")) continue; dirs.add(resolve(ROOT, t.cwd, "../..").replace(ROOT + "/", "")); }
  console.log(dirs.size, "artifact dirs");
  const r = scanRust([...dirs]);
  let n = 0; for (const [d, h] of r) if (h.length) { n++; console.log("##", d.slice(-70), h.length); for (const x of h.slice(0, 3)) console.log("  ", x.file.slice(-70), x.line, x.text); }
  console.log(n, "dirs with manifest-anchored writes");
}

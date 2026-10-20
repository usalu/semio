/** 👻 Strings in the reference files that name a path existing now but not after the delta (deleted glue, moved sources) and have no replacement. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { listAll } from "../exports-stage-tools/lib.ts";
import { ROOT } from "./lib.ts";
import { deleted, delta, net } from "./refs-map.ts";
import { FILES, sites } from "./ref-sites.ts";
const all = listAll();
const created = delta.newFiles.map(f => f.path);
const post = new Set<string>([...all.filter(f => !deleted.has(f) && !net.has(f)), ...net.values(), ...created]);
const dirs = new Set<string>();
for (const f of post) { let d = f; while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); if (dirs.has(d)) break; dirs.add(d); } }
const now = new Set<string>(all);
const nowDirs = new Set<string>();
for (const f of now) { let d = f; while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); if (nowDirs.has(d)) break; nowDirs.add(d); } }
const handled = new Set(sites().map(s => s.file + "\0" + s.value));
export function orphans(): { file: string; value: string }[] {
  const out: { file: string; value: string }[] = [];
  for (const file of FILES) {
    const seen = new Set<string>();
    for (const m of readFileSync(resolve(ROOT, file), "utf8").matchAll(/"((?:[^"\\]|\\.)*)"/gu)) {
      const raw = m[1]!, v = raw.replace(/^\{workspaceRoot\}\//u, "");
      if (seen.has(raw) || !v.startsWith("🧰️framework/")) continue; seen.add(raw);
      if ((now.has(v) || nowDirs.has(v)) && !(post.has(v) || dirs.has(v)) && !handled.has(file + "\0" + raw)) out.push({ file, value: raw });
    }
  }
  return out;
}
if (import.meta.main) for (const o of orphans()) console.log(o.file.slice(-40), o.value.replace("🧰️framework/", "FW/"));

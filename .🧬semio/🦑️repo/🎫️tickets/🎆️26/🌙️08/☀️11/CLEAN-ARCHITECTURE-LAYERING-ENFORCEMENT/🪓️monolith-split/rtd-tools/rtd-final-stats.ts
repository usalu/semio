// 📈️ Final change statistics against the baseline snapshot: changed source files, alias mentions before/after in code, manifest edges removed/added. argv: base dir (snapshot root), manifest list.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync} from "node:fs";
import {join} from "node:path";
import {mask} from "./rtd-rust.ts";
const ROOT = "/Users/ueli/Documents/semio";
const base = process.argv[2]!;
const index = JSON.parse(readFileSync(join(base, "index.json"), "utf8")) as Record<string, string[]>;
let changed = 0, before = 0, after = 0, prodIdent = 0;
const perCrate: Record<string, {files: number; before: number; after: number}> = {};
for (const [crate, files] of Object.entries(index)) {
  for (const f of files) {
    if (!f.endsWith(".rs")) continue;
    const a = readFileSync(join(base, "crates", crate, f), "utf8");
    const live = join(ROOT, f);
    if (!existsSync(live)) continue;
    const b = readFileSync(live, "utf8");
    if (a === b) continue;
    changed++;
    const ca = (mask(a).match(/(?<![A-Za-z0-9_:.$])ui_wgpu\b/g) ?? []).length, cb = (mask(b).match(/(?<![A-Za-z0-9_:.$])ui_wgpu\b/g) ?? []).length;
    before += ca; after += cb;
    prodIdent += (mask(b).match(/(?<![A-Za-z0-9_:.$])semio_framework_ui_[a-z0-9_]+\s*::/g) ?? []).length;
    const e = perCrate[crate] ??= {files: 0, before: 0, after: 0};
    e.files++; e.before += ca; e.after += cb;
  }
}
console.log(JSON.stringify({changedFiles: changed, aliasBefore: before, aliasAfter: after, producerPathsInChangedFiles: prodIdent}));
console.log(JSON.stringify(perCrate));
let rem = 0, add = 0, crates = 0;
for (const m of readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean)) {
  const a = (Bun.TOML.parse(readFileSync(join(base, "manifests", m), "utf8")) as any), b = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  const keys = (t: any) => new Set<string>(["dependencies", "dev-dependencies"].flatMap(tb => Object.keys(t[tb] ?? {}).map(k => tb + ":" + k)));
  const ka = keys(a), kb = keys(b);
  const r = [...ka].filter(k => !kb.has(k)), ad = [...kb].filter(k => !ka.has(k));
  if (r.length || ad.length) crates++;
  rem += r.length; add += ad.length;
}
console.log(JSON.stringify({manifestsChanged: crates, edgesRemoved: rem, edgesAdded: add}));

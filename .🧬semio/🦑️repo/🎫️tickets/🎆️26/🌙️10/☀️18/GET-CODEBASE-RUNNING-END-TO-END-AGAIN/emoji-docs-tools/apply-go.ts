import { statSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { choose, dress, norm } from "./pick.ts";
import { LEAD } from "./docs.ts";
import { basename, dirname } from "node:path";

const root = "/Users/ueli/Documents/semio";
const out = require("node:path").resolve(process.argv[2]);
const mode = process.argv[3] ?? "dry";
const WINDOW = 15 * 60 * 1000;
const FROZEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const bin = `${out}/godocs.bin`;
const files = (readFileSync(`${out}/go-files.txt`, "utf8")).split("\n").filter(Boolean).filter((p) => !p.startsWith(FROZEN));
const proc = spawnSync(bin, { input: files.join("\n"), maxBuffer: 1 << 28, cwd: root });
const docs: any[] = JSON.parse(proc.stdout.toString());
const byFile = new Map<string, any[]>();
for (const d of docs) { const l = byFile.get(d.file) ?? []; l.push(d); byFile.set(d.file, l); }
const applied: any[] = [];
const deferred: string[] = [];
const failed: any[] = [];
const label = (p: string) => p.split("/").slice(-3).map((s) => s.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^\p{L}\p{N}]+/gu, " ")).join(" ");
for (const [p, ds] of byFile) {
  const missing = ds.filter((d) => !LEAD.test(d.text.trimStart()));
  if (!missing.length) continue;
  const abs = `${root}/${p}`;
  const st = statSync(abs);
  if (Date.now() - st.mtimeMs < WINDOW) { deferred.push(p); continue; }
  const buf = readFileSync(abs);
  const text = buf.toString("utf8");
  const used = new Set<string>();
  for (const d of ds) { const m = d.text.trimStart().match(LEAD); if (m) used.add(norm(m[0])); }
  const edits: any[] = [];
  const cur = docs.filter((d) => d.file === p);
  for (const d of missing) {
    const raw: string = d.raw;
    if (buf.subarray(d.offset, d.offset + Buffer.byteLength(raw)).toString("utf8") !== raw) { failed.push({ p, why: "offset" }); continue; }
    const name = d.decl.split(" ").slice(1).join(" ");
    const pk = choose({ name, head: d.text.trim().replace(new RegExp("^" + name + "\\b"), "").slice(0, 140), moduleDir: label(p), moduleLevel: d.decl.startsWith("package"), tag: "" }, used, new Set());
    used.add(pk.emoji);
    const em = dress(pk.emoji);
    const afterSlashes = d.offset + 2;
    const hasSpace = buf[afterSlashes] === 0x20;
    edits.push({ at: hasSpace ? afterSlashes + 1 : afterSlashes, insert: hasSpace ? em : ` ${em}`, emoji: pk.emoji, line: d.line, name, head: d.text.trim().slice(0, 90), how: pk.how });
  }
  void cur;
  if (!edits.length) continue;
  let nb = buf;
  for (const e of [...edits].sort((a, b) => b.at - a.at)) nb = Buffer.concat([nb.subarray(0, e.at), Buffer.from(e.insert, "utf8"), nb.subarray(e.at)]);
  const next = nb.toString("utf8");
  const x = text.split("\n"), y = next.split("\n");
  let ok = x.length === y.length;
  for (let i = 0; ok && i < x.length; i++) if (x[i] !== y[i] && !x[i].trimStart().startsWith("//")) ok = false;
  if (ok) { const g = spawnSync("gofmt", ["-e"], { input: next, cwd: root }); if (g.status !== 0) { ok = false; } }
  if (!ok) { failed.push({ p, why: "verify" }); continue; }
  if (mode === "apply") {
    const st2 = statSync(abs);
    if (st2.mtimeMs !== st.mtimeMs) { deferred.push(p); continue; }
    writeFileSync(abs, nb);
  }
  applied.push({ p, n: edits.length, edits });
}
writeFileSync(`${out}/go-applied-${mode}.json`, JSON.stringify(applied));
console.log(mode, "go files", applied.length, "edits", applied.reduce((a, b) => a + b.n, 0), "deferred", deferred.length, "failed", failed.length);
if (failed.length) console.log(failed.slice(0, 10));

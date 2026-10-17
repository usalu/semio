// 🧲️ Tier 1 of FIXTURE-READER-DEDUPE: replaces the per-file `runtime_fixture_text` / `runtime_fixture_bytes` copies by the shared `fixture_text!` / `fixture_bytes!` macros.
// usage: bun migrate.ts [--apply] [--list=<json>] [--only=<substring>] [--hot=<minutes>] [--out=<prefix>]
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { afterModHeader } from "./lib.ts";
import { importFixtureMacros } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const F = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const G = join(F, "🗑️generated");
const ORIG = join(F, "frd-tools/orig");
const args = process.argv.slice(2);
const flag = (name: string) => args.find(a => a.startsWith(`--${name}=`))?.slice(name.length + 3);
const apply = args.includes("--apply");
const only = flag("only");
const hotMinutes = Number(flag("hot") ?? "10");
const outPrefix = flag("out") ?? "frd-migrate";
const listPath = flag("list") ?? join(G, "frd-files.json");

const excluded = (JSON.parse(readFileSync(join(F, "frd-tools/excluded.json"), "utf8")).fragments as { owner: string; fragment: string }[]);
const hdrOwned = new Set<string>();
for (const name of ["touched.txt"]) for (const line of readFileSync(join(F, "hdr-tools", name), "utf8").split("\n")) if (line.trim()) hdrOwned.add(line.trim());
for (const line of readFileSync(join(F, "hdr-tools/created.tsv"), "utf8").split("\n")) { const first = line.split("\t")[0]!.trim(); if (first) hdrOwned.add(first); }

const CANON = /(?<=\n)\n?([ \t]*)fn runtime_fixture_(text|bytes)\(relative: &str\) -> &'static (?:str|\[u8\]) \{\n\1    let path = (?:std::path::Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\.join\(relative\)|fixture_path!\(relative\));\n\1    Box::leak\(std::fs::read(?:_to_string)?\(&path\)\.unwrap_or_else\(\|error\| panic!\("fixture \{\}: \{error\}", path\.display\(\)\)\)\.into_boxed_(?:str|slice)\(\)\)\n\1\}\n?/gu;

type Outcome = { file: string; status: string; detail?: string; defs?: number; calls?: number };
const outcomes: Outcome[] = [];
mkdirSync(ORIG, { recursive: true });
const files = (JSON.parse(readFileSync(listPath, "utf8")) as string[]).filter(f => !only || f.includes(only));

for (const rel of files) {
  const abs = join(ROOT, rel);
  const skip = (status: string, detail?: string) => outcomes.push({ file: rel, status, detail });
  if (!existsSync(abs)) { skip("missing"); continue; }
  const owner = excluded.find(x => rel.includes(x.fragment));
  if (owner) { skip("excluded", owner.owner); continue; }
  if (hdrOwned.has(rel)) { skip("excluded", "hdr"); continue; }
  const age = (Date.now() - statSync(abs).mtimeMs) / 60000;
  if (age < hotMinutes) { skip("hot", age.toFixed(1) + " min"); continue; }
  const text = readFileSync(abs, "utf8");
  if (!text.includes("runtime_fixture")) { skip("no-helper"); continue; }
  const tokens = rustTokens(text);
  const depthAt = (offset: number): { depth: number; mod: number } => {
    const stack: number[] = [];
    for (let i = 0; i < tokens.length && tokens[i]!.start < offset; i += 1) {
      if (tokens[i]!.text === "{") stack.push(i); else if (tokens[i]!.text === "}") stack.pop();
    }
    return { depth: stack.length, mod: stack[stack.length - 1] ?? -1 };
  };
  const defs: { start: number; end: number; name: string }[] = [];
  CANON.lastIndex = 0;
  for (let m = CANON.exec(text); m; m = CANON.exec(text)) defs.push({ start: m.index, end: m.index + m[0].length, name: `fixture_${m[2]}` });
  const insideDef = (offset: number) => defs.some(d => offset >= d.start && offset < d.end);
  const edits: { start: number; end: number; text: string }[] = [];
  let bad: string | null = null;
  let calls = 0;
  const fileLevelNames: string[] = [];
  for (const d of defs) {
    const at = depthAt(d.start + (text[d.start] === "\n" ? 1 : 0));
    if (at.depth === 0) { fileLevelNames.push(d.name); edits.push({ start: d.start, end: d.end, text: "" }); continue; }
    const modToken = tokens[at.mod - 2], nameToken = tokens[at.mod - 1];
    if (at.depth === 1 && modToken?.text === "mod" && nameToken?.kind === "identifier") {
      edits.push({ start: d.start, end: d.end, text: "" });
      { const place = afterModHeader(text, tokens[at.mod]!.end); edits.push({ start: place, end: place, text: `\nuse semio_framework_test_fixtures::${d.name};\n` }); }
    } else { bad = "definition inside a non-module block"; }
  }
  for (let i = 0; i < tokens.length; i += 1) {
    const t = tokens[i]!;
    if (t.kind !== "identifier" || (t.text !== "runtime_fixture_text" && t.text !== "runtime_fixture_bytes") || insideDef(t.start)) continue;
    if (tokens[i + 1]?.text !== "(" || tokens[i - 1]?.text === "fn") { bad = `non-call use of ${t.text} at ${t.start}`; continue; }
    calls += 1;
    edits.push({ start: t.start, end: t.end, text: `${t.text.slice("runtime_".length)}!` });
  }
  if (bad) { skip("manual", bad); continue; }
  if (!defs.length) { skip("dependent", `${calls} calls without a definition`); continue; }
  let out = text;
  for (const e of edits.sort((a, b) => b.start - a.start || b.end - a.end)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  out = importFixtureMacros(out, fileLevelNames);
  if (out.includes("runtime_fixture")) { skip("manual", "runtime_fixture remains"); continue; }
  if (!apply) { outcomes.push({ file: rel, status: "plan", defs: defs.length, calls }); continue; }
  const fresh = readFileSync(abs, "utf8");
  if (fresh !== text) { skip("raced"); continue; }
  const key = createHash("sha1").update(rel).digest("hex");
  if (!existsSync(join(ORIG, key + ".rs"))) writeFileSync(join(ORIG, key + ".rs"), text);
  writeFileSync(abs, out);
  outcomes.push({ file: rel, status: "applied", defs: defs.length, calls, detail: createHash("sha1").update(text).digest("hex").slice(0, 10) + ">" + createHash("sha1").update(out).digest("hex").slice(0, 10) });
}
const counts: Record<string, number> = {};
for (const o of outcomes) counts[o.status] = (counts[o.status] ?? 0) + 1;
writeFileSync(join(G, `${outPrefix}.json`), JSON.stringify(outcomes, null, 1));
const key = (status: string) => outcomes.filter(o => o.status === status).length;
console.log(JSON.stringify(counts), "defs", outcomes.reduce((s, o) => s + (o.defs ?? 0), 0), "calls", outcomes.reduce((s, o) => s + (o.calls ?? 0), 0), key("plan") + key("applied"));

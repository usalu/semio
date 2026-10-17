// 🧲️ Tiers 1b, 2 and 3 of FIXTURE-READER-DEDUPE: module-style readers, the named leaking helpers and the ad-hoc `CARGO_MANIFEST_DIR` joins.
// usage: bun migrate2.ts [--apply] [--list=<json>] [--only=<substring>] [--hot=<minutes>] [--out=<prefix>] [--tiers=1b,2,3]
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { rustTokenPairs, rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { afterModHeader, gateAt, testLikePath } from "./lib.ts";
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
const outPrefix = flag("out") ?? "frd-migrate2";
const listPath = flag("list") ?? join(G, "frd-files.json");
const tiers = new Set((flag("tiers") ?? "1b,2,3").split(","));

const excluded = (JSON.parse(readFileSync(join(F, "frd-tools/excluded.json"), "utf8")).fragments as { owner: string; fragment: string }[]);
const hdrOwned = new Set<string>();
for (const line of readFileSync(join(F, "hdr-tools/touched.txt"), "utf8").split("\n")) if (line.trim()) hdrOwned.add(line.trim());
for (const line of readFileSync(join(F, "hdr-tools/created.tsv"), "utf8").split("\n")) { const first = line.split("\t")[0]!.trim(); if (first) hdrOwned.add(first); }
const owners: Record<string, { manifest: string; cls: string }[]> = existsSync(join(G, "frd-owners.json")) ? JSON.parse(readFileSync(join(G, "frd-owners.json"), "utf8")) : {};

const mine = new Map<string, Set<string>>();
for (const name of readdirSync(G).filter(n => /^frd-(mig|pdf|sweep|fix).*\.json$/u.test(n))) {
  try { for (const o of JSON.parse(readFileSync(join(G, name), "utf8")) as { file?: string; status?: string; detail?: string }[]) if (o.status === "applied" && o.file && o.detail?.includes(">")) { const set = mine.get(o.file) ?? new Set<string>(); set.add(o.detail.split(">")[1]!.slice(0, 10)); mine.set(o.file, set); } } catch { }
}
const sha10 = (value: string) => createHash("sha1").update(value).digest("hex").slice(0, 10);
const collapse = (s: string) => s.replace(/\s+/gu, " ").trim();
const SHAPES: Record<string, { kind: "read_static" | "fixture_text" | "read_node"; def: string }> = {
  read_static: { kind: "read_static", def: collapse(`fn read_static(path_from_manifest: &str) -> &'static str { let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest); Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str()) }`) },
  fixture_text: { kind: "fixture_text", def: collapse(`fn fixture_text(relative: &str) -> &'static str { Box::leak(std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)).expect(relative).into_boxed_str()) }`) },
  read_node: { kind: "read_node", def: collapse(`fn read_node(path: &str) -> &'static str { let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path); Box::leak(std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("{}: {error}", file.display())).into_boxed_str()) }`) },
};

type Outcome = { file: string; status: string; detail?: string; sites?: Record<string, number> };
const outcomes: Outcome[] = [];
mkdirSync(ORIG, { recursive: true });
const files = (JSON.parse(readFileSync(listPath, "utf8")) as string[]).filter(f => !only || f.includes(only));

function parseModule(text: string, tokens: ReturnType<typeof rustTokens>, pairs: ReadonlyMap<number, number>, modStart: number): { base: string; repo: boolean } | null {
  const close = pairs.get(modStart + 2);
  if (close === undefined) return null;
  const body = text.slice(tokens[modStart + 2]!.start, tokens[close]!.end);
  const repo = body.includes("HashMap");
  const joined = /(?:\.join\("([^"]*)"\)|fixture_path!\("([^"]*)"\))\s*\.join\((?:relative|path)\)/u.exec(body);
  const join_ = joined?.[1] ?? joined?.[2];
  if (join_ === undefined) return null;
  return { base: join_, repo };
}
const modInfo = new Map<string, { base: string; repo: boolean }>();
for (const rel of files) {
  const abs = join(ROOT, rel);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8");
  if (!text.includes("mod runtime_fixture")) continue;
  const tokens = rustTokens(text), pairs = rustTokenPairs(tokens);
  for (let i = 0; i + 2 < tokens.length; i += 1) if (tokens[i]!.text === "mod" && tokens[i + 1]!.text === "runtime_fixture" && tokens[i + 2]!.text === "{") {
    const info = parseModule(text, tokens, pairs, i);
    if (info) for (const o of owners[rel] ?? []) modInfo.set(o.manifest, info);
    break;
  }
}
for (const [manifest, info] of modInfo) if (info.repo && !existsSync(join(resolve(ROOT, dirname(manifest)), info.base, "nx.json"))) { modInfo.delete(manifest); console.error(`[DEBUG] repo-kind base of ${manifest} does not reach nx.json`); }

for (const rel of files) {
  const abs = join(ROOT, rel);
  const skip = (status: string, detail?: string) => outcomes.push({ file: rel, status, detail });
  if (!existsSync(abs)) { skip("missing"); continue; }
  const owner = excluded.find(x => rel.includes(x.fragment));
  if (owner) { skip("excluded", owner.owner); continue; }
  if (hdrOwned.has(rel)) { skip("excluded", "hdr"); continue; }
  const text = readFileSync(abs, "utf8");
  if ((Date.now() - statSync(abs).mtimeMs) / 60000 < hotMinutes && !mine.get(rel)?.has(sha10(text))) { skip("hot"); continue; }
  if (!text.includes("CARGO_MANIFEST_DIR") && !text.includes("runtime_fixture::") && !text.includes("mod runtime_fixture")) { skip("untouched"); continue; }
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const enclosingMod: number[] = new Array(tokens.length).fill(-1);
  const macroBody = new Array<boolean>(tokens.length).fill(false);
  {
    const stack: { index: number; isMod: boolean }[] = [];
    for (let i = 0; i < tokens.length; i += 1) {
      const t = tokens[i]!.text;
      if (t === "}") stack.pop();
      enclosingMod[i] = [...stack].reverse().find(entry => entry.isMod)?.index ?? -1;
      if (t === "{") stack.push({ index: i, isMod: tokens[i - 2]?.text === "mod" && tokens[i - 1]?.kind === "identifier" });
    }
    for (let i = 0; i + 3 < tokens.length; i += 1) if (tokens[i]!.text === "macro_rules" && tokens[i + 1]!.text === "!") { const close = pairs.get(i + 3); if (close !== undefined) for (let j = i + 3; j <= close; j += 1) macroBody[j] = true; }
  }
  const edits: { start: number; end: number; text: string }[] = [];
  const need = new Map<number, Set<string>>();
  const sites: Record<string, number> = {};
  const note = (name: string) => { sites[name] = (sites[name] ?? 0) + 1; };
  const want = (at: number, name: string) => { const mod = enclosingMod[at]!; const set = need.get(mod) ?? new Set<string>(); set.add(name); need.set(mod, set); };
  const removed: [number, number][] = [];
  const inRemoved = (offset: number) => removed.some(([a, b]) => offset >= a && offset < b);
  let manual: string | null = null;

  if (tiers.has("2")) {
    for (let i = 0; i + 4 < tokens.length; i += 1) {
      if (tokens[i]!.text !== "fn" || tokens[i + 1]?.kind !== "identifier") continue;
      const shape = Object.hasOwn(SHAPES, tokens[i + 1]!.text) ? SHAPES[tokens[i + 1]!.text] : undefined;
      if (!shape) continue;
      let open = i + 2;
      while (open < tokens.length && tokens[open]!.text !== "{") open += 1;
      const close = pairs.get(open);
      if (close === undefined) continue;
      let start = tokens[i]!.start;
      const before = text.slice(0, start);
      if (/pub(\([^)]*\))?\s*$/u.test(before)) { manual = `public helper ${shape.kind}`; continue; }
      if (collapse(text.slice(start, tokens[close]!.end)) !== shape.def) { manual = `non-canonical ${shape.kind}`; continue; }
      const lead = /\n[ \t]*\n[ \t]*$/u.exec(before);
      if (lead) start = lead.index + 1; else { const line = /\n[ \t]*$/u.exec(before); if (line) start = line.index + 1; }
      let end = tokens[close]!.end;
      if (text[end] === "\n") end += 1;
      edits.push({ start, end, text: "" });
      removed.push([tokens[i]!.start, tokens[close]!.end]);
      if (enclosingMod[i] === -1) want(i, "fixture_text"); else want(i, "fixture_text");
      note(`def:${shape.kind}`);
    }
    const removedNames = new Set<string>(edits.filter(e => e.text === "").map(e => (/fn (read_static|fixture_text|read_node)\b/u.exec(text.slice(e.start, e.end)) ?? [])[1]).filter((n): n is string => !!n));
    for (let i = 0; i < tokens.length; i += 1) {
      const t = tokens[i]!;
      if (t.kind !== "identifier" || inRemoved(t.start) || !["read_static", "fixture_text", "read_node"].includes(t.text) || tokens[i + 1]?.text !== "(" || tokens[i - 1]?.text === "fn" || tokens[i - 1]?.text === "!") continue;
      if (!removedNames.has(t.text)) continue;
      let from = i;
      while (tokens[from - 1]?.text === "::") {
        if (tokens[from - 2]?.text !== "super") { manual = `qualified call of ${t.text}`; from = -1; break; }
        from -= 2;
      }
      if (from < 0) continue;
      const kind = t.text;
      if (kind === "read_node") {
        const arg = tokens[i + 2], closeArg = tokens[i + 3];
        if (arg?.kind !== "string" || closeArg?.text !== ")") { manual = "read_node with a non-literal argument"; continue; }
        edits.push({ start: t.start, end: closeArg.end, text: `fixture_text!("../../../${JSON.parse(arg.text)}")` });
      } else edits.push({ start: t.start, end: t.end, text: "fixture_text!" });
      want(i, "fixture_text");
      note(`call:${kind}`);
    }
  }

  if (tiers.has("1b") && text.includes("runtime_fixture")) {
    let modStart = -1;
    for (let i = 0; i + 2 < tokens.length; i += 1) if (tokens[i]!.text === "mod" && tokens[i + 1]!.text === "runtime_fixture" && tokens[i + 2]!.text === "{") { modStart = i; break; }
    let info = modStart >= 0 ? parseModule(text, tokens, pairs, modStart) : null;
    if (modStart >= 0 && !info) manual = "runtime_fixture module of unknown shape";
    if (modStart < 0) {
      const infos = new Map<string, { base: string; repo: boolean }>();
      for (const o of owners[rel] ?? []) { const found = modInfo.get(o.manifest); if (found) infos.set(`${found.repo}:${found.base}`, found); }
      if (infos.size === 1) info = [...infos.values()][0]!; else manual = infos.size ? "conflicting runtime_fixture modules" : "runtime_fixture module not found in an owning crate";
    }
    if (info && !manual) {
      if (modStart >= 0) {
        const close = pairs.get(modStart + 2)!;
        let k = modStart;
        while (k > 0 && (tokens[k - 1]!.text === "pub" || tokens[k - 1]!.text === ")")) { k -= 1; if (tokens[k]!.text === ")") { const o = pairs.get(k); if (o !== undefined) k = o; } }
        const start = tokens[k]!.start;
        const pre = text.slice(0, start);
        let cut = start;
        const attrs = /((?:[ \t]*(?:#\[[^\n]*\]|\/\/\/[^\n]*)\n)+)[ \t]*$/u.exec(pre);
        if (attrs) cut = pre.length - attrs[0].length;
        if (/\n[ \t]*\n$/u.test(text.slice(0, cut))) cut -= 1;
        let end = tokens[close]!.end;
        if (text[end] === "\n") end += 1;
        edits.push({ start: cut, end, text: "" });
        removed.push([tokens[modStart]!.start, tokens[close]!.end]);
        note("def:runtime_fixture-module");
      }
      for (let i = 0; i + 4 < tokens.length; i += 1) {
        if (tokens[i]!.text !== "runtime_fixture" || tokens[i + 1]!.text !== "::" || tokens[i + 3]?.text !== "(" || inRemoved(tokens[i]!.start)) continue;
        const fn = tokens[i + 2]!.text;
        let from = i;
        while (from >= 2 && tokens[from - 1]!.text === "::" && tokens[from - 2]!.kind === "identifier") from -= 2;
        const open = i + 3, close = pairs.get(open)!;
        const argText = text.slice(tokens[open]!.end, tokens[close]!.start).trim();
        const isLit = tokens[open + 1]?.kind === "string" && close === open + 2;
        let replacement: string;
        if (info.repo && fn === "text") { replacement = `repo_text!(${argText})`; want(i, "repo_text"); }
        else if (!info.repo && (fn === "bytes" || fn === "text")) {
          const literal = isLit ? JSON.stringify(`${info.base}/${JSON.parse(tokens[open + 1]!.text)}`) : `&format!("${info.base}/{}", ${argText})`;
          replacement = fn === "bytes" ? `fixture_bytes!(${literal}).to_vec()` : `fixture_text!(${literal}).to_owned()`;
          want(i, fn === "bytes" ? "fixture_bytes" : "fixture_text");
        } else { manual = `runtime_fixture::${fn}`; continue; }
        edits.push({ start: tokens[from]!.start, end: tokens[close]!.end, text: replacement });
        note(`call:module-${fn}`);
      }
    }
  }

  const testContext = (at: number): boolean => testLikePath(rel) || gateAt(tokens, pairs, at) === "test";
  if (tiers.has("3")) {
    for (let k = 0; k + 4 < tokens.length; k += 1) {
      if (tokens[k]!.text !== "env" || tokens[k + 1]!.text !== "!" || tokens[k + 2]!.text !== "(" || tokens[k + 3]!.text !== '"CARGO_MANIFEST_DIR"' || tokens[k + 4]!.text !== ")" || inRemoved(tokens[k]!.start)) continue;
      if (macroBody[k]) continue;
      if (!testContext(k)) { note("left:production"); continue; }
      if (tokens[k - 1]?.text === "(" && ["new", "from"].includes(tokens[k - 2]?.text ?? "") && tokens[k - 3]?.text === "::" && ["Path", "PathBuf"].includes(tokens[k - 4]?.text ?? "") && pairs.get(k - 1) === k + 5) {
        let from = k - 4;
        while (from >= 2 && tokens[from - 1]!.text === "::" && tokens[from - 2]!.kind === "identifier") from -= 2;
        if (tokens[from - 1]?.text === "::") from -= 1;
        const tail = tokens[k + 5]!;
        if (tokens[k + 6]?.text === "." && tokens[k + 7]?.text === "join" && tokens[k + 8]?.text === "(") {
          const close = pairs.get(k + 8)!;
          edits.push({ start: tokens[from]!.start, end: tokens[close]!.end, text: `fixture_path!(${text.slice(tokens[k + 8]!.end, tokens[close]!.start).trim()})` });
          want(k, "fixture_path"); note("expr:path-join");
        } else if (tokens[k - 2]!.text === "from") {
          edits.push({ start: tokens[from]!.start, end: tail.end, text: "manifest_dir!()" });
          want(k, "manifest_dir"); note("expr:manifest-dir");
        } else note("left:path-new");
        continue;
      }
      if (tokens[k - 1]?.text === "(" && tokens[k - 2]?.text === "!" && tokens[k - 3]?.text === "concat" && tokens[k + 5]?.text === "," && tokens[k + 6]?.kind === "string" && tokens[k + 7]?.text === ")" && pairs.get(k - 1) === k + 7) {
        const consumer = tokens[k - 5]?.text, paren = tokens[k - 4]?.text;
        const literal = JSON.parse(tokens[k + 6]!.text) as string;
        if (paren === "(" && ["read_to_string", "read", "read_dir", "open"].includes(consumer ?? "") && literal.startsWith("/")) {
          edits.push({ start: tokens[k - 3]!.start, end: tokens[k + 7]!.end, text: `fixture_path!(${JSON.stringify(literal.slice(1))})` });
          want(k, "fixture_path"); note("expr:concat-read");
        } else note(tokens[k - 5]?.text === "include_str" || tokens[k - 5]?.text === "include_bytes" ? "left:include-concat" : "left:concat-other");
        continue;
      }
      note(tokens[k + 5]?.text === "," ? "left:other" : "left:bare-env");
    }
  }

  if (tiers.has("3")) {
    const ROOT_WALK = /(?:std::path::)?Path::new\(env!\("CARGO_MANIFEST_DIR"\)\)\s*\.ancestors\(\)\s*\.find\(\|(\w+)\|\s*\1\.join\("nx\.json"\)\.is_file\(\)\)\s*\.(?:expect\("[^"]*"\)|unwrap\(\))(?:\s*\.to_path_buf\(\))?/gu;
    for (let m = ROOT_WALK.exec(text); m; m = ROOT_WALK.exec(text)) {
      const index = tokens.findIndex(tok => tok.start >= m!.index);
      if (index < 0 || macroBody[index] || inRemoved(m.index) || !(testLikePath(rel) || gateAt(tokens, pairs, index) === "test")) continue;
      edits.push({ start: m.index, end: m.index + m[0].length, text: "repo_root!()" });
      want(index, "repo_root"); note("expr:repo-root-walk");
    }
  }
  if (manual) { skip("manual", manual); continue; }
  if (!edits.length) { outcomes.push({ file: rel, status: "nothing", sites }); continue; }
  let out = text;
  const insertions: { start: number; text: string }[] = [];
  for (const [mod, names] of need) {
    const sorted = [...names].sort();
    if (mod === -1) continue;
    const bodyStart = afterModHeader(text, tokens[mod]!.end);
    insertions.push({ start: bodyStart, text: `\n    use semio_framework_test_fixtures::${sorted.length === 1 ? sorted[0] : `{${sorted.join(", ")}}`};\n` });
  }
  for (const e of [...edits, ...insertions.map(i => ({ start: i.start, end: i.start, text: i.text }))].sort((a, b) => b.start - a.start || b.end - a.end)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  if (need.has(-1)) out = importFixtureMacros(out, [...need.get(-1)!]);
  if (!apply) { outcomes.push({ file: rel, status: "plan", sites, detail: manual ?? undefined }); continue; }
  if (readFileSync(abs, "utf8") !== text) { skip("raced"); continue; }
  const key = createHash("sha1").update(rel).digest("hex");
  if (!existsSync(join(ORIG, key + ".rs"))) writeFileSync(join(ORIG, key + ".rs"), text);
  writeFileSync(abs, out);
  outcomes.push({ file: rel, status: "applied", sites, detail: `${sha10(text)}>${sha10(out)}${manual ? " # " + manual : ""}` });
}
const counts: Record<string, number> = {};
for (const o of outcomes) counts[o.status] = (counts[o.status] ?? 0) + 1;
const siteTotals: Record<string, number> = {};
for (const o of outcomes) for (const [k, v] of Object.entries(o.sites ?? {})) siteTotals[k] = (siteTotals[k] ?? 0) + v;
writeFileSync(join(G, `${outPrefix}.json`), JSON.stringify(outcomes, null, 1));
console.log(JSON.stringify(counts), JSON.stringify(siteTotals));

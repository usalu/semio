/** 🧹️ Wave L2 codemod: removes first-party Cargo features. A feature is either switched permanently `on` (its code and optional
 * dependencies become unconditional) or `off` (dead: its code and exclusive optional dependencies are deleted). Rewrites the owning
 * manifest, every consumer manifest and every `cfg`/`cfg_attr`/`cfg!` in the owner's sources by partial evaluation.
 * Usage: `bun 🧹️defeature.ts <plan.json> [--apply] [--sources-only]` where the plan is `[{ "package", "feature", "mode": "on" | "off" }]`. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { PEER_OWNED, crateSourceClosure, crateSources, manifests, repoRoot } from "./🗺️inventory.ts";

export type Conversion = { package: string; feature: string; mode: "on" | "off" | "test" };
type Value = boolean | "test";
type Toml = Record<string, any>;

//#region 🔖️CfgExpression
type Cfg = { kind: "true" } | { kind: "false" } | { kind: "atom"; text: string } | { kind: "not"; arg: Cfg } | { kind: "all" | "any"; args: Cfg[] };

/** 🧮️ Parses a cfg predicate (`feature = "x"`, `not(..)`, `all(..)`, `any(..)`, opaque atoms). */
export function parseCfg(text: string): Cfg {
  let i = 0;
  const ws = () => { while (/\s/.test(text[i] ?? "")) i++; };
  const expr = (): Cfg => {
    ws();
    const head = text.slice(i).match(/^(all|any|not)\s*\(/);
    if (head) {
      i += head[0].length;
      const args: Cfg[] = [];
      ws();
      while (text[i] !== ")") { args.push(expr()); ws(); if (text[i] === ",") { i++; ws(); } }
      i++;
      return head[1] === "not" ? { kind: "not", arg: args[0]! } : { kind: head[1] as "all" | "any", args };
    }
    const start = i;
    let quoted = false;
    while (i < text.length && (quoted || (text[i] !== "," && text[i] !== ")"))) { if (text[i] === "\"") quoted = !quoted; i++; }
    return { kind: "atom", text: text.slice(start, i).trim() };
  };
  const result = expr();
  ws();
  if (i !== text.length) throw new Error(`cfg parse: trailing input in ${text}`);
  return result;
}

/** ✂️ Substitutes known features and folds constants. */
export function simplify(cfg: Cfg, known: Map<string, Value>): Cfg {
  if (cfg.kind === "atom") { const m = cfg.text.match(/^feature\s*=\s*"([^"]+)"$/); if (!m || !known.has(m[1]!)) return cfg; const value = known.get(m[1]!)!; return value === "test" ? { kind: "atom", text: "test" } : { kind: value ? "true" : "false" }; }
  if (cfg.kind === "not") { const arg = simplify(cfg.arg, known); return arg.kind === "true" ? { kind: "false" } : arg.kind === "false" ? { kind: "true" } : { kind: "not", arg }; }
  if (cfg.kind === "all" || cfg.kind === "any") {
    const neutral = cfg.kind === "all" ? "true" : "false", absorbing = cfg.kind === "all" ? "false" : "true";
    const args = cfg.args.map((arg) => simplify(arg, known)).filter((arg) => arg.kind !== neutral).filter((arg, index, all) => arg.kind !== "atom" || all.findIndex((other) => other.kind === "atom" && other.text === arg.text) === index);
    if (args.some((arg) => arg.kind === absorbing)) return { kind: absorbing };
    if (!args.length) return { kind: neutral };
    return args.length === 1 && cfg.args.length > 1 ? args[0]! : { kind: cfg.kind, args };
  }
  return cfg;
}

/** 🖨️ Prints a cfg predicate. */
export function printCfg(cfg: Cfg): string {
  if (cfg.kind === "atom") return cfg.text;
  if (cfg.kind === "not") return `not(${printCfg(cfg.arg)})`;
  if (cfg.kind === "all" || cfg.kind === "any") return `${cfg.kind}(${cfg.args.map(printCfg).join(", ")})`;
  throw new Error("constant cfg has no source form");
}
//#endregion 🔖️CfgExpression

//#region 🔖️RustScanner
/** 🔚️ Index just past the token group starting at `open` (one of `([{`), skipping strings, chars and comments. */
function skipGroup(text: string, open: number): number {
  let depth = 0;
  for (let i = open; i < text.length; i++) {
    const skipped = skipLexical(text, i);
    if (skipped !== i) { i = skipped - 1; continue; }
    const c = text[i]!;
    if ("([{".includes(c)) depth++;
    else if (")]}".includes(c)) { depth--; if (depth === 0) return i + 1; }
  }
  throw new Error("unbalanced group");
}

/** 💬️ If a string, raw string, char literal or comment starts at `i`, returns its end; otherwise `i`. */
function skipLexical(text: string, i: number): number {
  const rest = text.slice(i, i + 3);
  if (rest.startsWith("//")) { const end = text.indexOf("\n", i); return end < 0 ? text.length : end; }
  if (rest.startsWith("/*")) { let depth = 0; for (let j = i; j < text.length - 1; j++) { if (text.startsWith("/*", j)) { depth++; j++; } else if (text.startsWith("*/", j)) { depth--; j++; if (!depth) return j + 1; } } return text.length; }
  const raw = text.slice(i).match(/^b?r(#*)"/);
  if (raw && (i === 0 || !/[A-Za-z0-9_]/.test(text[i - 1]!))) { const close = `"${raw[1]}`; const end = text.indexOf(close, i + raw[0].length); return end + close.length; }
  if (text[i] === "\"" || (text[i] === "b" && text[i + 1] === "\"" && !/[A-Za-z0-9_]/.test(text[i - 1] ?? ""))) {
    let j = text[i] === "b" ? i + 2 : i + 1;
    for (; j < text.length; j++) { if (text[j] === "\\") j++; else if (text[j] === "\"") return j + 1; }
  }
  if (text[i] === "'") {
    const char = text.slice(i).match(/^'(\\(x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'])'/);
    if (char) return i + char[0].length;
  }
  return i;
}

/** 🔚️ End of the item, statement, field, variant or arm that starts at `start` (after its cfg attribute). */
function itemEnd(text: string, start: number): number {
  let i = start, angle = 0;
  const peek = (from: number): number => { let j = from; while (/\s/.test(text[j] ?? "")) j++; return j; };
  for (;;) {
    i = peek(i);
    if (text[i] === "#" && (text[i + 1] === "[" || text[i + 2] === "[")) { i = skipGroup(text, text.indexOf("[", i)); continue; }
    break;
  }
  for (; i < text.length; i++) {
    const skipped = skipLexical(text, i);
    if (skipped !== i) { i = skipped - 1; continue; }
    const c = text[i]!;
    if (c === "<" && text[i + 1] !== "=" && text[i - 1] !== "<" && text[i + 1] !== "<" && !(/\s/.test(text[i - 1] ?? "") && /\s/.test(text[i + 1] ?? ""))) angle++;
    else if (c === ">" && angle > 0 && text[i - 1] !== "-" && text[i - 1] !== "=" && text[i + 1] !== "=") angle--;
    else if (c === "(" || c === "[") i = skipGroup(text, i) - 1;
    else if (c === "{") {
      i = skipGroup(text, i);
      const next = peek(i);
      if (text[next] === ";" || text[next] === ",") return next + 1;
      if (/^(else\b|\.|\?|=>)/.test(text.slice(next))) { i = next - 1; continue; }
      return i;
    } else if ((c === ";" || c === ",") && angle === 0) return i + 1;
    else if (c === "}" || c === ")" || c === "]") return i;
  }
  return text.length;
}

/** ⬆️ Start of the line block (doc comments and attributes) that belongs to the item whose cfg attribute starts at `at`. */
function itemLead(text: string, at: number): number {
  let lineStart = text.lastIndexOf("\n", at - 1) + 1;
  if (text.slice(lineStart, at).trim()) return at;
  for (;;) {
    const prevEnd = lineStart - 1;
    if (prevEnd < 0) return lineStart;
    const prevStart = text.lastIndexOf("\n", prevEnd - 1) + 1;
    const line = text.slice(prevStart, prevEnd).trim();
    if (/^(\/\/\/|\/\/!)/.test(line) && !line.startsWith("//!")) { lineStart = prevStart; continue; }
    if (/^#\[[^\]]*\]$/.test(line) && !/^#\[cfg\(/.test(line)) { lineStart = prevStart; continue; }
    return lineStart;
  }
}
//#endregion 🔖️RustScanner

//#region 🔖️SourceRewrite
/** 🔁️ Rewrites every `#[cfg]`, `#![cfg]`, `#[cfg_attr]` and `cfg!` in one Rust source under the known feature values. */
export function rewriteRust(text: string, known: Map<string, Value>, log: string[]): string {
  const pattern = /#(!?)\[\s*(cfg|cfg_attr)\s*\(|\bcfg!\s*\(/g;
  let out = text;
  for (;;) {
    pattern.lastIndex = 0;
    let match: RegExpExecArray | null, changed = false;
    while ((match = pattern.exec(out))) {
      const at = match.index;
      if (skipLexicalContext(out, at)) continue;
      const parenOpen = out.indexOf("(", at + match[0].length - 1);
      const parenClose = skipGroup(out, parenOpen);
      const inner = out.slice(parenOpen + 1, parenClose - 1);
      const relevant = [...known.keys()].some((feature) => new RegExp(`feature\\s*=\\s*"${feature.replace(/[-]/g, "\\-")}"`).test(inner));
      if (!relevant) continue;
      if (match[0].startsWith("cfg!")) {
        const value = simplify(parseCfg(inner), known);
        const replacement = value.kind === "true" ? "true" : value.kind === "false" ? "false" : `cfg!(${printCfg(value)})`;
        out = out.slice(0, at) + replacement + out.slice(parenClose);
        log.push(`cfg! -> ${replacement}`);
        changed = true; break;
      }
      const attrEnd = skipGroup(out, out.indexOf("[", at));
      if (match[2] === "cfg_attr") {
        const comma = topLevelComma(inner);
        const value = simplify(parseCfg(inner.slice(0, comma)), known);
        const attrs = inner.slice(comma + 1).trim();
        const replacement = value.kind === "true" ? `#${match[1]}[${attrs}]` : value.kind === "false" ? "" : `#${match[1]}[cfg_attr(${printCfg(value)}, ${attrs})]`;
        out = removeSpan(out, at, attrEnd, replacement);
        log.push(`cfg_attr -> ${replacement || "(removed)"}`);
        changed = true; break;
      }
      const value = simplify(parseCfg(inner), known);
      if (value.kind === "true") { out = removeSpan(out, at, attrEnd, ""); log.push(`cfg(${inner}) -> on`); }
      else if (value.kind === "false") {
        if (match[1] === "!") throw new Error(`inner #![cfg] evaluates false: the whole file is dead (${inner})`);
        const lead = itemLead(out, at), end = itemEnd(out, attrEnd);
        log.push(`cfg(${inner}) -> deleted ${JSON.stringify(out.slice(attrEnd, Math.min(end, attrEnd + 90)).trim().split("\n")[0])}`);
        out = removeSpan(out, lead, end, "");
      } else { out = out.slice(0, at) + `#${match[1]}[cfg(${printCfg(value)})]` + out.slice(attrEnd); log.push(`cfg(${inner}) -> cfg(${printCfg(value)})`); }
      changed = true; break;
    }
    if (!changed) return out;
  }
}

function skipLexicalContext(text: string, at: number): boolean {
  const lineStart = text.lastIndexOf("\n", at - 1) + 1;
  const before = text.slice(lineStart, at);
  return /\/\/|^\s*\*/.test(before) || (before.split("\"").length - 1) % 2 === 1;
}

function topLevelComma(text: string): number {
  let depth = 0, quoted = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i]!;
    if (c === "\"") quoted = !quoted;
    else if (!quoted && c === "(") depth++;
    else if (!quoted && c === ")") depth--;
    else if (!quoted && c === "," && depth === 0) return i;
  }
  throw new Error(`cfg_attr without attribute: ${text}`);
}

/** ✂️ Replaces `[from, to)`; an emptied line (only whitespace left) is removed with its newline. */
function removeSpan(text: string, from: number, to: number, replacement: string): string {
  const lineStart = text.lastIndexOf("\n", from - 1) + 1;
  let lineEnd = text.indexOf("\n", to);
  if (lineEnd < 0) lineEnd = text.length;
  const head = text.slice(lineStart, from), tail = text.slice(to, lineEnd);
  if (!replacement && !head.trim() && !tail.trim()) return text.slice(0, lineStart) + text.slice(Math.min(lineEnd + 1, text.length));
  if (!replacement && !head.trim()) return text.slice(0, from) + text.slice(to).replace(/^[ \t]+/, "");
  return text.slice(0, from) + replacement + text.slice(to);
}
//#endregion 🔖️SourceRewrite

//#region 🔖️ManifestRewrite
type Span = { start: number; end: number };

/** 📑️ Byte span of the TOML table `[name]` (header to next header). */
function tableSpan(text: string, name: string): Span | undefined {
  const header = new RegExp(`^\\[${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\]\\s*$`, "m").exec(text);
  if (!header) return undefined;
  const next = /^\[/m;
  const after = text.slice(header.index + header[0].length);
  const m = next.exec(after);
  return { start: header.index, end: m ? header.index + header[0].length + m.index : text.length };
}

/** 🔑️ Span of the entry `key = value` (value possibly multi-line) inside `span`. */
function entrySpan(text: string, span: Span, key: string): Span | undefined {
  const body = text.slice(span.start, span.end);
  const re = new RegExp(`^[ \\t]*"?${key.replace(/[-.]/g, "\\$&")}"?\\s*=`, "m");
  const m = re.exec(body);
  if (!m) return undefined;
  const start = span.start + m.index;
  let i = start + m[0].length;
  while (text[i] === " " || text[i] === "\t") i++;
  if (text[i] === "[" || text[i] === "{") i = skipGroup(text, i);
  else if (text[i] === "\"") i = text.indexOf("\"", i + 1) + 1;
  else while (i < text.length && text[i] !== "\n") i++;
  const lineEnd = text.indexOf("\n", i);
  return { start, end: lineEnd < 0 ? text.length : lineEnd + 1 };
}

function setArray(entryText: string, items: string[]): string {
  const eq = entryText.indexOf("=");
  return `${entryText.slice(0, eq)}= [${items.map((item) => JSON.stringify(item)).join(", ")}]\n`;
}

function arrayItems(entryText: string): string[] {
  return [...entryText.slice(entryText.indexOf("=") + 1).matchAll(/"([^"]*)"/g)].map((m) => m[1]!);
}

const DEP_SECTIONS = /^\[(?:target\.[^\]]+\.)?(?:dependencies|dev-dependencies|build-dependencies)\]\s*$|^\[workspace\.dependencies\]\s*$/gm;

/** 🧾️ Every dependency entry `name = {...}` in dependency tables, with its span, key and resolved package name. */
function dependencyEntries(text: string): { span: Span; key: string; pkg: string; body: string }[] {
  const rows: { span: Span; key: string; pkg: string; body: string }[] = [];
  for (const header of text.matchAll(DEP_SECTIONS)) {
    const name = header[0].trim().slice(1, -1);
    const table = tableSpan(text, name)!;
    const body = text.slice(table.start, table.end);
    for (const line of body.matchAll(/^[ \t]*"?([A-Za-z0-9_-]+)"?\s*=\s*\{/gm)) {
      const span = entrySpan(text, table, line[1]!)!;
      const entry = text.slice(span.start, span.end);
      const pkg = entry.match(/package\s*=\s*"([^"]+)"/)?.[1] ?? line[1]!;
      rows.push({ span, key: line[1]!, pkg, body: entry });
    }
  }
  return rows.sort((a, b) => b.span.start - a.span.start);
}

function editInline(entry: string, edit: (fields: string) => string): string {
  const open = entry.indexOf("{"), close = skipGroup(entry, open);
  const fields = edit(entry.slice(open + 1, close - 1)).trim().replace(/^,\s*/, "").replace(/\s*,$/, "");
  return entry.slice(0, open) + (fields ? `{ ${fields} }` : "{}") + entry.slice(close);
}

const dropField = (fields: string, field: string): string => fields.replace(new RegExp(`\\s*,?\\s*${field}\\s*=\\s*(true|false)\\s*(?=,|\\s*$)`), "").replace(/^\s*,\s*/, " ").replace(/\{\s*,/, "{");

function removeFeatureItems(fields: string, remove: Set<string>): string {
  const m = fields.match(/features\s*=\s*\[([^\]]*)\]/);
  if (!m) return fields;
  const items = [...m[1]!.matchAll(/"([^"]*)"/g)].map((x) => x[1]!).filter((item) => !remove.has(item));
  if (items.length) return fields.replace(m[0], `features = [${items.map((item) => JSON.stringify(item)).join(", ")}]`);
  return fields.replace(new RegExp(`\\s*,?\\s*${m[0].replace(/[[\]().*+?^$|\\]/g, "\\$&")}`), "").replace(/^\s*,\s*/, " ");
}

function addFeatureItems(fields: string, add: string[]): string {
  const m = fields.match(/features\s*=\s*\[([^\]]*)\]/);
  if (m) { const items = [...new Set([...[...m[1]!.matchAll(/"([^"]*)"/g)].map((x) => x[1]!), ...add])]; return fields.replace(m[0], `features = [${items.map((item) => JSON.stringify(item)).join(", ")}]`); }
  return `${fields.trimEnd()}, features = [${add.map((item) => JSON.stringify(item)).join(", ")}] `;
}
//#endregion 🔖️ManifestRewrite

/** 🧭️ A manifest owned by peer fleet A (same rule as the inventory). */
function isPeerManifest(toml: Toml): boolean {
  const name = toml.package?.name;
  return typeof name === "string" && (PEER_OWNED.some((pattern) => pattern.test(name)) || (name !== "semio-framework-plugin" && toml.package?.metadata?.semio?.["rust-source-boundary"] !== undefined));
}

/** 🚚️ Applies a conversion plan; returns the changed files with their new text and a log. */
export async function defeature(plan: Conversion[], sourcesOnly = false): Promise<{ changed: Map<string, string>; log: string[] }> {
  const log: string[] = [];
  const changed = new Map<string, string>();
  const read = (path: string): string => changed.get(path) ?? readFileSync(join(repoRoot, path), "utf8");
  const write = (path: string, text: string): void => { if (text !== readFileSync(join(repoRoot, path), "utf8")) changed.set(path, text); else changed.delete(path); };
  const all = manifests().map((path) => ({ path, toml: Bun.TOML.parse(read(path)) as Toml }));
  const byPackage = new Map(all.filter((row) => row.toml.package?.name).map((row) => [row.toml.package.name as string, row]));
  const planned = new Map<string, Map<string, Value>>();
  for (const row of plan) { if (!byPackage.has(row.package)) throw new Error(`unknown package ${row.package}`); (planned.get(row.package) ?? planned.set(row.package, new Map()).get(row.package)!).set(row.feature, row.mode === "test" ? "test" : row.mode === "on"); }
  const forwardAdds = new Map<string, Map<string, Set<string>>>();
  for (const [pkg, known] of planned) {
    const { path, toml } = byPackage.get(pkg)!;
    const declared = toml.features ?? {};
    const optionalDeps = new Set<string>();
    const optionalOf = (t: Toml | undefined) => Object.entries<Toml>(t ?? {}).filter(([, spec]) => typeof spec === "object" && spec.optional).map(([dep]) => dep);
    for (const dep of [...optionalOf(toml.dependencies), ...optionalOf(toml["build-dependencies"]), ...Object.values<Toml>(toml.target ?? {}).flatMap((t) => [...optionalOf(t.dependencies), ...optionalOf(t["build-dependencies"])])]) optionalDeps.add(dep);
    const remaining = Object.keys(declared).filter((f) => f !== "default" && !known.has(f));
    const depsOn = new Set<string>(), depsOff = new Set<string>(), depsTest = new Set<string>();
    const expand = (feature: string, value: Value, seen = new Set<string>()): void => {
      const on = value === true;
      if (seen.has(feature)) return; seen.add(feature);
      const entries: string[] = declared[feature] ?? (optionalDeps.has(feature) ? [`dep:${feature}`] : []);
      for (const entry of entries) {
        if (entry.startsWith("dep:")) (value === "test" ? depsTest : on ? depsOn : depsOff).add(entry.slice(4));
        else if (entry.includes("/")) {
          const [dep, sub] = entry.replace("?/", "/").split("/") as [string, string];
          const target = Object.entries<Toml>({ ...toml.dependencies, ...toml["dev-dependencies"] }).find(([key]) => key === dep);
          const targetPkg = (target?.[1] as Toml)?.package ?? dep;
          if (on && !(planned.get(targetPkg)?.has(sub))) ((forwardAdds.get(path) ?? forwardAdds.set(path, new Map()).get(path)!).get(dep) ?? (forwardAdds.get(path)!.set(dep, new Set()).get(dep)!)).add(sub);
          if (on && optionalDeps.has(dep)) depsOn.add(dep);
        } else if (optionalDeps.has(entry) && !(entry in declared)) (value === "test" ? depsTest : on ? depsOn : depsOff).add(entry);
        else if (entry in declared && !known.has(entry)) throw new Error(`${pkg}/${feature} enables remaining feature ${entry}; plan it too`);
      }
    };
    for (const [feature, on] of known) if (feature in declared || optionalDeps.has(feature)) expand(feature, on);
    const stillUsed = new Set(remaining.flatMap((f) => (declared[f] as string[]).filter((e) => e.startsWith("dep:")).map((e) => e.slice(4))));
    for (const dep of depsOff) if (depsOn.has(dep) || stillUsed.has(dep)) depsOff.delete(dep);
    for (const dep of depsTest) if (depsOn.has(dep) || stillUsed.has(dep)) throw new Error(`${pkg}: test-only dep ${dep} is also needed by production`);
    let text = read(path);
    const features = sourcesOnly ? undefined : tableSpan(text, "features");
    if (features) {
      for (const feature of [...known.keys()]) {
        const span = entrySpan(text, tableSpan(text, "features")!, feature);
        if (!span) continue;
        let start = span.start;
        for (;;) { const prev = text.lastIndexOf("\n", start - 2) + 1; if (start > 0 && text.slice(prev, start).trim().startsWith("#")) start = prev; else break; }
        text = text.slice(0, start) + text.slice(span.end);
      }
      const defaultSpan = entrySpan(text, tableSpan(text, "features")!, "default");
      if (defaultSpan) {
        const items = arrayItems(text.slice(defaultSpan.start, defaultSpan.end)).filter((item) => !known.has(item) && !item.startsWith("dep:"));
        text = text.slice(0, defaultSpan.start) + (items.length ? setArray(text.slice(defaultSpan.start, defaultSpan.end), items) : "") + text.slice(defaultSpan.end);
      }
      for (const feature of remaining) {
        const span = entrySpan(text, tableSpan(text, "features")!, feature);
        if (!span) continue;
        const items = arrayItems(text.slice(span.start, span.end)).filter((item) => !known.has(item) && !(item.startsWith("dep:") && (depsOn.has(item.slice(4)) || depsOff.has(item.slice(4)) || depsTest.has(item.slice(4)))));
        text = text.slice(0, span.start) + setArray(text.slice(span.start, span.end), items) + text.slice(span.end);
      }
      const table = tableSpan(text, "features")!;
      if (!text.slice(table.start, table.end).split("\n").slice(1).some((line) => line.trim() && !line.trim().startsWith("#"))) text = text.slice(0, table.start) + text.slice(table.end).replace(/^\n+/, "");
    }
    for (const entry of sourcesOnly ? [] : dependencyEntries(text)) {
      if (depsOn.has(entry.key)) text = text.slice(0, entry.span.start) + editInline(entry.body, (f) => dropField(f, "optional")) + text.slice(entry.span.end);
      else if (depsOff.has(entry.key) && /optional\s*=\s*true/.test(entry.body)) text = text.slice(0, entry.span.start) + text.slice(entry.span.end);
      else if (depsTest.has(entry.key) && /optional\s*=\s*true/.test(entry.body)) {
        const moved = editInline(entry.body, (f) => dropField(f, "optional"));
        text = text.slice(0, entry.span.start) + text.slice(entry.span.end);
        const dev = tableSpan(text, "dev-dependencies");
        if (dev && entrySpan(text, dev, entry.key)) throw new Error(`${pkg}: ${entry.key} is already a dev-dependency; merge by hand`);
        text = dev ? text.slice(0, dev.end).replace(/\n*$/, "\n") + moved + text.slice(dev.end) : `${text.replace(/\n*$/, "\n")}\n[dev-dependencies]\n${moved}`;
      }
    }
    text = text.replace(/^([ \t]*required-features\s*=\s*)\[([^\]]*)\][ \t]*\n/gm, (line, head: string, items: string) => {
      const kept = [...items.matchAll(/"([^"]*)"/g)].map((m) => m[1]!).filter((item) => !known.has(item));
      return kept.length ? `${head}[${kept.map((item) => JSON.stringify(item)).join(", ")}]\n` : "";
    });
    write(path, text.replace(/\n{3,}/g, "\n\n"));
    log.push(`${pkg}: ${[...known].map(([f, v]) => `${f}=${v === "test" ? "test" : v ? "on" : "off"}`).join(" ")} deps+${[...depsOn].join(",")} deps-${[...depsOff].join(",")} dev+${[...depsTest].join(",")}`);
    const closure = await crateSourceClosure(path, [...known.keys()]);
    for (const failure of closure.filter((file) => file.startsWith("!"))) log.push(`  closure refused: ${failure.slice(1)}`);
    const sources = [...new Set([...crateSources(path.slice(0, path.lastIndexOf("/📦️packages/"))), ...closure.filter((file) => !file.startsWith("!"))])];
    for (const file of sources) {
      const before = read(file);
      if (![...known.keys()].some((f) => before.includes(`"${f}"`))) continue;
      const fileLog: string[] = [];
      const after = rewriteRust(before, known, fileLog);
      write(file, after);
      for (const line of fileLog) log.push(`  ${file.split("/").slice(-3).join("/")}: ${line}`);
    }
  }
  for (const { path } of sourcesOnly ? [] : all) {
    let text = read(path);
    const original = text;
    const toml = Bun.TOML.parse(text) as Toml;
    const keyToPkg = new Map<string, string>();
    for (const entry of dependencyEntries(text)) keyToPkg.set(entry.key, entry.pkg);
    for (const entry of dependencyEntries(text)) {
      const known = planned.get(entry.pkg);
      const adds = [...(forwardAdds.get(path)?.get(entry.key) ?? [])];
      if (!known && !adds.length) continue;
      let body = entry.body;
      if (known) body = editInline(body, (f) => removeFeatureItems(f, new Set(known.keys())));
      if (adds.length) body = editInline(body, (f) => addFeatureItems(f, adds));
      const target = byPackage.get(entry.pkg)?.toml;
      const defaults = ((target?.features?.default ?? []) as string[]).filter((item) => !known?.has(item));
      if (known && !defaults.length) body = editInline(body, (f) => dropField(f, "default-features"));
      text = text.slice(0, entry.span.start) + body.replace(/\{\s+\}/, "{}").replace(/\{\s*,\s*/, "{ ") + text.slice(entry.span.end);
    }
    const features = tableSpan(text, "features");
    if (features) for (const [feature] of Object.entries<string[]>(toml.features ?? {})) {
      const span = entrySpan(text, tableSpan(text, "features")!, feature);
      if (!span) continue;
      const items = arrayItems(text.slice(span.start, span.end));
      const kept = items.filter((item) => { const m = item.match(/^([^/:]+)\??\/(.+)$/); return !(m && planned.get(keyToPkg.get(m[1]!) ?? m[1]!)?.has(m[2]!)); });
      if (kept.length !== items.length) text = text.slice(0, span.start) + setArray(text.slice(span.start, span.end), kept) + text.slice(span.end);
    }
    if (text === original) continue;
    const peer = isPeerManifest(toml);
    if (peer && text.replace(/,?\s*default-features\s*=\s*false/g, "") !== original.replace(/,?\s*default-features\s*=\s*false/g, "").replace(/\{\s+\}/g, "{}")) throw new Error(`peer manifest ${path} would need a feature edit; plan it with its owner`);
    if (peer) { log.push(`peer consumer left unchanged (only a now-inert default-features = false): ${path}`); continue; }
    write(path, text); log.push(`consumer ${path}`);
  }
  return { changed, log };
}

if (import.meta.main) {
  const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as Conversion[];
  const { changed, log } = await defeature(plan, process.argv.includes("--sources-only"));
  for (const line of log) console.log(line);
  console.log(`[DEBUG] conversions=${plan.length} files=${changed.size}`);
  if (process.argv.includes("--apply")) { for (const [path, text] of changed) writeFileSync(join(repoRoot, path), text); console.log("[DEBUG] applied"); }
}

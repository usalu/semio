import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

/** Declares `metadata.semio.execution` on every authored command target of the explicit roots from the probe table of the OLD classification rule. Fails closed: explicit roots, empty scope refused, dry run by default, per-manifest sha256 drift refusal, anchored span edits, structural proof after every edit. */
const REPO = process.env.AUDIT_FIX_REPO ?? "/Users/ueli/Documents/semio";
const argv = process.argv.slice(2), flag = (name: string): boolean => argv.includes(name), option = (name: string): string | undefined => { const at = argv.indexOf(name); return at < 0 ? undefined : argv[at + 1]; };
const roots = (option("--roots") ?? "").split(",").filter(Boolean), probeDirectory = option("--probe"), write = flag("--write"), overlayDirectory = option("--overlay"), listPath = option("--list");
if (roots.length === 0 || probeDirectory === undefined) throw new Error("refused: --roots (explicit list) and --probe (directory of probe tables) are required");
if (write && overlayDirectory !== undefined) throw new Error("refused: --write and --overlay are exclusive");

interface Node { readonly kind: "object" | "array" | "scalar"; readonly start: number; readonly end: number; readonly members: readonly Member[] }
interface Member { readonly key: string; readonly keyStart: number; readonly value: Node }
function parse(text: string): Node {
  let at = 0;
  const skip = (): void => { while (at < text.length && /\s/u.test(text[at]!)) at++; };
  const string = (): string => { const start = at; at++; while (text[at] !== "\"") { if (text[at] === "\\") at++; at++; } at++; return JSON.parse(text.slice(start, at)) as string; };
  const value = (): Node => {
    skip(); const start = at, c = text[at];
    if (c === "{") {
      at++; const members: Member[] = []; skip();
      if (text[at] === "}") { at++; return { kind: "object", start, end: at, members }; }
      for (;;) { skip(); const keyStart = at, key = string(); skip(); at++; const node = value(); members.push({ key, keyStart, value: node }); skip(); if (text[at] === ",") { at++; continue; } at++; break; }
      return { kind: "object", start, end: at, members };
    }
    if (c === "[") { at++; skip(); if (text[at] === "]") { at++; return { kind: "array", start, end: at, members: [] }; } for (;;) { value(); skip(); if (text[at] === ",") { at++; continue; } at++; break; } return { kind: "array", start, end: at, members: [] }; }
    if (c === "\"") { string(); return { kind: "scalar", start, end: at, members: [] }; }
    while (at < text.length && !/[,\]}\s]/u.test(text[at]!)) at++;
    return { kind: "scalar", start, end: at, members: [] };
  };
  const root = value(); skip();
  if (at !== text.length) throw new Error("trailing bytes");
  return root;
}

interface Declaration { readonly kind: "native" | "source" | "orchestration"; readonly scope?: "production" | "test"; readonly preparation?: "none" | "production" | "test" }
const render = (declaration: Declaration): string => `{ "version": 1, "kind": "${declaration.kind}"${declaration.scope === undefined ? "" : `, "scope": "${declaration.scope}"`}${declaration.preparation === undefined ? "" : `, "preparation": "${declaration.preparation}"`} }`;
/** Mirror of the preparation gate of the native-manifests component for a target without a declaration: its own name rules, kept only to record the CURRENT behaviour once. */
const PREPARED = /^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/u, TESTING = /^(?:test|bench)(?:-|$)/u;
const member = (node: Node, key: string): Member | undefined => node.members.find(row => row.key === key);
function indentOf(text: string, offset: number): string { const line = text.lastIndexOf("\n", offset - 1) + 1; return /^[ \t]*/u.exec(text.slice(line, offset))![0]; }
function unit(text: string, node: Node): string | undefined { const first = node.members[0]; if (first === undefined) return undefined; const inner = indentOf(text, first.keyStart), outer = indentOf(text, node.start); return text.slice(0, node.start).lastIndexOf("\n") < 0 || text.slice(node.start, first.keyStart).indexOf("\n") < 0 ? undefined : inner.startsWith(outer) ? inner.slice(outer.length) || "  " : "  "; }

/** 🧱️ One anchored insertion: `{ at, text }` in original coordinates. */
function insertionFor(text: string, target: Node, declaration: Declaration): { at: number; text: string } {
  const metadata = member(target, "metadata"), semio = metadata === undefined || metadata.value.kind !== "object" ? undefined : member(metadata.value, "semio");
  const inline = unit(text, target) === undefined;
  const step = unit(text, target) ?? "";
  if (semio !== undefined && semio.value.kind === "object") {
    const node = semio.value, first = node.members[0];
    if (first === undefined) return { at: node.start + 1, text: ` "execution": ${render(declaration)} ` };
    const pad = unit(text, node) === undefined ? " " : "\n" + indentOf(text, first.keyStart);
    return { at: first.keyStart, text: `"execution": ${render(declaration)},${pad}` };
  }
  if (metadata !== undefined && metadata.value.kind === "object") {
    const node = metadata.value, first = node.members[0];
    if (first === undefined) return { at: node.start + 1, text: ` "semio": { "execution": ${render(declaration)} } ` };
    const inner = indentOf(text, first.keyStart), nested = unit(text, node) === undefined;
    return { at: first.keyStart, text: nested ? `"semio": { "execution": ${render(declaration)} }, ` : `"semio": {\n${inner}${step || "  "}"execution": ${render(declaration)}\n${inner}},\n${inner}` };
  }
  const last = target.members.at(-1);
  if (last === undefined) return { at: target.start + 1, text: ` "metadata": { "semio": { "execution": ${render(declaration)} } } ` };
  const inner = indentOf(text, last.keyStart), end = last.value.end;
  if (inline) return { at: end, text: `, "metadata": { "semio": { "execution": ${render(declaration)} } }` };
  const stepped = step || "  ";
  return { at: end, text: `,\n${inner}"metadata": {\n${inner}${stepped}"semio": {\n${inner}${stepped}${stepped}"execution": ${render(declaration)}\n${inner}${stepped}}\n${inner}}` };
}

interface ProbeRow { path: string; sha256: string; name: string; classes: [string, string, boolean, boolean, string | null, boolean, boolean, boolean, boolean, string | null, boolean | null, boolean, boolean][] }
const rows: ProbeRow[] = readdirSync(probeDirectory).filter(name => /^old-\d+\.json$/u.test(name)).sort().flatMap(name => (JSON.parse(readFileSync(join(probeDirectory, name), "utf8")) as { rows: ProbeRow[] }).rows);
if (rows.length === 0) throw new Error("refused: empty probe table");
const inScope = (path: string): boolean => roots.some(root => root === "." ? !path.includes("/") : path.startsWith(root + "/"));
const scoped = rows.filter(row => inScope(row.path));
if (scoped.length === 0) throw new Error("refused: empty scope");
const changed: { path: string; added: number; names: string[] }[] = [], refused: { path: string; reason: string }[] = [];
const target = overlayDirectory;
for (const row of scoped.sort((left, right) => left.path.localeCompare(right.path))) {
  const absolute = join(REPO, row.path);
  const original = readFileSync(absolute, "utf8");
  if (createHash("sha256").update(readFileSync(absolute)).digest("hex") !== row.sha256) { refused.push({ path: row.path, reason: "manifest drifted since the probe" }); continue; }
  const wanted = new Map<string, Declaration>();
  for (const [, name, , native, declared, orchestrator, command, authored, , , runsTests, nativePackage, generator] of row.classes) {
    if (!authored || declared !== null || !command) continue;
    if (!native) { wanted.set(name, orchestrator ? { kind: "orchestration" } : { kind: "source" }); continue; }
    const scope = runsTests === true ? "test" : "production", prepared = nativePackage && !generator && PREPARED.test(name), preparedTests = TESTING.test(name);
    const preparation = !nativePackage || generator ? undefined : !prepared ? "none" : preparedTests !== (scope === "test") ? (preparedTests ? "test" : "production") : undefined;
    wanted.set(name, { kind: "native", scope, ...(preparation === undefined ? {} : { preparation }) });
  }
  if (wanted.size === 0) continue;
  let root: Node, targets: Member | undefined;
  try { root = parse(original); targets = member(root, "targets"); } catch (error) { refused.push({ path: row.path, reason: "unparseable: " + (error as Error).message }); continue; }
  if (targets === undefined || targets.value.kind !== "object") { refused.push({ path: row.path, reason: "no targets object although the probe saw authored targets" }); continue; }
  const edits: { at: number; text: string }[] = [], names: string[] = [];
  for (const [name, declaration] of wanted) {
    const entry = member(targets.value, name);
    if (entry === undefined || entry.value.kind !== "object") { refused.push({ path: row.path, reason: `target ${name} not found as an object` }); edits.length = 0; names.length = 0; break; }
    edits.push(insertionFor(original, entry.value, declaration)); names.push(name);
  }
  if (edits.length !== wanted.size) continue;
  let result = original;
  for (const edit of edits.sort((left, right) => right.at - left.at)) result = result.slice(0, edit.at) + edit.text + result.slice(edit.at);
  try {
    const before = JSON.parse(original) as { targets: Record<string, any> }, after = JSON.parse(result) as { targets: Record<string, any> };
    for (const [name, declaration] of wanted) {
      const added = after.targets[name]?.metadata?.semio?.execution;
      if (JSON.stringify(added) !== JSON.stringify({ version: 1, kind: declaration.kind, ...(declaration.scope === undefined ? {} : { scope: declaration.scope }), ...(declaration.preparation === undefined ? {} : { preparation: declaration.preparation }) })) throw new Error(`declaration of ${name} not as planned`);
      delete after.targets[name].metadata.semio.execution;
      if (Object.keys(after.targets[name].metadata.semio).length === 0) delete after.targets[name].metadata.semio;
      if (Object.keys(after.targets[name].metadata).length === 0) delete after.targets[name].metadata;
    }
    if (JSON.stringify(before) !== JSON.stringify(after)) throw new Error("edit changed more than the declarations");
  } catch (error) { refused.push({ path: row.path, reason: "structural proof failed: " + (error as Error).message }); continue; }
  changed.push({ path: row.path, added: wanted.size, names });
  if (write || target !== undefined) {
    const destination = write ? absolute : join(target!, row.path);
    if (write) { const backup = join("/private/tmp/audit-fix/backup", row.path); mkdirSync(dirname(backup), { recursive: true }); if (!existsSync(backup)) copyFileSync(absolute, backup); } else mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, result);
  }
}
const summary = { mode: write ? "write" : target !== undefined ? "overlay" : "dry-run", roots, manifests: scoped.length, changed: changed.length, declarations: changed.reduce((sum, row) => sum + row.added, 0), refused: refused.length };
if (listPath !== undefined) writeFileSync(listPath, JSON.stringify({ summary, changed, refused }, null, 1));
console.log(JSON.stringify(summary));
for (const row of refused.slice(0, 20)) console.log("refused", row.path, row.reason);
if (refused.length > 0) process.exitCode = 1;

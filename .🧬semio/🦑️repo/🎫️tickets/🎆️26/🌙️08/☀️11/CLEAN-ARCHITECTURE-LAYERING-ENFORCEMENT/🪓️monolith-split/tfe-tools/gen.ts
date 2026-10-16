/** 🏭 Generate the owners' public fixture entries and re-point every cross-module consumer to them. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, normalize, relative, resolve } from "node:path";
import ts from "typescript";
import { classify, type Form } from "./forms.ts";
import { collect, ROOT, type Hit } from "./work.ts";

const TOOLS = import.meta.dir;
const FOLDER = dirname(TOOLS);
const SNAP = join(TOOLS, "orig");
const [mode, listFile, reportFile] = process.argv.slice(2) as ["plan" | "apply", string, string];
const ENTRY = "🧪️tests/🧫️fixtures/🟦️.ts";

type Mode = "eager" | "lazy" | "url";
interface Use { owner: string; target: string; mode: Mode }

const NOISE = new Set(["tests", "fixtures", "contract"]);
const kindOf = (target: string): string => {
  if (target.includes("/🧫️fixtures/")) return "Fixture";
  if (target.includes("/🧬️schema/") || /\.schema\.json$/.test(target)) return "Schema";
  if (/contract\.json$/.test(basename(target))) return "Contract";
  return "Document";
};
const wordsOf = (owner: string, target: string): string[] => {
  const segments = target.slice(owner.length + 1).split("/");
  const words: string[] = basename(owner).replace(/[^A-Za-z0-9]+/g, " ").trim().split(" ").filter(Boolean);
  segments.forEach((segment, index) => {
    const stem = index === segments.length - 1 ? segment.replace(/\.(json|jsonc)$/, "") : segment;
    for (const word of stem.replace(/[^A-Za-z0-9]+/g, " ").trim().split(" ")) {
      if (!word) continue;
      const lower = word.toLowerCase();
      if (NOISE.has(lower)) continue;
      if (lower === "schema" && kindOf(target) === "Schema") continue;
      words.push(word);
    }
  });
  return words;
};
const cap = (word: string) => word.slice(0, 1).toUpperCase() + word.slice(1);
const camel = (words: string[]) => words.map((word, index) => (index === 0 ? word.slice(0, 1).toLowerCase() + word.slice(1) : cap(word))).join("");

interface Registry { names: Map<string, string>; existing: Map<string, string>; header: string[]; keep: string[] }

function readExistingEntry(owner: string): { lines: string[]; existing: Map<string, string> } {
  const path = join(ROOT, owner, ENTRY);
  const existing = new Map<string, string>();
  if (!existsSync(path)) return { lines: [], existing };
  const lines = readFileSync(path, "utf8").split("\n").filter(line => line.length > 0);
  for (const line of lines) {
    const rel = (value: string) => normalize(resolve(dirname(path), value)).slice(ROOT.length + 1);
    const eager = /^export \{ default as ([A-Za-z0-9_$]+) \} from "([^"]+)" with \{ type: "json" \};$/.exec(line);
    if (eager) existing.set(rel(eager[2]!), eager[1]!);
    const lazy = /^export const load([A-Za-z0-9_$]+) = \(\) => import\("([^"?]+)"\)/.exec(line);
    if (lazy) existing.set(rel(lazy[2]!), lazy[1]!.slice(0, 1).toLowerCase() + lazy[1]!.slice(1));
    const url = /^export const ([A-Za-z0-9_$]+)Url = new URL\("([^"]+)"/.exec(line);
    if (url) existing.set(rel(url[2]!), url[1]!);
  }
  return { lines, existing };
}

function assignNames(owner: string, targets: string[], pinned: Map<string, string>): Map<string, string> {
  const unique = [...new Set(targets)].sort();
  const result = new Map<string, string>();
  const levels = new Map<string, number>();
  const words = new Map<string, string[]>();
  for (const target of unique) {
    if (pinned.has(target)) continue;
    words.set(target, wordsOf(owner, target));
    levels.set(target, Math.min(3, words.get(target)!.length));
  }
  const nameAt = (target: string) => {
    const all = words.get(target)!;
    const level = levels.get(target)!;
    let base = camel(all.slice(Math.max(0, all.length - level)));
    if (!base || /^[0-9]/.test(base)) base = "fixture" + cap(base);
    return base + kindOf(target);
  };
  const claimed = new Set(pinned.values());
  for (let round = 0; round < 40; round++) {
    const byName = new Map<string, string[]>();
    for (const target of words.keys()) {
      const name = nameAt(target);
      byName.set(name, [...(byName.get(name) ?? []), target]);
    }
    let changed = false;
    for (const [name, group] of byName) {
      if (group.length > 1 || claimed.has(name)) {
        for (const target of group) {
          if (levels.get(target)! < words.get(target)!.length) {
            levels.set(target, levels.get(target)! + 1);
            changed = true;
          }
        }
      }
    }
    if (!changed) break;
  }
  const taken = new Set(claimed);
  for (const target of words.keys()) {
    let name = nameAt(target);
    let index = 2;
    while (taken.has(name)) name = nameAt(target).replace(/(Fixture|Schema|Contract|Document)$/, `$1${index++}`);
    taken.add(name);
    result.set(target, name);
  }
  for (const [target, name] of pinned) result.set(target, name);
  return result;
}

const usedIn = new Map<string, Use[]>();
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const { hits } = collect(files);
const planned: { hit: Hit; form: Form }[] = [];
const manual: string[] = [];
for (const hit of hits) {
  const form = classify(hit);
  const line = hit.text.slice(0, hit.start).split("\n").length;
  if (form.kind === "other" || hit.lit.endsWith("?raw") || form.kind === "dyn-bare") {
    manual.push(`${hit.file}:${line}\t${hit.target}`);
    continue;
  }
  planned.push({ hit, form });
  const mode: Mode = form.kind === "static" ? "eager" : form.kind === "url" || form.kind === "reader-path" ? "url" : "lazy";
  usedIn.set(hit.owner, [...(usedIn.get(hit.owner) ?? []), { owner: hit.owner, target: hit.target, mode }]);
}

const entries = new Map<string, { names: Map<string, string>; modes: Map<string, Set<Mode>>; lines: string[] }>();
for (const [owner, uses] of usedIn) {
  const { lines, existing } = readExistingEntry(owner);
  const names = assignNames(owner, uses.map(use => use.target).concat([...existing.keys()]), existing);
  const modes = new Map<string, Set<Mode>>();
  for (const use of uses) modes.set(use.target, (modes.get(use.target) ?? new Set()).add(use.mode));
  entries.set(owner, { names, modes, lines });
}

const specifierFor = (consumerFile: string, owner: string) => {
  const spec = relative(dirname(join(ROOT, consumerFile)), join(ROOT, owner, ENTRY));
  return spec.startsWith(".") ? spec : "./" + spec;
};
const exportNames = (name: string, mode: Mode) => (mode === "eager" ? name : mode === "lazy" ? "load" + cap(name) : name + "Url");

function entryText(owner: string): string {
  const entry = entries.get(owner)!;
  const entryPath = join(ROOT, owner, ENTRY);
  const body: string[] = [];
  const header = entry.lines.find(line => line.startsWith("/**"));
  const kept = entry.lines.filter(line => !line.startsWith("/**"));
  const keptNames = new Set<string>();
  for (const line of kept) {
    const match = /^export (?:\{ default as |const )([A-Za-z0-9_$]+)/.exec(line);
    if (match) keptNames.add(match[1]!);
  }
  const rel = (target: string) => {
    const value = relative(dirname(entryPath), join(ROOT, target));
    return value.startsWith(".") ? value : "./" + value;
  };
  const seen = new Set<string>(keptNames);
  for (const target of [...entry.modes.keys()].sort()) {
    const name = entry.names.get(target)!;
    for (const mode of ["eager", "lazy", "url"] as Mode[]) {
      if (!entry.modes.get(target)!.has(mode)) continue;
      if (seen.has(exportNames(name, mode))) continue;
      seen.add(exportNames(name, mode));
      body.push(
        mode === "eager"
          ? `export { default as ${name} } from "${rel(target)}" with { type: "json" };`
          : mode === "lazy"
            ? `export const ${exportNames(name, mode)} = () => import("${rel(target)}").then(module => module.default);`
            : `export const ${exportNames(name, mode)} = new URL("${rel(target)}", import.meta.url);`,
      );
    }
  }
  const moduleName = basename(owner).replace(/^[^A-Za-z0-9]+/u, "");
  const docstring = header ?? `/** 🧫 The ${moduleName} module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */`;
  return [docstring, ...kept, ...body].join("\n") + "\n";
}

function insertAfterImports(text: string, statement: string): string {
  const source = ts.createSourceFile("x.tsx", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  let end = -1;
  for (const node of source.statements) if (ts.isImportDeclaration(node)) end = node.end;
  if (end >= 0) return text.slice(0, end) + "\n" + statement + text.slice(end);
  const comments = ts.getLeadingCommentRanges(text, 0) ?? [];
  const at = comments.length ? comments[comments.length - 1]!.end : 0;
  return text.slice(0, at) + (at ? "\n" : "") + statement + "\n" + text.slice(at);
}

const report: string[] = [];
const edits = new Map<string, number>();

function rewrite(file: string): void {
  const abs = join(ROOT, file);
  const original = readFileSync(abs, "utf8");
  const current = collect([file]).hits.filter(hit => hit.file === file);
  const items: { hit: Hit; form: Form }[] = [];
  for (const hit of current) {
    const form = classify(hit);
    if (form.kind === "other" || hit.lit.endsWith("?raw") || form.kind === "dyn-bare") continue;
    items.push({ hit, form });
  }
  if (items.length === 0) return;
  const imports = new Map<string, string[]>();
  const spec = new Map<string, string>();
  const firstStatic = new Map<string, number>();
  const replacements: { from: number; to: number; text: string; owner: string; isStatic: boolean }[] = [];
  for (const { hit, form } of items) {
    const entry = entries.get(hit.owner);
    const name = entry?.names.get(hit.target);
    if (!entry || !name) throw new Error(`no registry for ${file} -> ${hit.target}`);
    const specifier = specifierFor(file, hit.owner);
    spec.set(hit.owner, specifier);
    const list = imports.get(hit.owner) ?? [];
    let text = "";
    let isStatic = false;
    if (form.kind === "static") {
      const specifierText = form.binding === name ? name : `${name} as ${form.binding}`;
      if (!list.includes(specifierText)) list.push(specifierText);
      isStatic = true;
    } else if (form.kind === "dyn-default-destructure") {
      const loader = exportNames(name, "lazy");
      if (!list.includes(loader)) list.push(loader);
      text = `${form.binding} = await ${loader}()`;
    } else if (form.kind === "dyn-paren-default") {
      const loader = exportNames(name, "lazy");
      if (!list.includes(loader)) list.push(loader);
      text = `(await ${loader}())`;
    } else if (form.kind === "url" || form.kind === "reader-path") {
      const url = exportNames(name, "url");
      if (!list.includes(url)) list.push(url);
      text = url;
    }
    imports.set(hit.owner, list);
    if (form.kind === "other" || form.kind === "dyn-bare") continue;
    replacements.push({ from: form.from, to: form.to, text, owner: hit.owner, isStatic });
  }
  const introduced = new Set<string>();
  for (const { hit, form } of items) {
    if (form.kind === "static") continue;
    const name = entries.get(hit.owner)!.names.get(hit.target)!;
    introduced.add(form.kind === "url" || form.kind === "reader-path" ? exportNames(name, "url") : exportNames(name, "lazy"));
  }
  for (const id of introduced) {
    const escaped = id.replace(/[$]/g, "\\$&");
    const occurrences = original.match(new RegExp(`(?<![\\w$])${escaped}(?![\\w$])`, "g"))?.length ?? 0;
    if (occurrences > 0) throw new Error(`identifier collision ${id} in ${file}`);
  }
  replacements.sort((a, b) => b.from - a.from);
  let text = original;
  const staticByOwner = new Map<string, number>();
  for (const rep of replacements) if (rep.isStatic) staticByOwner.set(rep.owner, Math.min(staticByOwner.get(rep.owner) ?? Infinity, rep.from));
  for (const rep of replacements) {
    let replacement = rep.text;
    if (rep.isStatic) {
      replacement = staticByOwner.get(rep.owner) === rep.from ? `import { ${imports.get(rep.owner)!.join(", ")} } from "${spec.get(rep.owner)}";` : "";
      const lineStart = text.lastIndexOf("\n", rep.from - 1) + 1;
      const lineEnd = text.indexOf("\n", rep.to);
      const wholeLine = text.slice(lineStart, rep.from).trim() === "" && text.slice(rep.to, lineEnd < 0 ? text.length : lineEnd).trim() === "";
      if (replacement === "" && wholeLine) {
        text = text.slice(0, lineStart) + text.slice(lineEnd < 0 ? text.length : lineEnd + 1);
        continue;
      }
    }
    text = text.slice(0, rep.from) + replacement + text.slice(rep.to);
  }
  for (const [owner, list] of imports) {
    if (staticByOwner.has(owner)) continue;
    text = insertAfterImports(text, `import { ${list.join(", ")} } from "${spec.get(owner)}";`);
  }
  const diagnostics = ts.transpileModule(text, { reportDiagnostics: true, compilerOptions: { jsx: ts.JsxEmit.Preserve, target: ts.ScriptTarget.ESNext }, fileName: file.endsWith("x") ? "a.tsx" : "a.ts" }).diagnostics ?? [];
  if (diagnostics.length) throw new Error(`syntax diagnostics after rewrite in ${file}: ${diagnostics.map(d => ts.flattenDiagnosticMessageText(d.messageText, " ")).join("; ")}`);
  if (mode === "apply") {
    const live = readFileSync(abs, "utf8");
    if (live !== original) throw new Error(`file changed while rewriting: ${file}`);
    const snap = join(SNAP, Buffer.from(file).toString("hex").slice(0, 60) + "-" + Bun.hash(file).toString(36) + ".txt");
    mkdirSync(SNAP, { recursive: true });
    if (!existsSync(snap)) writeFileSync(snap, original);
    appendIndex(snap, file);
    writeFileSync(abs, text);
  }
  edits.set(file, replacements.length);
}

function appendIndex(snap: string, file: string) {
  const index = join(SNAP, "index.tsv");
  const prior = existsSync(index) ? readFileSync(index, "utf8") : "";
  if (!prior.includes(`\t${file}\n`)) writeFileSync(index, prior + `${snap}\t${file}\n`);
}

const consumerFiles = [...new Set(planned.map(item => item.hit.file))].sort();
if (mode === "apply") {
  for (const owner of entries.keys()) {
    const path = join(ROOT, owner, ENTRY);
    mkdirSync(dirname(path), { recursive: true });
    const index = join(SNAP, "index.tsv");
    mkdirSync(SNAP, { recursive: true });
    if (existsSync(path)) {
      const snap = join(SNAP, "entry-" + Bun.hash(path).toString(36) + ".txt");
      if (!existsSync(snap)) writeFileSync(snap, readFileSync(path, "utf8"));
      appendIndex(snap, join(owner, ENTRY));
    } else {
      writeFileSync(index, (existsSync(index) ? readFileSync(index, "utf8") : "") + `NEW\t${join(owner, ENTRY)}\n`);
    }
    writeFileSync(path, entryText(owner));
  }
}
if (mode === "plan") writeFileSync(reportFile + ".entries.txt", [...entries.keys()].map(owner => `=== ${owner}/${ENTRY}\n${entryText(owner)}`).join("\n"));
const errors: string[] = [];
for (const file of consumerFiles) {
  try {
    rewrite(file);
  } catch (error) {
    errors.push(`${file}\t${(error as Error).message}`);
  }
}
report.push(`mode ${mode}`, `owners ${entries.size}`, `consumer files ${consumerFiles.length}`, `rewritten files ${edits.size}`, `replacements ${[...edits.values()].reduce((a, b) => a + b, 0)}`, `manual hits ${manual.length}`, `errors ${errors.length}`);
report.push("--owners", ...[...entries.keys()].map(owner => `${owner}\t${entries.get(owner)!.modes.size} targets`));
report.push("--manual", ...manual, "--errors", ...errors);
writeFileSync(reportFile, report.join("\n") + "\n");
console.log(report.slice(0, 7).join("\n"));

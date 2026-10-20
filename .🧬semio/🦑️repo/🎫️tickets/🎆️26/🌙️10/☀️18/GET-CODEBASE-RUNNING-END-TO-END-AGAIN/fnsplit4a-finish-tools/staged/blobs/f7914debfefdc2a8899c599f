/** 🟢️ The shared declarations of transformTestClosure. */
import { dirname, resolve } from "node:path";
import { rustTokenPairs, rustTokens } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { parseRustSourceTree, prunedUse, useLeafNames, type RustModuleNode } from "../../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../../📝️render/🟦️.ts";

/** 🧪️ Marker the fixture rewrite leaves in a path until the owning crate directory is known. */
export const FIXTURE_PLACEHOLDER = "@fixtures/";

/** 🔁️ Which test-closure rewrites a carve may apply. */
export type TestTransformOptions = Readonly<{ fixtures: boolean; aggregate: boolean; hoist: boolean; detach?: boolean; minimumAggregateGroup?: number }>;

/** 🧾️ Files a test-closure rewrite changes, keyed by absolute path. */
export type TestTransformResult = {
  rootSource: string;
  readonly edits: Map<string, string>;
  readonly creates: Map<string, string>;
  readonly deletes: Set<string>;
  readonly notes: string[];
};

export type TestMount = { readonly hostFile: string; readonly node: RustModuleNode; readonly testFile: string; readonly ownerPath: readonly string[]; readonly hostIsRoot: boolean };

export const FIXTURE_READER_IDENT = "semio_framework_test_fixtures";

/** 📥️ Adds `use semio_framework_test_fixtures::{names};` to the top of a file module, after its inner attributes and documentation, at the place rustfmt keeps its first group of `use` lines (`self`, `super`, `crate`, then by path) and before the first item otherwise; names that are already imported stay alone and a one-line import of the crate in the first group grows by the missing names. */
export function importFixtureMacros(text: string, names: readonly string[]): string {
  const imported = new RegExp(String.raw`^use\s+${FIXTURE_READER_IDENT}::(?:(\w+)|\{([^}]*)\});\s*$`, "u");
  const lines = text.split("\n");
  let at = 0;
  let lastInner = -1;
  for (let index = 0; index < lines.length; index += 1) {
    const current = lines[index]!;
    if (/^\s*\/\/!/u.test(current) || /^\s*#!\[.*\]\s*$/u.test(current)) lastInner = index;
    else if (current.trim() !== "" && !/^\s*\/\/(?![/!])/u.test(current)) break;
  }
  at = lastInner + 1;
  while (at < lines.length && lines[at]!.trim() === "") at += 1;
  let end = at;
  while (end < lines.length && /^use\s[^\n]*;\s*$/u.test(lines[end]!)) end += 1;
  const render = (all: readonly string[]): string => (all.length === 1 ? `use ${FIXTURE_READER_IDENT}::${all[0]};` : `use ${FIXTURE_READER_IDENT}::{${all.join(", ")}};`);
  const existing = lines.findIndex((line, index) => index >= at && index < end && imported.test(line));
  const have = new Set<string>();
  for (const line of lines) { const match = imported.exec(line); if (match) for (const name of (match[1] ?? match[2] ?? "").split(",").map((part) => part.trim()).filter(Boolean)) have.add(name); }
  const missing = [...new Set(names)].filter((name) => !have.has(name));
  if (!missing.length) return text;
  if (existing >= 0) {
    const match = imported.exec(lines[existing]!)!;
    lines[existing] = render([...new Set([...(match[1] ?? match[2] ?? "").split(",").map((part) => part.trim()).filter(Boolean), ...missing])].sort());
    return lines.join("\n");
  }
  const line = render([...missing].sort());
  if (end === at) {
    lines.splice(at, 0, ...(at >= lines.length ? [line] : [line, ""]));
    return lines.join("\n");
  }
  const rank = (use: string): number => { const head = /^use\s+(self|super|crate)\b/u.exec(use)?.[1]; return head === undefined ? 3 : ["self", "super", "crate"].indexOf(head); };
  const before = (use: string): boolean => rank(use) < rank(line) || (rank(use) === rank(line) && use.slice(4) < line.slice(4));
  let place = at;
  while (place < end && before(lines[place]!)) place += 1;
  lines.splice(place, 0, line);
  return lines.join("\n");
}

/** 📁️ Rewrites `include_str!` of test fixtures into runtime reads, so fixtures leave the compiled source closure: every embedded text of a file below a `🧪️tests` directory, and every embedded text whose own path lies below one. */
export function fixturesToRuntime(text: string, fileAbs: string, repoRoot: string): string | null {
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const edits: { start: number; end: number; text: string }[] = [];
  const names = new Set<string>();
  const definitionRanges: [number, number][] = [];
  const literal = (index: number): string | null => (tokens[index]?.kind === "string" ? tokens[index]!.text.slice(1, -1) : null);
  const target = (relative: string): string => `${FIXTURE_PLACEHOLDER}${relativePosix(repoRoot, resolve(dirname(fileAbs), relative))}`;
  const inTestFile = relativePosix(repoRoot, fileAbs).includes("/🧪️tests/");
  const isFixture = (relative: string): boolean => inTestFile || relativePosix(repoRoot, resolve(dirname(fileAbs), relative)).includes("/🧪️tests/");
  const literalMacros: [number, number][] = [];
  for (const [open, close] of pairs) if (tokens[open]!.text === "(" && tokens[open - 1]?.text === "!" && ["concat", "stringify"].includes(tokens[open - 2]?.text ?? "")) literalMacros.push([open, close]);
  const inLiteralMacro = (index: number): boolean => literalMacros.some(([open, close]) => open < index && index < close);
  const depth = new Array<number>(tokens.length).fill(0);
  let current = 0;
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text === "}") current -= 1;
    depth[i] = current;
    if (tokens[i]!.text === "{") current += 1;
  }
  const moduleDepth = new Array<number>(tokens.length).fill(0);
  const braces: boolean[] = [];
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text === "}") braces.pop();
    moduleDepth[i] = braces.filter(Boolean).length;
    if (tokens[i]!.text === "{") braces.push(tokens[i - 2]?.text === "mod" && tokens[i - 1]?.kind === "identifier");
  }
  const converted = new Set<number>();
  const used = new Set<string>();
  for (let i = 0; i + 6 < tokens.length; i += 1) {
    const macroName = tokens[i]!.text;
    if ((macroName !== "include_str" && macroName !== "include_bytes") || tokens[i + 1]!.text !== "!" || tokens[i + 2]!.text !== "(") continue;
    const helperName = macroName === "include_str" ? "fixture_text" : "fixture_bytes";
    const relative = literal(i + 3);
    if (relative === null || tokens[i + 4]!.text !== ")" || !isFixture(relative) || inLiteralMacro(i)) continue;
    let head = i - 1;
    const level = depth[i];
    const declaresItem = macroName === "include_str" && tokens[head]?.text === "=" && tokens[head - 1]?.text === "str" && tokens[head - 2]?.text === "&" && tokens[head - 3]?.text === ":" && tokens[head - 4]?.kind === "identifier" && (tokens[head - 5]?.text === "const" || tokens[head - 5]?.text === "static");
    let back = i - 1;
    let nesting = 0;
    for (; back >= 0 && depth[back] === depth[i]; back -= 1) {
      const word = tokens[back]!.text;
      if (word === "]" || word === ")") nesting += 1;
      else if (word === "[" || word === "(") nesting = Math.max(0, nesting - 1);
      else if (nesting === 0 && (word === ";" || word === "{" || word === "}" || word === "const" || word === "static")) break;
    }
    const owner = back >= 0 && depth[back] === depth[i] && (tokens[back]!.text === "const" || tokens[back]!.text === "static") ? back : -1;
    const prefix = "super::".repeat(moduleDepth[i]!);
    used.add(helperName);
    if (owner >= 0 && !(declaresItem && depth[i] === 0)) {
      if (!converted.has(owner)) {
        const nameIndex = tokens[owner + 1]?.text === "mut" ? owner + 2 : owner + 1;
        let equals = nameIndex + 2;
        let inner = 0;
        for (; equals < tokens.length; equals += 1) {
          const word = tokens[equals]!.text;
          if (word === "[" || word === "(" || word === "<") inner += 1;
          else if (word === "]" || word === ")" || word === ">") inner -= 1;
          else if (word === "=" && inner <= 0) break;
        }
        let end = equals + 1;
        let open = 0;
        for (; end < tokens.length; end += 1) {
          const word = tokens[end]!.text;
          if (word === "[" || word === "(" || word === "{") open += 1;
          else if (word === "]" || word === ")" || word === "}") open -= 1;
          else if (word === ";" && open === 0) break;
        }
        if (tokens[nameIndex]?.kind !== "identifier" || tokens[nameIndex + 1]?.text !== ":" || equals >= tokens.length || end >= tokens.length) return null;
        const name = tokens[nameIndex]!.text;
        const type = text.slice(tokens[nameIndex + 2]!.start, tokens[equals - 1]!.end);
        converted.add(owner);
        const byReference = tokens[equals + 1]?.text === "&";
        edits.push({ start: tokens[owner]!.start, end: tokens[equals]!.end, text: `static ${name}: std::sync::LazyLock<${type}> = std::sync::LazyLock::new(||${byReference ? " &*Box::leak(Box::new(" : ""}` });
        if (byReference) edits.push({ start: tokens[equals + 1]!.start, end: tokens[equals + 1]!.end, text: "" });
        edits.push({ start: tokens[end]!.start, end: tokens[end]!.end, text: byReference ? ")));" : ");" });
        names.add(name);
        definitionRanges.push([tokens[owner]!.start, tokens[equals]!.end]);
      }
      edits.push({ start: tokens[i]!.start, end: tokens[i + 4]!.end, text: `${prefix}${helperName}!("${target(relative)}")` });
      continue;
    }
    const isConst = declaresItem && depth[i] === 0;
    if (isConst) {
      const name = tokens[head - 4]!.text;
      let start = head - 5;
      if (tokens[start - 1]?.text === ")" && pairs.get(start - 1) !== undefined) start = pairs.get(start - 1)! - 1;
      else if (tokens[start - 1]?.text === "pub") start -= 1;
      const end = tokens[i + 5]?.text === ";" ? i + 5 : i + 4;
      edits.push({ start: tokens[start]!.start, end: tokens[end]!.end, text: `static ${name}: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("${target(relative)}"));` });
      names.add(name);
      definitionRanges.push([tokens[start]!.start, tokens[end]!.end]);
    } else if (level !== undefined && level >= 1) {
      edits.push({ start: tokens[i]!.start, end: tokens[i + 4]!.end, text: `${prefix}${helperName}!("${target(relative)}")` });
    }
  }
  if (!edits.length) return null;
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.kind !== "identifier" || !names.has(token.text) || definitionRanges.some(([a, b]) => token.start >= a && token.end <= b)) continue;
    if (tokens[i - 1]?.text === "." || tokens[i - 1]?.text === "::") continue;
    if (inLiteralMacro(i)) return null;
    edits.push({ start: token.start, end: token.end, text: `(*${token.text})` });
  }
  let out = text;
  for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return importFixtureMacros(`${out.trimEnd()}\n`, [...used]);
}

/** 🧭️ Rewrites the file-level `super::` paths of a test module into the absolute path of the module they name, so the test no longer needs its parent module to be in the same crate; a `pub(super)` restriction widens to `pub(crate)` because a `pub(in path)` restriction cannot be resolved through the glob re-export a mirror root makes. `moduleEff` is the physical module path of the test, so a private `component` segment counts as one `super` level and is dropped from the result. */
export function absolutizeSuperPaths(text: string, moduleEff: readonly string[]): string | null {
  const tree = parseRustSourceTree(text);
  const spans: { start: number; end: number; names: readonly string[] }[] = [];
  const collect = (nodes: readonly RustModuleNode[], trail: readonly string[]): void => {
    for (const node of nodes) if (node.inline) { const names = [...trail, node.name]; spans.push({ start: node.bodyStart, end: node.bodyEnd, names }); collect(node.children, names); }
  };
  collect(tree.modules, []);
  const tokens = rustTokens(text);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let k = 0; k < tokens.length; k += 1) {
    if (tokens[k]!.text !== "super" || tokens[k - 1]?.text === "::") continue;
    const inside = spans.filter((span) => tokens[k]!.start >= span.start && tokens[k]!.start < span.end).sort((a, b) => b.names.length - a.names.length)[0];
    const nested = inside?.names ?? [];
    let count = 1;
    let last = k;
    while (tokens[last + 1]?.text === "::" && tokens[last + 2]?.text === "super") { count += 1; last += 2; }
    if (count <= nested.length) continue;
    const above = count - nested.length;
    if (above > moduleEff.length) return null;
    const target = moduleEff.slice(0, moduleEff.length - above).filter((segment) => segment !== "component");
    const restriction = tokens[k - 1]?.text === "(" && tokens[k - 2]?.text === "pub" && tokens[last + 1]?.text === ")";
    edits.push({ start: tokens[k]!.start, end: tokens[last]!.end, text: restriction ? "crate" : ["crate", ...target].join("::") });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out.replace(/\bcrate((?:::[A-Za-z_][A-Za-z0-9_]*)*?)::component(?=::)/gu, "crate$1");
}

export function usedAsItem(tokens: readonly { kind: string; text: string }[], name: string): boolean {
  const upper = /^[A-Z]/u.test(name);
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text !== name) continue;
    const previous = tokens[i - 1]?.text;
    const next = tokens[i + 1]?.text;
    if (previous === "." || previous === "let" || previous === "mut" || previous === "fn" || previous === "struct" || previous === "enum" || previous === "const" || previous === "static" || previous === "type" || previous === "trait" || previous === "mod") continue;
    if (upper || next === "(" || next === "::" || next === "!" || previous === "::" || previous === "&" || previous === "(" || previous === "," || previous === "=" || previous === "->" || previous === ":") return true;
  }
  return false;
}

/** 🔎️ Whether a body reads the name an import of its host brings in: not a field (`x.mep`, `Row { mep: … }`), not an associated item or variant (`Node::Component`), not a name the body binds itself (`let walls = …`, `for (a, walls) in …`; `let Name::Variant(x) = …` is a pattern that reads `Name`); a module path from `crate`, `super` or `self` (`crate::a::Private`) does name the import. */
function usedAsImport(tokens: readonly { kind: string; text: string }[], name: string, hostPath: readonly string[] | null): boolean {
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text !== "for") continue;
    for (let at = i + 1; at < tokens.length && tokens[at]!.text !== "in" && tokens[at]!.text !== "{" && at < i + 24; at += 1) if (tokens[at]!.text === name) return false;
  }
  let used = false;
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text !== name) continue;
    const previous = tokens[i - 1]?.text;
    if ((previous === "let" || (previous === "mut" && tokens[i - 2]?.text === "let")) && [ "=", ":", ";" ].includes(tokens[i + 1]?.text ?? "")) return false;
    if (previous === "." || tokens[i + 1]?.text === ":") continue;
    if (previous === "::") {
      let head = i - 2;
      while (tokens[head - 1]?.text === "::" && tokens[head - 2] !== undefined) head -= 2;
      if (!["crate", "super", "self"].includes(tokens[head]?.text ?? "")) continue;
      if (hostPath !== null && tokens[head]!.text === "crate") {
        const segments: string[] = [];
        for (let at = head + 2; at < i; at += 2) segments.push(tokens[at]!.text);
        const wanted = hostPath.filter((segment) => segment !== "component");
        if (segments.length !== wanted.length || segments.some((segment, index) => segment !== wanted[index])) continue;
      }
    }
    used = true;
  }
  return used;
}

export function hostPrivate(hostText: string, bodyIdentifiers: ReadonlySet<string>, declaredInBody: ReadonlySet<string>, bodyText: string | null = null, hostPath: readonly string[] | null = null): { items: string[]; carry: string[]; relative: string[] } {
  const tree = parseRustSourceTree(hostText);
  const childModules = new Set(tree.modules.map((node) => node.name));
  const bodyTokens = bodyText === null ? null : rustTokens(bodyText);
  const items: string[] = [];
  const carry: string[] = [];
  const relative: string[] = [];
  for (const item of tree.items) {
    const restricted = item.visibility === "private" || item.visibility === "pub(super)" || item.visibility === "pub(self)";
    if (!restricted) continue;
    if (item.kind === "use") {
      const statement = hostText.slice(item.declStart, item.end);
      const leaves = useLeafNames(statement).filter((leaf) => bodyIdentifiers.has(leaf) && !declaredInBody.has(leaf) && (!bodyTokens || usedAsImport(bodyTokens, leaf, hostPath)));
      if (!leaves.length) continue;
      const pruned = prunedUse(statement, leaves);
      if (!pruned) continue;
      if (/^use\s+(?:self|super)::/u.test(pruned) || /\*\s*;\s*$/u.test(pruned) || childModules.has(/^use\s+(\w+)::/u.exec(pruned)?.[1] ?? "")) relative.push(pruned);
      else carry.push(pruned);
    } else if (item.kind !== "extern-crate" && item.kind !== "macro" && item.kind !== "impl" && item.name && bodyIdentifiers.has(item.name) && !declaredInBody.has(item.name) && (!bodyTokens || usedAsItem(bodyTokens, item.name))) items.push(item.name);
  }
  for (const node of tree.modules) {
    const restricted = node.visibility === "private" || node.visibility === "pub(super)" || node.visibility === "pub(self)";
    if (restricted && !node.conditionalTest && node.name !== "component" && bodyIdentifiers.has(node.name) && !declaredInBody.has(node.name) && (!bodyTokens || usedAsItem(bodyTokens, node.name))) items.push(node.name);
  }
  return { items, carry, relative };
}

/** ⬆️ Puts lines in front of a source body but behind its inner doc comments and inner attributes, which must stay first. */
export function prependAfterInner(body: string, lines: readonly string[]): string {
  if (!lines.length) return body;
  const rows = body.split("\n");
  let cut = 0;
  while (cut < rows.length && (/^\s*\/\/!/u.test(rows[cut]!) || /^\s*#!\[/u.test(rows[cut]!) || (rows[cut]!.trim() === "" && cut > 0 && /^\s*(?:\/\/!|#!\[)/u.test(rows[cut - 1]!)))) cut += 1;
  return [...rows.slice(0, cut), ...lines, ...rows.slice(cut)].join("\n");
}

/** 🔓️ Edits that make the named restricted items and modules of a host file `pub`, or null when one of them cannot be found. */
export function widenHostItems(hostText: string, names: readonly string[]): { start: number; end: number; text: string }[] | null {
  const tree = parseRustSourceTree(hostText);
  const edits: { start: number; end: number; text: string }[] = [];
  for (const name of names) {
    const target = tree.items.find((item) => item.name === name && item.kind !== "use" && item.kind !== "impl" && item.kind !== "macro") ?? tree.modules.find((node) => node.name === name);
    if (!target) return null;
    const visibility = /^pub(?:\([^)]*\))?\s*/u.exec(hostText.slice(target.declStart, target.declStart + 40));
    edits.push({ start: target.declStart, end: target.declStart + (visibility ? visibility[0].length : 0), text: "pub " });
  }
  return edits;
}

export function declaredNames(body: string, bindsHost = false): Set<string> {
  const names = new Set<string>();
  for (const item of parseRustSourceTree(body).items) {
    if (item.name && item.kind !== "impl") names.add(item.name);
    if (item.kind !== "use") continue;
    const statement = body.slice(item.declStart, item.end);
    if (bindsHost && /^\s*(?:#\[[^\]]*\]\s*)*use\s+super::/u.test(statement)) continue;
    for (const leaf of useLeafNames(statement)) names.add(leaf);
  }
  return names;
}

export function snake(name: string): string {
  return name.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^A-Za-z0-9]+/gu, "_").replace(/^_+|_+$/gu, "").toLowerCase();
}

export function discoverMounts(rootFile: string, rootSource: string, readText: (path: string) => string | null): { mounts: TestMount[]; inline: Map<string, { node: RustModuleNode; base: string; file: string }> } {
  const mounts: TestMount[] = [];
  const inline = new Map<string, { node: RustModuleNode; base: string; file: string }>();
  const visit = (nodes: readonly RustModuleNode[], file: string, base: string, inFileScope: boolean, trail: readonly string[], isRoot: boolean): void => {
    for (const node of nodes) {
      const path = [...trail, node.name];
      if (node.inline) {
        inline.set(path.join("::"), { node, base: resolve(base, node.pathAttribute ?? node.name), file });
        visit(node.children, file, resolve(base, node.pathAttribute ?? node.name), false, path, isRoot);
        continue;
      }
      if (node.pathAttribute === null) continue;
      const child = resolve(inFileScope ? dirname(file) : base, node.pathAttribute);
      if (node.conditionalTest) { mounts.push({ hostFile: file, node, testFile: child, ownerPath: trail, hostIsRoot: isRoot }); continue; }
      const text = readText(child);
      if (text === null) continue;
      visit(parseRustSourceTree(text).modules, child, dirname(child), true, path, false);
    }
  };
  visit(parseRustSourceTree(rootSource).modules, rootFile, dirname(rootFile), true, [], true);
  return { mounts, inline };
}

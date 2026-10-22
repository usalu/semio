import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 🕸️ Lexes WIT: line comments, doc comments and nested block comments dropped, identifiers (kebab case, `%` escapes), semantic versions after `@`, punctuation. Returns an error for an unterminated comment or an unknown character. */
export function lexWitV1(text: string): {readonly tokens: readonly string[]; readonly error: string | null} {
  const tokens: string[] = [];
  let at = 0, error: string | null = null;
  const n = text.length;
  while (at < n) {
    const c = text[at]!;
    if (/\s/.test(c)) {at++; continue;}
    if (c === "/" && text[at + 1] === "/") {while (at < n && text[at] !== "\n") at++; continue;}
    if (c === "/" && text[at + 1] === "*") {
      let depth = 1; at += 2;
      while (at < n && depth > 0) {if (text[at] === "/" && text[at + 1] === "*") {depth++; at += 2;} else if (text[at] === "*" && text[at + 1] === "/") {depth--; at += 2;} else at++;}
      if (depth > 0) {error ??= "unterminated block comment"; break;}
      continue;
    }
    const version = /^@\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?/.exec(text.slice(at, at + 80));
    if (version) {tokens.push(version[0]!); at += version[0]!.length; continue;}
    const identifier = /^%?[A-Za-z][A-Za-z0-9-]*/.exec(text.slice(at, at + 200));
    if (identifier) {tokens.push(identifier[0]!); at += identifier[0]!.length; continue;}
    const number = /^\d+/.exec(text.slice(at, at + 40));
    if (number) {tokens.push(number[0]!); at += number[0]!.length; continue;}
    if (c === "-" && text[at + 1] === ">") {tokens.push("->"); at += 2; continue;}
    if ("{}()<>,:;=.@/*".includes(c)) {tokens.push(c); at++; continue;}
    error ??= `unexpected character ${JSON.stringify(c)}`; at++;
  }
  return {tokens, error};
}

/** 🕸️ One top-level or nested item: its leading keyword, its name when it has one, and its canonical text. */
export type WitItemV1 = {readonly keyword: string; readonly name: string; readonly text: string; readonly names: readonly string[]; readonly references: readonly WitReferenceV1[]};
/** 🕸️ A reference to a package from a `use`, `import`, `export` or `include`. */
export type WitReferenceV1 = {readonly package: string; readonly version: string | null; readonly interface: string; readonly imports: readonly string[]};
const BLOCK_KEYWORDS = new Set(["record", "variant", "enum", "flags", "resource", "interface", "world"]);

const balancedEnd = (tokens: readonly string[], start: number): number => {
  let depth = 0;
  for (let index = start; index < tokens.length; index++) {const token = tokens[index]!; if (token === "{" || token === "(" || token === "<") depth++; else if (token === "}" || token === ")" || token === ">") {depth--; if (depth === 0) return index;}}
  return tokens.length - 1;
};
const itemEnd = (tokens: readonly string[], start: number): number => {
  let index = start;
  while (tokens[index] === "@") {index += 2; if (tokens[index] === "(") index = balancedEnd(tokens, index) + 1;}
  const keyword = tokens[index] ?? "";
  let depth = 0;
  for (let at = index; at < tokens.length; at++) {
    const token = tokens[at]!;
    if (token === "{" || token === "(" || token === "<") depth++;
    else if (token === "}" || token === ")" || token === ">") {depth--; if (depth === 0 && token === "}" && (BLOCK_KEYWORDS.has(keyword) || tokens[at + 1] !== ";" && /^(import|export)$/.test(keyword) && /interface/.test(tokens.slice(index, at).join(" ")))) return tokens[at + 1] === ";" ? at + 1 : at;}
    else if (token === ";" && depth === 0) return at;
  }
  return tokens.length - 1;
};
const referenceOf = (tokens: readonly string[]): WitReferenceV1 | undefined => {
  const at = tokens.findIndex((token, index) => /^[a-z][a-z0-9-]*$/.test(token) && tokens[index + 1] === ":" && /^[a-z][a-z0-9-]*$/.test(tokens[index + 2] ?? "") && tokens[index + 3] === "/");
  if (at < 0) return undefined;
  let index = at + 4;
  const iface = tokens[index++] ?? "", version = tokens[index]?.startsWith("@") ? tokens[index++]!.slice(1) : null, imports: string[] = [];
  if (tokens[index] === "." && tokens[index + 1] === "{") for (let cursor = index + 2; cursor < tokens.length && tokens[cursor] !== "}"; cursor++) {if (tokens[cursor] !== ",") imports.push(tokens[cursor]!); if (tokens[cursor + 1] === "as") cursor += 2;}
  return {package: `${tokens[at]}:${tokens[at + 2]}`, version, interface: iface, imports};
};
const exportedNames = (keyword: string, tokens: readonly string[], inner: readonly WitItemV1[]): string[] => {
  const names: string[] = [];
  for (const item of inner) if (item.name && item.keyword !== "use" && item.keyword !== "import" && item.keyword !== "export" && item.keyword !== "include") names.push(item.name);
  void keyword; void tokens;
  return names;
};
/** 🕸️ Parses the items of a token list (sorted by text inside interfaces and worlds: item order is not part of a WIT package, order inside records, variants, enums, flags and function signatures is kept). */
export function parseWitItemsV1(tokens: readonly string[], nested = false): readonly WitItemV1[] {
  const items: WitItemV1[] = [];
  for (let at = 0; at < tokens.length;) {
    const end = itemEnd(tokens, at), slice = tokens.slice(at, end + 1);
    let index = 0;
    while (slice[index] === "@") {index += 2; if (slice[index] === "(") index = balancedEnd(slice, index) + 1;}
    const keyword = slice[index] ?? "", named = (slice[index + 1] ?? "").replace(/^%/, ""), isFunction = slice[index + 1] === ":" && slice[index + 2] === "func";
    const bodyStart = slice.indexOf("{", index), hasBody = (keyword === "interface" || keyword === "world") && bodyStart >= 0;
    let text = slice.join(" "), names: string[] = [], references: WitReferenceV1[] = [], name = keyword === "use" || keyword === "include" || keyword === "package" ? "" : isFunction ? slice[index]!.replace(/^%/, "") : keyword === "type" || BLOCK_KEYWORDS.has(keyword) || keyword === "import" || keyword === "export" ? named : "";
    if (hasBody) {
      const inner = parseWitItemsV1(slice.slice(bodyStart + 1, slice.lastIndexOf("}")), true), sorted = [...inner].sort((a, b) => compareCodePointsV1(a.text, b.text));
      text = `${slice.slice(0, bodyStart).join(" ")} { ${sorted.map(item => item.text).join(" ")} }`;
      names = exportedNames(keyword, slice, inner); references = inner.flatMap(item => item.references);
    }
    if (keyword === "use" || keyword === "include" || keyword === "import" || keyword === "export") {const reference = referenceOf(slice); if (reference) references = [...references, reference];}
    if (keyword === "package") name = "";
    items.push({keyword, name: isFunction || keyword !== "use" ? name : "", text, names, references});
    at = end + 1;
  }
  void nested;
  return items;
};

/** 🕸️ The canonical text of a package: the `package` declaration, then every item of every file sorted by canonical text. Files of a package are merged, their order is irrelevant. */
export function canonicalWitV1(files: readonly string[]): {readonly package: string | null; readonly text: string; readonly items: readonly WitItemV1[]; readonly error: string | null} {
  const items: WitItemV1[] = [];
  let error: string | null = null, declaration: string | null = null;
  for (const file of files) {
    const lexed = lexWitV1(file);
    error ??= lexed.error;
    let tokens = lexed.tokens;
    if (tokens[0] === "package") {const end = tokens.indexOf(";"); const stop = end < 0 ? tokens.length : end; declaration = declaration ?? tokens.slice(1, stop).join(""); tokens = tokens.slice(stop + 1);}
    const depth = tokens.reduce((sum, token) => sum + (token === "{" ? 1 : token === "}" ? -1 : 0), 0);
    if (depth !== 0) error ??= "unbalanced braces";
    items.push(...parseWitItemsV1(tokens));
  }
  const sorted = [...items].sort((a, b) => compareCodePointsV1(a.text, b.text));
  return {package: declaration, text: [declaration === null ? "" : `package ${declaration} ;`, ...sorted.map(item => item.text)].filter(Boolean).join("\n"), items: sorted, error};
}
/** 🕸️ The interface digest of a package. */
export const witDigestV1 = (files: readonly string[]): string => sha256V1("polyglot-wit-world-v1\n" + canonicalWitV1(files).text);
/** 🕸️ What dependents read of a package: its exported interfaces and worlds with the names they define. */
export function witArtifactV1(files: readonly string[]): string {
  const parsed = canonicalWitV1(files), exported: Record<string, {kind: string; names: string[]}> = {};
  for (const item of parsed.items) if ((item.keyword === "interface" || item.keyword === "world") && item.name) exported[item.name] = {kind: item.keyword, names: sortedCodePointsV1(item.names)};
  return JSON.stringify({package: parsed.package, items: Object.fromEntries(sortedCodePointsV1(Object.keys(exported)).map(name => [name, exported[name]!]))});
}

export type WitWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string};
/** 🕸️ Plans the unit graph of WIT packages: every directory with `.wit` files is a package (`deps` subdirectories hold vendored packages, planned as units of their own; a package declared twice gets the directory in its id), references between packages become `interface` edges, the digest is that of the canonical text. */
export function planWitUnitsV1(snapshot: WitWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly toolchain: string}): {readonly graph: UnitGraphV1; readonly packages: readonly {readonly directory: string; readonly package: string | null; readonly id: string}[]} {
  const byDirectory = new Map<string, string[]>();
  for (const path of sortedCodePointsV1(snapshot.files.filter(file => file.endsWith(".wit")))) {const directory = path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : ""; byDirectory.set(directory, [...(byDirectory.get(directory) ?? []), path]);}
  const rows = [...byDirectory.entries()].map(([directory, files]) => ({directory, files, parsed: canonicalWitV1(files.map(file => snapshot.read(file)))}));
  const count = new Map<string, number>();
  for (const row of rows) count.set(row.parsed.package ?? "", (count.get(row.parsed.package ?? "") ?? 0) + 1);
  const idOf = (row: typeof rows[number]): string => row.parsed.package !== null && count.get(row.parsed.package) === 1 ? `wit:${row.parsed.package}` : `wit:${row.parsed.package ?? "anonymous"}#${row.directory}`;
  const byPackage = new Map<string, typeof rows>();
  for (const row of rows) if (row.parsed.package !== null) byPackage.set(row.parsed.package.replace(/@.*$/, ""), [...(byPackage.get(row.parsed.package.replace(/@.*$/, "")) ?? []), row]);
  const units: UnitV1[] = rows.map(row => {
    const dependencies = new Map<string, UnitDependencyV1>(), own = row.parsed.package?.replace(/@.*$/, "");
    for (const item of row.parsed.items) for (const reference of item.references) {
      if (reference.package === own) continue;
      const candidates = byPackage.get(reference.package) ?? [], hit = candidates.find(candidate => reference.version === null || candidate.parsed.package === `${reference.package}@${reference.version}`) ?? candidates[0];
      if (hit && hit !== row) dependencies.set(idOf(hit), {unit: idOf(hit), use: "interface"});
    }
    return {
      id: idOf(row), language: "wit", kind: "library", label: row.parsed.package ?? row.directory, sources: [...row.files], inputs: [], dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: [],
      interface: {kind: "wit-world"}, outputs: ["package.json"], toolchain: {id: options.toolchain, parts: []}, action: {argv: ["wit", "check", ...row.files], env: {SEMIO_WIT_DIRECTORY: row.directory}},
    };
  });
  return {graph: {version: 1, units}, packages: rows.map(row => ({directory: row.directory, package: row.parsed.package, id: idOf(row)}))};
}

/** 🏭️ Executes a WIT unit with the owned parser: lexical validity, balanced items, and every `use`/`import`/`include` of a dependency package resolved against the dependency's interface artifact (the interface must exist and define the names a `use` imports). The interface digest is the canonical-text digest, the artifact the exported names. */
export function witUnitExecutorV1(options: {readonly workspace: string; readonly read: (path: string) => string}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  return async request => {
    const {unit} = request, started = performance.now(), files = unit.sources.map(path => options.read(path)), parsed = canonicalWitV1(files), diagnostics: string[] = [];
    if (parsed.error) diagnostics.push(parsed.error);
    const artifacts = new Map<string, {package: string; items: Record<string, {names: string[]}>}>();
    for (const dependency of request.dependencies) if (dependency.artifact) {const artifact = JSON.parse(dependency.artifact) as {package: string; items: Record<string, {names: string[]}>}; artifacts.set(artifact.package.replace(/@.*$/, ""), artifact);}
    const own = parsed.package?.replace(/@.*$/, "");
    for (const item of parsed.items) for (const reference of item.references) {
      if (reference.package === own) continue;
      const artifact = artifacts.get(reference.package);
      if (!artifact) continue;
      const target = artifact.items[reference.interface];
      if (!target) {diagnostics.push(`${reference.package} has no interface or world ${reference.interface}`); continue;}
      for (const name of reference.imports) if (!target.names.includes(name.replace(/^%/, ""))) diagnostics.push(`${reference.package}/${reference.interface} defines no ${name}`);
    }
    return {ok: diagnostics.length === 0, diagnostics: diagnostics.join("\n"), outputs: sha256V1(parsed.text), interface: witDigestV1(files), artifact: witArtifactV1(files), milliseconds: Math.round(performance.now() - started)};
  };
}

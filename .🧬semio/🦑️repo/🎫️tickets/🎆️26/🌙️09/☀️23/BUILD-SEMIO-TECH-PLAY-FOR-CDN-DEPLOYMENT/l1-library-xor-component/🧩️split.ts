/** 🧩️ Wave L1 codemod: every dual crate-type manifest becomes a library (rlib) plus, where it ships, one thin `🧩️component` cdylib crate.
 * Usage: `bun 🧩️split.ts [--apply]` (dry run prints the plan). */
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { inventory, repoRoot, type Row } from "./🗺️inventory.ts";

const apply = process.argv.includes("--apply");
const COMPONENT_FACET = "🧩️component";
const MOVE_TABLE = /^\[\[?package\.metadata\.(component|semio\.playground|semio\.assets|semio\.sources|wasm-pack\.[^\]]+)\]\]?$/;
const MOVE_SEMIO_KEYS = new Set(["deployment-directory", "component-kind", "extends", "contributes", "depends-on", "consumes", "host", "capabilities"]);
const ENTRY_FEATURES = new Set(["plugin-entry", "plugin-root", "component-guest", "extension-entry", "component-app-assembly-root"]);
const RENAMES: Record<string, string> = {
  "semio-s-plugin-flow-extension-bim": "semio-s-plugin-flow-bim",
  "semio-s-plugin-flow-extension-brep": "semio-s-plugin-flow-brep",
  "semio-s-plugin-flow-extension-dictionary": "semio-s-plugin-flow-dictionary",
  "semio-s-plugin-flow-extension-draw": "semio-s-plugin-flow-draw",
  "semio-s-plugin-flow-extension-list": "semio-s-plugin-flow-list",
  "semio-s-plugin-flow-extension-logic": "semio-s-plugin-flow-logic",
  "semio-s-plugin-flow-extension-math": "semio-s-plugin-flow-math",
  "semio-s-plugin-flow-extension-primitive": "semio-s-plugin-flow-primitive",
  "semio-s-plugin-flow-extension-text": "semio-s-plugin-flow-text",
  "semio-s-plugin-cad-aec-building-energy": "semio-s-plugin-cad-aec-energy",
  "semio-s-plugin-cad-aec-building-structure": "semio-s-plugin-cad-aec-structure",
};

const KNOWN: Record<string, string> = {
  "semio-framework-plugin": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
  "semio-framework-async": "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
  "semio-framework-async-macros": "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
};
type Chunk = { header: string | null; lines: string[] };
const writes = new Map<string, string>();
const moves: [string, string][] = [];
const log: string[] = [];

const abs = (path: string): string => resolve(repoRoot, path);
const read = (path: string): string => writes.get(abs(path)) ?? readFileSync(abs(path), "utf8");
const write = (path: string, text: string): void => { writes.set(abs(path), text); };
const rel = (from: string, to: string): string => relative(abs(from), abs(to)).replaceAll("\\", "/");
const ident = (name: string): string => name.replaceAll("-", "_");
const emojiOf = (path: string): string => basename(path).match(/^[^a-z0-9]+/u)?.[0] ?? "🧩️";

/** ✂️ Splits a manifest into header-led chunks; leading comments attach to the next header, `# /` region ends to the previous one. */
function chunks(text: string): Chunk[] {
  const out: Chunk[] = [{ header: null, lines: [] }];
  for (const line of text.replace(/\n$/, "").split("\n")) {
    if (/^\[\[?[^\]]+\]\]?\s*$/.test(line)) {
      const previous = out[out.length - 1]!, carried: string[] = [];
      while (previous.lines.length && /^#(?! \/)/.test(previous.lines[previous.lines.length - 1]!)) carried.unshift(previous.lines.pop()!);
      out.push({ header: line.trim(), lines: [...carried, line] });
    } else out[out.length - 1]!.lines.push(line);
  }
  return out;
}

const join_ = (parts: Chunk[]): string => parts.map((part) => part.lines.join("\n")).join("\n").replace(/\n{3,}/g, "\n\n").replace(/^\n+/, "").replace(/\n*$/, "\n");
const trimBlank = (lines: string[]): string[] => { const copy = [...lines]; while (copy.length && !copy[copy.length - 1]!.trim()) copy.pop(); return copy; };

/** 🧷️ Splits one `[package.metadata.semio]` chunk into kept and moved key lines (multi-line arrays stay whole). */
function splitSemio(chunk: Chunk): { kept: string[]; moved: string[] } {
  const kept: string[] = [], moved: string[] = [];
  let target: string[] | null = null, depth = 0;
  for (const line of chunk.lines) {
    if (depth > 0) { target!.push(line); depth += (line.match(/\[/g)?.length ?? 0) - (line.match(/\]/g)?.length ?? 0); continue; }
    const key = line.match(/^([a-z-]+)\s*=/)?.[1];
    target = key && MOVE_SEMIO_KEYS.has(key) ? moved : kept;
    target.push(line);
    if (key) depth = (line.match(/\[/g)?.length ?? 0) - (line.match(/\]/g)?.length ?? 0) - (line.startsWith("[") ? 1 : 0);
  }
  return { kept, moved };
}

/** 🧹️ Removes every entry-selection feature and its references from a `[features]` chunk; returns null when nothing remains. */
function stripFeatures(chunk: Chunk, extra: readonly string[]): Chunk | null {
  const drop = new Set([...ENTRY_FEATURES, ...extra]);
  const lines: string[] = [];
  for (const line of chunk.lines) {
    const key = line.match(/^([a-z0-9-]+)\s*=/)?.[1];
    if (key && drop.has(key)) continue;
    if (key === "default") {
      const kept = [...line.matchAll(/"([^"]+)"/g)].map((match) => match[1]!).filter((name) => !drop.has(name));
      if (kept.length) lines.push(`default = [${kept.map((name) => JSON.stringify(name)).join(", ")}]`);
      continue;
    }
    lines.push(key ? line.replace(/,?\s*"semio-framework-plugin\/component-guest"/g, "").replace(/\[\s*,\s*/, "[") : line);
  }
  const body = lines.filter((line) => /^[a-z0-9-]+\s*=/.test(line));
  if (!body.length) return null;
  return { header: chunk.header, lines: lines.filter((line, index) => !(index > 0 && /^#/.test(line) && /plugin-entry|plugin-root|extension-entry|component-guest|entry point/i.test(line))) };
}

/** 🔌️ Drops `component-guest` from every dependency feature list and turns deps only an entry feature enabled into plain deps. */
function stripDependencyFeatures(text: string, optionalEnabled: Set<string>): string {
  return text.split("\n").map((line) => {
    let next = line.replace(/features\s*=\s*\["component-guest"\]\s*,?\s*/g, "").replace(/"component-guest"\s*,\s*/g, "").replace(/,\s*"component-guest"/g, "");
    const name = next.match(/^([a-zA-Z0-9_-]+)\s*=\s*\{/)?.[1];
    if (name && optionalEnabled.has(name)) next = next.replace(/,\s*optional\s*=\s*true/, "").replace(/optional\s*=\s*true\s*,\s*/, "");
    return next.replace(/,\s*\}/, " }").replace(/\{\s*,/, "{");
  }).join("\n");
}

/** 📍️ The workspace root manifest of a crate (explicit `package.workspace` or the nearest ancestor `[workspace]`). */
function workspaceOf(crateDir: string): string {
  const text = read(join(crateDir, "Cargo.toml")), declared = text.match(/^workspace\s*=\s*"([^"]+)"/m)?.[1];
  if (declared) return relative(repoRoot, resolve(abs(crateDir), declared, "Cargo.toml"));
  for (let directory = dirname(abs(crateDir)); directory.startsWith(repoRoot); directory = dirname(directory)) {
    const candidate = join(directory, "Cargo.toml");
    if (existsSync(candidate) && /^\[workspace\]/m.test(readFileSync(candidate, "utf8"))) return relative(repoRoot, candidate);
  }
  throw new Error(`No workspace for ${crateDir}`);
}

/** ➕️ Inserts a member into a workspace's explicit `members` list, keeping the list's order (sorted insertion). */
function addMember(workspace: string, member: string): void {
  const text = read(workspace), start = text.indexOf("members = ["), end = text.indexOf("]", start);
  if (start < 0) throw new Error(`No members list in ${workspace}`);
  const body = text.slice(start + "members = [".length, end), rows = body.split("\n").filter((line) => line.trim());
  const entry = `    "${member}",`;
  if (rows.includes(entry)) return;
  const sorted = [...rows, entry].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  write(workspace, text.slice(0, start) + "members = [\n" + sorted.join("\n") + "\n" + text.slice(end));
}

/** 🔗️ The dependency spec a new crate uses for `name`: the workspace row when its workspace declares one, else a path relative to `crateDir`. */
function dependencySpec(workspace: string, crateDir: string, name: string, absolutePath: string, extra = ""): string {
  const declared = new RegExp(`^${name.replace(/[-]/g, "\\-")}\\s*=`, "m").test(read(workspace).split("[workspace.dependencies]")[1] ?? "");
  return declared ? `${name} = { workspace = true${extra} }` : `${name} = { path = "${relative(abs(crateDir), absolutePath)}"${extra} }`;
}

/** 🧭️ Absolute directory of a dependency named in a library manifest (path dep or workspace row). */
function dependencyDirectory(row: Row, workspace: string, name: string): string {
  const parsed = Bun.TOML.parse(read(row.manifest)) as any, deps = { ...parsed["dev-dependencies"], ...parsed.dependencies };
  for (const [key, spec] of Object.entries<any>(deps)) {
    if ((spec?.package ?? key) !== name) continue;
    if (spec.path) return resolve(abs(row.crateDir), spec.path);
    if (spec.workspace) { const ws = Bun.TOML.parse(read(workspace)) as any, row_ = Object.entries<any>(ws.workspace.dependencies).find(([k, v]) => (v.package ?? k) === name)![1]; return resolve(dirname(abs(workspace)), row_.path); }
  }
  const ws = Bun.TOML.parse(read(workspace)) as any, hit = Object.entries<any>(ws.workspace?.dependencies ?? {}).find(([k, v]) => (v.package ?? k) === name);
  if (hit) return resolve(dirname(abs(workspace)), hit[1].path);
  if (KNOWN[name]) return abs(KNOWN[name]!);
  throw new Error(`${row.name} names no ${name}`);
}

/** 🗒️ The `[package]` header of a new component crate, mirroring the library's version/edition lines. */
function packageHeader(row: Row, workspace: string, componentCrate: string, name: string, description: string): string[] {
  const lines = chunks(read(row.manifest)).find((chunk) => chunk.header === "[package]")!.lines.slice(1).filter((line) => /^(version|edition|rust-version|publish|license)/.test(line));
  const wsRel = relative(abs(componentCrate), dirname(abs(workspace))) || ".";
  return ["[package]", `workspace = "${wsRel}"`, `name = "${name}"`, ...lines, `description = ${JSON.stringify(description)}`];
}

/** 🔁️ Rebases the crate-relative paths of moved metadata (sources, publications) onto the component crate directory. */
function rebase(lines: string[], from: string, to: string): string[] {
  return lines.map((line) => {
    if (/^artifacts\s*=/.test(line)) return line.replace(/"([^"]+)"/g, (_, path) => JSON.stringify(relative(abs(to), resolve(abs(from), path))));
    if (/^manifest\s*=/.test(line)) return line.replace(/"([^"]+)"/, (_, path) => JSON.stringify(relative(abs(to), resolve(abs(from), path))));
    return line;
  });
}

/** 🔌️ Removes the export macro invocation (and its entry cfg attribute) from a library source and opens what it named to the
 * component crate: an enclosing guest module becomes `pub` without its entry cfg, named modules become `pub mod`, named fns `pub fn`.
 * Returns the invocation's arguments, qualified relative to the crate root. */
function removeExports(file: string, owner: string): string[] {
  let text = read(file);
  const match = text.match(/(?:^[ \t]*#\[cfg\([^\n]*\)\]\n)?^([ \t]*)semio_framework_plugin::(plugin|extension)_exports!\s*\(([\s\S]*?)\);\n?/m);
  if (!match) throw new Error(`No exports in ${file}`);
  const before = text.slice(0, text.indexOf(match[0]));
  text = text.replace(match[0], "");
  let prefix = "";
  if (match[1]) {
    const enclosing = [...before.matchAll(/^(#\[cfg\(feature = "component-guest"\)\]\n)?mod ([a-z_]+) \{$/gm)].pop();
    if (!enclosing) throw new Error(`Indented exports outside a guest module in ${file}`);
    prefix = `${enclosing[2]}::`;
    text = text.replace(enclosing[0], `pub mod ${enclosing[2]} {`);
  }
  const args = match[3]!.split(",").map((part) => part.trim()).filter(Boolean);
  const open = (path: string, source: string): string => {
    const head = path.split("::")[0]!;
    if (path.includes("::")) return source.replace(new RegExp(`^mod ${head};`, "m"), `pub mod ${head};`);
    return source.replace(new RegExp(`^(#\\[cfg\\(all\\(target_arch = "wasm32", feature = "extension-entry"\\)\\)\\]\\n)?([ \\t]*)fn ${head}\\(`, "m"), "$2pub fn " + head + "(");
  };
  for (const argument of args.slice(1)) text = open(argument, text);
  write(file, text);
  const root = join(owner, "🦀️.rs");
  if (abs(root) !== abs(file) && existsSync(abs(root))) { let rootText = read(root); for (const argument of args.slice(1)) rootText = open(argument, rootText); write(root, rootText); }
  return args.map((argument, index) => index === 0 ? argument : prefix + argument);
}

/** 🧾️ Resolves an export argument against the library crate: wiring-local paths gain the crate prefix. */
function qualify(argument: string, crate: string): string {
  return argument.startsWith("semio_framework_plugin::") ? argument : `${crate}::${argument}`;
}

function splitPluginComponent(row: Row): void {
  const library = RENAMES[row.name] ?? row.name, componentName = `${library}-component`;
  if (componentName.length > 48) throw new Error(`${componentName} exceeds 48 characters`);
  const componentDir = join(row.owner, COMPONENT_FACET), componentCrate = join(componentDir, "📦️packages/🦀️rust");
  if (existsSync(abs(componentDir))) throw new Error(`${componentDir} already exists`);
  const workspace = workspaceOf(row.crateDir), source = read(row.manifest), parts = chunks(source);
  const optionalEnabled = new Set<string>();
  for (const part of parts) if (part.header === "[features]") for (const line of part.lines) { const key = line.match(/^([a-z0-9-]+)\s*=/)?.[1]; if (key && ENTRY_FEATURES.has(key)) for (const m of line.matchAll(/"dep:([^"]+)"/g)) optionalEnabled.add(m[1]!); }
  const kept: Chunk[] = [], moved: string[][] = [];
  let semioMoved: string[] = [], role = row.role ?? "hub";
  for (const part of parts) {
    if (part.header && MOVE_TABLE.test(part.header)) { moved.push(trimBlank(rebase(part.lines, row.crateDir, componentCrate))); continue; }
    if (part.header === "[package.metadata.semio]") { const split = splitSemio(part); semioMoved = split.moved; kept.push({ header: part.header, lines: split.kept }); continue; }
    if (part.header === "[lib]") { kept.push({ header: part.header, lines: part.lines.filter((line) => !/^crate-type\s*=/.test(line)) }); continue; }
    if (part.header === "[features]") { const stripped = stripFeatures(part, row.name === "semio-hub-stdio" ? ["component-app-assembly"] : []); if (stripped) kept.push(stripped); continue; }
    if (part.header === "[package]" && RENAMES[row.name]) { kept.push({ header: part.header, lines: part.lines.map((line) => line.startsWith("name = ") ? `name = "${library}"` : line) }); continue; }
    kept.push(part);
  }
  write(row.manifest, stripDependencyFeatures(join_(kept), optionalEnabled));
  const args = removeExports(row.exportsFile!, row.owner);
  const macro = row.exportsLine!.includes("extension_exports!") ? "extension_exports" : "plugin_exports";
  const crate = ident(library), label = row.pluginId!;
  const pluginDir = dependencyDirectory(row, workspace, "semio-framework-plugin");
  const manifest = [
    ...packageHeader(row, workspace, componentCrate, componentName, `The \`${label}\` ${row.componentKind} component: the one cdylib of \`${library}\`, holding only its entry point`),
    "",
    ...(/^\[lints\]/m.test(source) ? ["[lints]", "workspace = true", ""] : []),
    ...moved.filter((lines) => lines.some((line) => line.trim() === "[package.metadata.component]")).flatMap((lines) => [...lines, ""]),
    "[package.metadata.semio]",
    `role = "${role}"`,
    ...semioMoved,
    "",
    ...moved.filter((lines) => !lines.some((line) => line.trim() === "[package.metadata.component]")).flatMap((lines) => [...lines, ""]),
    "[lib]",
    'crate-type = ["cdylib"]',
    'path = "../../🦀️.rs"',
    "",
    "[dependencies]",
    `${library} = { path = "${relative(abs(componentCrate), abs(row.crateDir))}" }`,
    dependencySpec(workspace, componentCrate, "semio-framework-plugin", pluginDir),
    "",
    "[dev-dependencies]",
    dependencySpec(workspace, componentCrate, "semio-framework-async", dependencyDirectory(row, workspace, "semio-framework-async")),
    ...(macro === "extension_exports" ? [dependencySpec(workspace, componentCrate, "semio-framework-async-macros", dependencyDirectory(row, workspace, "semio-framework-async-macros"))] : []),
    "",
  ].join("\n").replace(/\n{3,}/g, "\n\n");
  write(join(componentCrate, "Cargo.toml"), manifest);
  const call = `semio_framework_plugin::${macro}!(${args.map((argument) => qualify(argument, crate)).join(", ")});`;
  write(join(componentDir, "🦀️.rs"), `//! ${emojiOf(row.owner)} The \`${label}\` ${row.componentKind} component: the one cdylib of \`${library}\`, holding only its entry point.\n\n${call}\n`);
  for (const name of ["🔣️.json", "🛂️.descriptor.semio"]) if (existsSync(abs(join(row.owner, name)))) moves.push([join(row.owner, name), join(componentDir, name)]);
  addMember(workspace, relative(dirname(abs(workspace)), abs(componentCrate)));
  log.push(`${row.name}${RENAMES[row.name] ? ` → ${library}` : ""} + ${componentName} [${workspace}] ${call}`);
}

function setCrateType(row: Row, crateType: string | null): void {
  const text = read(row.manifest).split("\n").flatMap((line) => /^crate-type\s*=/.test(line) ? (crateType ? [`crate-type = ${crateType}`] : []) : [line]).join("\n");
  write(row.manifest, text);
  log.push(`${row.name}: crate-type → ${crateType ?? "(rlib default)"}`);
}

function splitModule(row: Row): void {
  const componentName = `${row.name}-component`, componentDir = join(row.owner, COMPONENT_FACET), componentCrate = join(componentDir, "📦️packages/🦀️rust");
  if (componentName.length > 48) throw new Error(`${componentName} exceeds 48 characters`);
  if (existsSync(abs(componentDir))) throw new Error(`${componentDir} already exists`);
  const workspace = workspaceOf(row.crateDir), parts = chunks(read(row.manifest)), kept: Chunk[] = [], moved: string[][] = [];
  for (const part of parts) {
    if (part.header && /^\[package\.metadata\.wasm-pack\./.test(part.header)) { moved.push(trimBlank(part.lines)); continue; }
    if (part.header === "[lib]") { kept.push({ header: part.header, lines: part.lines.filter((line) => !/^crate-type\s*=/.test(line)) }); continue; }
    kept.push(part);
  }
  write(row.manifest, join_(kept));
  const manifest = [
    ...packageHeader(row, workspace, componentCrate, componentName, `The browser module of \`${row.name}\`: the one cdylib wasm-bindgen links, holding no code of its own`),
    "",
    ...(/^\[lints\]/m.test(read(row.manifest)) ? ["[lints]", "workspace = true", ""] : []),
    "[package.metadata.semio]",
    `role = "${row.role}"`,
    "",
    ...moved.flatMap((lines) => [...lines, ""]),
    "[lib]",
    'crate-type = ["cdylib"]',
    'path = "../../🦀️.rs"',
    "",
    "[dependencies]",
    `${row.name} = { path = "${relative(abs(componentCrate), abs(row.crateDir))}" }`,
    "",
  ].join("\n").replace(/\n{3,}/g, "\n\n");
  write(join(componentCrate, "Cargo.toml"), manifest);
  write(join(componentDir, "🦀️.rs"), `//! ${emojiOf(row.owner)} The browser module of \`${row.name}\`: links the library, whose \`#[wasm_bindgen]\` exports are the module.\n\nextern crate ${ident(row.name)} as _;\n`);
  addMember(workspace, relative(dirname(abs(workspace)), abs(componentCrate)));
  log.push(`${row.name} + ${componentName} [${workspace}]`);
}

const rows = inventory();
for (const row of rows) {
  if (row.kind === "plugin-component" && row.name === "semio-hub-stdio") row.exportsFile = join(row.owner, "🦀️.rs"), row.exportsLine = "plugin_exports!";
  if (row.kind === "plugin-component") splitPluginComponent(row);
  else if (row.kind === "test-component" || row.kind === "module-only") setCrateType(row, '["cdylib"]');
  else if (row.kind === "library") setCrateType(row, null);
  else if (row.kind === "module-split") splitModule(row);
}
console.log(log.join("\n"));
console.log(`[DEBUG] writes=${writes.size} moves=${moves.length}`);
if (apply) {
  for (const [path, text] of writes) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, text); }
  for (const [from, to] of moves) { mkdirSync(dirname(abs(to)), { recursive: true }); renameSync(abs(from), abs(to)); }
  console.log("[DEBUG] applied");
} else {
  for (const [path, text] of writes) if (process.argv.includes("--show") && path.includes(process.argv[process.argv.indexOf("--show") + 1]!)) console.log(`==== ${relative(repoRoot, path)}\n${text}`);
}

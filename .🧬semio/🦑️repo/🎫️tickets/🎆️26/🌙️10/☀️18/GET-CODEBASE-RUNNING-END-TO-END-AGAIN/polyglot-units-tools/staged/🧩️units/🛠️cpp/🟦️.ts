import {mkdirSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 🛠️ One CMake target as the planner needs it (paths workspace relative). */
export type CppTargetV1 = {
  readonly name: string; readonly type: "static" | "shared" | "executable" | "object" | "interface"; readonly sourceDirectory: string; readonly sources: readonly string[]; readonly includeDirectories: readonly string[]; readonly publicIncludeDirectories: readonly string[];
  readonly defines: readonly string[]; readonly flags: readonly string[]; readonly dependencies: readonly string[];
};
export type CppProjectV1 = {readonly compiler: {readonly c: string; readonly cxx: string; readonly version: string}; readonly targets: readonly CppTargetV1[]};
export type CppWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string};

const HEADER = /\.(h|hh|hpp|hxx|h\+\+|inl|ipp|tpp)$/i, SOURCE = /\.(c|cc|cpp|cxx|c\+\+|m|mm)$/i;
const normalize = (path: string): string => {const out: string[] = []; for (const part of path.split("\\").join("/").split("/")) {if (part === "" || part === ".") continue; if (part === "..") out.pop(); else out.push(part);} return out.join("/");};

/** 🛠️ Strips comments and canonicalizes the token layout: outside string and character literals every line becomes its tokens joined by single spaces (so `f( int x )` equals `f(int x)`), blank lines vanish, and the one place where whitespace changes meaning is kept: `#define NAME(` with the parenthesis glued to the name is a function-like macro. The result is the form a header's interface digest hashes: a comment or formatting edit leaves it unchanged. Raw strings are kept verbatim. */
export function normalizeCppV1(text: string): string {
  const lines: string[] = [];
  let line: string[] = [], index = 0;
  const n = text.length, flush = (): void => {if (line.length) lines.push(line.join(" ").replace(/^# /, "#").replace(/^(#define \S+?) \(\)/, "$1()")); line = [];};
  const punctuation = ["<<=", ">>=", "...", "->*", "<=>", "::", "->", "++", "--", "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "##", ".*"];
  while (index < n) {
    const c = text[index]!, next = text[index + 1];
    if (c === "\n") {flush(); index++; continue;}
    if (c === " " || c === "\t" || c === "\r" || c === "\f" || c === "\v") {index++; continue;}
    if (c === "\\" && text[index + 1] === "\n") {index += 2; continue;}
    if (c === "/" && next === "/") {while (index < n && text[index] !== "\n") {if (text[index] === "\\" && text[index + 1] === "\n") index++; index++;} continue;}
    if (c === "/" && next === "*") {const end = text.indexOf("*/", index + 2); index = end < 0 ? n : end + 2; continue;}
    const rawStart = /^(?:u8|u|U|L)?R"/.exec(text.slice(index, index + 4));
    if (rawStart) {
      const open = text.indexOf("(", index + rawStart[0].length), delimiter = text.slice(index + rawStart[0].length, open), close = text.indexOf(")" + delimiter + '"', open);
      if (open >= 0 && close >= 0) {line.push(text.slice(index, close + delimiter.length + 2)); index = close + delimiter.length + 2; continue;}
    }
    const prefix = /^(?:u8|u|U|L)?(["'])/.exec(text.slice(index, index + 3));
    if (prefix) {
      let end = index + prefix[0].length;
      while (end < n && text[end] !== prefix[1] && text[end] !== "\n") end += text[end] === "\\" ? 2 : 1;
      line.push(text.slice(index, Math.min(end + 1, n))); index = Math.min(end + 1, n); continue;
    }
    const word = /^[A-Za-z_$][\w$]*/.exec(text.slice(index, index + 256)) ?? /^\.?\d(?:[\w.]|[eEpP][+-])*/.exec(text.slice(index, index + 256));
    if (word) {line.push(word[0]); index += word[0].length; continue;}
    const operator = punctuation.find(candidate => text.startsWith(candidate, index));
    if (operator) {line.push(operator); index += operator.length; continue;}
    if (c === "(" && line.length === 2 && line[0] === "#" && line[1] === "define" ) {line.push(c); index++; continue;}
    if (c === "(" && line.length === 3 && line[0] === "#" && line[1] === "define" && /[A-Za-z_]/.test(text[index - 1] ?? "")) {line[2] = line[2] + "("; index++; continue;}
    line.push(c); index++;
  }
  flush();
  return lines.join("\n");
}
/** 🛠️ The interface digest of a header set: SHA-256 over sorted rows of path and normalized text. */
export const cppHeadersDigestV1 = (headers: Readonly<Record<string, string>>): string => sha256V1(sortedCodePointsV1(Object.keys(headers)).map(path => `${path}\0${sha256V1(normalizeCppV1(headers[path]!))}`).join("\n"));

/** 🛠️ The `#include` directives of a source or header (comments ignored; conditional compilation is not evaluated, so the result is a superset of what a preprocessor reads). */
export function scanCppIncludesV1(text: string): readonly {readonly path: string; readonly system: boolean}[] {
  const found: {path: string; system: boolean}[] = [];
  let clean = "", index = 0;
  const n = text.length;
  while (index < n) {
    const c = text[index]!, next = text[index + 1];
    if (c === "/" && next === "/") {while (index < n && text[index] !== "\n") index++; continue;}
    if (c === "/" && next === "*") {const end = text.indexOf("*/", index + 2); const stop = end < 0 ? n : end + 2; clean += text.slice(index, stop).replace(/[^\n]/g, " "); index = stop; continue;}
    if (c === '"' && /#\s*(?:include|import)\s*$/.test(clean.slice(clean.lastIndexOf("\n") + 1))) {const end = text.indexOf('"', index + 1); const stop = end < 0 ? n : end + 1; clean += text.slice(index, stop); index = stop; continue;}
    clean += c; index++;
  }
  for (const line of clean.split("\n")) {const match = /^\s*#\s*(?:include|import)\s*([<"])([^>"]+)[>"]/.exec(line); if (match) found.push({path: match[2]!, system: match[1] === "<"});}
  return found;
}
/** 🛠️ The workspace files a source includes, transitively: quoted includes against the including file's directory then the include directories, angle includes against the include directories only; files outside the workspace are not followed. */
export function resolveCppIncludesV1(source: string, snapshot: CppWorkspaceSnapshotV1, includeDirectories: readonly string[]): readonly string[] {
  const files = new Set(snapshot.files), seen = new Set<string>(), queue = [source];
  while (queue.length) {
    const current = queue.pop()!, directory = current.includes("/") ? current.slice(0, current.lastIndexOf("/")) : "";
    if (!files.has(current)) continue;
    for (const include of scanCppIncludesV1(snapshot.read(current))) {
      const candidates = [...(include.system ? [] : [normalize(directory + "/" + include.path)]), ...includeDirectories.map(dir => normalize(dir + "/" + include.path))];
      const hit = candidates.find(candidate => files.has(candidate));
      if (hit && !seen.has(hit)) {seen.add(hit); queue.push(hit);}
    }
  }
  return sortedCodePointsV1(seen);
}

/** 🛠️ Reads a CMake file API reply (`codemodel-v2` plus its target files) into the planner's model. `read` returns the text of a file below the reply directory; `root` is the absolute source root. Public include directories of a library are the ones its dependents also compile with. */
export function cmakeReplyToCppProjectV1(reply: {readonly codemodel: any; readonly targets: readonly any[]}, root: string, compiler: CppProjectV1["compiler"]): CppProjectV1 {
  const prefix = root.endsWith("/") ? root : root + "/", relative = (path: string): string => normalize(path.startsWith(prefix) ? path.slice(prefix.length) : path);
  const byId = new Map<string, any>(reply.targets.map(target => [target.id as string, target] as const)), kinds: Record<string, CppTargetV1["type"]> = {STATIC_LIBRARY: "static", SHARED_LIBRARY: "shared", MODULE_LIBRARY: "shared", EXECUTABLE: "executable", OBJECT_LIBRARY: "object", INTERFACE_LIBRARY: "interface"};
  const includesOf = (target: any): string[] => sortedCodePointsV1(new Set((target.compileGroups ?? []).flatMap((group: any) => (group.includes ?? []).filter((include: any) => !include.isSystem).map((include: any) => relative(include.path)))));
  const publicOf = new Map<string, string[]>();
  const publicDirectories = (target: any): string[] => {
    const known = publicOf.get(target.id as string);
    if (known) return known;
    publicOf.set(target.id as string, []);
    const mine = includesOf(target), consumed = new Set(reply.targets.filter(other => (other.dependencies ?? []).some((row: any) => row.id === target.id)).flatMap(includesOf));
    const inherited = new Set((target.dependencies ?? []).flatMap((row: any) => byId.get(row.id) ? publicDirectories(byId.get(row.id)) : []));
    const result = mine.filter(dir => consumed.has(dir) && !inherited.has(dir));
    publicOf.set(target.id as string, result);
    return result;
  };
  const targets: CppTargetV1[] = reply.targets.filter(target => kinds[target.type] !== undefined).map(target => {
    const dependencies = (target.dependencies ?? []).map((row: any) => byId.get(row.id)?.name).filter((name: string | undefined): name is string => !!name);
    const sourceDirectory = relative(target.paths.source);
    return {
      name: target.name as string, type: kinds[target.type]!, sourceDirectory: sourceDirectory === "" ? "." : sourceDirectory,
      sources: sortedCodePointsV1((target.sources ?? []).filter((source: any) => !source.isGenerated && SOURCE.test(source.path)).map((source: any) => relative(source.path.startsWith("/") ? source.path : sourceDirectory + "/" + source.path))),
      includeDirectories: includesOf(target), publicIncludeDirectories: publicDirectories(target),
      defines: sortedCodePointsV1((target.compileGroups ?? []).flatMap((group: any) => (group.defines ?? []).map((row: any) => row.define as string))),
      flags: (target.compileGroups ?? []).flatMap((group: any) => (group.compileCommandFragments ?? []).flatMap((fragment: any) => String(fragment.fragment).split(/\s+/).filter(Boolean))).filter((flag: string, index: number, all: string[]) => all.indexOf(flag) === index),
      dependencies: sortedCodePointsV1(dependencies),
    };
  });
  void reply.codemodel;
  return {compiler, targets};
}

/** 🛠️ The plan the executor reads from a unit's environment. */
export type CppPlanV1 = {readonly kind: "static" | "shared" | "executable" | "object" | "interface"; readonly compiler: string; readonly cCompiler: string; readonly sources: readonly {readonly path: string; readonly language: "c" | "c++" | "objc" | "objc++"}[]; readonly flags: readonly string[]; readonly defines: readonly string[]; readonly includeDirectories: readonly string[]; readonly publicIncludeDirectories: readonly string[]; readonly dependencyIncludeDirectories: readonly string[]; readonly dependencySources: readonly string[]};
const languageOf = (path: string): "c" | "c++" | "objc" | "objc++" => /\.c$/i.test(path) && !/\.C$/.test(path) ? "c" : /\.mm$/i.test(path) ? "objc++" : /\.m$/i.test(path) ? "objc" : "c++";

/** 🛠️ Plans the unit graph of a C or C++ project: one unit per target. A library owns its sources and its public headers (the interface: normalized header digest, artifact = the normalized headers), private headers that sources include enter as inputs, dependents consume the interface of libraries; an executable compiles its own and its libraries' sources (`implementation` edges, sources read in the build tool). */
export function planCppUnitsV1(project: CppProjectV1, snapshot: CppWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1}): UnitGraphV1 {
  const files = new Set(snapshot.files), targets = new Map(project.targets.map(target => [target.name, target] as const));
  const headersUnder = (dir: string): string[] => snapshot.files.filter(path => HEADER.test(path) && (dir === "." || dir === "" || path.startsWith(dir + "/")));
  const publicHeaders = new Map(project.targets.map(target => [target.name, sortedCodePointsV1(new Set(target.publicIncludeDirectories.flatMap(headersUnder)))] as const));
  const ownerOfHeader = new Map<string, string>();
  for (const [name, headers] of publicHeaders) for (const header of headers) if (!ownerOfHeader.has(header)) ownerOfHeader.set(header, name);
  const closure = (name: string, seen = new Set<string>()): string[] => {if (seen.has(name)) return []; seen.add(name); const target = targets.get(name)!; return [name, ...target.dependencies.flatMap(dep => targets.has(dep) ? closure(dep, seen) : [])];};
  const includeDirsOf = (target: CppTargetV1): string[] => sortedCodePointsV1(new Set([...target.includeDirectories, ...target.dependencies.flatMap(dep => targets.get(dep)?.publicIncludeDirectories ?? [])]));
  const privateMemo = new Map<string, Set<string>>();
  const privateOf = (target: CppTargetV1): Set<string> => {
    let found = privateMemo.get(target.name);
    if (!found) {
      found = new Set<string>();
      const own = publicHeaders.get(target.name)!;
      for (const source of target.sources) for (const header of resolveCppIncludesV1(source, snapshot, includeDirsOf(target))) if (HEADER.test(header) && !own.includes(header) && !(ownerOfHeader.has(header) && ownerOfHeader.get(header) !== target.name)) found.add(header);
      privateMemo.set(target.name, found);
    }
    return found;
  };
  const units: UnitV1[] = [];
  const toolchain = {id: project.compiler.version, parts: [] as string[]};
  for (const target of project.targets) {
    const library = target.type !== "executable", own = publicHeaders.get(target.name)!;
    const includeDirs = includeDirsOf(target), privateHeaders = privateOf(target);
    const dependencies = new Map<string, UnitDependencyV1>();
    for (const dep of target.dependencies) if (targets.has(dep)) dependencies.set("cpp:" + dep, {unit: "cpp:" + dep, use: library ? "interface" : "both"});
    const transitive = target.type === "executable" ? closure(target.name).filter(name => name !== target.name).map(name => targets.get(name)!) : [];
    const dependencySources = sortedCodePointsV1(new Set(transitive.flatMap(row => row.sources)));
    const dependencyIncludes = sortedCodePointsV1(new Set(transitive.flatMap(row => [...row.includeDirectories, ...row.publicIncludeDirectories])));
    const sources = sortedCodePointsV1(new Set([...target.sources, ...(library ? own : []), ...(target.type === "executable" ? [...privateHeaders] : [])]));
    const plan: CppPlanV1 = {
      kind: target.type, compiler: project.compiler.cxx, cCompiler: project.compiler.c, sources: target.sources.map(path => ({path, language: languageOf(path)})), flags: target.flags, defines: target.defines, includeDirectories: target.includeDirectories,
      publicIncludeDirectories: target.publicIncludeDirectories, dependencyIncludeDirectories: dependencyIncludes, dependencySources,
    };
    const exec = target.type === "executable";
    units.push({
      id: "cpp:" + target.name, language: "cpp", kind: exec ? "binary" : "library", label: target.name, sources, inputs: sortedCodePointsV1(new Set([...(exec ? [...dependencySources, ...transitive.flatMap(row => [...privateOf(row)])] : []), ...(library ? [...privateHeaders] : []), ...(files.has("CMakeLists.txt") ? ["CMakeLists.txt"] : [])])).filter(path => !sources.includes(path)),
      dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: [], interface: library ? {kind: "cpp-public-headers"} : null,
      outputs: [exec ? target.name : `lib${target.name}.a`], toolchain,
      action: {argv: [project.compiler.cxx, "-c", ...target.sources], env: {SEMIO_CPP_PLAN: JSON.stringify(plan), ...(exec ? {SEMIO_BAZEL_INPUTS: "sources"} : {})}},
    });
  }
  return {version: 1, units};
}

/** 🏭️ The process effect of the C/C++ executor. */
export type CppRunnerV1 = (argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}) => Promise<{readonly code: number; readonly output: string}>;
/** 🏭️ Executes a C/C++ unit: every source is compiled with the target's flags, defines and include directories (a library against the normalized public headers of its dependencies when the request carries them, else against the workspace) and archived deterministically (`libtool -static -D` on macOS, `ar rcsD` elsewhere); an executable compiles its dependencies' sources too and links. The interface digest is that of the normalized public headers, the artifact the headers themselves with their include directories. */
export function cppUnitExecutorV1(options: {readonly workspace: string; readonly outputRoot: string; readonly run: CppRunnerV1; readonly platform: string}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  return async request => {
    const {unit} = request, plan = JSON.parse(unit.action.env.SEMIO_CPP_PLAN ?? "{}") as CppPlanV1, out = join(options.outputRoot, sha256V1(unit.id).slice(0, 24));
    rmSync(out, {recursive: true, force: true}); mkdirSync(out, {recursive: true});
    const started = performance.now(), elapsed = (): number => Math.round(performance.now() - started), fail = (diagnostics: string): ExecuteOutcomeV1 => ({ok: false, diagnostics: diagnostics.trim(), outputs: sha256V1("failed:" + diagnostics), interface: null, milliseconds: elapsed()});
    const headers: Record<string, string> = {}, includeDirectories = new Set<string>(), artifacts = request.dependencies.filter(row => row.use === "interface");
    const fromArtifacts = artifacts.length > 0 && artifacts.every(row => row.artifact);
    if (fromArtifacts) for (const row of artifacts) {const artifact = JSON.parse(row.artifact!) as {headers: Record<string, string>; includeDirectories: string[]}; Object.assign(headers, artifact.headers); for (const dir of artifact.includeDirectories) includeDirectories.add(dir);}
    const dependencyRoot = join(out, "deps");
    for (const [path, text] of Object.entries(headers)) {mkdirSync(dirname(join(dependencyRoot, path)), {recursive: true}); writeFileSync(join(dependencyRoot, path), text);}
    const own = unit.sources.filter(path => HEADER.test(path)), ownHeaders: Record<string, string> = {};
    for (const path of own) ownHeaders[path] = normalizeCppV1(readFileSync(join(options.workspace, path), "utf8"));
    const include = [...plan.includeDirectories.map(dir => join(options.workspace, dir)), ...(fromArtifacts ? [...includeDirectories].map(dir => join(dependencyRoot, dir)) : plan.dependencyIncludeDirectories.map(dir => join(options.workspace, dir)))];
    const sources = plan.kind === "executable" ? [...plan.sources.map(row => row.path), ...plan.dependencySources.filter(path => SOURCE.test(path)).map(path => path)] : plan.sources.map(row => row.path);
    const objects: string[] = [];
    for (const [index, source] of sources.entries()) {
      const object = join(out, `${String(index).padStart(4, "0")}.o`), language = languageOf(source), plusplus = language === "c++" || language === "objc++";
      const flags = plan.flags.filter(flag => flag !== "-c" && !flag.startsWith("-o") && (!/^-std=/.test(flag) || /\+\+|gnu\+\+/.test(flag) === plusplus));
      const result = await options.run([plusplus ? plan.compiler : plan.cCompiler, "-c", ...flags, ...plan.defines.map(define => "-D" + define), ...include.map(dir => "-I" + dir), "-o", object, source], {cwd: options.workspace, env: unit.action.env, signal: request.signal});
      if (result.code !== 0) return fail(result.output);
      objects.push(object);
    }
    let implementation: string;
    if (plan.kind === "executable") {
      const binary = join(out, unit.outputs[0]!), result = await options.run([plan.compiler, ...objects, "-o", binary], {cwd: options.workspace, env: unit.action.env, signal: request.signal});
      if (result.code !== 0) return fail(result.output);
      implementation = sha256V1(readFileSync(binary));
    } else {
      implementation = sha256V1(objects.map(object => sha256V1(readFileSync(object))).join("\n"));
      if (objects.length) {
        const archive = join(out, unit.outputs[0]!), result = await options.run(options.platform === "darwin" ? ["libtool", "-static", "-D", "-o", archive, ...objects] : ["ar", "rcsD", archive, ...objects], {cwd: options.workspace, env: unit.action.env, signal: request.signal});
        if (result.code !== 0) return fail(result.output);
      }
    }
    if (!unit.interface) return {ok: true, diagnostics: "", outputs: implementation, interface: null, milliseconds: elapsed()};
    const all = {...headers, ...ownHeaders};
    return {ok: true, diagnostics: "", outputs: implementation, interface: cppHeadersDigestV1(ownHeaders), artifact: JSON.stringify({headers: Object.fromEntries(sortedCodePointsV1(Object.keys(all)).map(path => [path, all[path]!])), includeDirectories: sortedCodePointsV1(new Set([...includeDirectories, ...plan.publicIncludeDirectories]))}), milliseconds: elapsed()};
  };
}

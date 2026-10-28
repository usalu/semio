/** 🧬️ Registers the structural Source parsers (taxonomy projection, command and module imports, script requirements, Rust references) under this revision's identity. */
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import { memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve } from "../../../../🏗️build/👁️observation/🟨️.mjs";

/** 🏷️ Names this component as its roster entry does. */
export const component = "source-analyses";
let requirePhysicalSourceV1, LIBRARY_ROOT, loaded, readPhysicalSource, revisionIdentity, register, inferrerStampV1;
export let TAXONOMY_PROJECTION, TAXONOMY;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ requirePhysicalSourceV1, LIBRARY_ROOT, readPhysicalSource } = registry.coordinates);
  ({ loaded } = registry.bootstrap);
  ({ revisionIdentity, register } = registry);
  ({ inferrerStampV1 } = registry.stamps);
  TAXONOMY_PROJECTION = register({ id: `build:taxonomy-projection:v1:${createHash("sha256").update(revisionIdentity.url).digest("hex")}`, revision: createHash("sha256").update(taxonomyProjectionV1.toString()).digest("hex"), analyze: (_path, bytes) => taxonomyProjectionV1(JSON.parse(bytes.toString("utf8"))) });
  TAXONOMY = readTaxonomyProjectionV1(join(LIBRARY_ROOT, "🔣️taxonomy.json"));
}

/** 🗺️ The only taxonomy facts graph inference consumes, so edits elsewhere in the taxonomy keep every inference record and parser revision valid. */
function taxonomyProjectionV1(taxonomy) {
  return { testDeliveryScopeDirectoryNames: taxonomy.testDeliveryScopeDirectoryNames, generatorContracts: taxonomy.generatorContracts ?? {} };
}

/** 🧭️ Reads the consumed taxonomy projection through physical authority ancestry as a semantic fact rather than whole-file bytes. */
export function readTaxonomyProjectionV1(path) {
  requirePhysicalSourceV1(path);
  return observedSourceAnalysisV1(TAXONOMY_PROJECTION, path);
}

/** 🦀️ Tokenizes module syntax without interpreting strings and comments as declarations. */
const RUST_RAW_PATTERN_V1 = /(?:b|c)?r(#+)?"/y, RUST_CHARACTER_PATTERN_V1 = /(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/uy, RUST_IDENTIFIER_PATTERN_V1 = /(?:r#)?[\p{ID_Start}_][\p{ID_Continue}]*/uy, UNICODE_WHITESPACE_V1 = /\s/u, RAW_IDENTIFIER_PREFIX_V1 = /^r#/;

function rustInputTokens(source) {
  const unescapeRustString = (body) => body.replace(/\\(?:\r?\n\s*|u\{([0-9a-fA-F_]+)\}|x([0-9a-fA-F]{2})|(.))/gsu, (_, brace, hex, plain) => brace !== undefined ? String.fromCodePoint(parseInt(brace.replaceAll("_", ""), 16)) : hex !== undefined ? String.fromCharCode(parseInt(hex, 16)) : plain === undefined ? "" : ({ n: "\n", r: "\r", t: "\t", 0: "\0" })[plain] ?? plain);
  const tokens = [], rawPattern = RUST_RAW_PATTERN_V1, characterPattern = RUST_CHARACTER_PATTERN_V1, identifierPattern = RUST_IDENTIFIER_PATTERN_V1;
  const match = (pattern, at) => { pattern.lastIndex = at; return pattern.exec(source); };
  for (let i = 0; i < source.length;) {
    const char = source[i], code = source.charCodeAt(i);
    if (code === 32 || code >= 9 && code <= 13 || code > 127 && UNICODE_WHITESPACE_V1.test(char)) { i++; continue; }
    if (char === "/" && source[i + 1] === "/") { const end = source.indexOf("\n", i); i = end < 0 ? source.length : end; continue; }
    if (char === "/" && source[i + 1] === "*") {
      let depth = 1; i += 2;
      while (i < source.length && depth) {
        if (source.startsWith("/*", i)) { depth++; i += 2; }
        else if (source.startsWith("*/", i)) { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    const raw = (char === "r" || char === "b" || char === "c") && match(rawPattern, i);
    if (raw) {
      const start = i + raw[0].length, delimiter = '"' + (raw[1] ?? ""), end = source.indexOf(delimiter, start);
      if (end < 0) throw new Error("Unterminated Rust raw string");
      tokens.push({ text: source.slice(start, end), string: true }); i = end + delimiter.length; continue;
    }
    if (char === '"' || (char === "b" || char === "c") && source[i + 1] === '"') {
      const start = char === '"' ? i : i + 1; i = start + 1;
      while (i < source.length && source[i] !== '"') { i += source[i] === "\\" ? 2 : 1; }
      const literal = source.slice(start, ++i);
      tokens.push({ text: unescapeRustString(literal.slice(1, -1)), string: true });
      continue;
    }
    const character = (char === "'" || char === "b") && match(characterPattern, i);
    if (character) { tokens.push({ text: character[0], string: true }); i += character[0].length; continue; }
    const identifier = (code > 127 || char === "_" || code >= 65 && code <= 90 || code >= 97 && code <= 122) && match(identifierPattern, i);
    if (identifier) { tokens.push({ text: identifier[0], identifier: true }); i += identifier[0].length; continue; }
    tokens.push({ text: source[i++] });
  }
  return tokens;
}

/** 🧾️ Retains only module mounts and include expressions, independently of a file's location; the mounts and includes that only exist under `#[cfg(test)]` (an item or module under `cfg(test)` or `cfg(all(test, …))`, the indexes of its includes, or a whole `#![cfg(test)]` file, which the compiler still reads but never expands) are marked, so a consumer that compiles without that configuration leaves them out. A `macro_rules!` body is refused only when it can mount a file (`mod name;` or a `#[path]` attribute): an inline `mod name { … }` it emits reads no file; an `include*!` inside `quote!` is code the crate emits into its consumers, counted as `emitted` and never an input of the crate itself. */
function rustSourceReferences(source) {
  const tokens = rustInputTokens(source), pairs = new Map(), stack = [], includes = [];
  for (let i = 0; i < tokens.length; i++) {
    if (tokens[i].string) continue;
    if (["(", "[", "{"].includes(tokens[i].text)) stack.push(i);
    else if ([")", "]", "}"].includes(tokens[i].text)) { const open = stack.pop(); if (open !== undefined) pairs.set(open, i); }
  }
  const literal = (start, end) => {
    if (tokens[start]?.string && start + 1 === end) return tokens[start].text;
    if (tokens[start]?.text === "env" && tokens[start + 1]?.text === "!" && tokens[start + 3]?.text === "CARGO_MANIFEST_DIR") return { manifest: true };
    if (tokens[start]?.text === "concat" && tokens[start + 1]?.text === "!") {
      const parts = []; let from = start + 3;
      for (let i = from; i < end - 1; i++) {
        if (pairs.has(i)) { i = pairs.get(i); continue; }
        if (tokens[i].text === ",") { parts.push(literal(from, i)); from = i + 1; }
      }
      if (from < end - 1) parts.push(literal(from, end - 1));
      return parts;
    }
    throw new Error("Dynamic Rust include requires explicit source inputs");
  };
  const quoted = [];
  for (let i = 0; i < tokens.length - 2; i++) if (tokens[i].identifier && ["quote", "quote_spanned", "parse_quote", "parse_quote_spanned"].includes(tokens[i].text) && tokens[i + 1].text === "!" && pairs.get(i + 2) !== undefined) quoted.push([i + 2, pairs.get(i + 2)]);
  let emitted = 0;
  const places = [];
  for (let i = 0; i < tokens.length - 3; i++) {
    if (!tokens[i].identifier || !["include", "include_str", "include_bytes"].includes(tokens[i].text) || tokens[i + 1].text !== "!") continue;
    if (quoted.some(([from, to]) => i > from && i < to)) { emitted++; continue; }
    const end = pairs.get(i + 2);
    if (end !== undefined) { places.push(i); includes.push(literal(i + 3, tokens[end - 1]?.text === "," ? end - 1 : end)); }
  }
  const requiresTest = (i) => {
    const text = tokens[i]?.text;
    if (tokens[i + 1]?.text === "(" && pairs.get(i + 1) !== undefined) {
      const close = pairs.get(i + 1);
      let gated = false;
      if (text === "all") for (let at = i + 2; at < close;) { const [inner, next] = requiresTest(at); gated ||= inner; at = next; if (tokens[at]?.text === ",") at++; }
      return [gated, close + 1];
    }
    return [text === "test" && tokens[i + 1]?.text !== "=", tokens[i + 1]?.text === "=" ? i + 3 : i + 1];
  };
  const emitsMount = (from, to) => {
    for (let at = from; at < to; at++) {
      if (tokens[at].string) continue;
      const named = tokens[at].text === "mod" && tokens[at + 1]?.text === "$" ? at + 2 : at + 1;
      if (tokens[at].text === "mod" && tokens[named]?.identifier && tokens[named + 1]?.text === ";") return true;
      if (tokens[at].text === "#" && (tokens[at + 1]?.text === "[" || tokens[at + 1]?.text === "!" && tokens[at + 2]?.text === "[")) {
        const open = tokens[at + 1].text === "[" ? at + 1 : at + 2;
        if (tokens[open + 1]?.text === "path" && tokens[open + 2]?.text === "=") return true;
      }
    }
    return false;
  };
  const gated = [];
  let fileTest = false;
  const scope = (start, end, top = false) => {
    const modules = [];
    for (let i = start; i < end;) {
      let path, test = false;
      const first = i;
      while (tokens[i]?.text === "#") {
        const inner = tokens[i + 1]?.text === "!", open = inner ? i + 2 : i + 1, close = pairs.get(open);
        if (close === undefined) break;
        if (tokens[open + 1]?.text === "cfg_attr" && tokens.slice(open + 2, close).some((token) => token.text === "path")) throw new Error("Conditional Rust mount requires conservative source inputs");
        if (tokens[open + 1]?.text === "path" && tokens[open + 2]?.text === "=" && tokens[open + 3]?.string) path = tokens[open + 3].text;
        if (tokens[open + 1]?.text === "cfg" && tokens[open + 2]?.text === "(" && pairs.get(open + 2) !== undefined && requiresTest(open + 3)[0]) { if (!inner) test = true; else if (top) fileTest = true; }
        i = close + 1;
      }
      if (tokens[i]?.text === "pub") { i++; if (tokens[i]?.text === "(") i = (pairs.get(i) ?? i) + 1; }
      if (tokens[i]?.text === "mod" && tokens[i + 1]?.identifier) {
        const name = tokens[i + 1].text.replace(RAW_IDENTIFIER_PREFIX_V1, ""), boundary = i + 2;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary);
          if (close !== undefined) { modules.push({ name, path, ...(test ? { test } : {}), modules: scope(boundary + 1, close) }); if (test) gated.push([first, close]); i = close + 1; continue; }
        }
        if (tokens[boundary]?.text === ";") { modules.push({ name, path, ...(test ? { test } : {}) }); if (test) gated.push([first, boundary]); i = boundary + 1; continue; }
      }
      const macro = tokens[i]?.text === "macro_rules", bodyFrom = i;
      while (i < end && tokens[i].text !== ";" && tokens[i].text !== "{") { if (pairs.has(i)) i = pairs.get(i); i++; }
      if (macro && emitsMount(bodyFrom, tokens[i]?.text === "{" ? pairs.get(i) ?? i : i)) throw new Error("Generated Rust modules require conservative source inputs");
      i = (pairs.get(i) ?? i) + 1;
      if (test) gated.push([first, i]);
    }
    return modules;
  };
  const modules = scope(0, tokens.length, true), testIncludes = places.flatMap((place, index) => (gated.some(([from, to]) => place >= from && place <= to) ? [index] : []));
  return { includes, modules, ...(emitted ? { emitted } : {}), ...(testIncludes.length ? { testIncludes } : {}), ...(fileTest ? { fileTest } : {}) };
}

/** 🪶️ Bounds parsed source facts by serialized bytes plus per-entry bookkeeping. */
export function createRustSourceCache(limit = 16 * 1024 * 1024) {
  if (!Number.isSafeInteger(limit) || limit < 1) throw new Error("Rust source cache requires a positive byte budget");
  return { entries: new Map(), bytes: 0, limit, hits: 0, misses: 0 };
}
export const RUST_SOURCE_CACHE = createRustSourceCache();
const RUST_REFERENCE_NAMESPACE = `rust-source-references-v1:${createHash("sha256").update(rustInputTokens.toString()).update(rustSourceReferences.toString()).digest("hex")}`;
export const RUST_INPUT_SNAPSHOTS = new WeakMap();
let sourceAnalysesV1;

/** 🪪️ Names the exact evaluated module URL and its originally bound implementation revision. */
export function sourceAnalysisOwnerIdentityV1(url,revision){return createHash("sha256").update(JSON.stringify([url,revision])).digest("hex");}

const compilerRevisionsV1=new Map();
/** 🔩️ The revision the module parsers carry, bound to the bytes of the TypeScript compiler they use, as every caller records it: resolved and read once per process, and the facts of that read are replayed into each receiving project, so the rows of a project never depend on which project of the same process asked first (a farm worker infers in a fresh process). */
function compilerRevisionV1(stamp){return memoizedObservedComputationV1(compilerRevisionsV1,stamp,()=>createHash("sha256").update(stamp).update(readPhysicalSource(observedRequireResolve(fileURLToPath(revisionIdentity.url),"typescript"))).digest("hex"));}

/** 📖️ Binds structural parsers to their evaluated implementation and compiler bytes. */
export function prepareSourceAnalysesV1(){
  const stamp=inferrerStampV1("source-analyses"),revision=compilerRevisionV1(stamp);
  if(!sourceAnalysesV1){
    const commandParser=loaded.commandSourceImports,urlParser=loaded.commandUrlReferences,runtimeParser=loaded.runtimeModuleSourceImports,moduleParser=loaded.moduleSourceImports,productionParser=loaded.productionModuleSourceImports,stampRevision=createHash("sha256").update(stamp).digest("hex");
    const owner=sourceAnalysisOwnerIdentityV1(revisionIdentity.url,stamp),identity=role=>`build:${role}:v1:${owner}`;
    sourceAnalysesV1={
      command:register({id:identity("command-imports"),revision,analyze:(path,bytes)=>commandParser(path,bytes.toString("utf8"))}),
      references:register({id:identity("command-references"),revision,analyze:(path,bytes)=>{const text=bytes.toString("utf8");return {runtime:runtimeParser(path,text),urls:urlParser(path,text)};}}),
      module:register({id:identity("module-imports"),revision,analyze:(path,bytes)=>({imports:moduleParser(path,bytes.toString("utf8")),runtimeImports:runtimeParser(path,bytes.toString("utf8")),productionImports:productionParser(path,bytes.toString("utf8"))})}),
      script:register({id:identity("script-requirements"),revision:stampRevision,analyze:(_path,bytes)=>{const text=bytes.toString("utf8");return {go:/["']go["']\s*,|\brunGo\w*\(/.test(text),wasm:/\b(?:buildWasmWebV1|buildRepositoryWasmWebV1)\s*\(/.test(text)};}}),
      nativeScript:register({id:identity("native-script-admission"),revision:stampRevision,analyze:(_path,bytes)=>loaded.nativeHostScriptAdmissionV1(bytes.toString("utf8"))}),
      rust:register({id:identity("rust-references"),revision:createHash("sha256").update(stamp).update(RUST_REFERENCE_NAMESPACE).digest("hex"),analyze:(_path,bytes)=>boundedRustReferences(bytes,RUST_SOURCE_CACHE)})
    };
  }
  return sourceAnalysesV1;
}

/** 🔬️ Parses one source's references through the bounded in-process cache keyed by its content digest. */
function boundedRustReferences(source, cache) {
  const digest = createHash("sha256").update(source).digest("hex");
  let entry = cache.entries.get(digest);
  if (entry) { cache.hits++; cache.entries.delete(digest); cache.entries.set(digest, entry); }
  else {
    cache.misses++;
    let references;
    try { references = rustSourceReferences(source.toString("utf8")); }
    catch (error) { references = { error: error.message }; }
    entry = { references, bytes: Buffer.byteLength(JSON.stringify(references)) + 192 };
    if (entry.bytes <= cache.limit) {
      while (cache.bytes + entry.bytes > cache.limit) {
        const key = cache.entries.keys().next().value;
        cache.bytes -= cache.entries.get(key).bytes;
        cache.entries.delete(key);
      }
      cache.entries.set(digest, entry); cache.bytes += entry.bytes;
    }
  }
  return entry.references;
}

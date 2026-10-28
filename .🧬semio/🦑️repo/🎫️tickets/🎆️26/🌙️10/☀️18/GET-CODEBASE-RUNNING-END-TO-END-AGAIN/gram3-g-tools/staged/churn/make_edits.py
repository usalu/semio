"""[gram3-g] writes edits.json of the batch `churn` from the anchored edits below (each `old` must occur exactly once in the live file). usage: make_edits.py (from anywhere)."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 10)))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = LIB + "🏗️build/"
RECORDS = LIB + "⚡️caching/🗃️records/🟦️.ts"
RESOLUTION = B + "📥️inputs/🦀️resolution/🟦️.ts"
INPUTS = B + "📥️inputs/🟨️.mjs"
OBSERVATION = B + "👁️observation/🟨️.mjs"

edits = []


def edit(file, name, old, new):
    edits.append({"file": file, "name": name, "old": old, "new": new})


HEAD = 'createHash("sha256").update('


def balanced_end(text, start):
    depth = 0
    quote = None
    i = start
    while i < len(text):
        c = text[i]
        if quote:
            if c == "\\":
                i += 1
            elif c == quote:
                quote = None
        elif c in "\"'`":
            quote = c
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced")


def hash_edits(file, label, skip=()):
    """one edit per `createHash("sha256").update(X).digest("hex")` of the live file: becomes `sha256HexV1(X)`; the anchor grows to the left until it is unique."""
    text = open(os.path.join(ROOT, file), encoding="utf8").read()
    at, found = 0, []
    while True:
        i = text.find(HEAD, at)
        if i < 0:
            break
        close = balanced_end(text, i + len(HEAD) - 1)
        tail = '.digest("hex")'
        if text.startswith(tail, close + 1):
            found.append((i, close + 1 + len(tail), text[i + len(HEAD):close]))
        at = i + 1
    count = 0
    for i, end, inner in found:
        if inner in skip:
            continue
        left = 0
        while True:
            anchor = text[max(0, i - left):end]
            if text.count(anchor) == 1:
                break
            left += 8
            if i - left < 0:
                raise ValueError("no unique anchor for " + inner)
        prefix = text[max(0, i - left):i]
        count += 1
        edit(file, label + " digest " + str(count), anchor, prefix + "sha256HexV1(" + inner + ")")


edit(RECORDS, "records import", 'import { createHash, randomBytes } from "node:crypto";', 'import { createHash, hash, randomBytes } from "node:crypto";')
edit(RECORDS, "records value digest", '  return createHash("sha256").update(value).digest("hex").slice(0, 32);', '  return hash("sha256", value, "hex").slice(0, 32);')
edit(RECORDS, "records key",
     """  const hash = createHash("sha256");
  for (const part of [namespace, ...inputs]) {
    const bytes = typeof part === "string" ? Buffer.from(part, "utf8") : part;
    hash.update(`${bytes.byteLength}:`).update(bytes);
  }
  return hash.digest("hex");
}""",
     """  const parts = [namespace, ...inputs];
  if (parts.every((part) => typeof part === "string")) {
    let text = "";
    for (const part of parts as string[]) text += `${Buffer.byteLength(part)}:${part}`;
    return hash("sha256", text, "hex");
  }
  const chained = createHash("sha256");
  for (const part of parts) chained.update(`${typeof part === "string" ? Buffer.byteLength(part) : part.byteLength}:`).update(part);
  return chained.digest("hex");
}""")

edit(RESOLUTION, "resolution import", 'import {createHash} from "node:crypto";', 'import {compareUtf8V1, sha256HexV1} from "../../🧂️text-bytes/🟨️.mjs";')
edit(RESOLUTION, "resolution sha", 'const sha = (value: string): string => createHash("sha256").update(value).digest("hex");\nconst bytes = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));',
     'const sha = (value: string): string => sha256HexV1(value);\nconst bytes = (left: string, right: string): number => compareUtf8V1(left, right);')

edit(INPUTS, "inputs import", 'import { createHash, randomBytes, randomUUID } from "node:crypto";', 'import { createHash, randomBytes, randomUUID } from "node:crypto";\nimport { sha256DelimitedHexV1, sha256FirstByteV1, sha256HexV1 } from "../🧂️text-bytes/🟨️.mjs";')
edit(INPUTS, "inputs memo", 'const INVENTORY_MEMO_V1 = new Map(), INVENTORY_MEMO_LIMIT_V1 = 4096;', 'const INVENTORY_MEMO_V1 = new Map(), INVENTORY_MEMO_LIMIT_V1 = 4096, CHUNK_CUTS_V1 = new Map(), CHUNK_CUTS_LIMIT_V1 = 131072;')
edit(INPUTS, "inputs release", 'export function releaseInputMemoryV1() { INVENTORY_MEMO_V1.clear(); VERIFIED_CLOSURES_V1.clear(); }', 'export function releaseInputMemoryV1() { INVENTORY_MEMO_V1.clear(); VERIFIED_CLOSURES_V1.clear(); CHUNK_CUTS_V1.clear(); }')
edit(INPUTS, "inputs inventory key",
     """  const hash = createHash("sha256").update(workspaceRoot).update("\\0");
  for (const file of files) hash.update(file).update("\\0");
  const key = hash.digest("hex"), known = INVENTORY_MEMO_V1.get(key);""",
     """  const key = sha256DelimitedHexV1(files, "\\0", workspaceRoot), known = INVENTORY_MEMO_V1.get(key);""")
edit(INPUTS, "inputs chunk cut",
     """    if (current.length >= INPUT_CLOSURE_CHUNK_MINIMUM_V1 && createHash("sha256").update(file).digest()[0] % INPUT_CLOSURE_CHUNK_MODULUS_V1 === 0) { chunks.push(current); current = []; }""",
     """    if (current.length >= INPUT_CLOSURE_CHUNK_MINIMUM_V1 && chunkEndsAfterV1(file)) { chunks.push(current); current = []; }""")
edit(INPUTS, "inputs chunk cut memo",
     """const INPUT_CLOSURE_CHUNK_MINIMUM_V1 = 16, INPUT_CLOSURE_CHUNK_MODULUS_V1 = 128;""",
     """const INPUT_CLOSURE_CHUNK_MINIMUM_V1 = 16, INPUT_CLOSURE_CHUNK_MODULUS_V1 = 128;

/** ✂️ Whether a chunk ends after this path: a pure function of the path, asked again for every roster that holds it, so it is answered once per path until the memo is released. */
function chunkEndsAfterV1(file) {
  let ends = CHUNK_CUTS_V1.get(file);
  if (ends === undefined) {
    if (CHUNK_CUTS_V1.size >= CHUNK_CUTS_LIMIT_V1) CHUNK_CUTS_V1.clear();
    ends = sha256FirstByteV1(file) % INPUT_CLOSURE_CHUNK_MODULUS_V1 === 0;
    CHUNK_CUTS_V1.set(file, ends);
  }
  return ends;
}""")
edit(INPUTS, "inputs chunk key", 'const contract = capturedInventoryV1(chunk, workspaceRoot).text, key = createHash("sha256").update(contract).digest("hex"); publishInputClosureV1(join(directory, `${key}.json`), contract);', 'const contract = capturedInventoryV1(chunk, workspaceRoot).text, key = sha256HexV1(contract); publishInputClosureV1(join(directory, `${key}.json`), contract);')
edit(INPUTS, "inputs lock key", 'contract = captured.text, key = createHash("sha256").update(contract).digest("hex"), directory = repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")', 'contract = captured.text, key = sha256HexV1(contract), directory = repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")')

edit(OBSERVATION, "observation import", 'import { createHash } from "node:crypto";', 'import { createHash } from "node:crypto";\nimport { sha256HexV1, utf8TextV1 } from "../🧂️text-bytes/🟨️.mjs";\n\n/** 🔢️ A hex SHA-256 digest, compiled once. */\nconst SHA256_HEX_V1 = /^[a-f0-9]{64}$/u;')
hash_edits(OBSERVATION, "observation")
edit(OBSERVATION, "observation analysis hit", 'if(hit!==undefined)try{const candidate=JSON.parse(Buffer.from(hit).toString("utf8"));', 'if(hit!==undefined)try{const candidate=JSON.parse(utf8TextV1(hit));')
edit(OBSERVATION, "observation record hit", 'if (hit !== undefined) { try { return JSON.parse(Buffer.from(hit).toString("utf8")); } catch {} }', 'if (hit !== undefined) { try { return JSON.parse(utf8TextV1(hit)); } catch {} }')
edit(OBSERVATION, "observation file record hit", 'try { const value = JSON.parse(Buffer.from(hit).toString("utf8")); record("file", key, hint); return value; } catch {}', 'try { const value = JSON.parse(utf8TextV1(hit)); record("file", key, hint); return value; } catch {}')
edit(OBSERVATION, "observation async file record hit", 'if(hit!==undefined)try{const value=JSON.parse(Buffer.from(hit).toString("utf8"));record("file",key,hint);return value;}catch{}', 'if(hit!==undefined)try{const value=JSON.parse(utf8TextV1(hit));record("file",key,hint);return value;}catch{}')
edit(OBSERVATION, "observation settle owned",
     """  settleRead(absolute(path), stamp, bytes, fact);
  const encoding = encodingOf(options);
  return encoding ? bytes.toString(encoding) : bytes;""",
     """  const encoding = encodingOf(options);
  settleRead(absolute(path), stamp, bytes, fact, encoding !== undefined);
  return encoding ? bytes.toString(encoding) : bytes;""")
edit(OBSERVATION, "observation settle head", 'function settleRead(key, stamp, bytes, fact) {', 'function settleRead(key, stamp, bytes, fact, owned = false) {')
edit(OBSERVATION, "observation settle store", 'ino: stamp.ino, bytes: Buffer.from(bytes), row: canonicalObservationV1("file", key, fact) });', 'ino: stamp.ino, bytes: owned ? bytes : Buffer.from(bytes), row: canonicalObservationV1("file", key, fact) });')

PRECISION = B + "📥️inputs/🎯️precision/🟦️.ts"
DISCOVERY = B + "🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"
SCRIPTS = B + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
ASSEMBLY = B + "🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
ARGUMENTS = B + "📤️arguments/🟨️.mjs"
ANALYSES = B + "🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs"

edit(RESOLUTION, "resolution key regexes",
     r'''const normalizeKey = (pattern: string): string => pattern.replace(/^!?\{(?:workspaceRoot|projectRoot)\}\//u, "").replace(/^(?:\.\/)+/u, "");''',
     r'''/** 🔤️ The prefixes of an input pattern that name its root, compiled once instead of once per pattern. */
const ROOT_PREFIX_V1 = /^!?\{(?:workspaceRoot|projectRoot)\}\//u, DOT_SLASH_V1 = /^(?:\.\/)+/u;
const normalizeKey = (pattern: string): string => pattern.replace(ROOT_PREFIX_V1, "").replace(DOT_SLASH_V1, "");''')

edit(PRECISION, "precision bare and toml regexes",
     r'''const bare = (pattern: string): string => pattern.replace(/^!?\{(?:workspaceRoot|projectRoot)\}\//u, "").replace(/^(?:\.\/)+/u, "");''',
     r'''/** 🔤️ The prefixes of an input pattern that name its root and the shapes of a manifest header and a router branch, compiled once instead of once per call. */
const ROOT_PREFIX_V1 = /^!?\{(?:workspaceRoot|projectRoot)\}\//u, DOT_SLASH_V1 = /^(?:\.\/)+/u, MANIFEST_HEADER_V1 = /^\s*\[([^\]]+)\]\s*$/u;
const ROUTER_BRANCH_V1 = /^(\s*)if \(segments\[0\] === "([^"]+)"(?: && segments\[1\] === "([^"]+)")?/u;
const bare = (pattern: string): string => pattern.replace(ROOT_PREFIX_V1, "").replace(DOT_SLASH_V1, "");''')
edit(PRECISION, "precision manifest header", r'''const header = /^\s*\[([^\]]+)\]\s*$/u.exec(line);''', r'''const header = MANIFEST_HEADER_V1.exec(line);''')
edit(PRECISION, "precision router branch", r'''const head = /^(\s*)if \(segments\[0\] === "([^"]+)"(?: && segments\[1\] === "([^"]+)")?/u.exec(lines[index]!);''', r'''const head = ROUTER_BRANCH_V1.exec(lines[index]!);''')

edit(INPUTS, "inputs pattern characters",
     r'''function compileBuildInputPatternV1(pattern) {
  const compile=(source)=>{''',
     r'''/** 🔤️ The characters of a portable input pattern that a regular expression must escape or that make it a glob, compiled once instead of once per character. */
const REGEX_METACHARACTER_V1 = /[\^$.*+?()[\]{}|]/g, GLOB_CHARACTER_V1 = /[!*?{}[\]]/u;

function compileBuildInputPatternV1(pattern) {
  const compile=(source)=>{''')
edit(INPUTS, "inputs escape", r'''else result+=character.replace(/[\^$.*+?()[\]{}|]/g,"\\$&");''', r'''else result+=character.replace(REGEX_METACHARACTER_V1,"\\$&");''')
edit(INPUTS, "inputs glob character", r'''input.startsWith("{workspaceRoot}/") && !/[!*?{}[\]]/u.test(input.slice("{workspaceRoot}/".length))''', r'''input.startsWith("{workspaceRoot}/") && !GLOB_CHARACTER_V1.test(input.slice("{workspaceRoot}/".length))''')

edit(DISCOVERY, "discovery unsafe path",
     r'''/** 🏭️ Resolves only one exact ignored output contract to its physical authored task owner. */
export function readBuildGeneratedCompileOutputV1(''',
     r'''/** 🚧️ The characters a generated import path may not hold, compiled once. */
const UNSAFE_GENERATED_PATH_V1 = /[\\%*?#[\]{}]/u;

/** 🏭️ Resolves only one exact ignored output contract to its physical authored task owner. */
export function readBuildGeneratedCompileOutputV1(''')
edit(DISCOVERY, "discovery unsafe use", r'''&&!/[\\%*?#[\]{}]/u.test(value);const refusal=''', r'''&&!UNSAFE_GENERATED_PATH_V1.test(value);const refusal=''')

edit(SCRIPTS, "scripts source use 1", r'''return { path, source: /\.[cm]?[jt]sx?$/u.test(path) };''', r'''return { path, source: DECLARED_MODULE_SUFFIX_V1.test(path) };''')
edit(SCRIPTS, "scripts source use 2", r'''if(!source||!/\.[cm]?[jt]sx?$/u.test(path)){''', r'''if(!source||!DECLARED_MODULE_SUFFIX_V1.test(path)){''')

edit(ASSEMBLY, "assembly prefix pattern", 'import { createHash } from "node:crypto";', 'import { createHash } from "node:crypto";\n\n/** 🧭️ The workspace placeholder of an input path, compiled once. */\nconst WORKSPACE_PLACEHOLDER_V1 = /^\\{workspaceRoot\\}\\//u;')
edit(ASSEMBLY, "assembly prefix use 1", r'''.map((path) => path.replace(/^\{workspaceRoot\}\//u, "")),''', r'''.map((path) => path.replace(WORKSPACE_PLACEHOLDER_V1, "")),''')
edit(ASSEMBLY, "assembly prefix use 2", r'''loaded.buildInputPathMatchesV1(path.replace(/^\{workspaceRoot\}\//u, "")''', r'''loaded.buildInputPathMatchesV1(path.replace(WORKSPACE_PLACEHOLDER_V1, "")''')
edit(ASSEMBLY, "assembly prefix use 3", r'''files.add(path.replace(/^\{workspaceRoot\}\//u, ""));''', r'''files.add(path.replace(WORKSPACE_PLACEHOLDER_V1, ""));''')

edit(ARGUMENTS, "arguments whitespace head",
     r'''/** 🔤️ Reads a literal process command without evaluating shell operators or substitutions. */
export function literalCommandTokensV1(command){''',
     r'''/** 🔤️ Whitespace and shell operator characters, compiled once instead of once per character. */
const WHITESPACE_V1 = /\s/u, SHELL_OPERATOR_V1 = /[;&|<>`$"']/u;

/** 🔤️ Reads a literal process command without evaluating shell operators or substitutions. */
export function literalCommandTokensV1(command){''')
edit(ARGUMENTS, "arguments whitespace 1", r'''while(/\s/u.test(command[cursor]??"")&&cursor<command.length)cursor++;''', r'''while(WHITESPACE_V1.test(command[cursor]??"")&&cursor<command.length)cursor++;''')
edit(ARGUMENTS, "arguments whitespace 2", r'''else{while(cursor<command.length&&!/\s/u.test(command[cursor]))cursor++;const token=command.slice(start,cursor);if(/[;&|<>`$"']/u.test(token))''', r'''else{while(cursor<command.length&&!WHITESPACE_V1.test(command[cursor]))cursor++;const token=command.slice(start,cursor);if(SHELL_OPERATOR_V1.test(token))''')
edit(ARGUMENTS, "arguments whitespace 3", r'''if(cursor<command.length&&!/\s/u.test(command[cursor]))throw''', r'''if(cursor<command.length&&!WHITESPACE_V1.test(command[cursor]))throw''')

edit(ANALYSES, "analyses rust patterns",
     r'''function rustInputTokens(source) {
  const tokens = [], rawPattern = /(?:b|c)?r(#+)?"/y, characterPattern = /(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/uy, identifierPattern = /(?:r#)?[\p{ID_Start}_][\p{ID_Continue}]*/uy;''',
     r'''const RUST_RAW_PATTERN_V1 = /(?:b|c)?r(#+)?"/y, RUST_CHARACTER_PATTERN_V1 = /(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/uy, RUST_IDENTIFIER_PATTERN_V1 = /(?:r#)?[\p{ID_Start}_][\p{ID_Continue}]*/uy, UNICODE_WHITESPACE_V1 = /\s/u, RAW_IDENTIFIER_PREFIX_V1 = /^r#/;

function rustInputTokens(source) {
  const tokens = [], rawPattern = RUST_RAW_PATTERN_V1, characterPattern = RUST_CHARACTER_PATTERN_V1, identifierPattern = RUST_IDENTIFIER_PATTERN_V1;''')
edit(ANALYSES, "analyses whitespace", r'''code > 127 && /\s/u.test(char)) { i++; continue; }''', r'''code > 127 && UNICODE_WHITESPACE_V1.test(char)) { i++; continue; }''')
edit(ANALYSES, "analyses raw prefix", r'''tokens[i + 1].text.replace(/^r#/, "")''', r'''tokens[i + 1].text.replace(RAW_IDENTIFIER_PREFIX_V1, "")''')

edit(OBSERVATION, "observation digest pattern", r'''!/^[a-f0-9]{64}$/u.test(fact.source.sha256??"")||!/^[a-f0-9]{64}$/u.test(fact.resultSha256??"")''', r'''!SHA256_HEX_V1.test(fact.source.sha256??"")||!SHA256_HEX_V1.test(fact.resultSha256??"")''')

edit(OBSERVATION, "observation file record images",
     r'''const settledReads = new Map(), SETTLED_READ_BYTES = 128 * 1024 * 1024;''',
     r'''const settledReads = new Map(), SETTLED_READ_BYTES = 128 * 1024 * 1024;

/** 🧠️ The JSON image of the analysis of a settled file, per namespace and path, while the file keeps its stamp: the analysis is a pure function of the bytes, so a file that many projects ask about is read, hashed, parsed and stored once and every later asker parses the image. Bounded like the settled reads. */
const fileRecordImages = new Map(), FILE_RECORD_IMAGE_BYTES = 64 * 1024 * 1024, FILE_RECORD_IMAGE_LIMIT = 1024 * 1024;
let fileRecordImageBytes = 0;
function rememberFileRecordImageV1(memoKey, key, image) {
  const settled = settledReads.get(key), previous = fileRecordImages.get(memoKey);
  if (previous) { fileRecordImageBytes -= previous.image.length; fileRecordImages.delete(memoKey); }
  if (settled === undefined || image.length > FILE_RECORD_IMAGE_LIMIT) return;
  while (fileRecordImageBytes + image.length > FILE_RECORD_IMAGE_BYTES && fileRecordImages.size > 0) {
    const oldest = fileRecordImages.keys().next().value;
    fileRecordImageBytes -= fileRecordImages.get(oldest).image.length;
    fileRecordImages.delete(oldest);
  }
  fileRecordImages.set(memoKey, { size: settled.size, mtimeMs: settled.mtimeMs, ctimeMs: settled.ctimeMs, ino: settled.ino, row: settled.row, image });
  fileRecordImageBytes += image.length;
}
function rememberedFileRecordImageV1(memoKey, path) {
  const known = fileRecordImages.get(memoKey);
  if (known === undefined) return undefined;
  let stamp;
  try { stamp = fs.statSync(path); } catch {}
  if (stamp && stamp.size === known.size && stamp.mtimeMs === known.mtimeMs && stamp.ctimeMs === known.ctimeMs && stamp.ino === known.ino) return known;
  fileRecordImageBytes -= known.image.length;
  fileRecordImages.delete(memoKey);
  return undefined;
}''')
edit(OBSERVATION, "observation file record release", ''' settledReads.clear();settledReadBytes=0;sourceAnalysisImages.clear();sourceAnalysisBytes=0;''', ''' settledReads.clear();settledReadBytes=0;fileRecordImages.clear();fileRecordImageBytes=0;sourceAnalysisImages.clear();sourceAnalysisBytes=0;''')
edit(OBSERVATION, "observation file record memo hit",
     '''  const key = absolute(path), hint = scope.hints.get(key);
  if (hint) {''',
     '''  const key = absolute(path), memoKey = namespace + "\\0" + key, remembered = rememberedFileRecordImageV1(memoKey, path);
  if (remembered !== undefined) { recordRowV1(remembered.row); return JSON.parse(remembered.image); }
  const hint = scope.hints.get(key);
  if (hint) {''')
edit(OBSERVATION, "observation file record memo store",
     '''  const bytes = observedFs.readFileSync(path), value = compute(bytes), image = jsonImage(value);
  if (image === undefined) return value;''',
     '''  const bytes = observedFs.readFileSync(path), value = compute(bytes), image = jsonImage(value);
  if (image === undefined) return value;
  rememberFileRecordImageV1(memoKey, key, image);''')

NATIVE = B + "🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs"
edit(NATIVE, "native source inputs memo",
     r'''export function cargoSourceInputs(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml");''',
     r'''export function cargoSourceInputs(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  if (facts === null || typeof facts !== "object") return cargoSourceInputsOfV1(root, workspaceRoot, facts, includeTests, compiled);
  let memo = CARGO_SOURCE_INPUTS_V1.get(facts);
  if (memo === undefined) CARGO_SOURCE_INPUTS_V1.set(facts, memo = new Map());
  const files = memoizedObservedComputationV1(memo, JSON.stringify([root, workspaceRoot, includeTests, compiled]), () => cargoSourceInputsOfV1(root, workspaceRoot, facts, includeTests, compiled));
  return files === undefined ? undefined : [...files];
}

/** 🧠️ The answers of {@link cargoSourceInputs} per fact table (one per project): the targets of a project ask for the same crates again and again, and a replayed answer hands every asker the rows it rests on without probing the tree again. */
const CARGO_SOURCE_INPUTS_V1 = new WeakMap();

function cargoSourceInputsOfV1(root, workspaceRoot, facts, includeTests, compiled) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml");''')

edit(ANALYSES, "analyses string literal",
     r'''      try { tokens.push({ text: JSON.parse(literal), string: true }); }
      catch { tokens.push({ text: literal.slice(1, -1), string: true }); }''',
     r'''      if (!literal.includes("\\")) tokens.push({ text: literal.slice(1, -1), string: true });
      else try { tokens.push({ text: JSON.parse(literal), string: true }); }
      catch { tokens.push({ text: literal.slice(1, -1), string: true }); }''')

edit(SCRIPTS, "scripts source use 3", r'''if(requiredSources||resource.suffix||!/\.[cm]?[jt]sx?$/u.test(resource.specifier))''', r'''if(requiredSources||resource.suffix||!DECLARED_MODULE_SUFFIX_V1.test(resource.specifier))''')
edit(SCRIPTS, "scripts source use 4", r'''&& /\.[cm]?[jt]sx?$/u.test(input) &&''', r'''&& DECLARED_MODULE_SUFFIX_V1.test(input) &&''')
edit(ASSEMBLY, "assembly prefix use 4", r'''.replaceAll("{projectRoot}", nativeRoot).replace(/^\{workspaceRoot\}\//u, "");''', r'''.replaceAll("{projectRoot}", nativeRoot).replace(WORKSPACE_PLACEHOLDER_V1, "");''')
edit(INPUTS, "inputs overlap wildcard", r'''wildcard = source.search(/[!*?{}[\]]/u)''', r'''wildcard = source.search(GLOB_CHARACTER_V1)''')
edit(OBSERVATION, "observation revision pattern", r'''typeof revision!=="string"||!/^[a-f0-9]{64}$/u.test(revision)||''', r'''typeof revision!=="string"||!SHA256_HEX_V1.test(revision)||''')
edit(OBSERVATION, "observation publication pattern", r'''if(row.kind==="file"&&/^[a-f0-9]{64}$/u.test(row.fact?.sha256??"")&&''', r'''if(row.kind==="file"&&SHA256_HEX_V1.test(row.fact?.sha256??"")&&''')

if __name__ == "__main__":
    for e in edits:
        text = open(os.path.join(ROOT, e["file"]), encoding="utf8").read()
        if text.count(e["old"]) != 1:
            sys.exit(f'anchor "{e["name"]}" occurs {text.count(e["old"])} times in {e["file"]}')
    json.dump({"version": 1, "edits": edits}, open(os.path.join(HERE, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(len(edits), "edits")

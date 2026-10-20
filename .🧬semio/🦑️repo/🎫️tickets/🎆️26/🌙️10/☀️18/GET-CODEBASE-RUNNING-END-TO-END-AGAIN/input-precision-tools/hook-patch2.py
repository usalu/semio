#!/usr/bin/env python3
# 🪝 Anchored edits of the live inference hook and the graph storage schema that replace the whole-program closure by the receipt-backed program analysis: the compiler side (`precisionProgramHostV1`), the receipt leaf (`ip/<24 hex>.json`) and its admission as an optional graph cache leaf. Each function maps the CURRENT live text to the new text and is idempotent.
import re
MARK = "precisionProgramHostV1"

HOST = '''/** 🔷️ The compiler side of the TypeScript program analysis: one file's imports resolved with the compiler's own module resolution under the tsconfig options (a fresh resolution cache per file, so the facts it consulted are all recorded), and the answer to any recorded fact, both through the observed file system so the enclosing inference depends on exactly what a derivation would have read. */
function precisionProgramHostV1(workspaceRoot, scripts, documentOf, compilerOf) {
  const prefix = `${workspaceRoot}/`, absolutePath = (path) => (isAbsolute(path) ? path : join(workspaceRoot, path)), relativePath = (path) => (path.startsWith(prefix) ? buildPath(path.slice(prefix.length)) : path), sha = (value) => createHash("sha256").update(value).digest("hex");
  const programs = new Map();
  let observed, analysis;
  const sysOf = () => (observed ??= observedTypeScriptHost(compilerOf().sys));
  const optionsOf = (tsconfig) => {
    let options = programs.get(tsconfig);
    if (!options) {
      const compiler = compilerOf(), path = join(workspaceRoot, tsconfig), read = compiler.readConfigFile(path, compiler.sys.readFile);
      options = compiler.parseJsonConfigFileContent(read.config, { ...compiler.sys, readDirectory: () => [] }, dirname(path), undefined, path).options;
      programs.set(tsconfig, options);
    }
    return options;
  };
  const answer = (kind, path) => {
    const sys = sysOf();
    if (kind === "file") return sys.fileExists(path);
    if (kind === "directory") return sys.directoryExists(path);
    if (kind === "text") { const text = sys.readFile(path); return text === undefined ? "!missing" : sha(text); }
    if (kind === "realpath") return sys.realpath?.(path) ?? null;
    if (kind === "directories") return JSON.stringify(sys.getDirectories(path));
    try { return sha(readFileSync(path)); } catch { return null; }
  };
  const receipts = precisionReceiptsV1(workspaceRoot);
  return {
    get compiler() { return compilerOf().version; },
    fact: (kind, path) => answer(kind, absolutePath(path)),
    imports: async (file, tsconfig) => {
      const compiler = compilerOf(), options = optionsOf(tsconfig), base = sysOf(), facts = [], recorded = new Set(), source = join(workspaceRoot, file), resolved = new Set();
      const record = (kind, path, value) => {
        const fact = { kind, path: relativePath(path), value }, id = JSON.stringify(fact);
        if (!recorded.has(id)) { recorded.add(id); facts.push(fact); }
        return value;
      };
      const sys = {
        ...base,
        fileExists: (path) => record("file", path, base.fileExists(path)),
        directoryExists: (path) => record("directory", path, base.directoryExists(path)),
        readFile: (path, encoding) => { const text = base.readFile(path, encoding); record("text", path, text === undefined ? "!missing" : sha(text)); return text; },
        realpath: base.realpath && ((path) => { const value = base.realpath(path); record("realpath", path, value ?? null); return value; }),
        getDirectories: (path) => { const value = base.getDirectories(path); record("directories", path, JSON.stringify(value)); return value; },
      };
      const cache = compiler.createModuleResolutionCache(workspaceRoot, (value) => value, options);
      for (const specifier of observedSourceAnalysisV1(analysis ??= prepareSourceAnalysesV1().module, source).imports) {
        const hit = compiler.resolveModuleName(specifier, source, options, sys, cache).resolvedModule?.resolvedFileName;
        if (hit && hit.startsWith(prefix) && !hit.includes("/node_modules/")) resolved.add(buildPath(hit.slice(prefix.length)));
      }
      return { resolved: [...resolved].sort(), facts };
    },
    receipts,
    check: (phase, index) => yieldInferenceV1(scripts.context, phase, index),
    report: (event) => {
      const document = documentOf();
      process.stderr.write(`[input-precision] ${inputPrecisionTextV1(document, event, inputPrecisionLanguageV1(document, process.env))}\\n`);
    },
  };
}

/** 📇️ Keeps the program receipts as optional, recomputable graph cache leaves (`ip/<24 hex>.json`, brotli) under the graph data directory; they are read outside the observation because every fact they carry is re-checked through it. */
function precisionReceiptsV1(workspaceRoot) {
  const directory = () => join(workspaceDataDirectory(workspaceRoot), "ip"), leaf = (key) => join(directory(), `${createHash("sha256").update(key).digest("hex").slice(0, 24)}.json`);
  return {
    read: (key) => { try { return brotliDecompressSync(rawReadFileSync(leaf(key))).toString("utf8"); } catch { return undefined; } },
    write: (key, image) => {
      const target = leaf(key), bytes = brotliCompressSync(Buffer.from(image, "utf8"), { params: { [zlibConstants.BROTLI_PARAM_QUALITY]: 5, [zlibConstants.BROTLI_PARAM_SIZE_HINT]: image.length } });
      let previous = 0;
      try { previous = rawLstatSync(target).size; } catch {}
      withBuildGraphOptionalPublicationV1(target, { peakBytes: bytes.byteLength, retainedDeltaBytes: Math.min(bytes.byteLength, bytes.byteLength - previous) }, () => {
        const temporary = `${target}.${process.pid}.${Date.now()}.tmp`;
        rawMkdirSync(directory(), { recursive: true });
        rawWriteFileSync(temporary, bytes, { flag: "wx" });
        rawRenameSync(temporary, target);
      });
    },
  };
}

'''

IMPORTS = '''import { createHash } from "node:crypto";
import { lstatSync as rawLstatSync, mkdirSync as rawMkdirSync, readFileSync as rawReadFileSync, renameSync as rawRenameSync, writeFileSync as rawWriteFileSync } from "node:fs";
import { brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";
import { withBuildGraphOptionalPublicationV1 } from "../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
'''

def assembly(text):
    if MARK in text: return text, False
    def once(anchor):
        if text.count(anchor) != 1: raise SystemExit("anchor count %d: %s" % (text.count(anchor), anchor[:80]))
    anchor = 'import { createRequire } from "node:module";\n'; once(anchor)
    text = text.replace(anchor, anchor + IMPORTS)
    anchor = "import { deriveInputsV1, inputPrecisionDecisionV1, readInputPrecisionV1, selectRuleV1 } from"; once(anchor)
    text = text.replace(anchor, "import { deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from")
    anchor = "let targetScriptSources,libraryBootstrap,"; once(anchor)
    text = text.replace(anchor, "let workspaceDataDirectory, targetScriptSources,libraryBootstrap,")
    anchor = "({ buildPath, LIBRARY_ROOT, POLICY, WORKSPACE_POLICY, PROJECT_BASENAME, SOURCE_INPUT_MODULE, SCRIPT_BASENAME, exampleCollectionInputs, owned } = registry.coordinates);"; once(anchor)
    text = text.replace(anchor, anchor.replace("owned }", "owned, workspaceDataDirectory }"))
    start = text.find("/** 🔷️ The files of a TypeScript program reachable"); end = text.find("/** 🎯️ The input precision rules and the engine host")
    if start < 0 or end < start or text.count("/** 🔷️ The files of a TypeScript program reachable") != 1 or text.count("/** 🎯️ The input precision rules and the engine host") != 1: raise SystemExit("anchor: program closure block")
    text = text[:start] + HOST + text[end:]
    match = re.search(r"^    importClosure: async \(entries, program\) => program \? precisionProgramClosureV1\(.*\n", text, re.M)
    if not match or len(re.findall(r"^    importClosure: async \(entries, program\)", text, re.M)) != 1: raise SystemExit("anchor: importClosure line")
    line = match.group(0)
    tail = line[line.index(": (await relativeScriptInputs"):]
    text = text.replace(line, "    importClosure: async (entries) => " + tail[2:].replace("(await relativeScriptInputs", "(await relativeScriptInputs", 1))
    anchor = "    cargoToolchain: POLICY.toolchains.cargo.files,\n  };\n  const context = { document, host };"; once(anchor)
    text = text.replace(anchor, '    cargoToolchain: POLICY.toolchains.cargo.files,\n    program: precisionProgramHostV1(workspaceRoot, scripts, () => document, () => (compiler ??= createRequire(import.meta.url)("typescript"))),\n  };\n  const context = { document, host };')
    return text, True

STORAGE_OLD = "(?:np/[a-f0-9]{24}\\\\.json|"
STORAGE_NEW = "(?:(?:np|ip)/[a-f0-9]{24}\\\\.json|"
def storage_schema(text):
    if STORAGE_NEW in text: return text, False
    if text.count(STORAGE_OLD) != 1: raise SystemExit("storage schema anchor count %d" % text.count(STORAGE_OLD))
    return text.replace(STORAGE_OLD, STORAGE_NEW), True

FIXTURE_OLD = '      "np/aaaaaaaaaaaaaaaaaaaaaaaa.json",\n'
def graph_fixture(text):
    new = FIXTURE_OLD + '      "ip/aaaaaaaaaaaaaaaaaaaaaaaa.json",\n'
    if new in text: return text, False
    if text.count(FIXTURE_OLD) != 1: raise SystemExit("graph fixture anchor count %d" % text.count(FIXTURE_OLD))
    return text.replace(FIXTURE_OLD, new), True

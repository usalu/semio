#!/usr/bin/env python3
"""Staged show-perf patch: anchored edits of the graph inference hot path (each anchor must occur exactly once; idempotent; refuses on drift).

usage: apply.py check                  report per anchor: pending | applied | DRIFT
       apply.py apply [--dry-run] [--ignore-running]
       apply.py diff                   unified diff of the pending edits
       apply.py dump-ops               JSON {file: [[anchor, replacement], ...]} for the benchmark harness
Targets (library `L`): 🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs and 🏗️build/📥️inputs/🟨️.mjs."""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def E(code):
    return chr(code) + "️"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIBRARY, E(0x1F3D7) + "build")
SCRIPT_INPUTS = os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F4E5) + "inference", E(0x1F9ED) + "script-inputs", E(0x1F7E8) + ".mjs")
INPUTS = os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1F7E8) + ".mjs")
OBSERVATION = '"../../../../' + E(0x1F3D7) + "build/" + E(0x1F441) + "observation/" + E(0x1F7E8) + '.mjs"'

DIGEST_HELPERS = '''/** \U0001F511️ Names a memo entry by the digest of its ordered parts, so a long path list is hashed once and never retained as a key string. */
function memoDigestV1(...parts) {
  const hash = createHash("sha256");
  for (const part of parts) {
    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : "\\u0001").update("\\0"); hash.update("]"); }
    else hash.update(String(part)).update("\\0");
  }
  return hash.digest("hex");
}

/** \U0001F4DB️ The workspace-relative name of one closure file, shared by every closure of one graph construction instead of re-created for each. */
function closurePathNameV1(cache, workspaceRoot, file) {
  let names = cache.paths.get(workspaceRoot);
  if (!names) cache.paths.set(workspaceRoot, names = new Map());
  let name = names.get(file);
  if (name === undefined) names.set(file, name = `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`);
  return name;
}

/** \U0001F9EE️ Resolves the literal module inputs of one target list through one observed memo per distinct list: the command inputs that every generated Cargo target repeats verbatim are probed once, and every receiving project still replays the same existence facts. */
function declaredModuleInputsV1(inputs, workspaceRoot, projectRoot, cache) {
  if (!Array.isArray(inputs) || !inputs.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1(workspaceRoot, projectRoot, inputs), () => inputs.flatMap(input => { const declared = declaredModuleInputV1(input, workspaceRoot, projectRoot); return declared ? [declared] : []; }));
}

'''

INVENTORY_MEMO = '''const INVENTORY_MEMO_V1 = new Map(), INVENTORY_MEMO_LIMIT_V1 = 512;

/** \U0001F5C3️ Reuses the immutable captured inventory of an identical file roster: it is a pure function of the roster and the workspace root, and thousands of projects share the same command closure. */
function capturedInventoryV1(files, workspaceRoot) {
  const hash = createHash("sha256").update(workspaceRoot).update("\\0");
  for (const file of files) hash.update(file).update("\\0");
  const key = hash.digest("hex"), known = INVENTORY_MEMO_V1.get(key);
  if (known !== undefined) return known;
  const captured = captureSourceInputInventoryV1({ version: 1, files }, workspaceRoot);
  if (INVENTORY_MEMO_V1.size >= INVENTORY_MEMO_LIMIT_V1) INVENTORY_MEMO_V1.delete(INVENTORY_MEMO_V1.keys().next().value);
  INVENTORY_MEMO_V1.set(key, captured);
  return captured;
}

'''

OPS = {
    SCRIPT_INPUTS: [
        ('import { createRequire, isBuiltin } from "node:module";\n',
         'import { createRequire, isBuiltin } from "node:module";\nimport { createHash } from "node:crypto";\n'),
        ('import { memoizedObservedAsyncComputationV1, observedSourceAnalysisV1,',
         'import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1,'),
        ('generatedImports:new Map(),context,providerDesignation:',
         'generatedImports:new Map(),declared:new Map(),paths:new Map(),context,providerDesignation:'),
        ('/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */',
         DIGEST_HELPERS + '/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */'),
        ('key = JSON.stringify([resolve(workspaceRoot), roots,requiredSources]);',
         'key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources);'),
        ('if (buildPath(relative(workspaceRoot, path)).startsWith("../")) throw new Error(`Command import escapes workspace: ${path}`);',
         'if (closurePathNameV1(cache, workspaceRoot, path).startsWith("{workspaceRoot}/../")) throw new Error(`Command import escapes workspace: ${path}`);'),
        ('const result = [...files].map((file) => `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`).sort();',
         'const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();'),
        ('for(const input of target.inputs??[]){const declared=declaredModuleInputV1(input,workspaceRoot,projectRoot);if(declared)entries.add(declared);}',
         'for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts))entries.add(declared);'),
        ('const key=JSON.stringify([closure,sources.fallback?fallback:null]);',
         'const key=memoDigestV1(closure,sources.fallback?fallback:null);'),
    ],
    INPUTS: [
        ('/** \U0001F9EE️ Shares exact inventories while backends hash their current byte digest once per unique closure. */',
         INVENTORY_MEMO + '/** \U0001F9EE️ Shares exact inventories while backends hash their current byte digest once per unique closure. */'),
        ('captured = captureSourceInputInventoryV1({ version: 1, files },workspaceRoot), contract = captured.text;',
         'captured = capturedInventoryV1(files, workspaceRoot), contract = captured.text;'),
    ],
}


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def classify(text, anchor, replacement):
    if replacement in text:
        return "applied"
    count = text.count(anchor)
    return "pending" if count == 1 else "DRIFT(anchor occurs %d times)" % count


def running():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout
    return [line.strip()[:140] for line in out.splitlines() if ("repo show projects" in line or "dependencies sync" in line or "dependencies lock" in line or "repo tools setup" in line) and "apply.py" not in line and "ps -axo" not in line]


def main():
    command = sys.argv[1] if len(sys.argv) > 1 else "check"
    if command == "dump-ops":
        print(json.dumps({os.path.basename(os.path.dirname(path)): [[a, r] for a, r in ops] for path, ops in OPS.items()}, ensure_ascii=False))
        return
    states, texts, drift = [], {}, False
    for path, ops in OPS.items():
        absolute = os.path.join(ROOT, path)
        texts[path] = read(absolute)
        for index, (anchor, replacement) in enumerate(ops):
            state = classify(texts[path], anchor, replacement)
            drift |= state.startswith("DRIFT")
            states.append((path, index, state))
    if command == "check":
        for path, index, state in states:
            print("%-8s %s#%d" % (state, os.path.basename(os.path.dirname(path)), index))
        sys.exit(1 if drift else 0)
    if command == "diff":
        import difflib
        for path, ops in OPS.items():
            text = texts[path]
            for anchor, replacement in ops:
                if replacement not in text:
                    text = text.replace(anchor, replacement)
            sys.stdout.writelines(difflib.unified_diff(texts[path].splitlines(True), text.splitlines(True), "a/" + path, "b/" + path, n=1))
        return
    if command != "apply":
        raise SystemExit("unknown command " + command)
    if drift:
        for path, index, state in states:
            if state.startswith("DRIFT"):
                print("refused: %s#%d %s" % (os.path.basename(os.path.dirname(path)), index, state), file=sys.stderr)
        raise SystemExit("drift: re-stage against the live files; nothing written")
    live = running()
    if live and "--ignore-running" not in sys.argv:
        raise SystemExit("refused: a graph/dependency command is running (a changed inference module restarts its walk):\n" + "\n".join(live))
    dry = "--dry-run" in sys.argv
    for path, ops in OPS.items():
        text = texts[path]
        for anchor, replacement in ops:
            if replacement in text:
                continue
            text = text.replace(anchor, replacement)
        if text != texts[path]:
            print(("would write " if dry else "writing ") + path)
            if not dry:
                temporary = os.path.join(ROOT, path) + ".showperf.tmp"
                with open(temporary, "w", encoding="utf8") as handle:
                    handle.write(text)
                os.replace(temporary, os.path.join(ROOT, path))
        else:
            print("unchanged " + path)


main()

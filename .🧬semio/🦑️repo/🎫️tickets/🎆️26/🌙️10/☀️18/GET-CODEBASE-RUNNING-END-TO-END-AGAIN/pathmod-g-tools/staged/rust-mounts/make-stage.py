#!/usr/bin/env python3
"""🏗️ make-stage.py : author-time only. Reads the LIVE library files, builds the ANCHORED edits of the rust-mounts batch (old text exactly as it is now, new text), records the base sha256 of every touched file and writes edits.json. stage.py then applies / reverts / checks them on any root. Re-run when the live anchors changed."""
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
INF = LIB + "/🏗️build/🕸️graph/📥️inference"
SRC, NAT, INT, ASM = INF + "/🧬️source-analyses/🟨️.mjs", INF + "/🦀️native-manifests/🟨️.mjs", INF + "/🧰️internals/🟨️.mjs", INF + "/🧱️project-assembly/🟨️.mjs"
def read(rel): return open(os.path.join(ROOT, rel), encoding="utf-8").read()
def between(text, start, end):
    a = text.index(start); b = text.index(end, a) + len(end)
    return text[a:b]
edits = []
def edit(file, old, new, note):
    text = read(file)
    if text.count(old) != 1: sys.exit("anchor not unique (%d) in %s: %r" % (text.count(old), file, old[:80]))
    edits.append({"file": file, "old": old, "new": new, "note": note})

edit(SRC,
 "  const gated = [];\n  let fileTest = false;\n",
 "  const emitsMount = (from, to) => {\n    for (let at = from; at < to; at++) {\n      if (tokens[at].string) continue;\n      const named = tokens[at].text === \"mod\" && tokens[at + 1]?.text === \"$\" ? at + 2 : at + 1;\n      if (tokens[at].text === \"mod\" && tokens[named]?.identifier && tokens[named + 1]?.text === \";\") return true;\n      if (tokens[at].text === \"#\" && (tokens[at + 1]?.text === \"[\" || tokens[at + 1]?.text === \"!\" && tokens[at + 2]?.text === \"[\")) {\n        const open = tokens[at + 1].text === \"[\" ? at + 1 : at + 2;\n        if (tokens[open + 1]?.text === \"path\" && tokens[open + 2]?.text === \"=\") return true;\n      }\n    }\n    return false;\n  };\n  const gated = [];\n  let fileTest = false;\n",
 "macro bodies: mount detector")
edit(SRC,
 "      const macro = tokens[i]?.text === \"macro_rules\";\n      while (i < end && tokens[i].text !== \";\" && tokens[i].text !== \"{\") { if (pairs.has(i)) i = pairs.get(i); i++; }\n      if (macro && tokens.slice(i, pairs.get(i) ?? i).some((token) => !token.string && token.text === \"mod\")) throw new Error(\"Generated Rust modules require conservative source inputs\");\n",
 "      const macro = tokens[i]?.text === \"macro_rules\", bodyFrom = i;\n      while (i < end && tokens[i].text !== \";\" && tokens[i].text !== \"{\") { if (pairs.has(i)) i = pairs.get(i); i++; }\n      if (macro && emitsMount(bodyFrom, tokens[i]?.text === \"{\" ? pairs.get(i) ?? i : i)) throw new Error(\"Generated Rust modules require conservative source inputs\");\n",
 "macro_rules: refuse only a body that can mount a file")
edit(SRC,
 "which the compiler still reads but never expands) are marked, so a consumer that compiles without that configuration leaves them out. */",
 "which the compiler still reads but never expands) are marked, so a consumer that compiles without that configuration leaves them out. A `macro_rules!` body is refused only when it can mount a file (`mod name;` or a `#[path]` attribute): an inline `mod name { … }` it emits reads no file; an `include*!` inside `quote!` is code the crate emits into its consumers, counted as `emitted` and never an input of the crate itself. */",
 "docstring")

edit(SRC,
 "  const places = [];\n  for (let i = 0; i < tokens.length - 3; i++) {\n    if (!tokens[i].identifier || ![\"include\", \"include_str\", \"include_bytes\"].includes(tokens[i].text) || tokens[i + 1].text !== \"!\") continue;\n    const end = pairs.get(i + 2);\n",
 "  const quoted = [];\n  for (let i = 0; i < tokens.length - 2; i++) if (tokens[i].identifier && [\"quote\", \"quote_spanned\", \"parse_quote\", \"parse_quote_spanned\"].includes(tokens[i].text) && tokens[i + 1].text === \"!\" && pairs.get(i + 2) !== undefined) quoted.push([i + 2, pairs.get(i + 2)]);\n  let emitted = 0;\n  const places = [];\n  for (let i = 0; i < tokens.length - 3; i++) {\n    if (!tokens[i].identifier || ![\"include\", \"include_str\", \"include_bytes\"].includes(tokens[i].text) || tokens[i + 1].text !== \"!\") continue;\n    if (quoted.some(([from, to]) => i > from && i < to)) { emitted++; continue; }\n    const end = pairs.get(i + 2);\n",
 "includes inside quote! bodies are emitted code, not inputs of this crate")
edit(SRC,
 "  return { includes, modules, ...(testIncludes.length ? { testIncludes } : {}), ...(fileTest ? { fileTest } : {}) };",
 "  return { includes, modules, ...(emitted ? { emitted } : {}), ...(testIncludes.length ? { testIncludes } : {}), ...(fileTest ? { fileTest } : {}) };",
 "emitted count in the reference fact")
edit(SRC,
 "      try { tokens.push({ text: JSON.parse(literal), string: true }); }\n      catch { tokens.push({ text: literal.slice(1, -1), string: true }); }\n",
 "      tokens.push({ text: unescapeRustString(literal.slice(1, -1)), string: true });\n",
 "string literals unescape as Rust does (\\u{...}, \\xNN, line continuation), not as JSON")
edit(SRC,
 "  const tokens = [], rawPattern = /(?:b|c)?r(#+)?\"/y,",
 "  const unescapeRustString = (body) => body.replace(/\\\\(?:\\r?\\n\\s*|u\\{([0-9a-fA-F_]+)\\}|x([0-9a-fA-F]{2})|(.))/gsu, (_, brace, hex, plain) => brace !== undefined ? String.fromCodePoint(parseInt(brace.replaceAll(\"_\", \"\"), 16)) : hex !== undefined ? String.fromCharCode(parseInt(hex, 16)) : plain === undefined ? \"\" : ({ n: \"\\n\", r: \"\\r\", t: \"\\t\", 0: \"\\0\" })[plain] ?? plain);\n  const tokens = [], rawPattern = /(?:b|c)?r(#+)?\"/y,",
 "unescape helper inside rustInputTokens so the parser revision hashes it")
nat = read(NAT)
old_block = between(nat, "/** 🧭️ Cargo owns compilation while the build graph tracks mounted sources and embedded assets. */\nexport function cargoSourceInputs(", "  catch { return undefined; }\n}\n")
new_block = '''const RUST_REFUSALS = [["Generated Rust modules", "macro-mount"], ["Conditional Rust mount", "conditional-mount"], ["Dynamic Rust include", "dynamic-include"]];

/** 🧾️ The Rust source set of one crate as the `rust-source-set` fact: the files a compilation of the crate reads (and the files that emit `include*!` into their consumers, whose reads no static reading of the crate can see), or the reason no static reading exists (a build script, a macro or conditional mount, a dynamic include, no entry point, an unparsable file).
 * @see ./🧬️schema/🔣️.json */
export function cargoSourceSetV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml"), unresolved = (reason, message) => ({ version: 1, root, resolved: false, reason, ...(message ? { message } : {}) });
  if (!existsSync(manifest)) return unresolved("no-manifest");
  const cargo = readToml(manifest), entries = [cargo.lib?.path ?? "src/lib.rs", "src/main.rs", ...(cargo.bin ?? []).map((target) => target.path), ...(includeTests ? [...(cargo.test ?? []), ...(cargo.bench ?? []), ...(cargo.example ?? [])].map((target) => target.path) : []), ...(cargo.package?.build === false ? [] : [typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"])].filter(Boolean).map((path) => resolve(directory, path)).filter(existsSync);
  if (cargo.package?.build !== false && existsSync(resolve(directory, typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"))) return unresolved("build-script");
  for (const source of ["src/bin", ...(includeTests ? ["tests", "examples", "benches"] : [])]) {
    if (!existsSync(join(directory, source))) continue;
    for (const entry of readdirSync(join(directory, source), { withFileTypes: true })) {
      const path = join(directory, source, entry.name, ...(entry.isDirectory() ? ["main.rs"] : []));
      if (path.endsWith(".rs") && existsSync(path)) entries.push(path);
    }
  }
  if (!entries.length) return unresolved("no-entries");
  try {
    const read = rustSourceFiles(entries, directory, RUST_SOURCE_CACHE, facts, compiled), place = (file) => `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`;
    const emitters = read.filter((file) => file.endsWith(".rs") && existsSync(file) && cachedRustReferences(file, RUST_SOURCE_CACHE, facts).emitted > 0).map(place);
    return { version: 1, root, resolved: true, files: read.map(place), ...(emitters.length ? { emitters } : {}) };
  }
  catch (error) { const message = String(error?.message ?? error); return unresolved(RUST_REFUSALS.find(([prefix]) => message.startsWith(prefix))?.[1] ?? "unparsable", message.split(workspaceRoot + sep).join("")); }
}

/** 🧭️ Cargo owns compilation while the build graph tracks mounted sources and embedded assets; a crate without a static reading has no source list and keeps its directory and owner tree. */
export function cargoSourceInputs(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const set = cargoSourceSetV1(root, workspaceRoot, facts, includeTests, compiled);
  return set.resolved ? set.files : undefined;
}

/** 🛑 The source list an input precision rule may derive a key from: a crate that has no compiled entry point contributes none, a crate whose mounts cannot be read statically refuses, because its directory alone misses the files a `#[path]` mount reaches outside of it (a stale cache hit after an edit of a mounted owner file).
 * @see ./🧬️schema/🔣️.json */
export function cargoSourcesOrRefuseV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const set = cargoSourceSetV1(root, workspaceRoot, facts, includeTests, compiled);
  if (set.resolved) return set.files;
  if (set.reason === "no-entries" || set.reason === "no-manifest") return undefined;
  throw Object.assign(new Error(`Rust sources of ${root} cannot be read statically (${set.reason}${set.message ? ": " + set.message : ""}); a key derived from its directory would miss the files its mounts reach outside of it`), { code: "BUILD_RUST_SOURCES_UNRESOLVED", set });
}
'''
edit(NAT, old_block, new_block, "cargoSourceSetV1 fact + cargoSourceInputs wrapper + refusing reader")

edit(INT, "let sourceAnalysisOwnerIdentityV1,", "let cargoSourceSetV1, cargoSourcesOrRefuseV1, sourceAnalysisOwnerIdentityV1,", "internals let")
edit(INT, "rustSourceFiles, cargoSourceInputs } = registry.nativeManifests);", "rustSourceFiles, cargoSourceInputs, cargoSourceSetV1, cargoSourcesOrRefuseV1 } = registry.nativeManifests);", "internals destructure")
edit(INT, "cacheInternals = { sourceAnalysisOwnerIdentityV1,", "cacheInternals = { cargoSourceSetV1, cargoSourcesOrRefuseV1, sourceAnalysisOwnerIdentityV1,", "internals export")

edit(ASM, "PROJECT_BASENAME, cargoSourceInputs, createScriptInputCache", "PROJECT_BASENAME, cargoSourceInputs, cargoSourcesOrRefuseV1, createScriptInputCache", "assembly let")
edit(ASM, "({ nativeDependencyRoots, readToml, cargoSourceInputs, goManifest } = registry.nativeManifests);", "({ nativeDependencyRoots, readToml, cargoSourceInputs, cargoSourcesOrRefuseV1, goManifest } = registry.nativeManifests);", "assembly destructure")
edit(ASM, "    cargoSources: (root, named) => cargoSourceInputs(root, workspaceRoot, facts, named, !named),", "    cargoSources: (root, named) => cargoSourcesOrRefuseV1(root, workspaceRoot, facts, named, !named),", "precision host refuses unresolved crates")

files = {}
for e in edits:
    if e["file"] not in files: files[e["file"]] = hashlib.sha256(read(e["file"]).encode()).hexdigest()
json.dump({"version": 1, "base": files, "edits": edits}, open(HERE + "/edits.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(edits), "edits,", len(files), "files")

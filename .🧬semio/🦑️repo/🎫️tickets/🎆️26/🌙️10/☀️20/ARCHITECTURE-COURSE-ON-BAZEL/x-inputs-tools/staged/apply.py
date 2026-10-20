#!/usr/bin/env python3
"""Idempotent, anchored apply of the x-inputs staging: exact course inputs, declared module globs and declared TypeScript projects in the command source closure.

usage: apply.py [--root <workspace>] [--dry]
Every write replaces the file atomically (temp file + rename), so a file never keeps a second link and an interrupted run leaves no half edit."""
import json, os, re, sys, tempfile

here = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
root = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio"
dry = "--dry" in args

SITE = "🎓️teaching/🏛️architecture/📚️course"
MANIFEST = f"{SITE}/📦️packages/🟦️typescript/📋️project.json"
INFERENCE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"
SCRIPT_INPUTS = f"{INFERENCE}/🧭️script-inputs/🟨️.mjs"
URLS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs"
README = "🧰️framework/🛍️products/🦑️repo/README.md"
SCHEMA = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"
def staged_tests():
    base = os.path.join(here, "inference-tests")
    for directory, _, names in os.walk(base):
        for name in sorted(names):
            path = os.path.join(directory, name)
            yield path, f"{INFERENCE}/🧪️tests/{os.path.relpath(path, base)}"


def read(rel):
    with open(os.path.join(root, rel), encoding="utf8") as handle:
        return handle.read()

def write(rel, text):
    if dry:
        print("would write", rel)
        return
    path = os.path.join(root, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path))
    with os.fdopen(handle, "w", encoding="utf8") as out:
        out.write(text)
    os.chmod(temporary, 0o644)
    os.replace(temporary, path)
    print("wrote", rel)

def once(text, anchor, replacement, marker):
    if marker in text:
        return text
    if text.count(anchor) != 1:
        raise SystemExit(f"anchor not unique ({text.count(anchor)}): {anchor[:80]!r}")
    return text.replace(anchor, replacement)

def block_span(text, name):
    start = text.index(f'    "{name}": {{')
    match = re.compile(r'\n    "[^"\n]+": \{').search(text, start + 10)
    return start, (match.start() if match else len(text))

def edit_block(text, name, edit):
    start, end = block_span(text, name)
    return text[:start] + edit(text[start:end]) + text[end:]

def manifest(text):
    roots = json.load(open(os.path.join(here, "rust-roots.json"), encoding="utf8"))["roots"]
    def lines(items, indent="      "):
        return ",\n".join(f'{indent}{json.dumps(item, ensure_ascii=False)}' for item in items)
    manifests = ["{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/🎓️teaching/Cargo.toml", "{workspaceRoot}/🎓️teaching/Cargo.lock", "{workspaceRoot}/🧰️framework/Cargo.toml", "{workspaceRoot}/🧰️framework/Cargo.lock", "{workspaceRoot}/🎓️teaching/🛂️proctor/**/Cargo.toml"] + [f"{{workspaceRoot}}/{r}/**/Cargo.toml" for r in roots]
    sources = [f"{{workspaceRoot}}/{r}/**/*.rs" for r in roots]
    named = (
        '    ],\n'
        '    "workspaceManifests": [\n' + lines(["{workspaceRoot}/bun.lock", "{workspaceRoot}/🎓️teaching/bun.lock", "{workspaceRoot}/🎓️teaching/package.json", "{workspaceRoot}/🧰️framework/**/package.json", "{workspaceRoot}/🧰️framework/**/tsconfig.json"]) + '\n    ],\n'
        '    "proctorTestReads": [\n' + lines([f"{{workspaceRoot}}/🎓️teaching/🛂️proctor/{p}" for p in ["🧪️tests/🧫️fixtures/**/*", "🔨️modules/📚️course/🧰️fixtures/**/*", "🧪️tests/🌐️end-to-end/🧰️harness/**/*", "🔨️modules/🗄️storage/🦀️.rs"]]) + '\n    ],\n'
        '    "proctorCargoManifests": [\n' + lines(manifests) + '\n    ],\n'
        '    "proctorRustSources": [\n' + lines(sources) + '\n    ]\n  },\n  "targets": {'
    )
    text = once(text, '\n    ]\n  },\n  "targets": {', '\n' + named, '"workspaceManifests"')
    def test(block):
        if '"workspaceManifests"' in block:
            return block
        new = ('"inputs": [\n        "default",\n        "workspaceManifests",\n        "proctorTestReads",\n        "proctorCargoManifests",\n'
               '        { "env": "SEMIO_TEST_LEVEL" },\n        { "env": "SEMIO_TEST_BUDGET_MS" },\n        { "runtime": "node --version" },\n        { "dependentTasksOutputFiles": "**/*", "transitive": false }\n      ]')
        out, n = re.subn(r'"inputs": \[\s*"default"\s*\]', new.replace("\\", "\\\\"), block, count=1)
        if n != 1:
            raise SystemExit("test inputs anchor missing")
        return out
    def closure_metadata(block, declaration):
        if '"closure"' in block:
            return block
        if '"metadata"' in block:
            raise SystemExit("target already has metadata; add closure by hand")
        out, n = re.subn(r'(\n      "cache": true,)', '\n      "metadata": { "semio": { "closure": ' + declaration.replace("\\", "\\\\") + ' } },' + r'\1', block, count=1)
        if n != 1:
            raise SystemExit("closure metadata anchor missing")
        return out
    def check(block):
        if '"proctorRustSources"' in block:
            return block
        out, n = re.subn(r'("\{workspaceRoot\}/🎓️teaching/🛂️proctor/\*\*/\*",)', r'\1\n        "proctorCargoManifests",\n        "proctorRustSources",', block, count=1)
        if n != 1:
            raise SystemExit("check inputs anchor missing")
        return out
    def test_depends(block):
        if '"dependsOn"' in block:
            return block
        start, end = block_span(text, "typecheck")
        found = re.search(r'\n      "dependsOn": \[.*?\n      \],', text[start:end], re.S)
        if not found:
            raise SystemExit("typecheck dependsOn anchor missing")
        out, n = re.subn(r'("executor": "repo:script",)', lambda m: m.group(1) + found.group(0), block, count=1)
        if n != 1:
            raise SystemExit("test executor anchor missing")
        return out
    text = edit_block(text, "test", test_depends)
    text = edit_block(text, "test", test)
    text = edit_block(text, "test", lambda block: closure_metadata(block, '{ "modules": ["{workspaceRoot}/' + SITE + '/🧪️tests/**/🟦️.ts"] }'))
    text = edit_block(text, "typecheck", lambda block: closure_metadata(block, '{ "typescript": ["{projectRoot}/tsconfig.json"] }'))
    text = edit_block(text, "check", check)
    if True:
        line = '      "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/**/*",\n'
        if line in text:
            text = text.replace(line, "", 1)
    return text

def inference(text):
    pure = open(os.path.join(here, "declared-inputs.block.mjs"), encoding="utf8").read()
    glue = open(os.path.join(here, "declared-inputs.glue.mjs"), encoding="utf8").read()
    text = once(text, "/** 🏭️ The generated-output targets that the modules of one closure import", pure + glue + "/** 🏭️ The generated-output targets that the modules of one closure import", "declaredModuleGlobsV1")
    text = once(text, "scripts.compiledNames.get(base)))entries.add(declared);", "scripts.compiledNames.get(base)))entries.add(declared);\n for(const declared of declaredModuleGlobEntriesV1(target.metadata?.semio?.closure?.modules,workspaceRoot,projectRoot,scripts))entries.add(declared);", "declaredModuleGlobEntriesV1(target.metadata")
    text = once(text, "const closure=entries.size===baseSize?base:", "let closure=entries.size===baseSize?base:", "let closure=entries.size===baseSize")
    ret = "return {closure,fallback,...(generatedDependencies.length?{generatedDependencies}:{})};"
    glue_ret = ("const project=closure?declaredProjectClosureV1(target.metadata?.semio?.closure?.typescript,workspaceRoot,projectRoot,scripts,closure):undefined;\n"
                " if(project){closure=project.closure;generatedDependencies=scripts.generatedImports.size?generatedTargetsV1(scripts,closure,workspaceRoot):[];}\n"
                " return {closure,fallback,...(generatedDependencies.length?{generatedDependencies}:{}),...(project?.packages.length?{packages:project.packages,projectConfig:project.config}:{})};")
    text = once(text, ret, glue_ret, "declaredProjectClosureV1(target.metadata")
    text = once(text, "const key=memoDigestV1(closure,sources.fallback?fallback:null);", "const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[]);", "sources.packages??[]);")
    anchor = "  const inputs = [...new Set([...closure,...(sources.fallback?"
    extra = ("  for (const name of sources.packages ?? []) {\n"
             "    if (!graph) { packages.add(name); continue; }\n"
             "    const key = graph.resolveImport(sources.projectConfig.slice(\"{workspaceRoot}/\".length), name);\n"
             "    if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);\n"
             "  }\n")
    text = once(text, anchor, extra + anchor, "sources.projectConfig.slice")
    return text

def urls(text):
    old = 'compiler.isPropertyAccessExpression(node.arguments[1]) && node.arguments[1].name.text === "url" && compiler.isMetaProperty(node.arguments[1].expression) && node.arguments[1].expression.keywordToken === compiler.SyntaxKind.ImportKeyword) urls.add(node.arguments[0].text);'
    new = 'mentionsImportMetaUrl(node.arguments[1])) urls.add(node.arguments[0].text);\n    else if (compiler.isCallExpression(node) && isJoin(node.expression) && node.arguments.length >= 2 && isModuleDirectory(node.arguments[0]) && node.arguments.slice(1).every(compiler.isStringLiteralLike)) urls.add(`./${node.arguments.slice(1).map(argument => argument.text).join("/")}`);'
    text = once(text, old, new, "mentionsImportMetaUrl(node.arguments[1])")
    helper = 'const compiler = require("typescript"), urls = new Set(), visit = node => {'
    helpers = ('const compiler = require("typescript"), urls = new Set(), isImportMetaUrl = node => compiler.isPropertyAccessExpression(node) && node.name.text === "url" && compiler.isMetaProperty(node.expression) && node.expression.keywordToken === compiler.SyntaxKind.ImportKeyword, '
               'mentionsImportMetaUrl = node => isImportMetaUrl(node) || compiler.forEachChild(node, child => mentionsImportMetaUrl(child) || undefined) === true, '
               'calleeName = node => compiler.isIdentifier(node) ? node.text : compiler.isPropertyAccessExpression(node) ? node.name.text : "", '
               'isJoin = node => ["join", "resolve"].includes(calleeName(node)), '
               'isModuleDirectory = node => compiler.isPropertyAccessExpression(node) && compiler.isMetaProperty(node.expression) && node.expression.keywordToken === compiler.SyntaxKind.ImportKeyword && ["dir", "dirname"].includes(node.name.text) || compiler.isCallExpression(node) && calleeName(node.expression) === "dirname" && node.arguments.length === 1 && compiler.isCallExpression(node.arguments[0]) && calleeName(node.arguments[0].expression) === "fileURLToPath" && node.arguments[0].arguments.length === 1 && isImportMetaUrl(node.arguments[0].arguments[0]), '
               'visit = node => {')
    text = once(text, helper, helpers, "isModuleDirectory = node")
    text = once(text, 'if (!/new\\s+URL\\s*\\(/u.test(source) || !source.includes("import.meta.url") ||', 'if (!/new\\s+URL\\s*\\(|\\b(?:join|resolve)\\s*\\(/u.test(source) || !/import\\.meta\\.(?:url|dir|dirname)\\b/u.test(source) ||', "(?:join|resolve)")
    return once(text, 'JSON.stringify([path, source, "urls"])', 'JSON.stringify([path, source, "urls-nested"])', '"urls-nested"')

def readme(text):
    old = "Deliberately not covered: in-source test imports guarded by `import.meta.vitest`, files read through `import.meta.dir` or joined string paths, and targets that are never cached (continuous, live)."
    new = ("A target opts into more derived closure with `metadata.semio.closure`: `modules` lists `{projectRoot}`/`{workspaceRoot}` globs of module files (a test directory the router never imports) whose matches join as entries together with their import closure, and `typescript` lists tsconfig paths whose compiler program (include set, path mappings, `.d.ts` siblings, type-only imports, `extends` chain) joins the closure and names the installed type packages it resolved. "
           "References such as `new URL(\"x.json\", origin(import.meta.url))` and `join`/`resolve(<module directory>, \"x.json\")` with literal segments join as files. "
           "Deliberately not covered: in-source test imports guarded by `import.meta.vitest`, files read through computed paths, and targets that are never cached (continuous, live). The course keeps this honest with `🏗️build/🕸️graph/📥️inference/🧪️tests/🎓️course-inputs` (esbuild, the TypeScript program and an independent Python implementation against the manifest).")
    return once(text, old, new, "metadata.semio.closure")

def schema(text):
    anchor = '                "sourceInputObservation": {'
    closure = (
        '                "closure": {\n'
        '                  "type": "object",\n'
        '                  "additionalProperties": false,\n'
        '                  "properties": {\n'
        '                    "modules": { "type": "array", "uniqueItems": true, "items": { "type": "string", "pattern": "^!?\\\\{(?:projectRoot|workspaceRoot)\\\\}/" } },\n'
        '                    "typescript": { "type": "array", "uniqueItems": true, "items": { "type": "string", "pattern": "^\\\\{(?:projectRoot|workspaceRoot)\\\\}/(?:[^*?{}\\\\[\\\\]]*/)?tsconfig(?:\\\\.[^/*?{}\\\\[\\\\]]+)?\\\\.json$" } }\n'
        '                  }\n'
        '                },\n')
    return once(text, anchor, closure + anchor, '"closure": {')

for source, target_rel in [(MANIFEST, manifest), (SCRIPT_INPUTS, inference), (SCHEMA, schema), (URLS, urls), (README, readme)]:
    before = read(source)
    after = target_rel(before)
    if after != before:
        write(source, after)
    else:
        print("unchanged", source)

for staged, destination in staged_tests():
    text = open(staged, encoding="utf8").read()
    path = os.path.join(root, destination)
    if os.path.exists(path) and open(path, encoding="utf8").read() == text:
        print("unchanged", destination)
    else:
        write(destination, text)

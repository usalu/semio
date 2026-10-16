"""✏️ Anchored text edits of live shared files (scripts, project file, ownership law); every edit asserts its anchor count."""
import json, re
from rlib import *
import rcrates as C

def q(names, quote='"'):
    return "[" + ",".join(quote + n + quote for n in names) + "]"

def once(text, old, new):
    assert text.count(old) == 1, (old[:90], text.count(old))
    return text.replace(old, new)

def pkg_manifest_expr(key):
    rel = C.pkgdir(key)[len(R) - 1:] if False else None
    return 'resolve(this.root, "' + os.path.relpath(REPO + C.pkgdir(key) + "/Cargo.toml", REPO + FACADE_PKG) + '")'

import os

def facade_script(text):
    names = [C.cname(k) for k in sorted(C.KEYS, key=lambda k: (C.level(k), k))] + ["semio-framework-replication"]
    anchor = "class TestScript extends BundleScript {"
    const = ("/** 📡️ The replication packages bottom up: leaf contracts, wire core, format and mutation, edit and causal, history and frames, the message ledgers, then the façade every consumer links. */\n"
             "const REPLICATION_PACKAGES = " + q(names) + ";\n\n")
    text = once(text, anchor, const + anchor)
    text = once(text, 'await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:rest }',
                'await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:REPLICATION_PACKAGES,cwd:this.root,extraArgs:rest }')
    text = once(text, 'packages:["semio-framework-replication"],cwd:this.root,extraArgs:["--lib","local_interaction_",...rest]',
                'packages:["' + C.cname("local-interaction") + '"],cwd:this.root,extraArgs:["--lib","local_interaction_",...rest]')
    text = once(text, 'packages:["semio-framework-replication"],cwd:this.root,extraArgs:["--lib","--no-fail-fast"]',
                'packages:["' + C.cname("command-ingress") + '"],cwd:this.root,extraArgs:["--lib","--no-fail-fast"]')
    fmt, pres = C.cname("format"), C.cname("presence")
    fm = os.path.relpath(REPO + C.pkgdir("format") + "/Cargo.toml", REPO + FACADE_PKG)
    pm = os.path.relpath(REPO + C.pkgdir("presence") + "/Cargo.toml", REPO + FACADE_PKG)
    old = 'manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") },cwd: this.repoRoot,groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `format::retained::tests::${law}`) }]'
    new = 'manifestPaths: { "' + fmt + '": resolve(this.root, "' + fm + '") },cwd: this.repoRoot,groups: [{ package: "' + fmt + '", target: { kind: "lib", name: "' + C.ident("format") + '" }, laws: laws.map(law => `component::retained::tests::${law}`) }]'
    text = once(text, old, new)
    old = 'manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") },cwd: this.repoRoot,groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: [`format::retained::record::tests::${law}`] }]'
    new = 'manifestPaths: { "' + fmt + '": resolve(this.root, "' + fm + '") },cwd: this.repoRoot,groups: [{ package: "' + fmt + '", target: { kind: "lib", name: "' + C.ident("format") + '" }, laws: [`component::retained::record::tests::${law}`] }]'
    text = once(text, old, new)
    old = 'manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") },cwd: this.repoRoot,groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `wire::frames::presence_codec_tests::${law}`) }]'
    new = 'manifestPaths: { "' + pres + '": resolve(this.root, "' + pm + '") },cwd: this.repoRoot,groups: [{ package: "' + pres + '", target: { kind: "lib", name: "' + C.ident("presence") + '" }, laws: laws.map(law => `presence_codec_tests::${law}`) }]'
    text = once(text, old, new)
    text = once(text, 'const source = readFileSync(join(this.root, "../../📡️wire/🦀️.rs"), "utf8");', 'const source = readFileSync(join(this.root, "../../📡️wire/👥️presence/🦀️.rs"), "utf8");')
    text = once(text, ".register(\"presence-peer-codec-check\", PresencePeerCodecScript);",
                ".register(\"presence-peer-codec-check\", PresencePeerCodecScript).register(\"test-crate-partition\", CratePartitionLawScript);")
    law = ('/** 🏛️ Runs the language-neutral crate-partition law of the replication crates against its independent TOML and graph oracles. */\n'
           'class CratePartitionLawScript extends BundleScript {\n'
           '  async run(segments: string[]): Promise<void> {\n'
           '    if (segments.length) throw new Error("test-crate-partition accepts no arguments");\n'
           '    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🏛️crate-partition/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });\n'
           '  }\n'
           '}\n\n')
    text = once(text, "const router = new ScriptRouter(import.meta.dir)", law + "const router = new ScriptRouter(import.meta.dir)")
    return text

def facade_project(text):
    d = json.loads(text)
    t = d["targets"]
    if "test-crate-partition" in t: return text
    new = {}
    for k, v in t.items():
        if k == "test-original-conflict-source":
            new["test-crate-partition"] = {"executor": "repo:script", "cache": False,
                "options": {"cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test-crate-partition", "forwardAllArgs": True},
                "inputs": ["default", "^default", "{workspaceRoot}/Cargo.toml", "{workspaceRoot}/🧰️framework/Cargo.toml",
                           "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🧪️tests/🏛️crate-partition/🟦️.ts",
                           "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json",
                           "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🧬️schema/🏛️crate-partition/🔣️.json"],
                "outputs": []}
        new[k] = v
    d["targets"] = new
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"

IDS_TREES = ["HybridLogicalTimestamp", "MutationId", "ActorId", "ArtifactId", "SchemaId", "PayloadHash"]
FRAG = "🚪️io/📝️text/🧵️canonical/"

def ownership_fixture(text):
    d = json.loads(text)
    rows = []
    for t in d["canonical"]["borrowedTrees"]:
        if t in IDS_TREES: o = FRAG + "🆔️ids/🦀️.rs"
        elif t == "ForeignTarget": o = FRAG + "🌐️foreign/🦀️.rs"
        elif t == "UndoPolicy": o = FRAG + "🧾️policy/🦀️.rs"
        else: o = d["canonical"]["textOwner"]
        rows.append({"type": t, "owner": o})
    d["canonical"]["borrowedTrees"] = rows
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"

def ownership_schema(text):
    d = json.loads(text)
    props = d["properties"]["canonical"]["properties"]
    props["borrowedTrees"]["items"] = {"type": "object", "required": ["type", "owner"], "additionalProperties": False,
                                       "properties": {"type": {"type": "string", "minLength": 1}, "owner": {"type": "string", "minLength": 1}}}
    props["borrowedTrees"]["uniqueItems"] = True
    return json.dumps(d, ensure_ascii=False, indent=1) + "\n"

def ownership_ts(text):
    old = 'for (const type of canonical.borrowedTrees as string[]) assert.match(canonicalText, new RegExp(`ArtifactCanonicalJsonTree for ${type}\\\\b`));'
    new = ('for (const row of canonical.borrowedTrees as {type: string; owner: string}[]) {\n'
           '  assert.equal(minimatch(row.owner, "🚪️io/📝️text/**/*.rs"), true);\n'
           '  assert.match(readFileSync(resolve(owner, row.owner), "utf8"), new RegExp(`ArtifactCanonicalJsonTree for ${row.type}\\\\b`));\n}')
    text = once(text, old, new)
    text = once(text, 'const canonicalText = readFileSync(resolve(owner, canonical.textOwner), "utf8");\n', "")
    text = once(text, "assert.match(consumer, /crate::io::binary::causal::(?:encode|decode)_/);", "assert.match(consumer, /(?:crate::io::binary::causal|semio_framework_replication_causal::binary)::(?:encode|decode)_/);")
    pattern_old = "new RegExp(`impl crate::value::${trait} for ${type}\\\\b`)"
    assert text.count(pattern_old) == 2
    text = text.replace(pattern_old, "new RegExp(`impl (?:crate::value|semio_framework_value)::${trait} for ${type}\\\\b`)")
    return text

SOURCE_FIXTURE_OLD = '"🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml"'

def source_fixture(text):
    new = '"' + C.pkgdir("format") + '/Cargo.toml"'
    return once(text, SOURCE_FIXTURE_OLD, new)

def ingress_test(text):
    a = text.index('    const mounts = nodes(entryAst).filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "command_ingress");')
    marker = '    expect(facades[0]!.childForFieldName("argument")!.childForFieldName("list")!.namedChildren.map(node => node.text)).toEqual([...fixture.exports.types, ...fixture.exports.constants]);\n'
    b = text.index(marker) + len(marker)
    new = '''    const wires = nodes(entryAst).filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "wire");
    expect(wires).toHaveLength(1);
    executable(wires[0]!);
    expect(wires[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    const mounts = nodes(wires[0]!).filter(node => node.type === "mod_item" && node.childForFieldName("name")?.text === "command_ingress");
    expect(mounts).toHaveLength(1);
    executable(mounts[0]!);
    expect(mounts[0]!.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
    expect(attributes(mounts[0]!)).toEqual([]);
    const facades = nodes(entryAst).filter(node => node.type === "use_declaration" && tokens(node.childForFieldName("argument")!.childForFieldName("path") ?? node) === "semio_framework_replication_command_ingress");
    expect(facades).toHaveLength(2);
    expect(facades.filter(node => { for (let parent = node.parent; parent; parent = parent.parent) if (parent.id === mounts[0]!.id) return true; return false; })).toHaveLength(1);
    for (const facade of facades) {
      executable(facade);
      expect(facade.namedChildren.some(node => node.type === "visibility_modifier" && node.text === "pub")).toBe(true);
      expect(attributes(facade)).toEqual([]);
      expect(facade.childForFieldName("argument")!.childForFieldName("list")!.namedChildren.map(node => node.text).sort()).toEqual([...fixture.exports.types, ...fixture.exports.constants].sort());
    }
'''
    text = text[:a] + new + text[b:]
    text = once(text, '    expect(manifest(fixture.paths.manifest).lib.name).toBe("protocol");\n',
                '    expect(manifest(fixture.paths.manifest).package.name).toBe("semio-framework-replication-command-ingress");\n'
                '    expect(resolve(repository, dirname(fixture.paths.manifest), manifest(fixture.paths.manifest).lib.path)).toBe(resolve(repository, owner, "🦀️.rs"));\n'
                '    expect(manifest(fixture.paths.facadeManifest).dependencies["semio-framework-replication-command-ingress"]).toBeDefined();\n')
    text = once(text, "paths: { entry: string; manifest: string; neutralTests: string };", "paths: { entry: string; manifest: string; facadeManifest: string; neutralTests: string };")
    text = once(text, """expect(script).toContain('packages:["semio-framework-replication"]');""", """expect(script).toContain('packages:["semio-framework-replication-command-ingress"]');""")
    text = once(text, 'replace("../../📡️wire/🎮️command/📥️ingress/🦀️.rs", "../../../../🛍️products/💻️os/🦀️.rs")', 'replace("semio_framework_replication_command_ingress", "semio_framework_os_hostile")')
    text = once(text, 'replace("pub mod command_ingress;", "#[cfg(any())]\\npub mod command_ingress;")', 'replace("pub mod command_ingress {", "#[cfg(any())]\\npub mod command_ingress {")')
    text = once(text, 'replace("pub use wire::command_ingress::{", "#[cfg(any())]\\npub use wire::command_ingress::{")', 'replace("pub use semio_framework_replication_command_ingress::{", "#[cfg(any())]\\npub use semio_framework_replication_command_ingress::{")')
    return text

def ingress_fixture(text, closure=None):
    d = json.loads(text)
    d["paths"] = {"entry": d["paths"]["entry"], "manifest": C.pkgdir("command-ingress") + "/Cargo.toml", "facadeManifest": d["paths"]["manifest"], "neutralTests": d["paths"]["neutralTests"]}
    if closure is not None: d["closure"] = closure
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"

TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
NEW_MODULE_MEMBERS = ["✏️edit", "🕹️interaction", "👥️presence"]

def taxonomy_edit(text):
    """Admits the three new module member folders (depth three below `🔨️modules`) into the registered member names."""
    head = '    "members-of-members-of-members-of-modules": {'
    a = text.index(head)
    m = re.compile(r'"memberNames": \[\n(.*?)\n      \]', re.S).search(text, a)
    body = m.group(1)
    existing = re.findall(r'"([^"]+)"', body)
    add = [n for n in NEW_MODULE_MEMBERS if n not in existing]
    if not add: return text
    new_body = body + "".join(',\n        "' + n + '"' for n in add)
    out = text[:m.start(1)] + new_body + text[m.end(1):]
    json.loads(out)
    return out

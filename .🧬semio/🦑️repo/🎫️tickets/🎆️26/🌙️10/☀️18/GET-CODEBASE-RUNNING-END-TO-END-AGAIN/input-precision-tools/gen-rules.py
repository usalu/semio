#!/usr/bin/env python3
# 🧬 Generates staged/rules.json: the input-precision derivation data (profiles, owner routes, ordered rules) from the live manifests and the hand-authored rule structure below.
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
W = "{workspaceRoot}/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SCRIPT = "./📜️script.ts"
VERIFY_SOURCE = LIB + "/🎛️commands/✅️verify/🟦️.ts"
REPLACES = ["default", "^default", "nativeSources", "^nativeSources", "nativeTestSources", "^nativeTestSources", "production", "^production"]

def routes():
    files = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "-z", "--", "*project.json"], capture_output=True, cwd=ROOT).stdout.decode().split("\0")
    rows = []
    for f in files:
        if not f.endswith("project.json") or f.startswith(".🧬semio/"): continue
        try: p = json.load(open(os.path.join(ROOT, f)))
        except Exception: continue
        for target, t in (p.get("targets") or {}).items():
            words = (((t.get("metadata") or {}).get("semio") or {}).get("workspaceCommand"))
            if words: rows.append({"words": words, "project": p["name"], "target": target, "root": os.path.dirname(f)})
    return sorted(rows, key=lambda r: (r["words"], r["project"]))

def kinds(ids, under=("."), named=False):
    r = {"kinds": ids, "under": list(under)}
    if named: r["named"] = True
    return r

PROFILES = {
    "package-discovery": {
        "description": "Every file the owner-route dispatcher reads when it scans the repository for packages: package manifests, direct package sources and the emoji-named entry files of each package, as observed by tracing a verify command and checked by the containment lane.",
        "reads": [{"globs": [
            W + "**/📦️packages/*/{Cargo.toml,package.json,📋️project.json,📜️script.ts,pyproject.toml,go.mod}",
            W + "**/📦️packages/*/*.{ts,tsx,mts,cts,js,mjs,cjs,rs,go,py,cs}",
            W + "**/📦️packages/*/*/*.{rs,py}",
            W + "**/📦️packages/**/{🟦️,🟨️,🦀️,🐹️,🐍️,🔷️}.{ts,tsx,mts,cts,d.ts,d.mts,d.cts,js,mjs,cjs,rs,go,py,cs}",
            W + "**/📦️packages/**/📜️script.ts",
            W + "**/📦️packages/🟦️typescript/app/**/*.{ts,tsx}",
            W + "**/📦️packages/**/🧰️vitest.setup.ts",
            W + "📋️project.json", W + "package.json", W + "✏️s/🧑‍💻dev/📋️project.json"]}],
    },
    "library-resources": {
        "description": "Data files the repository library reads by computed path: its schemas and asset policies, and the taxonomy.",
        "reads": [{"globs": [
            W + LIB + "/🔣️taxonomy.json",
            W + LIB + "/**/🧬️schema/**/🔣️.json",
            W + LIB + "/🖼️assets/**/🔣️.json",
            W + "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/**/🔣️.json"]}],
    },
    "repository-sources": {
        "description": "The sources a repository-wide scanner reads: every file of the source kinds, the schema JSON and the package manifests; not images, documents, fixture data or build output.",
        "reads": [
            kinds(["rust-source", "typescript-source", "javascript-source", "go-source", "python-source", "dotnet-source", "toml", "gherkin-feature"]),
            {"globs": [W + "**/🧬️schema/**/🔣️.json", W + "**/📋️project.json", W + "**/package.json"]}],
    },
    "manifests": {
        "description": "Every package manifest of the repository.",
        "reads": [{"globs": [W + "**/{Cargo.toml,package.json,📋️project.json,pyproject.toml,go.mod}"]}],
    },
    "test-discovery": {
        "description": "What test discovery reads: the taxonomy, feature files and the canonical emoji-named source of every language (the set the test domain's own discovery targets declare).",
        "reads": [kinds(["rust-source", "typescript-source", "go-source", "python-source", "dotnet-source"], named=True), kinds(["gherkin-feature"])],
    },
    "stdio-tree": {
        "description": "The stdio plugin tree the ledger gates walk (STDIO_ROOT_REL) and the hub composition that consumes it.",
        "reads": [{"globs": [W + "✏️s/🔌️plugins/🗄️stdio/**/*", W + "🌎️hub/🧩️compositions/🗄️stdio/**/*"]}],
    },
}

MESSAGES = {
    "languages": ["en", "de"],
    "texts": {
        "deriving": {"en": "Deriving the TypeScript program {tsconfig}: files without a valid receipt entry are analysed", "de": "Leite das TypeScript-Programm {tsconfig} ab: Dateien ohne gültigen Belegeintrag werden analysiert"},
        "progress": {"en": "{tsconfig}: {files} files visited, {derived} analysed, {reused} reused ({seconds} s)", "de": "{tsconfig}: {files} Dateien besucht, {derived} analysiert, {reused} wiederverwendet ({seconds} s)"},
        "done": {"en": "{tsconfig}: program complete, {files} files, {derived} analysed, {reused} reused ({seconds} s)", "de": "{tsconfig}: Programm vollständig, {files} Dateien, {derived} analysiert, {reused} wiederverwendet ({seconds} s)"},
    },
}

MANIFEST_SCANNERS = ["package-purity", "build-owners", "zero-touch"]
SCANNERS = ["taxonomy", "generated-corruption", "typescript", "rust-warnings", "production-placeholders", "docstrings", "agnostic", "mutation-caps", "debug-tags", "interface-owners", "interactivity", "dependencies", "layering", "canonical-architecture", "history-closure", "composed-child-refs", "semantic-vocabulary", "mutation-outcome-law", "abstraction-ownership", "policy-breach", "gate", "artifact-field-parity"]

def head(*words): return [SCRIPT, *words]

def artifact_owners():
    fixture = json.load(open(os.path.join(ROOT, LIB, "📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json")))
    return sorted({p.split("/🏅️standards/")[0] + "/📦️packages/🦀️rust" for p in fixture["artifactSchemas"]})

def commands(*rows): return [head(*row) for row in rows]

def build():
    rs = routes()
    root_routes = [r for r in rs if r["words"][0] == "verify"]
    S = "workspace"
    base = [{"profile": "library-resources"}]
    rules = [
        {"id": "workspace-owner-delegates", "evidence": "static", "description": "A root verify command that forwards to an owner target reads what the dispatcher scan reads and what that owner reads: its project, its crate closure and its script closure.",
         "select": {"project": S, "commands": [head(*r["words"]) for r in root_routes]}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"owner": "route"}]},
        {"id": "workspace-verify-manifest-scanners", "evidence": "traced", "description": "The package-purity, zero-touch and build-owners gates read the dispatcher scan and every package manifest (traced: 9.4k, 10.7k and 12.8k reads, all inside this set).",
         "select": {"project": S, "commands": [head("verify", w) for w in MANIFEST_SCANNERS]}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"profile": "manifests"}]},
        {"id": "workspace-artifact-contract-ownership", "evidence": "static", "description": "The artifact contract ownership law tests the owner crates of every artifact schema its fixture lists (generated from the fixture, checked by the rules test).",
         "select": {"project": S, "commands": commands(["verify", "artifact-contract-ownership"])}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"globs": [W + LIB + "/📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json", W + "**/🏅️standards/**/🧬️schema/**/🔣️.json"]}, {"cargo": {"directories": artifact_owners()}}, {"dispatch": {"source": VERIFY_SOURCE, "after": ["verify"]}}]},
        {"id": "workspace-verify-scanners", "evidence": "bound", "description": "A verify gate that scans the repository reads the sources of every source kind.",
         "select": {"project": S, "targets": ["verify", "verify-gate"], "commands": [head("verify", w) for w in SCANNERS]}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"profile": "repository-sources"}]},
        {"id": "workspace-verify-laws", "evidence": "static", "description": "A verify law reads the dispatcher scan, the TypeScript entries and Cargo crates its dispatcher branch names and the closure of those.",
         "select": {"project": S, "commands": commands(["verify"])}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"dispatch": {"source": VERIFY_SOURCE, "after": ["verify"]}}]},
        {"id": "workspace-lint-repo", "evidence": "static", "description": "`lint repo` only reports completion of the lint graph that its dependsOn already ran.",
         "select": {"project": S, "commands": commands(["lint", "repo"])}, "replaces": REPLACES, "reads": []},
        {"id": "workspace-lint", "evidence": "static", "description": "`lint` runs dependency-cruiser over the four source roots with the lint boundary configuration.",
         "select": {"project": S, "commands": commands(["lint"])}, "replaces": REPLACES,
         "reads": [{"kinds": ["typescript-source", "javascript-source"], "under": ["🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"]}, {"globs": [W + LIB.replace("📚️library", "🧹️lint") + "/**/*", W + "tsconfig.json"]}]},
        {"id": "workspace-schema", "evidence": "bound", "description": "The schema family reads the schema directories its sibling generate, docs and check targets already declare (schemaSources) and tests the schema crate.",
         "select": {"project": S, "commands": commands(["schema"])}, "replaces": REPLACES,
         "reads": [*base, {"input": "schemaSources"}, {"cargo": {"packages": ["semio-framework-schema"]}}]},
        {"id": "workspace-test-discovery", "evidence": "static", "description": "Test discovery reads the taxonomy, feature files and the canonical emoji-named sources, as the test domain's own discovery targets declare.",
         "select": {"project": S, "commands": commands(*[["test", w] for w in ["discover", "contract", "oracle", "subject", "parity"]])}, "replaces": REPLACES,
         "reads": [*base, {"profile": "test-discovery"}]},
        {"id": "workspace-test-canvas", "evidence": "static", "description": "The canvas test family runs the canvas module's native crate or React renderer tests.",
         "select": {"project": S, "commands": commands(["test", "canvas"])}, "replaces": REPLACES,
         "reads": [{"globs": [W + "🧰️framework/🔨️modules/🖼️canvas/**/*"]}, {"cargo": {"directories": ["🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust"]}}]},
        {"id": "workspace-test-build-library", "evidence": "bound", "description": "The build library's own tests read the build library.",
         "select": {"project": S, "targets": ["test-workspace-build", "test-runtime-build-producer"]}, "replaces": REPLACES,
         "reads": [{"globs": [W + LIB + "/🏗️build/**/*", W + "📋️project.json"]}]},
        {"id": "workspace-stdio-gates", "evidence": "static", "description": "The stdio structural gates walk the stdio plugin tree (STDIO_ROOT_REL) through the artifact ledger.",
         "select": {"project": S, "commands": commands(*[["stdio", w] for w in ["quick", "schema-parity", "standards-coverage", "codec", "mutation-law", "inference", "runtime", "cross-platform"]])}, "replaces": REPLACES,
         "reads": [*base, {"profile": "stdio-tree"}]},
        {"id": "workspace-generate", "evidence": "bound", "description": "The generate config check reads the repository's tool configuration manifests.",
         "select": {"project": S, "commands": commands(["generate", "config"])}, "replaces": REPLACES,
         "reads": [*base, {"profile": "manifests"}, {"globs": [W + ".github/**/*", W + "go.work", W + "go.work.sum", W + "**/go.mod", W + "**/pyproject.toml", W + "**/*.csproj"]}]},
        {"id": "workspace-test-entrypoint", "evidence": "static", "description": "The entrypoint test runs one test file of the root command startup; it reads that test and its import closure, the library resources and the command's own script closure.",
         "select": {"project": S, "commands": commands(["test", "entrypoint"])}, "replaces": REPLACES,
         "reads": [*base, {"script": {"source": "🧪️tests/🧭️entrypoint/🟦️.ts"}}]},
        {"id": "workspace-census", "evidence": "whole", "description": "The semantic census reads every component of the repository by design: the census lists and compares all of them, so every file of every kind is an input.",
         "select": {"project": S, "commands": commands(["generate", "taxonomy"])}, "replaces": [],
         "reads": []},
        {"id": "workspace-reentrant", "evidence": "assumed", "description": "These root commands re-enter the build tool (`bun repo run` or `run-many`) and their result is a function of the inputs of the targets they launch, which no static read of this command bounds; the owner decision is an uncached aggregate or explicit dependsOn.",
         "select": {"project": S, "targets": ["scale-fixture-check", "stdio-long", "stdio-exhaustive"]}, "replaces": [],
         "reads": [{"profile": "repository-sources"}]},
        {"id": "workspace-cpp", "evidence": "assumed", "description": "The C++ custody and test-toolchain commands drive the CMake build library; their reads of the C++ trees and toolchain files are not yet traced, so they keep the inherited inputs.",
         "select": {"project": S, "targets": ["cpp-custody", "cpp-test-toolchain"]}, "replaces": [],
         "reads": [{"profile": "repository-sources"}]},
        {"id": "workspace-storybook", "evidence": "assumed", "description": "The Storybook browser test reads every story and the components it imports; the story set is not yet derived from the Storybook configuration, so the inherited inputs stay.",
         "select": {"project": S, "targets": ["build-storybook", "test-storybook"]}, "replaces": [],
         "reads": [{"profile": "repository-sources"}]},
        {"id": "workspace-repository-wide", "evidence": "bound", "description": "Remaining root commands that scan or aggregate the repository read the source kinds, not the whole tree.",
         "select": {"project": S, "commands": commands(["test"], ["examples"])}, "replaces": REPLACES,
         "reads": [{"profile": "package-discovery"}, *base, {"profile": "repository-sources"}]},
        {"id": "os-hub-typecheck", "evidence": "static", "description": "The hub typecheck reads exactly its tsconfig program: the include globs, the imports they reach and the package manifests resolution consults.",
         "select": {"project": "os-hub-ts", "targets": ["typecheck"]}, "replaces": ["default", "^default", W + "🌎️hub/**/*.{ts,tsx,json}", W + "🧰️framework/**/*.{ts,tsx,json}", W + "✏️s/**/*.{ts,tsx,json}"],
         "reads": [{"program": {"tsconfig": "🌎️hub/📦️packages/🟦️typescript/tsconfig.json"}}]},
        {"id": "s-dev-typecheck", "evidence": "static", "description": "The plugin layer typecheck reads exactly the program of ✏️s/tsconfig.json.",
         "select": {"project": "@semio-tech/s-dev-composition", "targets": ["typecheck"]}, "replaces": ["default", "^default", W + "✏️s/**/*.{ts,tsx,json}", W + "🧰️framework/**/*.{ts,tsx,json}"],
         "reads": [{"program": {"tsconfig": "✏️s/tsconfig.json"}}]},
        {"id": "s-dev-catalog-generate", "evidence": "bound", "description": "The catalog publication reads plugin manifests, TypeScript and TOML sources of the plugin layer and the registry projection.",
         "select": {"project": "@semio-tech/s-dev-composition", "targets": ["generate"]}, "replaces": ["default", "^default"],
         "reads": [kinds(["typescript-source", "toml"], under=["✏️s"]), kinds(["json"], under=["✏️s"], named=True), {"globs": [W + "✏️s/**/{📋️project.json,package.json}", W + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*.{ts,tsx,json,semio,wit,sql}"]}]},
        {"id": "stdio-plugin-native-check", "evidence": "static", "description": "The stdio native checks compile the stdio crate and its path dependencies, which is what ^default stands for here.",
         "select": {"project": "@semio-tech/stdio-plugin", "targets": ["home-io-surface-native-check"]}, "replaces": ["^default"],
         "reads": [{"cargo": {"directories": ["🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust"]}}]},
        {"id": "plugin-composition-test", "evidence": "assumed", "description": "The cross-plugin composition laws read their own directory and the closure of the tests they run.",
         "select": {"project": "@semio-tech/s-plugin-composition", "targets": ["test"]}, "replaces": ["default", "^default", W + "✏️s/🔌️plugins/**/*", W + "🧰️framework/**/*"],
         "reads": [{"script": {"source": "✏️s/🧪️tests/🏘️plugin-composition/📜️script.ts"}}]},
        {"id": "repo-lib-fixture-placement", "evidence": "whole", "description": "The fixture placement laws walk all nine physical scopes of the repository (every top-level source tree) and compare every fixture path, so the whole tree is an input by design.",
         "select": {"project": "@semio-tech/repo-lib", "targets": ["test-fixture-placement", "test-mutation-fixture-placement"]}, "replaces": [],
         "reads": []},
        {"id": "hub-stdio-oracles", "evidence": "assumed", "description": "The stdio hub oracles read the hub stdio composition and the stdio plugin tree, and for the native cohorts the stdio crate closure.",
         "select": {"project": "@semio-tech/hub-stdio-test-oracle", "targets": ["test-drawing-reader", "test-composition", "test-private-reader", "test-type-ownership", "test-production-preparation", "test-native-oracles", "test-native-drawing-reader"]}, "replaces": ["stdioSources", "rustWorkspace"],
         "reads": [{"profile": "stdio-tree"}, {"globs": [W + "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/**/*"]}, {"cargo": {"directories": ["🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust"]}}]},
    ]
    return {"$schema": "./🧬️schema/🔣️.json", "version": 1, "retained": ["sharedGlobals", "{projectRoot}/📋️project.json"], "messages": MESSAGES,
            "exclusions": [x.replace("{projectRoot}", "{workspaceRoot}") for x in ["!{projectRoot}/**/node_modules/**", "!{projectRoot}/**/.git/**", "!{projectRoot}/**/.nx/**", "!{projectRoot}/**/.cache/**", "!{projectRoot}/**/target/**", "!{projectRoot}/**/dist/**", "!{projectRoot}/**/📤️dist/**", "!{projectRoot}/**/build/**", "!{projectRoot}/**/out/**", "!{projectRoot}/**/coverage/**", "!{projectRoot}/**/test-results/**", "!{projectRoot}/**/playwright-report/**", "!{projectRoot}/**/storybook-static/**", "!{projectRoot}/**/.venv/**", "!{projectRoot}/**/__pycache__/**", "!{projectRoot}/**/bin/**", "!{projectRoot}/**/obj/**", "!{projectRoot}/**/⚡️cache/**", "!{projectRoot}/**/🗑️generated/**", "!{projectRoot}/**/.lake/**", "!{projectRoot}/**/pkg/**"]],
            "profiles": PROFILES, "routes": rs, "rules": rules}

if __name__ == "__main__":
    out = sys.argv[1] if len(sys.argv) > 1 else HERE + "/staged2/rules.json"
    json.dump(build(), open(out, "w"), ensure_ascii=False, indent=1)
    open(out, "a").write("\n")
    print("rules written", out)

"""Group extension (decision D1): the VS Code extension leaves this repository and its dependents are cut with it.
Opt-in only (apply.py never takes it by default): result-ide-free.md lists the tests to run right after the apply."""
import json, os, re, subprocess
from blib import *

g = "extension"
EXT = M + "/💻️client/🧩️vscode"
TOK = re.compile(r"🧩️vscode|repo-vscode|vscode-extension|vscode-test|vscode-package-ignore|vscodeignore|@vscode/|usalu\.repo-vscode")
REMOVED = []


def mentions(x):
    return isinstance(x, str) and bool(TOK.search(x))


def clean(node, where=""):
    if isinstance(node, list):
        out = []
        for i, x in enumerate(node):
            if isinstance(x, str):
                if mentions(x): REMOVED.append(where + f"[{i}] " + x[-70:]); continue
            elif isinstance(x, dict) and any(mentions(v) for v in x.values()):
                REMOVED.append(where + f"[{i}] <object>"); continue
            else:
                x = clean(x, where + f"[{i}]")
            out.append(x)
        return out
    if isinstance(node, dict):
        out = {}
        for k, v in node.items():
            if mentions(k) or mentions(v): REMOVED.append(where + "/" + k); continue
            out[k] = clean(v, where + "/" + k)
        return out
    return node


def JCLEAN(path, tag):
    def fn(doc):
        new = clean(doc)
        doc.clear(); doc.update(new)
    JEDIT(g, path, fn, tag)


# ---- 1. the module (AGENTS.md files are not edited by agents: module AGENTS.md is left for its owner)
tracked = subprocess.run(["git", "-C", ROOT, "ls-files", EXT], capture_output=True, text=True).stdout.split("\n")
dirs = set()
for f in sorted(x for x in tracked if x and not x.endswith("/AGENTS.md")):
    DELETE(g, f, "extension-file")
    d = os.path.dirname(f)
    while d.startswith(EXT):
        dirs.add(d); d = os.path.dirname(d)
for d in sorted(dirs, key=lambda s: -s.count("/")): RMDIR(g, d)

# ---- 2. root registries
JCLEAN("📋️build-owners.json", "extension-build-owners")


def deps_clean(doc):
    def walk(node):
        if isinstance(node, list):
            return [walk(x) for x in node if not (isinstance(x, str) and x.startswith(EXT + "/"))]
        if isinstance(node, dict):
            return {k: walk(v) for k, v in node.items()}
        return node
    new = walk(doc)

    def prune(node):
        if isinstance(node, dict):
            for k, v in list(node.items()):
                if isinstance(v, list) and v and all(isinstance(x, dict) and "users" in x for x in v):
                    node[k] = [x for x in v if x["users"]]
                else:
                    prune(v)
    prune(new)
    doc.clear(); doc.update(new)


JEDIT(g, "🔒️dependencies.json", deps_clean, "extension-dependencies-lock")

# ---- 3. library registries, policies, fixtures, tests
JCLEAN(L + "/🖼️assets/🏠️owner-projection-policy/🔣️.json", "extension-owner-projection-policy")
JCLEAN(L + "/🧪️tests/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json", "extension-readme-license-fixture")
JCLEAN(L + "/🧪️tests/🧫️fixtures/🎚️tool-configuration-ownership/🔣️.json", "extension-tool-configuration-fixture")
JCLEAN(L + "/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json", "extension-repo-source-fixture")
def capacity(doc):
    rows = [r for r in doc["census"]["rows"] if mentions(r["path"])]
    new = clean(doc)
    new["census"]["files"] -= len(rows)
    new["census"]["literalCalls"] -= sum(r["literalCalls"] for r in rows)
    doc.clear(); doc.update(new)


JEDIT(g, "🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🧫️fixtures/🔣️.json", capacity, "extension-capacity-fixture")

TT = L + "/🧪️tests/🎚️tool-configuration-ownership/🟦️.ts"
ED(g, TT, '"dependency-cruiser" | "vscode-test";\n  previousPath', '"dependency-cruiser";\n  previousPath')
ED(g, TT, '"eslint" | "dependency-cruiser" | "vscode-test";\n  mode:', '"eslint" | "dependency-cruiser";\n  mode:')
txt = read(TT)
i = txt.index('\n    const vscodeOwner = ownerById.get("vscode-extension-test")!;')
j = txt.index("\n  }, 60_000);", i)
ED(g, TT, txt[i:j], "")
ED(g, L + "/🔍️discovery/🏷️package-source-classification/🟦️.ts", '  if (validator === "vscode-test-configuration") return configuration(/from\\s+["\']@vscode\\/test-cli["\']/u);\n', "")
for f in ("/🔍️discovery/📜️taxonomy-contracts/🟦️.ts", "/🧹️normalization/🔣️taxonomy/🏭️generators-packages/🟦️.ts"):
    ED(g, L + f, ' | "eslint-configuration" | "vscode-test-configuration";', ' | "eslint-configuration";')
for f in ("/🔍️discovery/🚛️audit-package-boundary-contracts/🟦️.ts", "/🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts"):
    ED(g, L + f, ', "vscode-test-configuration": "vscode-test-cli-config" };', ' };')
ED(g, L + "/🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts", ', "eslint-configuration", "vscode-test-configuration"] as const)', ', "eslint-configuration"] as const)')

ED(g, L + "/🎛️commands/🗂️registry/🔣️.json", "build [canvas fonts|storybook|assets|repo-cli|repo-server|repo-vscode]", "build [canvas fonts|storybook|assets|repo-cli|repo-server]")
for f in ("/🏗️build/📜️script.ts", "/🏗️build/.profiled-script.ts", "/🏗️build/🧭️invocation/🟦️.ts"):
    ED(g, L + f, ', "repo-vscode": "@semio-tech/repo-vscode:build-vsix" };', " };")
ED(g, L + "/📦️packages/🟦️typescript/📋️project.json", '        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/{🧪️tests/🎚️config/🟨️.mjs,📦️packages/🟦️typescript/{📋️project.json,📜️script.ts}}",\n', "")

CC = L + "/⚡️caching/🧪️tests/⚡️cache-contracts/🧾️command-inputs/🟦️.ts"
ED(g, CC, '  const { testExtensionHostBuild } = await import("repo-vscode/testing/host-build");\n  await testExtensionHostBuild(output);\n  const { testExtensionPackage } = await import("repo-vscode/testing/package");\n  await testExtensionPackage(output);\n', "")
WB = L + "/⚡️caching/🧪️tests/🏗️web-build/🟦️.ts"
ED(g, WB, 'import { extensionBuildConfig } from "repo-vscode/builder";\n', "")
ED(g, WB, ', extension = extensionBuildConfig(workspace, "entry.ts", "out", "extension.cjs", row.mode), coordinator =', ', coordinator =')
ED(g, WB, '    assert.equal(extension.build.minify, row.minify); assert.equal(extension.build.sourcemap, row.sourcemap);\n', "")
ED(g, WB, ' assert.equal(extension.build.rollupOptions.treeshake, row.mode === "release");', "")
txt = read(WB)
i = txt.index('  for (const row of fixture.modes) {\n    const extension = extensionBuildConfig(')
j = txt.index("\n  }\n", i) + len("\n  }\n")
ED(g, WB, txt[i:j], "")
txt = read(WB)
i = txt.index("    const watchedSource = join(temporary")
j = txt.index("    const project = join(app,", i)
ED(g, WB, txt[i:j], "")
def cases(doc):
    doc.pop("extensionSource"); doc.pop("watch")
JEDIT(g, L + "/⚡️caching/🧪️tests/🏗️web-build/🧫️cases.json", cases, "extension-web-build-cases")

AT = M + "/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts"
txt = read(AT)
i = txt.index('\ntest("the editor extension exposes source navigation without developer operations"')
j = txt.index("\n});\n", i) + len("\n});\n")
ED(g, AT, txt[i:j], "")

ED(g, M + "/README.md".replace("/README.md", "") if False else "🧰️framework/🛍️products/🦑️repo/README.md", "- [`💻️client/🧩️vscode`](🔨️modules/💻️client/🧩️vscode) – Passive downstream integration artifact; the dashboard exclusively owns human source editing and developer operations\n", "")
ED(g, M + "/💻️client/🔌️mcp/README.md", "implements the executor and the CLI imports it, `🧩️vscode` writes documents against it, and neither may add a field that this", "implements the executor and the CLI imports it, and neither may add a field that this")
print("[extension] structural JSON removals:", len(REMOVED))
open(os.path.join(HERE, "staged", "extension-removals.txt"), "w", encoding="utf-8").write("\n".join(REMOVED) + "\n")
ED(g, ".gitignore", "#--------------------------------------VSCODE--------------------------------------\n**.vscode-test\n*.vsix\n# *.code-workspace\n", "", (1,), "extension-gitignore")

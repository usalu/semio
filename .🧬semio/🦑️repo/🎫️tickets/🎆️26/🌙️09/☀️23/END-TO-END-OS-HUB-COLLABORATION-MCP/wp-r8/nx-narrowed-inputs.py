#!/usr/bin/env python3
"""R8 session 12 prepared patch set S12-7 (lands after W2's `--packages all` publish, on the coordinator's word).

Root cause (measured with Nx's own HashPlanInspector, `nx-fileset-semantics-1.json`): a named input whose workspace-rooted
negations reach their positives only through a REFERENCE (`production = ["default", "!{workspaceRoot}/…"]`) is planned as
"every workspace file except …" — a lone `!{workspaceRoot}/…` fileset expands to the whole repository (92 807 files), while the
same negation beside its positives subtracts (31 → 24). Every `production`/`^production` consumer (all materialize-*) hashed
~73 k files incl. `♻️mit-bestand` research data, so any edit anywhere missed every plugin (audit S12 R3: 0–3 % hits).

Changes:
  1. `nx.json` `namedInputs.production`: inline `default`'s entries; drop the lone `!{workspaceRoot}/**/🧫️fixtures/**/*`
     (the project-rooted fixture/test exclusions stay).
  2. `📚️library/🟨️.mjs` `projectInputs`: `production` inlines the project's `default` entries instead of referencing it.
  3. `📚️library/🟨️.mjs`: a component project's `describe` hashes its native closure (`nativeSources` + Cargo-closure
     `nativeSources` + its command sources) like `component-*`, not `default`/`^default` (owner trees incl. fixtures).
  4. `🕸️graph` project: the generator's cross-repo inputs (`✏️s/🔌️plugins/**/*manifest.json`, repo-lib discovery/taxonomy)
     move from `default` (inherited by every dependent through `^default`/`^production`) to `generatorSources`, used only by
     its `generate` / `preview-generated` targets.

Proof by planning (in memory on the real graph, `nx-narrowed-inputs-simulation.ts`): baseline vs narrowed — see 📓️wp-r8.md.
Usage: python3 nx-narrowed-inputs.py [--apply]
"""
import json, sys

ROOT = "/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
ok = True
out = {}

def edit(path, pairs):
    global ok
    text = out.get(path) or open(ROOT + path, encoding="utf8").read()
    for old, new in pairs:
        count = text.count(old)
        print(f"{'OK ' if count == 1 else 'BAD'} {count}x {path.rsplit('/', 1)[-1]}: {old.strip()[:90]}")
        ok &= count == 1
        text = text.replace(old, new)
    out[path] = text

edit("nx.json", [(
    '"production": ["default", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{workspaceRoot}/**/🧫️fixtures/**/*", ',
    '"production": ["{projectRoot}/**/*", "sharedGlobals", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", ',
), (
    '"default": ["{projectRoot}/**/*", "sharedGlobals"],',
    '"default": ["{projectRoot}/**/*", "sharedGlobals"],',
)])

MJS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"
edit(MJS, [
    (
        '  const production = ["default", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{workspaceRoot}/**/🧫️fixtures/**/*", "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];',
        '  const production = ["!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{workspaceRoot}/**/🧫️fixtures/**/*", "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];',
    ),
    (
        "default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...production, ...(declarations.production ?? [])],",
        "default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...inputs, ...(declarations.default ?? []), ...exclusions, ...production, ...(declarations.production ?? [])],",
    ),
    (
        '    const nativeTarget = nativeProject && /^(build|wasm|native|test(?:-(?:quick|long|exhaustive))?$|lint|check$)/.test(name) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts");',
        '    const nativeTarget = nativeProject && (/^(build|wasm|native|test(?:-(?:quick|long|exhaustive))?$|lint|check$)/.test(name) || name === "describe" && Boolean(declared["component-dev"])) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts");',
    ),
    (
        "    if (!/^(?:build|check|lint|test|wasm|native|component|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;",
        "    if (!/^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;",
    ),
])

GRAPH = "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📋️project.json"
graph = json.loads(open(ROOT + GRAPH, encoding="utf8").read())
default = graph["namedInputs"].get("default", [])
foreign = [entry for entry in default if entry.startswith("{workspaceRoot}/") and not entry.startswith("{workspaceRoot}/🧰️framework/🔨️modules/🕸️graph/")]
print(f"{'OK ' if len(foreign) == 5 else 'BAD'} graph foreign default entries {len(foreign)}")
ok &= len(foreign) == 5
graph["namedInputs"] = {"generatorSources": default, "default": [entry for entry in default if entry not in foreign]}
for name in ("generate", "preview-generated"):
    print(f"{'OK ' if name in graph['targets'] else 'BAD'} graph target {name}")
    ok &= name in graph["targets"]
    graph["targets"][name]["inputs"] = ["generatorSources"]
out[GRAPH] = json.dumps(graph, ensure_ascii=False, indent=2) + "\n"

if not ok:
    sys.exit("dry run failed: an anchor moved; re-derive the hunk")
if apply:
    for path, text in out.items():
        open(ROOT + path, "w", encoding="utf8").write(text)
    print("applied — next: re-run nx-narrowed-inputs-simulation.ts WITHOUT the in-memory edits (they are now the source) and compare, then the cache-input laws")
else:
    print("dry run clean")

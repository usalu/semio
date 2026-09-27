#!/usr/bin/env python3
"""🧊️ Z4 prepared set for window 3 — the fresh-clone blockers B1–B3 (Z2 run 2, re-derived 2026-09-27 on a fresh-clone view):

B1  `external-emoji-shortcodes` is an external input declared `inclusion: ignored` inside `🤖️generated`: no clone has it,
    so `assets:build` (`renderShortcodes`) and os-infinite's `build.rs` (on the hub's path) die. The pinned snapshot
    becomes a tracked input beside the icon catalog: `🔣️icons/🔣️shortcodes.json`.
B2  `wgpu-frame-worker` declares its bundle `🎞️frame-worker/🤖️generated/🟨️.js` tracked, but `.gitignore` ignores every
    `🤖️generated` tree and the indexed-generated-output law forbids indexing it: `validateGeneratorContractsAgainstWorkspace`
    (every `loadTaxonomy()` caller) refuses every clone. Declared `ignored` like its `browser-boot` sibling (taxonomy,
    browser-profile parser, both fixtures, package-integration oracle); the wgpu `serve`/`dev` targets gain their producer.
B3  `scale-fixture` declares `⚖️scale/🤖️generated` tracked (same contradiction). Declared `ignored`; the consumers that read
    it (plugin-host `include_str!` test and its native marshalling check, renderer `native-scale*`) gain the producer edge.
B5  `flow-browser-package` declares `🌊️flow/🫀️core/🕸️bindings` tracked, but `**/🕸️bindings/**` is ignored and the
    indexed-generated-output law flags its one force-indexed file (`package.json`, which the `flow-core-package` bootstrap
    source publishes on every clone anyway). Declared `ignored`; the stale index entry needs `git rm --cached` by someone
    allowed to run git.
LAW Two taxonomy laws in `validateTaxonomy` keep it from recurring: no tracked output inside `🤖️generated`; an external
    input is tracked.  Law test in `🔬️workspace-contract` with git's own ignore engine as the oracle.

usage: b123-fresh-clone.py <repo root> [--apply]   (dry run by default; idempotent: an applied hunk reports `applied`)
"""
import os, sys

ROOT = os.path.abspath(sys.argv[1])
APPLY = "--apply" in sys.argv[2:]
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
DISCOVERY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"
OLD_SNAPSHOT = "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔣️shortcodes.json"
NEW_SNAPSHOT = "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔣️shortcodes.json"
WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
FRAME_OUT = f"{WGPU}/🎞️frame-worker/🤖️generated/🟨️.js"
SCALE_OUT = "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🤖️generated"
ACTIVATE = '        "@semio-tech/framework-os-dev:activate-s-wgpu-dev"\n      ],'
SCALE_WASM = '"@semio-tech/framework-os-scale-fixture:build-wasm"'
SCALE_GEN = '"@semio-tech/framework-os-dev:generate-scale-fixture"'

LAW_ANCHOR = '          if (!["tracked", "ignored"].includes(output.inclusion)) problems.push(`${key}.inclusion must be tracked or ignored.`);\n'
LAW_ADDED = LAW_ANCHOR + (
    '          if (output.inclusion === "tracked" && typeof output.path === "string" && output.path.split("/").some((part) => part.replace("\\uFE0F", "") === "🤖generated")) problems.push(`${key} declares a tracked output inside 🤖️generated, which .gitignore ignores and the indexed-generated-output law never indexes: no clone has it.`);\n'
    '          if (contract.ownership === "external" && output.inclusion !== "tracked") problems.push(`${key} is an external input that is not tracked: nothing produces it, so no clone has it.`);\n'
)

TEST_ANCHOR = 'describe("materialized JCO interface filename boundaries", () => {\n'
TEST_ADDED = '''describe("fresh-clone generator outputs", () => {
  test("every declared-tracked output is visible to a clone and every external input is tracked", () => {
    const taxonomy = loadCatalogTaxonomy();
    expect(validateTaxonomy(taxonomy).filter((problem) => problem.includes("no clone has it"))).toEqual([]);
    const outputs = Object.values(taxonomy.generatorContracts).flatMap((contract) => contract.outputRoots.map((output) => ({ ownership: contract.ownership, ...output })));
    const tracked = outputs.filter((output) => output.inclusion === "tracked").map((output) => output.path);
    expect(outputs.filter((output) => output.ownership === "external" && output.inclusion !== "tracked")).toEqual([]);
    const probes = tracked.map((path) => (existsSync(join(getWorkspaceRoot(), path)) && lstatSync(join(getWorkspaceRoot(), path)).isDirectory() ? `${path}/probe` : path));
    const oracle = Bun.spawnSync(["git", "check-ignore", "--no-index", "--stdin"], { cwd: getWorkspaceRoot(), stdin: new TextEncoder().encode(`${probes.join("\\n")}\\n`), stdout: "pipe", stderr: "pipe" });
    expect(oracle.stdout.toString().split("\\n").filter(Boolean)).toEqual([]);
    const violated = probeClone(taxonomy) as unknown as Taxonomy;
    const scale = violated.generatorContracts["scale-fixture"]!.outputRoots[0] as { inclusion: string };
    scale.inclusion = "tracked";
    const shortcodes = violated.generatorContracts["external-emoji-shortcodes"]!.outputRoots[0] as { inclusion: string };
    shortcodes.inclusion = "ignored";
    const problems = validateTaxonomy(violated).filter((problem) => problem.includes("no clone has it"));
    expect(problems.some((problem) => problem.includes("scale-fixture") && problem.includes("inside 🤖️generated"))).toBe(true);
    expect(problems.some((problem) => problem.includes("external-emoji-shortcodes") && problem.includes("external input"))).toBe(true);
  }, 60_000);
});

''' + TEST_ANCHOR

HUNKS = [
    ("B1", TAXONOMY, f'        "{OLD_SNAPSHOT}"\n      ],', f'        "{NEW_SNAPSHOT}"\n      ],'),
    ("B1", TAXONOMY, f'          "path": "{OLD_SNAPSHOT}",\n          "inclusion": "ignored"', f'          "path": "{NEW_SNAPSHOT}",\n          "inclusion": "tracked"'),
    ("B1", "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts", 'function renderShortcodes(icons: Record<string, string>, generatedDir: string): AssetArtifact {\n  const catalog = Object.keys(icons).sort();\n  const snapshotPath = join(generatedDir, "🔣️shortcodes.json");', 'function renderShortcodes(icons: Record<string, string>, iconsDir: string, generatedDir: string): AssetArtifact {\n  const catalog = Object.keys(icons).sort();\n  const snapshotPath = join(iconsDir, "🔣️shortcodes.json");'),
    ("B1", "🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts", 'if (target === "all") artifacts.push(renderShortcodes(icons, generatedDir));', 'if (target === "all") artifacts.push(renderShortcodes(icons, iconsDir, generatedDir));'),
    ("B1", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs", 'let shortcodes_path = ui_assets.join("🔣️icons/🤖️generated/🔣️shortcodes.json");', 'let shortcodes_path = ui_assets.join("🔣️icons/🔣️shortcodes.json");'),
    ("B1", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs", '.unwrap_or_else(|e| panic!("read {}: {e}. Run `bun nx run @semio-tech/assets:build` first.", shortcodes_path.display()));', '.unwrap_or_else(|e| panic!("read the tracked emoji shortcode snapshot {}: {e}", shortcodes_path.display()));'),
    ("B1", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json", '"{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔣️shortcodes.json"', '"{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔣️shortcodes.json"'),
    ("B2", TAXONOMY, '              "outputRelativePath": "🎞️frame-worker/🤖️generated/🟨️.js",\n              "inclusion": "tracked"', '              "outputRelativePath": "🎞️frame-worker/🤖️generated/🟨️.js",\n              "inclusion": "ignored"'),
    ("B2", TAXONOMY, f'          "path": "{FRAME_OUT}",\n          "inclusion": "tracked",', f'          "path": "{FRAME_OUT}",\n          "inclusion": "ignored",'),
    ("B2", DISCOVERY, 'const expected = [{ id: "frame-worker", inclusion: "tracked" }, { id: "browser-boot", inclusion: "ignored" }];', 'const expected = [{ id: "frame-worker", inclusion: "ignored" }, { id: "browser-boot", inclusion: "ignored" }];'),
    ("B2", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧊️wgpu-browser-entry-authority/🔣️.json", '"outputRelativePath": "🎞️frame-worker/🤖️generated/🟨️.js", "inclusion": "tracked" }', '"outputRelativePath": "🎞️frame-worker/🤖️generated/🟨️.js", "inclusion": "ignored" }'),
    ("B2", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts", 'entry.inclusion === (index === 0 ? "tracked" : "ignored");', 'entry.inclusion === "ignored";'),
    ("B2", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-artifact-dependency-source/🔣️.json", f'      "path": "{FRAME_OUT}",\n      "inclusion": "tracked",', f'      "path": "{FRAME_OUT}",\n      "inclusion": "ignored",'),
    ("B2", f"{WGPU}/📦️packages/🟦️typescript/📋️project.json", f'    "serve": {{\n      "executor": "nx:run-commands",\n      "cache": false,\n      "continuous": true,\n      "outputs": [],\n      "dependsOn": [\n{ACTIVATE}', f'    "serve": {{\n      "executor": "nx:run-commands",\n      "cache": false,\n      "continuous": true,\n      "outputs": [],\n      "dependsOn": [\n        "generate-frame-worker",\n{ACTIVATE}'),
    ("B2", f"{WGPU}/📦️packages/🟦️typescript/📋️project.json", f'    "dev": {{\n      "executor": "nx:run-commands",\n      "cache": false,\n      "continuous": true,\n      "outputs": [],\n      "dependsOn": [\n{ACTIVATE}', f'    "dev": {{\n      "executor": "nx:run-commands",\n      "cache": false,\n      "continuous": true,\n      "outputs": [],\n      "dependsOn": [\n        "generate-frame-worker",\n{ACTIVATE}'),
    ("B3", TAXONOMY, f'          "path": "{SCALE_OUT}",\n          "inclusion": "tracked"\n        }}\n      ],\n      "reason": "The deterministic seeded fixture target writes two tracked JSON outputs and has a byte freshness check"', f'          "path": "{SCALE_OUT}",\n          "inclusion": "ignored"\n        }}\n      ],\n      "reason": "The deterministic seeded fixture target writes two ignored JSON outputs that every reader regenerates through its Nx edge; a byte freshness check guards the renderer"'),
    ("B3", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📋️project.json", f'        "command": "bun 📜️script.ts test",\n        "cwd": "{{projectRoot}}",\n        "forwardAllArgs": true\n      }},\n      "dependsOn": [\n        {SCALE_WASM}\n      ]', f'        "command": "bun 📜️script.ts test",\n        "cwd": "{{projectRoot}}",\n        "forwardAllArgs": true\n      }},\n      "dependsOn": [\n        {SCALE_WASM},\n        {SCALE_GEN}\n      ]'),
    ("B3", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📋️project.json", f'        "command": "bun 📜️script.ts ui-patch-marshalling-check --native",\n        "cwd": "{{projectRoot}}"\n      }},\n      "dependsOn": [\n        {SCALE_WASM}\n      ]', f'        "command": "bun 📜️script.ts ui-patch-marshalling-check --native",\n        "cwd": "{{projectRoot}}"\n      }},\n      "dependsOn": [\n        {SCALE_WASM},\n        {SCALE_GEN}\n      ]'),
    ("B3", f"{WGPU}/📦️packages/🟦️typescript/📋️project.json", f'        "native-build",\n        {SCALE_WASM}\n      ]', f'        "native-build",\n        {SCALE_WASM},\n        {SCALE_GEN}\n      ]'),
    ("B3", f"{WGPU}/📦️packages/🟦️typescript/📋️project.json", f'        "native-build-release",\n        {SCALE_WASM}\n      ]', f'        "native-build-release",\n        {SCALE_WASM},\n        {SCALE_GEN}\n      ]'),
    ("B5", TAXONOMY, '          "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings",\n          "inclusion": "tracked"', '          "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings",\n          "inclusion": "ignored"'),
    ("LAW", DISCOVERY, LAW_ANCHOR, LAW_ADDED),
    ("LAW", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts", TEST_ANCHOR, TEST_ADDED),
]

texts, status, failed = {}, [], False
for label, rel, old, new in HUNKS:
    path = os.path.join(ROOT, rel)
    if rel not in texts: texts[rel] = open(path, encoding="utf-8").read()
    text = texts[rel]
    if text.count(new) == 1 and (old in new or old not in text): state = "applied"
    elif text.count(old) == 1 and new not in text: texts[rel] = text.replace(old, new); state = "ready"
    else: state = f"CONFLICT (old x{text.count(old)}, new x{text.count(new)})"; failed = True
    status.append((label, rel, state))
snapshot_state = "applied" if os.path.exists(os.path.join(ROOT, NEW_SNAPSHOT)) else "ready" if os.path.exists(os.path.join(ROOT, OLD_SNAPSHOT)) else "CONFLICT (no snapshot)"
failed |= snapshot_state.startswith("CONFLICT")
for label, rel, state in status: print(f"{label:4} {state:9} {rel}")
print(f"B1   {snapshot_state:9} move {OLD_SNAPSHOT} -> {NEW_SNAPSHOT}")
if failed: sys.exit("dry run found conflicts; nothing written")
if not APPLY: sys.exit(0)
for rel, text in texts.items():
    with open(os.path.join(ROOT, rel), "w", encoding="utf-8") as handle: handle.write(text)
if snapshot_state == "ready": os.replace(os.path.join(ROOT, OLD_SNAPSHOT), os.path.join(ROOT, NEW_SNAPSHOT))
print("applied")

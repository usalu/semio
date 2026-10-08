"""🌱️ Removes the authored launch seed from the Draw source scenario (slice L-S4a hand-off).

Run together with L-S2's removal of `.vscode/🧩️launch.seed.jsonc` from
`generatorContracts["plugin-registry"].inputPatterns` in `📚️library/🔣️taxonomy.json`: before that edit the scenario
needs the authored seed (the isolated generator still reads it), after it `artifactProjectionProducerInputs` throws
"Authored producer inputs must be exact declared producer inputs" until this script has run.

Usage: `python3 l-s4a-draw-seed-removal.py` (count-checked dry run) | `python3 l-s4a-draw-seed-removal.py --apply`.
"""
import json, sys

ROOT = "/Users/ueli/Documents/semio/"
LIB = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
TEST = LIB + "🧪️tests/🔬️workspace-contract/🟦️.ts"
FIXTURE = LIB + "🧫️fixtures/🖍️draw-source-scenario/🔣️.json"
EDITS = [
    (TEST, "  launchSeed: DrawSourceScenarioInput;\n", ""),
    (TEST, '''    const authored = new Map([DRAW_SOURCE_SCENARIO.launchSeed].map(({ path, content }) => [path, content]));
    for (const path of [...contract.inputPatterns, ...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(context.files[path]?.content).toBe(authored.get(path) ?? readFileSync(join(getWorkspaceRoot(), path), "utf8"));
    for (const path of authored.keys()) expect(readPaths).not.toContain(path);
''', '''    for (const path of [...contract.inputPatterns, ...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...DRAW_SOURCE_SCENARIO.producerContext.runtimeData]) expect(context.files[path]?.content).toBe(readFileSync(join(getWorkspaceRoot(), path), "utf8"));
'''),
    (TEST, '''  const authored = [DRAW_SOURCE_SCENARIO.launchSeed];
  if (authored.some(({ path }) => !contract.inputPatterns.includes(path))) throw new Error("Authored producer inputs must be exact declared producer inputs");
  const files: Record<string, ArtifactProducerInput> = Object.fromEntries(authored.map(({ path, content }) => [path, { content, mode: 0o644, sha256: createHash("sha256").update(content).digest("hex"), origin: "authored-scenario" as const }]));
''', '''  const files: Record<string, ArtifactProducerInput> = {};
'''),
]

texts = {path: open(path, encoding="utf-8").read() for path in {TEST, FIXTURE}}
for path, old, new in EDITS:
    found = texts[path].count(old)
    if found != 1:
        raise SystemExit(f"REFUSED: expected exactly one occurrence, found {found}: {old[:90]!r}")
    texts[path] = texts[path].replace(old, new)
start, end = texts[FIXTURE].index('  "launchSeed": {\n'), texts[FIXTURE].index('  "catalogContext": [')
block = texts[FIXTURE][start:end]
if block.count('"path"') != 1 or '".vscode/🧩️launch.seed.jsonc"' not in block or not block.endswith("  },\n"):
    raise SystemExit("REFUSED: unexpected launchSeed extent in the scenario fixture")
texts[FIXTURE] = texts[FIXTURE][:start] + texts[FIXTURE][end:]
json.loads(texts[FIXTURE])
if "launchSeed" in texts[TEST] or "launchSeed" in texts[FIXTURE]:
    raise SystemExit("REFUSED: launchSeed would survive the edit")
if "--apply" in sys.argv[1:]:
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied: authored launch seed removed from the Draw source scenario")
else:
    print("dry run ok: 3 test edits and 1 fixture block match exactly once; pass --apply to write")

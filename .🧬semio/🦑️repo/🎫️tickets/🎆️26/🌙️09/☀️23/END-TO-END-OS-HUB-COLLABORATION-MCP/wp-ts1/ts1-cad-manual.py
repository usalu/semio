"""🧩 TS1 one-off: finishes the cad + spatial-kernel type sweep after `ts1-cad-test-deps.ts`.
- preview owns the helpers brepjs still calls since the 09-03 preview split (runtime ReferenceErrors) → exported + imported;
- suites take `preciseSpatialKernelMath` / `aabbVolume` from their owner (preview) instead of the brepjs module that never exported them;
- remaining `type X = any` aliases resolve to the real refs; the stately spec interface is exported for its suite;
- `parseCadBounds` returns the tuple its interface declares; brepjs compares `import.meta.env.VITEST` as the string it is.
Idempotent. usage: python3 ts1-cad-manual.py [--dry-run]"""
import pathlib, sys

R = pathlib.Path("/Users/ueli/Documents/semio")
CAD = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any"
SK = "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine"
DRY = "--dry-run" in sys.argv
changed = []


def rel(from_file, to_file):
    import os
    r = os.path.relpath(R / to_file, (R / from_file).parent)
    return r if r.startswith(".") else "./" + r


def edit(path, pairs):
    p = R / path
    before = p.read_text()
    text = before
    for old, new in pairs:
        if new in text and old not in text:
            continue
        assert text.count(old) == 1, (path, old[:90], text.count(old))
        text = text.replace(old, new)
    if text != before:
        changed.append(path)
        if not DRY:
            p.write_text(text)


PREVIEW = f"{SK}/🧮️preview/🟦️.ts"
GEOMETRY = f"{SK}/📐️geometry/🟦️.ts"

edit(PREVIEW, [
    ("\nfunction faceNormalFromPoints(points: readonly Vec3[]): Vec3 | null {", "\nexport function faceNormalFromPoints(points: readonly Vec3[]): Vec3 | null {"),
    ("\nfunction derivedFacePoints(model: Model, face: FaceRecord): readonly Vec3[] {", "\nexport function derivedFacePoints(model: Model, face: FaceRecord): readonly Vec3[] {"),
    ("\nfunction aabbVolume(a: Aabb): number {", "\nexport function aabbVolume(a: Aabb): number {"),
    ("\nfunction readVec3(v: unknown): Vec3 | null {", "\nexport function readVec3(v: unknown): Vec3 | null {"),
])

edit(f"{SK}/🧱️brepjs/🟦️.ts", [
    ("  circleFromCenterRadiusPoint,\n  edgeCurveLength,", "  circleFromCenterRadiusPoint,\n  derivedFacePoints,\n  edgeCurveLength,"),
    ("  faceCentroid,\n  fuseSolidsToExternalFaces,", "  faceCentroid,\n  faceNormalFromPoints,\n  fuseSolidsToExternalFaces,"),
    ("  nurbsDisplaySamplePoints,\n  vec3Add,", "  nurbsDisplaySamplePoints,\n  readVec3,\n  vec3Add,"),
    ('const isBrepjsTestRun = import.meta.env.VITEST === true ||', 'const isBrepjsTestRun = import.meta.env.VITEST === "true" ||'),
])

INFERENCES = f"{CAD}/🧬️schema/💡️inferences/🟦️.ts"
edit(INFERENCES, [
    ('''export function parseCadBounds(value: unknown, at = "$"): CadBounds {
  const row = cadCadInferenceGuardObject(value, at);
  return {
    min: cadCadInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: cadCadInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}''', '''/** 📐️ One `[x, y, z]` corner the schema admits as exactly three finite numbers. */
const parseCadBoundsCorner = (value: unknown, at: string): [number, number, number] => {
  const [x, y, z] = cadCadInferenceGuardArray(value, at, {"minItems": 3, "maxItems": 3}).map((item, index) => cadCadInferenceGuardNumber(item, `${at}[${index}]`));
  return [x, y, z];
};

export function parseCadBounds(value: unknown, at = "$"): CadBounds {
  const row = cadCadInferenceGuardObject(value, at);
  return {
    min: parseCadBoundsCorner(row["min"], `${at}.min`),
    max: parseCadBoundsCorner(row["max"], `${at}.max`),
  };
}'''),
])

STATELY = f"{CAD}/✏️editor/⚙️engine/🎰️stately/🟦️.ts"
edit(STATELY, [("\ninterface StatelyMachineSpec extends MachineSpec {", "\nexport interface StatelyMachineSpec extends MachineSpec {")])
STATELY_TEST = f"{CAD}/✏️editor/⚙️engine/🎰️stately/🧪️tests/🧪️semio-tech-cad-js-stately/🟦️.ts"
edit(STATELY_TEST, [
    ("\n  type StatelyMachineSpec = any;", ""),
    ('import type { StatelyTestDependencies } from "../../🟦️.ts";', 'import type { StatelyMachineSpec, StatelyTestDependencies } from "../../🟦️.ts";'),
])

ACTIONS = f"{CAD}/✏️editor/⚙️engine/🎬️actions/🟦️.ts"
edit(ACTIONS, [
    ('const __actionsTestKernel = import.meta.vitest ? await import("../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts") : null;\n', ""),
    ("  readonly __actionsTestKernel: typeof __actionsTestKernel;\n", ""),
    ("{ __actionsTestKernel, __actionsTestRuntime, buildBoxInteractionSpec, resolveDisplay }", "{ __actionsTestRuntime, buildBoxInteractionSpec, resolveDisplay }"),
])

SUITES = {
    f"{CAD}/✏️editor/⚙️engine/🎬️actions/🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts": [
        ("const { __actionsTestKernel, __actionsTestRuntime, buildBoxInteractionSpec, resolveDisplay } = dependencies;", "const { __actionsTestRuntime, buildBoxInteractionSpec, resolveDisplay } = dependencies;"),
        ("  const { preciseSpatialKernelMath } = __actionsTestKernel!;\n", ""),
    ],
    f"{CAD}/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx": [
        ("  const { BrepjsKernel, preciseSpatialKernelMath: M } = __cadRendererTestKernel!;", "  const { BrepjsKernel } = __cadRendererTestKernel!;\n  const M = preciseSpatialKernelMath;"),
        ("\n  type AnchorRef = any;\n  type EdgeRef = any;\n  type ShellRef = any;\n  type SolidRef = any;\n  type VertexRef = any;\n  type WireRef = any;", ""),
        ('const faceId = "f0" as kernelGeometry.FaceRef;', 'const faceId = "f0" as FaceRef;'),
    ],
    f"{CAD}/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts": [
        ("  const { BrepjsKernel, preciseSpatialKernelMath } = __artifactTestKernel!;", "  const { BrepjsKernel } = __artifactTestKernel!;"),
    ],
    f"{CAD}/🧬️schema/💡️inferences/🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts": [
        ("  const { BrepjsKernel, preciseSpatialKernelMath } = __spatialQueryTestKernel!;", "  const { BrepjsKernel } = __spatialQueryTestKernel!;"),
        ("\n  type FaceRef = any;\n  type ShellRef = any;\n  type SolidRef = any;", ""),
    ],
    f"{SK}/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts": [
        ("  const { BrepjsKernel, preciseSpatialKernelMath } = __geometryTestKernel!;", "  const { BrepjsKernel } = __geometryTestKernel!;"),
    ],
}
for ext in ["🏛️aec-building-structure", "📐️spatial-shape", "🔥️aec-building-energy"]:
    stem = {"🏛️aec-building-structure": "aec-building-structure", "📐️spatial-shape": "spatial-shape", "🔥️aec-building-energy": "aec-building-energy"}[ext]
    path = f"✏️s/🔌️plugins/📐️cad/🧩️extensions/{ext}/🧪️tests/🧪️semio-tech-cad-js-module-{stem}/🟦️.ts"
    pairs = [
        ("  const { BrepjsKernel, preciseSpatialKernelMath } = brepjs;", "  const { BrepjsKernel } = brepjs;"),
        ("  type TypologyRef = any;", "  type TypologyRef = core.TypologyRef;"),
    ]
    if stem == "aec-building-structure":
        pairs.append(("  type Model = any;", "  type Model = core.Model;"))
    SUITES[path] = pairs

IMPORTS = {
    f"{CAD}/✏️editor/⚙️engine/🎬️actions/🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts": [("value", PREVIEW, ["preciseSpatialKernelMath"])],
    f"{CAD}/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx": [("value", PREVIEW, ["preciseSpatialKernelMath"]), ("type", GEOMETRY, ["AnchorRef", "EdgeRef", "FaceRef", "ShellRef", "SolidRef", "VertexRef", "WireRef"])],
    f"{CAD}/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🟦️.ts": [("value", PREVIEW, ["preciseSpatialKernelMath"])],
    f"{CAD}/🧬️schema/💡️inferences/🧪️tests/🧪️semio-tech-cad-js-query-parse/🟦️.ts": [("value", PREVIEW, ["preciseSpatialKernelMath"]), ("type", GEOMETRY, ["FaceRef", "ShellRef", "SolidRef"])],
    f"{SK}/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts": [("value", PREVIEW, ["preciseSpatialKernelMath"])],
    f"{SK}/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts": [("value", PREVIEW, ["aabbVolume"])],
}
for ext, stem in [("🏛️aec-building-structure", "aec-building-structure"), ("📐️spatial-shape", "spatial-shape"), ("🔥️aec-building-energy", "aec-building-energy")]:
    path = f"✏️s/🔌️plugins/📐️cad/🧩️extensions/{ext}/🧪️tests/🧪️semio-tech-cad-js-module-{stem}/🟦️.ts"
    IMPORTS[path] = [("value", PREVIEW, ["preciseSpatialKernelMath"]), ("typens", "@semio-tech/cad-js", ["core"])]

for path, pairs in SUITES.items():
    edit(path, pairs)
for path, groups in IMPORTS.items():
    lines = []
    for kind, target, names in groups:
        spec = target if target.startswith("@") else rel(path, target)
        keyword = "import" if kind == "value" else "import type"
        lines.append(f"{keyword} {{ {', '.join(names)} }} from \"{spec}\";")
    p = R / path
    text = p.read_text()
    missing = [line for line in lines if line not in text]
    if missing:
        changed.append(path + " (imports)")
        if not DRY:
            p.write_text("\n".join(missing) + "\n" + text)

print(("would change: " if DRY else "changed: ") + "\n".join(changed or ["nothing"]))

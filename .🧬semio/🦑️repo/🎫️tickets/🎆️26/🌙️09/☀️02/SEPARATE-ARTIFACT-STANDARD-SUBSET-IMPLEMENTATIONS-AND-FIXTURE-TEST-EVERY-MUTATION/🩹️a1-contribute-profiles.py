#!/usr/bin/env python3
"""🩹️ Contributes the missing comparison/tolerance profiles and the missing generator-oracle
registration for the 🧿️semio brep/drawing/mesh subsets, and repoints the fixture manifests that
were naming a profile close-but-not-equal to one that already exists.

Every new profile below has real, load-bearing values derived from this subset's own committed
fixture corpus and oracle rationale text (read by hand before writing this script) — none is a
stub. Repoints are used only where an existing CORE or subset profile already carries the exact
semantics the fixture needs.
"""
import json

REPO = "/Users/ueli/Documents/semio"
BASE = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"


def load(path):
    with open(path, encoding="utf8") as f:
        return json.load(f)


def dump(path, data):
    with open(path, "w", encoding="utf8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.write("\n")


# ---------------------------------------------------------------------------------------------
# brep: contribute the kernel-edit comparison profile and the tessellation tolerance profile;
# repoint the two fixture-manifest tolerance names that were already exact synonyms of profiles
# the framework itself owns.
# ---------------------------------------------------------------------------------------------
brep_path = f"{BASE}/✳️brep/🧪️oracle/🔣️.json"
brep = load(brep_path)

new_brep = {}
for k, v in brep.items():
    new_brep[k] = v
    if k == "mutationCatalogs":
        new_brep["comparisonProfiles"] = [
            {
                "id": "semantic-brep-kernel-edit-v1",
                "description": "⚖️ A BRep edit compared as a KERNEL-LEVEL bundle: the re-exported STEP (topology and exact analytic geometry as OCCT actually classified it, through brepjs-occt), the tessellated mesh projection (size+digest, the same treatment this fleet gives every large binary payload) and the measured metrics (volume, area, bounding box) computed independently by the cross-language semio-brep-python-independent reader. A structural-only compare of the STEP text would fail on legitimate entity-numbering and formatting freedom OCCT itself is not obliged to preserve; comparing the measured and tessellated projections instead compares what the mutation actually claimed to change.",
                "arrays": "ordered",
            }
        ]
new_brep["toleranceProfiles"] = new_brep.get("toleranceProfiles", []) + [
    {
        "id": "geometry-tessellated",
        "description": "📏️ Curve-replacement edits (arc→spline and line→arc) that legitimately re-tessellate: the new curve is geometrically different by construction, so the gate is the DEVIATION of the tessellated approximation from the analytic curve it replaces, not bit-identity with a previous tessellation. Wider than `mechanical-standard` because a spline approximating an arc carries real, expected chord error at the segment's own scale.",
        "absoluteLength": 1e-6,
        "relativeLength": 1e-5,
        "absoluteArea": 1e-6,
        "relativeArea": 1e-5,
        "absoluteVolume": 1e-6,
        "relativeVolume": 1e-5,
        "normalizedHausdorffMax": 1e-4,
        "normalizedCentroidDistanceMax": 1e-6,
        "maxOverrideFactor": 5,
    }
]
brep = new_brep
for fixture in brep["fixtureManifests"]:
    if fixture.get("toleranceProfile") == "boolean-standard":
        fixture["toleranceProfile"] = "mechanical-standard"
    elif fixture.get("toleranceProfile") == "topology-exact":
        fixture["toleranceProfile"] = "analytic-strict"
dump(brep_path, brep)
print("brep: contributed semantic-brep-kernel-edit-v1 + geometry-tessellated; repointed boolean-standard->mechanical-standard, topology-exact->analytic-strict")

# ---------------------------------------------------------------------------------------------
# drawing: contribute the XML-element-tree comparison profile the quick-xml-generated SVG corpus
# actually needs (distinct from the whole-bundle semantic-drawing-carrier-v1 profile, which
# compares SVG+DXF+PDF together rather than one parsed SVG document); repoint the tolerance name
# to the profile this subset already contributed under its real name.
# ---------------------------------------------------------------------------------------------
drawing_path = f"{BASE}/✳️drawing/🧪️oracle/🔣️.json"
drawing = load(drawing_path)
drawing["comparisonProfiles"].append(
    {
        "id": "xml-element-tree",
        "description": "⚖️ One parsed SVG document compared as an XML element tree through quick-xml — element order, tag names, attributes and text content — rather than as the whole SVG+DXF+PDF bundle `semantic-drawing-carrier-v1` covers. Element order is significant (it is SVG paint order), so arrays stay ordered; no keys are ignored because every attribute quick-xml recovers is part of the document's own contract.",
        "arrays": "ordered",
    }
)
for fixture in drawing["fixtureManifests"]:
    if fixture.get("toleranceProfile") == "exact":
        fixture["toleranceProfile"] = "drawing-exact"
dump(drawing_path, drawing)
print("drawing: contributed xml-element-tree; repointed exact->drawing-exact")

# ---------------------------------------------------------------------------------------------
# mesh: contribute the manifold-measurement comparison profile, the four tessellation-scale
# tolerance profiles the fixture corpus already groups itself into, and the paired-tool generator
# oracle (manifold-3d computes the boolean/primitive geometry, three.js exports/imports the four
# carriers) that produced every third-party-generated fixture here.
# ---------------------------------------------------------------------------------------------
mesh_path = f"{BASE}/✳️mesh/🧪️oracle/🔣️.json"
mesh = load(mesh_path)

new_mesh = {}
for k, v in mesh.items():
    new_mesh[k] = v
    if k == "oracles":
        new_mesh["comparisonProfiles"] = [
            {
                "id": "semantic-mesh-manifold-v1",
                "description": "⚖️ A mesh edit compared by MEASUREMENT rather than by triangle-for-triangle identity: relative volume error, relative area error and normalized symmetric Hausdorff distance, all computed by manifold-3d (`manifold-mesh-measure`) from the mesh three.js recovered from each of the four exported carriers (STL/OBJ/PLY/glTF). Triangle-soup identity is not the contract — a re-tessellation that changes vertex count while preserving volume/area/topology is a correct answer — so the profile measures the shape THE MUTATION claimed to produce, matching this oracle's own measured baseline: 0.000e+00 on an unchanged round-trip against 1.073e-01 / 9.988e-01 for a genuinely different solid at identical tessellation.",
                "arrays": "ordered",
            }
        ]
new_mesh["toleranceProfiles"] = [
    {
        "id": "mesh-exact",
        "description": "📏️ Analytic primitives (a cube, whose eight vertices and six faces have a closed-form volume/area) tessellated by manifold-3d with no boolean or scale operation applied — the tightest mesh floor this subset uses, admitting only IEEE-754 and triangulation-seam noise.",
        "absoluteLength": 1e-9,
        "relativeLength": 1e-10,
        "absoluteArea": 1e-8,
        "relativeArea": 1e-9,
        "absoluteVolume": 1e-8,
        "relativeVolume": 1e-9,
        "normalizedHausdorffMax": 1e-7,
        "normalizedCentroidDistanceMax": 1e-8,
        "maxOverrideFactor": 5,
    },
    {
        "id": "mesh-tessellated",
        "description": "📏️ The default for this subset's boolean/primitive/topology corpus: ordinary mechanical solids at millimetre scale re-tessellated by manifold-3d after a boolean or topology edit. Measured against this oracle's own baseline (0.000e+00 round-trip noise, 1.073e-01/9.988e-01 for a genuinely different 5mm-vs-6mm bore) so the floor sits comfortably below any real defect while absorbing ordinary re-tessellation seam noise.",
        "absoluteLength": 1e-6,
        "relativeLength": 1e-7,
        "absoluteArea": 1e-5,
        "relativeArea": 1e-6,
        "absoluteVolume": 1e-5,
        "relativeVolume": 1e-6,
        "normalizedHausdorffMax": 1e-4,
        "normalizedCentroidDistanceMax": 1e-5,
        "maxOverrideFactor": 8,
    },
    {
        "id": "mesh-degenerate",
        "description": "📏️ Slivers, hairline grooves, needle cones and tiny bores at or below manifold's own welding tolerance, where the RESULT CLASS (does the feature survive at all) is the assertion this subset's `degenerate-*` family is making and the metric is secondary — the loosest floor here, mirroring `epsilon-degenerate`'s reasoning but sized to manifold-3d's 1e-7 relative welding grid rather than an analytic kernel's.",
        "absoluteLength": 1e-5,
        "relativeLength": 1e-6,
        "absoluteArea": 1e-4,
        "relativeArea": 1e-5,
        "absoluteVolume": 1e-4,
        "relativeVolume": 1e-5,
        "normalizedHausdorffMax": 1e-3,
        "normalizedCentroidDistanceMax": 1e-4,
        "maxOverrideFactor": 4,
    },
    {
        "id": "mesh-scale-relative",
        "description": "📏️ The `scale-bore-boss-*`/`scale-torus-*` family, the SAME geometry re-expressed from 1e-3 to 1e6 length units, where a fixed absolute floor sized for one scale would either swallow the whole model at the small end or reject legitimate noise at the large end. Only the relative terms are meaningful here, mirroring `large-coordinate`'s and `micro-scale`'s reasoning combined into one profile because this family spans both directions from unit scale.",
        "absoluteLength": 1e-9,
        "relativeLength": 1e-6,
        "absoluteArea": 1e-9,
        "relativeArea": 1e-5,
        "absoluteVolume": 1e-9,
        "relativeVolume": 1e-5,
        "normalizedHausdorffMax": 1e-5,
        "normalizedCentroidDistanceMax": 1e-6,
        "maxOverrideFactor": 5,
    },
]
mesh = new_mesh
mesh["oracles"].append(
    {
        "id": "manifold3d-three",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "manifold-3d",
        "version": "3.5.1",
        "packages": [
            {
                "package": "three",
                "version": "0.182.0",
                "license": "MIT",
                "role": "exports/imports the four committed carriers (STL/OBJ/PLY/glTF) the generator writes fixtures as",
                "homepage": "https://threejs.org",
            }
        ],
        "source": {"repository": "https://github.com/elalish/manifold", "license": "Apache-2.0"},
        "engine": {"family": "manifold", "implementation": "manifold-3d wasm + three.js exporters", "version": "3.5.1"},
        "capabilities": ["semio-v1-mesh-mutate", "mesh.generate.boolean", "mesh.generate.primitive", "mesh.carrier.stl", "mesh.carrier.obj", "mesh.carrier.ply", "mesh.carrier.gltf"],
        "license": "Apache-2.0",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"],
        "homepage": "https://github.com/elalish/manifold",
        "rationale": "🏭️ THE GENERATOR PAIR, DISTINCT FROM THE TWO RUNTIME-COMPARISON ORACLES ABOVE. `manifold-mesh-measure` and `three-carrier-reader` judge a mutation already applied by this subset's own code; this entry is the tool that PRODUCED the expected fixture files in the first place — manifold-3d computes the boolean/primitive geometry (the same engine and version `manifold-mesh-measure` uses to judge it later), and three.js's exporters/loaders round-trip that geometry through all four committed carrier formats so every fixture ships STL, OBJ, PLY and glTF built by the same real pipeline this subset's mutations claim to produce. Registered separately from `manifold-mesh-measure` because a fixture's authority is the tool that GENERATED it, and that tool is this pair used together, not either library alone.",
    }
)
dump(mesh_path, mesh)
print("mesh: contributed semantic-mesh-manifold-v1 + 4 tolerance profiles + manifold3d-three generator oracle")

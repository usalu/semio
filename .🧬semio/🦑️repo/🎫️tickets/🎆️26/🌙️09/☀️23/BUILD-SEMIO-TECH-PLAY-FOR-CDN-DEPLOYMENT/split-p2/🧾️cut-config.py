#!/usr/bin/env python3
"""🧾️ Writes the lane-P2 cut configuration (`📸️remodeling-cut.json` + `📸️remodeling-layers.json`) for `✂️crate-cut.py`:
one layer per crate of the remodeling family, the file → layer rules along the taxonomy, the allowed edges and every
crate root header (lane P2)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
S = "🏅️standards/🔖️1/🪆️subsets/✳️any"
PREFIX = "semio-s-artifact-remodel-remodeling"
MUT = {
    "mut-results": ["replace-sparse", "replace-dense", "replace-mesh-result", "replace-trajectory", "replace-tracks", "replace-geo-products", "replace-qc", "commit-reconstruction"],
    "mut-media": ["create-stream", "delete-stream", "change-stream-sync", "add-stream-frame", "remove-stream-frame", "replace-stream", "create-asset", "delete-asset", "append-content", "remove-content"],
    "mut-calib": ["create-camera", "update-camera", "delete-camera", "create-rig", "delete-rig-extrinsic", "update-rig-extrinsic", "create-gcp", "delete-gcp", "add-gcp-observation", "remove-gcp-observation"],
    "mut-params": ["update-ingest-params", "update-feature-params", "update-match-params", "update-sfm-params", "update-dense-params", "update-mesh-params", "update-motion", "update-geo-params"],
}
CMD = {"cmd-ingest": ["import-frame-payload", "import-video-frame-payload", "import-video-done", "import-video-bytes-payload", "add-stream", "remove-stream", "set-stream-sync", "import-frames", "import-video", "import-abort", "set-active-example"]}
crates = {
    "algebra": (f"{S}/🧬️schema/➕️algebra-internals", "🧮️ remodeling linear algebra internals: dense and fixed-size vectors and matrices, decompositions and solvers."),
    "lie": (f"{S}/🧬️schema/🔷️lie-internals", "🔷️ remodeling Lie-group internals: SO(3), SE(3) and Sim(3) with their tangent maps and alignment."),
    "optimize": (f"{S}/🧬️schema/🎯️optimize-internals", "🎯️ remodeling optimisation internals: Levenberg–Marquardt, RANSAC, robust losses and the Schur bundle solver."),
    "signal": (f"{S}/🧬️schema/📶️signal-internals", "📶️ remodeling signal internals: filters and smoothing over sampled tracks."),
    "spatial": (f"{S}/🧬️schema/🗺️spatial-internals", "🗺️ remodeling spatial internals: k-d trees and spatial grids."),
    "model": (f"{S}/🧬️schema/📸️snapshot", "📸️ remodeling model: the snapshot and its domain records, and the snapshot's text, binary and SQLite codecs."),
    "schema": (f"{S}/🧬️schema", "🧬️ remodeling artifact schema: the artifact facet, document ids and the artifact and snapshot format leaves."),
    "model-diff": (f"{S}/🧬️schema/🔺️diff", "🔺️ remodeling diff: the sparse document delta and its format leaves."),
    "inferences": (f"{S}/🧬️schema/💡️inferences", "💡️ remodeling inferences: bounds and relative-pose inference owners and their descriptor."),
    "mut-results": (f"{S}/🧬️schema/🧬️mutations/🧵️results", "🧵️ remodeling result mutations: the replace-* result leaves and commit-reconstruction."),
    "mut-media": (f"{S}/🧬️schema/🧬️mutations/🎞️media", "🎞️ remodeling media mutations: stream, asset and durable content leaves."),
    "mut-calib": (f"{S}/🧬️schema/🧬️mutations/🎯️calibration", "🎯️ remodeling calibration mutations: camera calibration, rig extrinsic and ground control point leaves."),
    "mut-params": (f"{S}/🧬️schema/🧬️mutations/⚙️params", "⚙️ remodeling parameter mutations: the eight update-*-params leaves."),
    "mutations": (f"{S}/🧬️schema/🧬️mutations", "🧬️ remodeling mutation vocabulary: the dispatch enum, its apply boundary, text and binary codecs and format leaves."),
    "read": (f"{S}/🚪️io/📥️import", "📥️ remodeling import: deserializers from las, ply, png, json, txt, stl, gltf and obj."),
    "write": (f"{S}/🚪️io/📤️export", "📤️ remodeling export: serializers to las, ply, png, json, txt, stl, gltf and obj."),
    "io": (f"{S}/🚪️io", "🚪️ remodeling io: the subset io declaration, builder/analyzer/composer facets and the diff and inference codecs."),
    "orbit-q1": (f"{S}/📚️examples/🛰️synthetic-orbit/🌑️quarter-1", "🌑️ synthetic-orbit example: views 0–8."),
    "orbit-q2": (f"{S}/📚️examples/🛰️synthetic-orbit/🌓️quarter-2", "🌓️ synthetic-orbit example: views 9–17."),
    "orbit-q3": (f"{S}/📚️examples/🛰️synthetic-orbit/🌕️quarter-3", "🌕️ synthetic-orbit example: views 18–26."),
    "orbit-q4": (f"{S}/📚️examples/🛰️synthetic-orbit/🌗️quarter-4", "🌗️ synthetic-orbit example: views 27–35."),
    "examples": (f"{S}/📚️examples", "📚️ remodeling examples: the demo document and the synthetic-orbit capture."),
    "images": (f"{S}/✏️editor/⚙️engine/🖼️images", "🖼️ remodeling engine images: rasters, pyramids, PNG/JPEG codecs and the interactive step ceiling law."),
    "camera": (f"{S}/✏️editor/⚙️engine/📷️camera", "📷️ remodeling engine camera: intrinsics, distortion, poses and calibration."),
    "feature": (f"{S}/✏️editor/⚙️engine/🌟️feature", "🌟️ remodeling engine features: detection, description and matching."),
    "video-h264": (f"{S}/✏️editor/⚙️engine/🎥️video/🎞️h264", "🎞️ remodeling engine H.264: the baseline decoder and the I_PCM/P_SKIP encoder."),
    "video": (f"{S}/✏️editor/⚙️engine/🎥️video", "🎥️ remodeling engine video: container admission, providers and frame extraction."),
    "dense": (f"{S}/✏️editor/⚙️engine/🌫️dense", "🌫️ remodeling engine dense: depth maps and TSDF fusion."),
    "sfm-geometry": (f"{S}/✏️editor/⚙️engine/📸️sfm/📐️geometry", "📐️ remodeling engine multi-view geometry: two-view models, triangulation, PnP, tracks, averaging and loop closure."),
    "sfm": (f"{S}/✏️editor/⚙️engine/📸️sfm", "📸️ remodeling engine structure from motion: incremental reconstruction, bundle adjustment and pose priors."),
    "mesh-surface": (f"{S}/✏️editor/⚙️engine/🥽️mesh/🔺️surface", "🔺️ remodeling engine mesh surface: triangle mesh, topology and iso-surface extraction."),
    "mesh-repair": (f"{S}/✏️editor/⚙️engine/🥽️mesh/🩹️repair", "🩹️ remodeling engine mesh repair: cleaning, orientation, hole filling and watertightness."),
    "mesh-atlas": (f"{S}/✏️editor/⚙️engine/🥽️mesh/🗺️atlas", "🗺️ remodeling engine mesh atlas: simplification, unwrapping and texture baking."),
    "mesh": (f"{S}/✏️editor/⚙️engine/🥽️mesh", "🥽️ remodeling engine mesh pipeline: the bounded watertight surface pipeline and interchange."),
    "motion": (f"{S}/✏️editor/⚙️engine/🏃️motion", "🏃️ remodeling engine motion: tracking and trajectory estimation."),
    "geo": (f"{S}/✏️editor/⚙️engine/🗺️geo", "🗺️ remodeling engine geo: georeferencing and geo products."),
    "reconstruct": (f"{S}/✏️editor/⚙️engine/🏭️reconstruction", "🏭️ remodeling engine reconstruction: the staged reconstruction pipeline."),
    "engine": (f"{S}/✏️editor/⚙️engine", "⚙️ remodeling engine bridge: engine parameters and results from and to the document."),
    "editor-kit": (f"{S}/✏️editor/🧰️kit", "🧰️ remodeling editor kit: shared editor plumbing, window transient state and terminology."),
    "cmd-ingest": (f"{S}/✏️editor/🎮️commands/📥️ingest", "📥️ remodeling ingest commands: frame, video and stream import, abort and the example picker."),
    "cmd-workflow": (f"{S}/✏️editor/🎮️commands/🎛️workflow", "🎛️ remodeling workflow commands: calibration, GCPs, parameters, clearing, view state and QC export."),
    "modes": (f"{S}/✏️editor/🎭️modes", "🎭️ remodeling editor modes: capture, model and analyze modes with their windows and tools."),
    "panels": (f"{S}/✏️editor/📌️panels", "📌️ remodeling editor panels."),
    "session": (f"{S}/✏️editor/🧵️reconstruction-session", "🧵️ remodeling reconstruction session: the reconstruction tool run."),
    "editor": (f"{S}/✏️editor", "✏️ remodeling editor: the command enum, action bridge, play app and manifest."),
    "viewer": (f"{S}/👁️viewer", "👁️ remodeling viewer."),
}
TESTS = {"it-editor": ("🧪️tests/🔬️editor", "🧪️ remodeling editor integration tests: every law that drives the play app through its command dispatch.")}
IT_EDITOR = ["✏️editor/🧪️tests/", "✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🧪️tests/", "✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🧪️tests/",
             "✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/", "✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🧪️tests/",
             "✏️editor/🎮️commands/[^/]+/🧪️tests/", "✏️editor/📌️panels/[^/]+/🧪️tests/", "📚️examples/🛰️synthetic-orbit/🧪️tests/"]
def alt(names):
    return "(" + "|".join(names) + ")"
rules = [[f"^{S}/{path}", "it-editor"] for path in IT_EDITOR] + [
    ["^🧪️tests/🔬️editor/", "it-editor"],
    ["^🦀️\\.rs$", "aggregator"], ["^🧪️tests/", "aggregator"],
    ["^🏅️standards/🔖️1/🦀️\\.rs$", "aggregator"], [f"^{S}/🦀️\\.rs$", "aggregator"],
    ["/🧬️schema/➕️algebra-internals/", "algebra"], ["/🧬️schema/🔷️lie-internals/", "lie"], ["/🧬️schema/🎯️optimize-internals/", "optimize"],
    ["/🧬️schema/📶️signal-internals/", "signal"], ["/🧬️schema/🗺️spatial-internals/", "spatial"],
    ["/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/", "aggregator"], ["/🚪️io/🧪️tests/", "aggregator"],
    ["/🧬️schema/📸️snapshot/", "model"], ["/🚪️io/📝️text/📸️snapshot/", "model"], ["/🚪️io/💾️binary/📸️snapshot/", "model"], ["/🚪️io/🪶️sqlite/📸️snapshot/", "model"],
    ["/🧬️schema/🔺️diff/", "model-diff"], ["/🧬️schema/💡️inferences/", "inferences"],
]
for layer, leaves in MUT.items():
    rules.append([f"/🧬️schema/🧬️mutations/[^/]*{alt(leaves)}/", layer])
rules += [
    ["/🧬️schema/🧬️mutations/", "mutations"], ["/🚪️io/📝️text/🧬️mutations/", "mutations"], ["/🚪️io/💾️binary/🧬️mutations/", "mutations"],
    ["/🧬️schema/", "schema"],
    ["/🚪️io/📥️import/", "read"], ["/🚪️io/📤️export/", "write"], ["/🚪️io/", "io"],
    ["/📚️examples/🛰️synthetic-orbit/🌑️quarter-1/", "orbit-q1"], ["/📚️examples/🛰️synthetic-orbit/🌓️quarter-2/", "orbit-q2"],
    ["/📚️examples/🛰️synthetic-orbit/🌕️quarter-3/", "orbit-q3"], ["/📚️examples/🛰️synthetic-orbit/🌗️quarter-4/", "orbit-q4"],
    [f"^{S}/📚️examples/", "examples"],
    ["/⚙️engine/🥽️mesh/🔺️surface/", "mesh-surface"], ["/⚙️engine/🥽️mesh/🩹️repair/", "mesh-repair"], ["/⚙️engine/🥽️mesh/🗺️atlas/", "mesh-atlas"], ["/⚙️engine/🥽️mesh/", "mesh"],
    ["/⚙️engine/📸️sfm/📐️geometry/", "sfm-geometry"], ["/⚙️engine/📸️sfm/", "sfm"],
    ["/⚙️engine/🎥️video/🎞️h264/", "video-h264"], ["/⚙️engine/🎥️video/", "video"],
    ["/⚙️engine/🏭️reconstruction/🧪️tests/(🔬️unit|🔬️retention)/", "engine"],
    ["/⚙️engine/🖼️images/", "images"], ["/⚙️engine/🧪️tests/🔬️step-ceiling/", "images"], ["/⚙️engine/📷️camera/", "camera"], ["/⚙️engine/🌟️feature/", "feature"],
    ["/⚙️engine/🌫️dense/", "dense"], ["/⚙️engine/🏃️motion/", "motion"], ["/⚙️engine/🗺️geo/", "geo"], ["/⚙️engine/🏭️reconstruction/", "reconstruct"],
    ["/⚙️engine/", "engine"],
    ["/✏️editor/🧰️kit/", "editor-kit"], ["/✏️editor/🗣️terminology/", "editor-kit"], ["/✏️editor/📚️examples/", "editor-kit"], ["/✏️editor/🫧️transient/", "modes"],
    [f"/✏️editor/🎮️commands/[^/]*{alt(CMD['cmd-ingest'])}/", "cmd-ingest"], ["/✏️editor/🎮️commands/", "cmd-workflow"],
    ["/✏️editor/🎭️modes/", "modes"], ["/✏️editor/📌️panels/", "panels"], ["/✏️editor/🧵️reconstruction-session/", "session"],
    ["/✏️editor/", "editor"], ["/👁️viewer/", "viewer"],
    [".", "?"],
]
order = ["algebra", "lie", "optimize", "signal", "spatial", "model", "schema", "model-diff", "mut-results", "mut-media", "mut-calib", "mut-params", "mutations", "io", "read", "write", "inferences",
         "orbit-q1", "orbit-q2", "orbit-q3", "orbit-q4", "examples", "images", "camera", "feature", "video-h264", "video", "dense", "sfm-geometry", "sfm", "mesh-surface", "mesh-repair", "mesh-atlas", "mesh",
         "motion", "geo", "reconstruct", "editor-kit", "engine", "session", "modes", "cmd-ingest", "cmd-workflow", "panels", "editor", "viewer", "aggregator", "it-editor"]
allow = {layer: order[:index] for index, layer in enumerate(order)}
json.dump({"order": order, "allow": allow, "rules": rules}, open(os.path.join(HERE, "📸️remodeling-layers.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
header = ["extern crate semio_framework_os_kernel as dsl;", "extern crate semio_framework_os_kernel as protocol;", "extern crate semio_framework_os_kernel as store;", "extern crate semio_framework_schema as framework_schema;"]
config = {
    "artifact": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling",
    "layers": "📸️remodeling-layers.json",
    "crate": "semio_s_artifact_remodel_remodeling",
    "aggregator": "aggregator",
    "above_aggregator": ["it-editor"],
    "crates": {**{name: {"package": f"{PREFIX}-{name}", "dir": f"{directory}/📦️packages/🦀️rust"} for name, (directory, _) in crates.items()}, **{name: {"package": f"{PREFIX}-{name}", "dir": directory} for name, (directory, _) in TESTS.items()}},
    "header": {**{name: [f"//! {doc}", ""] + header for name, (_, doc) in crates.items()}, **{name: [f"//! {doc}", ""] + header for name, (_, doc) in TESTS.items()}},
    "consumers": ["🌎️hub/🧩️compositions/📸️remodel", "✏️s/🔌️plugins/📸️remodel/🏭️bridge"],
}
for name, spec in config["crates"].items():
    assert len(spec["package"]) <= 48, spec["package"]
json.dump(config, open(os.path.join(HERE, "📸️remodeling-cut.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(crates), "crates")

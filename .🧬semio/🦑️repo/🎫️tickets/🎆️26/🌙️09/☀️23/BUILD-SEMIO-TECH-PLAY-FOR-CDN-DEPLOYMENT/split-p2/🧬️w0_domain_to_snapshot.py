#!/usr/bin/env python3
"""🧬️ Wave 0: moves the remodeling domain records (composition handles, durable content, buffers, entity records,
`default_remodeling_scene`) and the document schema/dialect constants out of the crate root into the snapshot facet
`🧬️schema/📸️snapshot/🦀️.rs`, and drops the root re-export facades (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
ROOT = os.path.join(ART, "🦀️.rs")
SNAP = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs")
root = open(ROOT, encoding="utf-8").read()
start = root.index("pub use crate::schema::mutations::RemodelingMutation;")
end = root.index("//#endregion 🔖️Domain\n") + len("//#endregion 🔖️Domain\n")
moved = root[start:end]
root = root[:start] + root[end:]
moved = moved.replace("pub use crate::schema::mutations::RemodelingMutation;\n\npub use crate::schema::diff::RemodelingDiff;\n\n", "")
moved = moved.replace("/// 📸️ Persisted remodeling snapshot — re-exported from `📸️snapshot/🧬️schema`.\npub use crate::schema::snapshot::RemodelingSnapshot;\n\n", "")
assert "pub use" not in moved, [l for l in moved.split("\n") if "pub use" in l]
moved = moved.replace("pub const REMODELING_DOCUMENT_SCHEMA", "/// 🧾️ Document schema id every remodeling snapshot carries.\npub const REMODELING_DOCUMENT_SCHEMA", 1)
for line in ["use semio_framework::MeshData;\n", "use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};\n",
             "use semio_framework_value_derive::{FromValue, ToValue};\n", "use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;\n",
             "use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;\n", "use serde::{Deserialize, Serialize};\n",
             "use std::collections::BTreeMap;\n", "use std::sync::Arc;\n"]:
    assert line in root, line
    root = root.replace(line, "", 1)
root = root.replace("\n//#endregion 🧮️MathInternals\n", "\n//#endregion 🧮️MathInternals\n\nuse semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};\n", 1)
snap = open(SNAP, encoding="utf-8").read()
old_use = "use crate::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore, REMODELING_DOCUMENT_SCHEMA};\n"
assert old_use in snap
snap = snap.replace(old_use, "use semio_framework::MeshData;\nuse semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;\nuse semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;\nuse serde::{Deserialize, Serialize};\nuse std::sync::Arc;\n")
snap = snap.rstrip("\n") + "\n\n" + moved
open(SNAP, "w", encoding="utf-8").write(snap)
open(ROOT, "w", encoding="utf-8").write(root)
print("moved", moved.count("\n"), "lines; root now", root.count("\n"), "lines")

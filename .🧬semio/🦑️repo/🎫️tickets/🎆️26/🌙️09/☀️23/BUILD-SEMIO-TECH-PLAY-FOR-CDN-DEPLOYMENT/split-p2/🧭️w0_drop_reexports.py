#!/usr/bin/env python3
"""🧭️ Wave 0: deletes every re-export facade of the remodeling monolith that would cross a crate boundary after the cut
(mutation leaf re-exports, the diff's artifact re-export, the engine topics' algebra/lie/optimize/camera/image/mesh/sfm
re-exports, the editor's `ui_node_list`). `🧭️canonical-paths.py` then points every consumer at the owning module (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
A = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
E = os.path.join(A, "✏️editor/⚙️engine")
targets = {
    os.path.join(A, "🧬️schema/🧬️mutations/🦀️.rs"): [r"(?m)^pub use super::[a-z_]+::\{[^}]*\};\n"],
    os.path.join(A, "🧬️schema/🔺️diff/🦀️.rs"): [r"(?m)^pub use crate::schema::RemodelingArtifact;\n"],
    os.path.join(E, "📸️sfm/🦀️.rs"): [r"(?m)^pub use crate::lie::\{Se3, Sim3, So3\};\n", r"(?m)^pub use crate::optimize::RobustLoss;\n", r"(?m)^pub use remodeling_camera::\{CameraPose, Distortion, Intrinsics\};\n"],
    os.path.join(E, "🏃️motion/🦀️.rs"): [r"(?m)^pub use crate::lie::Se3;\n", r"(?m)^pub use remodeling_camera::\{CameraPose, Intrinsics\};\n", r"(?m)^pub use remodeling_image::\{ImageGray, Pyramid\};\n"],
    os.path.join(E, "🗺️geo/🦀️.rs"): [r"(?m)^pub use crate::lie::Sim3;\n", r"(?m)^pub use remodeling_camera::\{CameraPose, Intrinsics\};\n", r"(?m)^pub use remodeling_mesh::WatertightReport;\n", r"(?m)^pub use remodeling_sfm::Reconstruction;\n"],
    os.path.join(E, "📷️camera/🦀️.rs"): [r"(?m)^pub use crate::algebra::\{MatD, VecD\};\n", r"(?m)^pub use crate::lie::Se3;\n", r"(?m)^pub use crate::optimize::\{levenberg_marquardt, LeastSquaresProblem, LmConfig, LmResult, RobustLoss\};\n"],
    os.path.join(A, "✏️editor/🦀️.rs"): [r"(?m)^pub use semio_framework_plugin::ui_node_list;\n"],
}
replacements = {
    os.path.join(E, "📸️sfm/🦀️.rs"): "use crate::lie::{Se3, Sim3, So3};\nuse crate::optimize::RobustLoss;\nuse remodeling_camera::{CameraPose, Distortion, Intrinsics};\n",
    os.path.join(E, "🏃️motion/🦀️.rs"): "use crate::lie::Se3;\nuse remodeling_camera::{CameraPose, Intrinsics};\nuse remodeling_image::{ImageGray, Pyramid};\n",
    os.path.join(E, "🗺️geo/🦀️.rs"): "use crate::lie::Sim3;\nuse remodeling_camera::{CameraPose, Intrinsics};\nuse remodeling_mesh::WatertightReport;\nuse remodeling_sfm::Reconstruction;\n",
    os.path.join(E, "📷️camera/🦀️.rs"): "use crate::algebra::{MatD, VecD};\nuse crate::lie::Se3;\nuse crate::optimize::{levenberg_marquardt, LeastSquaresProblem, LmConfig, LmResult, RobustLoss};\n",
}
for path, patterns in targets.items():
    text = open(path, encoding="utf-8").read()
    first = None
    for pattern in patterns:
        m = re.search(pattern, text)
        assert m, (path, pattern)
        first = m.start() if first is None else min(first, m.start())
        text = re.sub(pattern, "", text)
    if path in replacements:
        text = text[:first] + replacements[path] + text[first:]
    open(path, "w", encoding="utf-8").write(text)
    print("rewrote", os.path.relpath(path, A))

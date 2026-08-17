#!/usr/bin/env python3
"""Fix snapshot schema codecs and types after generator."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ARTIFACTS = [
    ("📓️iso16757", "Iso16757"),
    ("📔️vdi3805", "Vdi3805"),
    ("📕️din4108", "Din4108"),
    ("📗️din16798", "Din16798"),
    ("📙️din18599", "Din18599"),
]

for folder, prefix in ARTIFACTS:
    snap = ROOT / folder / "📸️snapshot/🧬️schema/🦀️component.rs"
    if not snap.exists():
        continue
    text = snap.read_text(encoding="utf-8")
    text = text.replace(f"{prefix}SnapshotDsl", "DocumentDsl")
    text = text.replace(f"{prefix}SnapshotPack", "DocumentPack")
    text = text.replace(f"Handcrafted{prefix}SnapshotCodecs", "HandcraftedDocumentCodecs")
    text = text.replace(f"SourceMode::{prefix}Snapshot", "SourceMode::Document")
    text = text.replace(f"JoinMode::{prefix}Snapshot", "JoinMode::Document")
    text = text.replace(f"Layer{prefix}Snapshot", "LayerDocument")
    text = text.replace("Vec<LayerDin4108Snapshot>", "Vec<crate::artifacts::din4108::LayerDocument>")
    if "use crate::document::ClimateZoneDe" not in text and "ClimateZoneDe" in text:
        text = text.replace(
            "use serde::{Deserialize, Serialize};",
            "use crate::document::ClimateZoneDe;\nuse serde::{Deserialize, Serialize};",
        )
    if "use crate::document::AnnexChoice" not in text and "AnnexChoice" in text:
        text = text.replace(
            "use serde::{Deserialize, Serialize};",
            "use crate::document::AnnexChoice;\nuse serde::{Deserialize, Serialize};",
        )
    snap.write_text(text, encoding="utf-8")
    comp = ROOT / folder / "🦀️component.rs"
    if comp.exists():
        c = comp.read_text(encoding="utf-8")
        c = c.replace("#endregion 🔖️Types", "//#endregion 🔖️Types")
        c = c.replace("// \n/// 📸️", "\n/// 📸️")
        comp.write_text(c, encoding="utf-8")

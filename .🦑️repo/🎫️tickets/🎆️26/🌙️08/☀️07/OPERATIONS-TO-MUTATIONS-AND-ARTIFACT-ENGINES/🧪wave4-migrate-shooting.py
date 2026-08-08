from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/🎥️shooting"
ART = next((PLUGIN / "🗿️artifacts").iterdir())
MUT = ART / "🧬️mutations"
OP = ART / "🔧️op" / "🦀️component.rs"

MUTATIONS = [
    ("📦", "assets", "Assets"),
    ("📸", "shots", "Shots"),
    ("🎥", "saved-cameras", "SavedCameras"),
    ("🎯", "set-active-shot", "SetActiveShot"),
    ("📌", "set-active-asset", "SetActiveAsset"),
    ("📷", "set-shot-camera", "SetShotCamera"),
    ("☀️", "patch-scene", "PatchScene"),
    ("↔️", "translate-assets", "TranslateAssets"),
    ("🔄", "rotate-assets", "RotateAssets"),
    ("↕️", "scale-assets", "ScaleAssets"),
    ("📄", "set-fixture", "SetFixture"),
]
assert len({e for e,_,_ in MUTATIONS}) == len(MUTATIONS)

def write(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    if not content.endswith("\n"):
        content += "\n"
    path.write_text(content)

def ts_stub(label: str) -> str:
    return "/** 🧩 %s facade stub. */\nexport {};\n" % label

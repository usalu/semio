"""🔥 Print the heating calculation constants that the en/de narration figures quote."""

import sys
from importlib import import_module
from pathlib import Path

ROOT = Path("C:/git/semio")
sys.path.insert(0, str(ROOT))
mod = import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")
for name in ("_PHI_T", "_PHI_V", "_PHI_LOSS", "_GT", "_Q_LOSS_SHOWN", "_Q_SOL", "_Q_INT", "_Q_GAIN", "_ETA_LIGHT", "_ETA_HEAVY", "_Q_USE", "_Q_H", "_Q_CE", "_Q_D", "_Q_S", "_Q_G", "_Q_E"):
    print(name, getattr(mod, name))

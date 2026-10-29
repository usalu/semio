"""🔍 Import every Grasshopper and Excel scene class and report titles and clause counts."""

from __future__ import annotations

import importlib
import os
import sys

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
sys.dont_write_bytecode = True
from series import ARCH, ROOT, SERIES

sys.path.insert(0, str(ROOT))
missing = 0
for series in SERIES.values():
    for video, scene, folder, chapter, output, icon, title in series["videos"]:
        module = f"🎓️teaching.🏛️architecture.{series['folder']}.🎬️clip.{folder}.🐍️"
        try:
            cls = getattr(importlib.import_module(module), scene)
            print(f"[DEBUG] OK {video} {scene} TITLE={getattr(cls, 'TITLE', None)}")
        except Exception as error:
            missing += 1
            print(f"[DEBUG] MISSING {video} {scene}: {type(error).__name__}: {str(error)[:150]}")
raise SystemExit(1 if missing else 0)

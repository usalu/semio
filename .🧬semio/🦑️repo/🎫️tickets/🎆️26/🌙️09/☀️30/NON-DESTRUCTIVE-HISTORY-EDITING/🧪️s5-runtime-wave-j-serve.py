"""🟦️ Wave J, the halves the live serves bundle or import (rule 61: saved only while holding `serve`): the lifecycle-law
fixture and schema with the `timeTravel.unchanged` refusal, the TS twin and its conformance driver, and the kernel's
`draftChanged` (TS twin, `history-patch` schema and fixture). The patches are wave J's (`🧪️s5-runtime-wave-j.py`); loaded
by `🧪️s5-runtime-land.py`.
"""

import importlib.util
import pathlib

HERE = pathlib.Path(__file__).resolve().parent


def files(root):
    spec = importlib.util.spec_from_file_location("wave_j", HERE / "🧪️s5-runtime-wave-j.py")
    wave = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(wave)
    return wave.served(root)

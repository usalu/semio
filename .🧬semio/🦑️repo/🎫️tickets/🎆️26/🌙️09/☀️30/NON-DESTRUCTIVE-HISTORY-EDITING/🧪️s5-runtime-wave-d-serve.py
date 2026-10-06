"""🟦️ Wave D, the halves the live serves bundle or import (rules 57 and 61: saved only while holding `serve`): the
lifecycle-law fixture and schema without the base's store generation, the TS twin, and the two TS law drivers. The patches
are wave D's (`🧪️s5-runtime-wave-d.py`); loaded by `🧪️s5-runtime-land.py`.
"""

import importlib.util
import pathlib

HERE = pathlib.Path(__file__).resolve().parent


def files(root):
    spec = importlib.util.spec_from_file_location("wave_d", HERE / "🧪️s5-runtime-wave-d.py")
    wave = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(wave)
    return wave.served(root)

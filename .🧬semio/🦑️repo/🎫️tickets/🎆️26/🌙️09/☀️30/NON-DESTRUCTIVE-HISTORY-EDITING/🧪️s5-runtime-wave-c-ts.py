"""🟦️ Wave C, bundled TS twins (rule 57: saved only while holding `serve`): the manifest's thirteen history-edit verbs and the
kernel wire row's `withdrawable`. The patches are wave C's (`🧪️s5-runtime-wave-c.py`); loaded by `🧪️s5-runtime-land.py`.
"""

import importlib.util
import pathlib

HERE = pathlib.Path(__file__).resolve().parent


def files(_root):
    spec = importlib.util.spec_from_file_location("wave_c", HERE / "🧪️s5-runtime-wave-c.py")
    wave = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(wave)
    return {f"{wave.MANIFEST}/🟦️.ts": wave.MANIFEST_TS, f"{wave.KERNEL}/🟦️.ts": wave.KERNEL_TS}

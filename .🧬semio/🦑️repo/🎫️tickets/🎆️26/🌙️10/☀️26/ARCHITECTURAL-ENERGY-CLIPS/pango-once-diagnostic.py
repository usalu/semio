"""🩺 Bound a single-scene diagnostic with periodic native Python tracebacks."""
import faulthandler
import runpy
import sys
import os
import threading
from pathlib import Path
root = Path(__file__).parent
output = root / "🗑️generated" / "smoke-pango-once"
output.mkdir(parents=True, exist_ok=True)
with (output / "stack.log").open("w", encoding="utf-8") as log:
    log.write("[DEBUG] Bounded smoke diagnostic stack snapshots every 30 seconds\n")
    log.flush()
    faulthandler.dump_traceback_later(30, repeat=True, file=log)
    def expire():
        threading.Event().wait(100)
        log.write("[DEBUG] Diagnostic time budget expired after 100 seconds\n")
        log.flush()
        os._exit(124)
    threading.Thread(target=expire, daemon=True).start()
    import ctypes
    import ctypes.wintypes as w
    import json
    import time
    from manim import Text, config
    class Memory(ctypes.Structure):
        _fields_ = [("cb", w.DWORD), ("PageFaultCount", w.DWORD), *[(name, ctypes.c_size_t) for name in
                     ("PeakWorkingSetSize", "WorkingSetSize", "QuotaPeakPagedPoolUsage", "QuotaPagedPoolUsage",
                      "QuotaPeakNonPagedPoolUsage", "QuotaNonPagedPoolUsage", "PagefileUsage", "PeakPagefileUsage")]]
    kernel = ctypes.WinDLL("kernel32")
    kernel.GetCurrentProcess.restype = w.HANDLE
    process = kernel.GetCurrentProcess()
    psapi = ctypes.WinDLL("psapi")
    psapi.GetProcessMemoryInfo.argtypes = [w.HANDLE, ctypes.c_void_p, w.DWORD]
    def memory():
        result = Memory()
        result.cb = ctypes.sizeof(result)
        psapi.GetProcessMemoryInfo(process, ctypes.byref(result), result.cb)
        return result.WorkingSetSize // 1048576
    import manimpango
    native_pango = manimpango.text2svg
    attached = set()
    def attach_once(*args, **kwargs):
        redundant = manimpango.registered_fonts & attached
        pending = {font for font in manimpango.registered_fonts if font.type == "win32"}
        manimpango.registered_fonts.difference_update(redundant)
        try:
            result = native_pango(*args, **kwargs)
            attached.update(pending)
            return result
        finally:
            manimpango.registered_fonts.update(redundant)
    manimpango.text2svg = attach_once
    native = Text._text2svg
    counter = [0]
    records = (output / "text.jsonl").open("w", encoding="utf-8")
    def measured(self, color):
        counter[0] += 1
        number = counter[0]
        before = memory()
        started = time.monotonic()
        record = {"debug": "[DEBUG] text begin", "number": number, "text": self.text,
                  "font": self.font, "size": self._font_size, "slant": self.slant,
                  "line_spacing": self.line_spacing, "t2f": getattr(self, "t2f", getattr(self, "text2font", {})), "canvas": [config.pixel_width, config.pixel_height],
                  "working_mb": before}
        records.write(json.dumps(record, ensure_ascii=False) + "\n")
        records.flush()
        result = native(self, color)
        records.write(json.dumps({"debug": "[DEBUG] text end", "number": number,
                                  "seconds": round(time.monotonic()-started, 4), "working_mb": memory()}) + "\n")
        records.flush()
        return result
    Text._text2svg = measured
    try:
        sys.argv = [str(root / "smoke.py"), *sys.argv[1:]]
        source = (root / "smoke.py").read_text(encoding="utf-8")
        source = source.replace('output = Path(__file__).parent / "🗑️generated" / "smoke"', 'output = Path(__file__).parent / "🗑️generated" / "smoke-pango-once"')
        source = source.replace('report = Path(__file__).parent / "smoke-verification.md"', 'report = Path(__file__).parent / "🗑️generated" / "smoke-pango-once" / "report.md"')
        exec(compile(source, str(root / "smoke.py"), "exec"), {"__file__": str(root / "smoke.py"), "__name__": "__main__"})
    finally:
        faulthandler.cancel_dump_traceback_later()

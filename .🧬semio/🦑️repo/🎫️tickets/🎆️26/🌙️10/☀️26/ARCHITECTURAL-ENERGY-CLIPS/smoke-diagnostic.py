"""🩺 Bound a single-scene diagnostic with periodic native Python tracebacks."""
import faulthandler
import runpy
import sys
import os
import threading
from pathlib import Path
root = Path(__file__).parent
output = root / "🗑️generated" / "smoke-diagnostic"
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
    try:
        sys.argv = [str(root / "smoke.py"), *sys.argv[1:]]
        source = (root / "smoke.py").read_text(encoding="utf-8")
        source = source.replace('output = Path(__file__).parent / "🗑️generated" / "smoke"', 'output = Path(__file__).parent / "🗑️generated" / "smoke-diagnostic"')
        source = source.replace('report = Path(__file__).parent / "smoke-verification.md"', 'report = Path(__file__).parent / "🗑️generated" / "smoke-diagnostic" / "report.md"')
        exec(compile(source, str(root / "smoke.py"), "exec"), {"__file__": str(root / "smoke.py"), "__name__": "__main__"})
    finally:
        faulthandler.cancel_dump_traceback_later()

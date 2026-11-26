import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

def sample(executable, test, delays, raw_path, environment):
    with raw_path.open("w") as raw:
        with subprocess.Popen([executable, test, "--nocapture", "--test-threads=1"], env=environment, stdout=raw, stderr=raw) as process:
            started = time.monotonic()
            for delay in delays:
                time.sleep(max(0, delay - (time.monotonic() - started)))
                if process.poll() is not None:
                    break
                raw.write(f"\n[DEBUG] Native history stack at {delay} seconds\n")
                raw.flush()
                for _ in range(10):
                    if process.poll() is not None:
                        break
                    for task in Path(f"/proc/{process.pid}/task").glob("*"):
                        try:
                            if (task / "comm").read_text().startswith(("history_edit_r", "mirror::editor")):
                                os.kill(int(task.name), signal.SIGUSR2)
                        except (FileNotFoundError, ProcessLookupError):
                            continue
                    time.sleep(0.2)
            if process.poll() is None:
                process.terminate()
                process.wait()

if sys.argv[1] == "--resolve":
    raw_path = Path(sys.argv[2]).resolve()
    executable = Path(sys.argv[3]).resolve()
else:
    output = Path(__file__).resolve().parent / "🗑️generated"
    executable = Path(os.environ.get("SEMIO_UI_HISTORY_EXECUTABLE", str(output / "ui-preview-native/history-focused"))).resolve()
    test = sys.argv[1]
    delays = [int(seconds) for seconds in sys.argv[2:]]
    raw_path = output / f"ui-history-{test}-stack-raw.log"
    environment = {**os.environ, "LD_PRELOAD": str(output / "current-native-stack.so"), "RUST_MIN_STACK": "134217728"}
    sample(executable, test, delays, raw_path, environment)

raw = raw_path.read_text()
pattern = re.compile(rf"{re.escape(str(executable))}\(\+(0x[0-9a-f]+)\)")
offsets = sorted({int(offset, 16) for offset in pattern.findall(raw)})
if not offsets:
    raise SystemExit("no native test-thread samples")

symbols = {}
pending = iter(offsets)
address = next(pending, None)
previous = (0, 0, "")
with subprocess.Popen(["nm", "-n", "-S", "-C", "--defined-only", executable], stdout=subprocess.PIPE, text=True) as names:
    for line in names.stdout:
        fields = line.split(maxsplit=3)
        if len(fields) != 4 or fields[2] not in ("T", "t", "W", "w"):
            continue
        location = int(fields[0], 16)
        while address is not None and address < location:
            start, size, name = previous
            if address < start + size:
                symbols[address] = name
            address = next(pending, None)
        if address is None:
            break
        previous = location, int(fields[1], 16), fields[3].strip()

for line in raw.splitlines():
    if line.startswith("[DEBUG]") or "panicked" in line or "test result" in line:
        print(line)
    else:
        match = pattern.search(line)
        if match:
            address = int(match.group(1), 16)
            print(symbols.get(address, match.group(1)))

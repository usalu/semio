import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
executable = Path(sys.argv[1]).resolve()
manifest = Path(sys.argv[2]).resolve()
delays = [int(delay) for delay in sys.argv[3:]] or [30, 120, 300]
test = "mirror::editor::puzzle5d::unit_tests::history_edits_end_to_end"
environment = {
    **os.environ,
    "LD_PRELOAD": str(output / "current-native-stack.so"),
    "RUST_MIN_STACK": "134217728",
}
raw_path = output / f"law-5d-history-sample-{executable.parent.name}-raw.log"
with raw_path.open("w") as raw:
    with subprocess.Popen(
        [executable, test, "--exact", "--nocapture", "--test-threads=1"],
        cwd=manifest, env=environment, stdout=raw, stderr=raw,
    ) as process:
        start = time.monotonic()
        for delay in delays:
            if process.poll() is not None:
                break
            time.sleep(max(0, delay - (time.monotonic() - start)))
            raw.write(f"\n[DEBUG] Canonical history stack burst at {delay} seconds\n")
            raw.flush()
            for _ in range(8):
                if process.poll() is not None:
                    break
                try:
                    tasks = list(Path(f"/proc/{process.pid}/task").iterdir())
                except FileNotFoundError:
                    break
                for task in tasks:
                    try:
                        if (task / "comm").read_text().startswith("mirror::editor"):
                            os.kill(int(task.name), signal.SIGUSR2)
                    except (FileNotFoundError, ProcessLookupError):
                        continue
                time.sleep(0.25)
        if process.poll() is None:
            process.terminate()
            process.wait()

raw = raw_path.read_text()
offsets = tuple(dict.fromkeys(re.findall(rf"{re.escape(str(executable))}\(\+(0x[0-9a-f]+)\)", raw)))
if not offsets:
    raise SystemExit("no test-thread frames were captured")
symbols = subprocess.check_output(["addr2line", "-f", "-C", "-e", executable, *offsets], text=True).splitlines()
frames = {offset: symbols[index * 2] for index, offset in enumerate(offsets)}
for line in raw.splitlines():
    if line.startswith("[DEBUG]") or "panicked" in line or "test result" in line:
        print(line)
    else:
        found = re.search(r"\(\+(0x[0-9a-f]+)\)", line)
        if found and line.startswith(str(executable)):
            print(frames[found.group(1)])

import hashlib
import os
import shutil
import subprocess
import sys
from pathlib import Path

T = Path(__file__).resolve().parent.parent
cache = T / "🗑️generated/cv2/race-cache"
shutil.rmtree(cache, ignore_errors=True)
cache.mkdir(parents=True)
env = {**os.environ, "SEMIO_TEACHING_CACHE_ROOT": str(cache)}
worker = str(Path(__file__).with_name("race_worker.py"))
sets = ["shared-lamp-neutral,shared-lamp-off-neutral", "shared-lamp-neutral,shared-lamp-off-neutral", "shared-lamp-off-neutral,shared-lamp-neutral"]
procs = [subprocess.Popen([sys.executable, worker, ids], env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8") for ids in sets]
codes = []
for index, proc in enumerate(procs):
    out, err = proc.communicate()
    codes.append(proc.returncode)
    print(f"worker {index} exit {proc.returncode}: {out.strip()[:300]}")
    if proc.returncode:
        print(err[-800:])
staffage = cache / "staffage"
print("leftovers:", [p.name for p in staffage.iterdir() if p.name.startswith(".")])
keys = [p for p in staffage.iterdir() if not p.name.startswith(".")]
print("key folders:", len(keys))
for key in keys:
    for cast in sorted(key.iterdir()):
        digest = hashlib.sha256(b"".join(f.read_bytes() for f in sorted(cast.iterdir()))).hexdigest()[:12]
        print(" ", key.name[:8], cast.name, len(list(cast.iterdir())), "files", digest)
sys.exit(1 if any(codes) else 0)

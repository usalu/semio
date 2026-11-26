"""🧹 Retire superseded, unlocked units from this ticket's private compiler lanes."""
import fcntl
import os
import pathlib
import shutil
import subprocess
import time

lanes = [pathlib.Path(".cache/b/r/m/tmp").resolve() / lane for lane in ("flow-native-n25", "flow-plugin-n25", "flow-web-n25", "g3d-brep-fast-n25", "g3d-frozen-web-n25", "g3d-p-n25")]
open_files=subprocess.run(["lsof","-nP","-a","-c","cargo","-Fn"],capture_output=True,check=True).stdout.decode(errors="replace").splitlines()
units={pathlib.Path(line[1:]).parent for line in open_files if line.startswith("n/") and line.endswith("/.lock")}
total=count=0
for lane in lanes:
    locked={unit for unit in units if unit.is_relative_to(lane)}
    print("[DEBUG]",lane.name,"live units",len(locked),flush=True)
    if len(locked)<300: continue
    packages={unit.parent for unit in locked}
    if lane.name.startswith("g3d-"):
        packages.update(parent for target in lane.glob("**/debug/build") for parent in target.iterdir() if parent.is_dir())
    for package in sorted(packages):
        for unit in package.iterdir():
            if unit in locked or not unit.is_dir() or len(unit.name)!=16: continue
            lock=unit/".lock"
            if not lock.is_file() or time.time()-unit.stat().st_mtime<3600: continue
            with lock.open("rb") as handle:
                try: fcntl.flock(handle,fcntl.LOCK_EX|fcntl.LOCK_NB)
                except BlockingIOError: continue
                for child in unit.iterdir():
                    if child.is_dir() and child.name in ("out","fingerprint"):
                        total+=sum(f.stat().st_size for f in child.rglob("*") if f.is_file())
                        shutil.rmtree(child)
                count+=1
print("[DEBUG] Superseded private units and bytes",count,total)

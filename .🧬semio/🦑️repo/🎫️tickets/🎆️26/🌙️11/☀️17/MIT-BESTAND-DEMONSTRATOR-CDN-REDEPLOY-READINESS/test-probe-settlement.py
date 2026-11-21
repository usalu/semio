import os
from pathlib import Path
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as temporary:
    root = Path(temporary)
    local = root / "1/2/3/4/5/6/7"
    local.mkdir(parents=True)
    (root / ".cache/b/r/m/components/runs/wfixture/r").mkdir(parents=True)
    for name in ("native-lanes.sh", "probe-linked.sh"):
        (local / name).symlink_to(ticket / name)
    repo = root / "repo"
    repo.write_text("#!/bin/sh\necho 'runs/wfixture: selecting'\nexit 41\n")
    repo.chmod(0o755)
    env = dict(os.environ, PATH=str(root) + os.pathsep + os.environ["PATH"])
    lane = subprocess.run(["bash", local / "native-lanes.sh", "step", "browser-demonstrator", "", "fixture"], env=env, capture_output=True)
    assert lane.returncode == 41
    probe = subprocess.run(["bash", local / "probe-linked.sh", "demonstrator", "wfixture", "8798", "both"], env=env, capture_output=True, timeout=2)
    assert probe.returncode == 1 and b"ended without a probe input" in probe.stdout, probe
    print("Probe settled for a failed source selection without a build report.")

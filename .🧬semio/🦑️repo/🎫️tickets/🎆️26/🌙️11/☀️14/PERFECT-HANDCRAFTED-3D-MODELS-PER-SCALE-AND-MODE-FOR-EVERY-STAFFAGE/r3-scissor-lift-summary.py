"""📋 Short reading of the last product check: failing measurements, silhouette IoU and outer areas per region."""
import os

path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "r3-scissor-lift", "check.txt")
for line in open(path, encoding="utf-8"):
    if line.startswith("[check]"):
        continue
    if line.startswith(("ok", "FAIL ")) and "volumes-" in line:
        tokens = line.split()
        bad = [token for token in tokens if token.startswith("!")]
        extra = []
        if "thermal-coverage" in line:
            extra = [token.replace("info:outerAreaM2:", "A:")[:22] for token in tokens if token.startswith(("info:outerAreaM2", "thermal.areaDeviationTotal"))]
        if "silhouette" in line or "cross-scale" in line:
            extra = [token[:20] for token in tokens if token.startswith("iou:")]
        if bad or extra:
            print(line[:40].strip(), " ".join(bad), " ".join(extra))
    elif not line.startswith("ok"):
        print(line.strip()[:300])

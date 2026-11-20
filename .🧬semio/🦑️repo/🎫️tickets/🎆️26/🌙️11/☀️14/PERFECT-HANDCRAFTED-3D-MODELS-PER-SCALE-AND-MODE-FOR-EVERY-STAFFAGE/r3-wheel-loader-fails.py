"""🚦 Prints the failing measurements of a volumes-check output file, the silhouette numbers and the measured outer areas per region."""
import re, sys
for line in open(sys.argv[1]):
    if line.startswith(("FAIL", "ok")) and ("FAIL" in line[:4] or "silhouette" in line or "thermal-coverage" in line):
        parts = line.split()
        keep = [p for p in parts[4:] if p.startswith("!") or (line.startswith("ok") and "silhouette" in line and p.startswith(("iou", "p95")))]
        areas = [p.replace("info:outerAreaM2:", "") [:28] for p in parts if p.startswith("info:outerAreaM2:")] if "thermal" in line else []
        print(parts[0], parts[1], parts[2], " ".join(re.sub(r"(\.\d{4})\d+", r"\1", k) for k in keep), " ".join(re.sub(r"(\.\d{2})\d+", r"\1", a) for a in areas))
    elif line.startswith(("FAIL:", "PASS", "OK")):
        print(line.strip())

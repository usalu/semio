"""Outer extent of drawn and volume along scan lines: python3 vehicles-outer.py <asset> <scale> <view> <v|h> <from> <to> <step>"""
import subprocess, sys, re
a = sys.argv[1:]
out = subprocess.run(["./vehicles-run.sh", "scan", a[0], a[1], a[2], a[3], a[4], a[5], a[6], "0"], capture_output=True, text=True).stdout
for l in out.splitlines():
    m = re.match(r"(\S+) drawn: (.*) \| volume: (.*)", l)
    if not m: continue
    def ext(s):
        v = [float(x) for x in re.findall(r"-?\d+\.\d+", s)]
        return (min(v), max(v)) if v else None
    d, v = ext(m.group(2)), ext(m.group(3))
    print(m.group(1), "drawn", d, "volume", v)

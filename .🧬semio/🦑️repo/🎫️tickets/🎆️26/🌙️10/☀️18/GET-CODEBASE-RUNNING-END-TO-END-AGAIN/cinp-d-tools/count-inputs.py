"""python3 count-inputs.py <execlog> : per spawn the number of inputs and how many lie under a tests directory, a .feature or a stories file"""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)).replace("cinp-d-tools", "cwarm-d-tools"))
from execlog import spawns
for row in spawns(sys.argv[1]):
    paths = [i.get("path", "") for i in row.get("inputs", [])]
    tests = [p for p in paths if "/🧪️tests/" in p or p.endswith(".feature") or ".stories." in p]
    print(row.get("targetLabel"), "inputs", len(paths), "test-like", len(tests))

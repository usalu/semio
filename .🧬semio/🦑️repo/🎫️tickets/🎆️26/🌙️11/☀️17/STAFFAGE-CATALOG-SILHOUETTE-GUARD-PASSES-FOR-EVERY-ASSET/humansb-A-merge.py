"""🔗 Merges op files into one. python3 humansb-A-merge.py out.json in1.json in2.json ..."""
import json, sys
ops = []
for f in sys.argv[2:]: ops += json.load(open(f))
json.dump(ops, open(sys.argv[1], "w"))

"""🧷 Concatenates ops files. python3 humansb-A2-cat.py out.json in1.json in2.json ..."""
import json, sys
o = []
for f in sys.argv[2:]: o += json.load(open(f))
json.dump(o, open(sys.argv[1], 'w'))

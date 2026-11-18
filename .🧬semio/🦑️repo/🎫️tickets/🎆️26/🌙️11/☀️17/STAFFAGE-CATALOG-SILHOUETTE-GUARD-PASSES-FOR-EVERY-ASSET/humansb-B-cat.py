"""🧵 Concatenates op files. python3 humansb-B-cat.py out.json in1.json in2.json ..."""
import json, sys
json.dump([o for f in sys.argv[2:] for o in json.load(open(f))], open(sys.argv[1], "w"), indent=1, ensure_ascii=False)

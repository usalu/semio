import json, sys
t = open(sys.argv[1], encoding="utf8").read()
print(json.dumps(json.loads(t), ensure_ascii=False, indent=2) + "\n" == t)

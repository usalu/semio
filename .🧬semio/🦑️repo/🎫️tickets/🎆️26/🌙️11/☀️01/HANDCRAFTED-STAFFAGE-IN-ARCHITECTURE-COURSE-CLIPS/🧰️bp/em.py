import json, sys
a = json.load(open(sys.argv[1], encoding="utf8"))
def find(o, path=""):
    if isinstance(o, dict):
        for k, v in o.items():
            if k in ("emitters", "breathing", "respiration", "Respiration", "transport") : print(path + "/" + k, json.dumps(v, ensure_ascii=False)[:int(sys.argv[2]) if len(sys.argv)>2 else 1800])
            find(v, path + "/" + k)
    elif isinstance(o, list):
        for i, v in enumerate(o): find(v, path + f"[{i}]")
find(a)

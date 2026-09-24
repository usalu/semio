import json, subprocess, collections, os, sys
files = subprocess.run(["git","grep","-l","serde-json\\|serde_json","--","*/🔮️oracles/🔣️.json"],capture_output=True,text=True).stdout.split("\n")
for p in filter(None, files):
    d = json.load(open(p))
    hits = collections.Counter()
    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items(): walk(x, path + [k])
        elif isinstance(v, list):
            for x in v: walk(x, path + ["[]"])
        elif isinstance(v, str) and ("serde-json" in v or "serde_json" in v):
            hits["/".join(path)] += 1
    walk(d, [])
    fm = d.get("fixtureManifests", [])
    missing = 0
    for f in fm:
        if "serde" in json.dumps(f.get("generator", {})):
            for fl in f.get("files", []):
                if not os.path.exists(os.path.join(os.path.dirname(p), fl["path"])): missing += 1
    print("==", p.split("/🔌️plugins/")[1].split("/🔮️oracles")[0], "missing-files:", missing)
    for k, n in hits.most_common(): print(f"   {n:4d} {k}")

import os, json, hashlib, build
C = os.path.dirname(build.HERE); S = C + "/oracle2-g-tools/staged/files/"
hp = os.path.join(build.HERE, "base-hashes.json"); hashes = json.load(open(hp)); out = {}
for p in build.PATHS:
    if "🔭️read-oracle" not in p or not os.path.exists(S + p): continue
    key = hashlib.sha256(p.encode()).hexdigest()[:10]
    data = open(S + p, "rb").read()
    open(os.path.join(build.HERE, "base", key), "wb").write(data)
    hashes[p] = hashlib.sha256(data).hexdigest(); out[p] = hashes[p]
json.dump(hashes, open(hp, "w"), ensure_ascii=False, indent=1)
json.dump(out, open(os.path.join(build.HERE, "prereq-oracle2.json"), "w"), ensure_ascii=False, indent=1)
print(len(out), "files rebased onto oracle2-g staged")

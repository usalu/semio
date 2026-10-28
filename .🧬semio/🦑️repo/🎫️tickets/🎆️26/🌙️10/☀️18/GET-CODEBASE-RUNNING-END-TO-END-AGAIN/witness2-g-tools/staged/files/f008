"""🗻️ Independent model of the remote-file-system decision: shares no code with the TypeScript module, only the declared type lists of the schema and the vectors. Usage: python3 -I 🐍️.py <vectors.json> <schema.json> -> one JSON verdict (or null) per vector on stdout."""
import json, re, sys

def decide(vector, policy):
    platform, root = vector["platform"], vector["root"]
    def listed(rule, name):
        name = name.lower()
        return name in rule["exact"] or any(name.startswith(prefix) for prefix in rule["prefixes"])
    def inside(mount):
        return root.startswith("/") if mount == "/" else root == mount or root.startswith(mount + "/")
    if platform == "linux":
        best = None
        for line in vector.get("mountinfo", "").split("\n"):
            if " - " not in line: continue
            left, right = line.split(" - ", 1)
            fields, tail = left.split(" "), right.split(" ")
            if len(fields) < 5 or not tail[0]: continue
            mount = re.sub(r"\\([0-7]{3})", lambda m: chr(int(m.group(1), 8)), fields[4])
            if inside(mount) and (best is None or len(mount) >= len(best[0])): best = (mount, tail[0])
        return {"type": best[1], "mount": best[0]} if best and listed(policy["linux"], best[1]) else None
    if platform == "darwin":
        best = None
        firm = policy["darwinFirmlinkPrefix"]
        def plain(path):
            return "/" if path == firm else path[len(firm):] if path.startswith(firm + "/") else path
        place = plain(root)
        for line in vector.get("mountTable", "").split("\n"):
            m = re.match(r"^(.+?) on (.+) \(([^()]*)\)$", line.rstrip())
            if not m: continue
            parts = [p.strip() for p in m.group(3).split(",") if p.strip()]
            alias = plain(m.group(2))
            contained = place.startswith("/") if alias == "/" else place == alias or place.startswith(alias + "/")
            if not parts or not contained: continue
            if best is None or len(alias) >= len(plain(best[0])): best = (m.group(2), parts[0], parts[1:])
        if not best: return None
        mount, kind, flags = best
        remote = listed(policy["darwin"], kind) or (policy["darwinLocalFlag"] not in flags and kind.lower() not in policy["darwinLocalTypes"])
        return {"type": kind, "mount": mount} if remote else None
    if platform == "win32":
        for prefix in policy["windowsUncPrefixes"]:
            if root.startswith(prefix):
                head = re.split(r"[\\/]", root[len(prefix):])[:2]
                return {"type": "unc", "mount": prefix + prefix[0].join(head)}
        if vector.get("driveType", "").strip() == policy["windowsNetworkDriveType"]: return {"type": "network-drive", "mount": root[:2]}
        return None
    return None

vectors = json.load(open(sys.argv[1], encoding="utf8"))["vectors"]
policy = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Policy"]["const"]
json.dump([decide(v, policy) for v in vectors], sys.stdout)

import json, sys, re

def absolute(p): return p.startswith("/") or re.match(r"^[A-Za-z]:/", p) is not None

def norm(p):
    prefix = ""
    m = re.match(r"^([A-Za-z]:)(/.*)$", p)
    if m: prefix, p = m.group(1), m.group(2)
    out = []
    for part in p.split("/"):
        if part in ("", "."): continue
        if part == "..":
            if out: out.pop()
            continue
        out.append(part)
    return prefix + "/" + "/".join(out)

def dirname(p):
    i = p.rfind("/")
    return "/" if i <= 0 else p[:i]

def consulted(frm, request, fact):
    if not isinstance(fact, str): return None
    frm, request = frm.replace("\\", "/"), request.replace("\\", "/")
    if request.startswith("node:"): return [] if fact == request else None
    if not absolute(frm): return None
    rel = request in (".", "..") or request.startswith("./") or request.startswith("../")
    if not rel and not absolute(request): return None
    target = norm(dirname(frm) + "/" + request) if rel else norm(request)
    folder = dirname(target)
    if not fact.startswith("!"):
        a = fact.replace("\\", "/")
        if not absolute(a) or not a.startswith(target): return None
        if dirname(a) != folder and not a.startswith(target + "/"): return None
    return [target] if folder == target else [folder, target]

cases = json.load(open(sys.argv[1]))["cases"]
print(json.dumps([consulted(c["from"], c["request"], c["fact"]) for c in cases]))

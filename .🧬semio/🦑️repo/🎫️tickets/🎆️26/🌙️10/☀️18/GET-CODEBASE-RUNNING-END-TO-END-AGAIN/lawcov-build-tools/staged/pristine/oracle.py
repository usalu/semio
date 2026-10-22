"""🧊️ Independent Python oracle of the pristine registry restore (standard library only): from the clone's tree, the registry archive and the shared binary receipt it derives the refusal or the final tree (text, executable bit, link count of every file) and how many store rechecks the restore needs, with its own decision table for each file. Requests arrive as JSON on stdin."""
import hashlib, json, sys


def parents(paths):
    found = set()
    for path in paths:
        parts = path.split("/")[:-1]
        for index in range(1, len(parts) + 1):
            found.add("/".join(parts[:index]))
    return found


def disposition(path, links, archive, shared, matches):
    if path.startswith("node_modules/"):
        return "nested"
    if shared:
        if not archive or not matches or links < 2:
            raise Refusal("shared")
        return "preserve"
    if links != 1:
        raise Refusal("unknown")
    return "replace" if archive else "remove"


class Refusal(Exception):
    pass


def expand(case):
    nodes = list(case["clone"])
    if case["generate"]:
        nodes += [{"kind": "file", "path": "%s%04d.txt" % (case["generate"]["prefix"], i), "text": "g"} for i in range(case["generate"]["count"])]
    return nodes


def restore(case, root_length, limit, path_limit, window):
    archive = [e for e in case["archive"] if e["kind"] == "file"]
    if any(e["path"] == "node_modules" or e["path"].startswith("node_modules/") for e in case["archive"]):
        return {"error": "nested"}
    files = {e["path"]: e for e in archive}
    shared = {s["path"]: s for s in case["shared"]}
    nodes = expand(case)
    tree = {n["path"]: n for n in nodes}
    dirs = set()
    for n in nodes:
        dirs |= parents([n["path"]])
        if n["kind"] == "dir":
            dirs.add(n["path"])
    keep = parents(list(files))
    visited, doomed, removed_dirs = 0, [], set()
    try:
        def inspect(path):
            nonlocal visited
            visited += 1
            if visited > limit:
                raise Refusal("namespace")
            if root_length + 1 + len(path) > path_limit:
                raise Refusal("ceiling")
            if path == "node_modules":
                return
            if path in dirs:
                for child in sorted({p.split("/")[len(path.split("/"))] for p in list(tree) + list(dirs) if p.startswith(path + "/")}):
                    inspect(path + "/" + child)
                if path not in keep:
                    removed_dirs.add(path)
                return
            node = tree[path]
            if node["kind"] == "symlink":
                raise Refusal("shape")
            expected = files.get(path)
            known = shared.get(path)
            text = expected["text"].encode() if expected else b""
            matches = bool(known and expected and len(known["text"].encode()) == len(text) and hashlib.sha256(known["text"].encode()).hexdigest() == hashlib.sha256(text).hexdigest())
            links = 1 + (1 if node.get("shared") else 0) + node.get("extraLinks", 0)
            if disposition(path, links, bool(expected), bool(known), matches) == "remove":
                doomed.append(path)
        top = sorted({p.split("/")[0] for p in list(tree) + list(dirs)})
        for name in top:
            inspect(name)
        count, replaced, loop = visited, {}, 0
        for path, entry in files.items():
            if count % window == 0:
                loop += 1
            count += 1
            node = tree.get(path)
            if node is None:
                raise Refusal("missing")
            if node["kind"] != "file":
                raise Refusal("shape")
            known = shared.get(path)
            text = entry["text"].encode()
            matches = bool(known and len(known["text"].encode()) == len(text) and hashlib.sha256(known["text"].encode()).hexdigest() == hashlib.sha256(text).hexdigest())
            links = 1 + (1 if node.get("shared") else 0) + node.get("extraLinks", 0)
            if disposition(path, links, True, bool(known), matches) == "replace":
                replaced[path] = {"text": entry["text"], "exec": bool(entry["mode"] & 0o111), "links": 1}
            else:
                replaced[path] = {"text": node["text"], "exec": bool(node.get("exec")), "links": links}
    except Refusal as error:
        return {"error": str(error)}
    final = dict(replaced)
    for path, node in tree.items():
        if path.startswith("node_modules/") and node["kind"] == "file":
            final[path] = {"text": node["text"], "exec": bool(node.get("exec")), "links": 1 + (1 if node.get("shared") else 0) + node.get("extraLinks", 0)}
    return {"error": None, "tree": final, "rechecks": loop + 1, "publishes": loop}


request = json.load(sys.stdin)
print(json.dumps([restore(c, request["rootLength"], request["maximumEntries"], request["maximumPathBytes"], request["recheckEvery"]) for c in request["cases"]]))

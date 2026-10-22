"""🧱️ Python twin of the schema materializer (same rules as the TypeScript one, written separately); the oracle embeds this logic."""
import copy, json, re
TOKEN = re.compile(r"\{\{([A-Za-z0-9_.]+)(?:\|(re|lower))?\}\}")
def schema_view(d):
    primary = next(l for l in d["locks"] if l["role"] == "primary")
    r, inputs = d["recipe"], d["producerInputs"]
    stripped = lambda name: re.sub(r"\.[A-Za-z0-9]+$", "", re.sub(r"^\.", "", name))
    seen, label = [], []
    for name in inputs["ambientHomeFiles"] + inputs["ambientConfigFiles"]:
        s = stripped(name)
        if s not in seen:
            seen.append(s)
    view = dict(d)
    view.update({"lockFile": primary["name"], "scopeConfig": (d["configurations"]["scope"] or [""])[0],
                 "storeRoleFiles": {r["home"]: inputs["ambientHomeFiles"], r["config"]: inputs["ambientConfigFiles"]},
                 "producerFileLabel": " and ".join(seen),
                 "resolutionOracles": [d["resolution"]["oracle"], "typescript", "node", "node-missing", "none"],
                 "managerProtocol": {"manager": d["id"], "label": d["label"], "lockFile": primary["name"], "scopeConfig": (d["configurations"]["scope"] or [""])[0], "generationDirectory": d["generationDirectory"], "strictProfilePrefix": d["resolution"]["strictPrefix"] or "", "hostVersionKey": d["runtime"]["hostVersionKey"] or "", "cacheVariable": next((n for n, s in d["environment"]["set"].items() if s == "role:cache" and n != "XDG_CACHE_HOME"), "")}})
    return view
def lookup(view, path):
    value = view
    for key in path.split("."):
        if not isinstance(value, dict) or key not in value:
            raise KeyError(path)
        value = value[key]
    return value
def materialize(node, view):
    if isinstance(node, str):
        def one(m):
            value = lookup(view, m.group(1))
            if not isinstance(value, str):
                raise TypeError(m.group(1))
            return re.sub(r"[.*+?^${}()|\[\]\\]", lambda x: "\\" + x.group(0), value) if m.group(2) == "re" else value.lower() if m.group(2) == "lower" else value
        return TOKEN.sub(one, node)
    if isinstance(node, list):
        return [materialize(x, view) for x in node]
    if isinstance(node, dict):
        if "$manager" in node:
            value = copy.deepcopy(lookup(view, node["$manager"]))
            if "extend" not in node:
                return value
            if isinstance(value, list) and isinstance(node["extend"], list):
                return value + [materialize(x, view) for x in node["extend"]]
            if isinstance(value, dict) and isinstance(node["extend"], dict):
                value.update(materialize(node["extend"], view))
                return value
            raise TypeError(node["$manager"])
        return {k: materialize(v, view) for k, v in node.items()}
    return node

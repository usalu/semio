import re
# 🗺️ Monolith module names: from the root and the seven hubs, name -> (file | extern crate | crate re-export).
import glob, os, re
from common import *
from index import Tree, FileInfo

def module_map(tree=None):
    t = tree or Tree()
    res = {}
    files = [("🦀️.rs", "root")] + [("🪝️mounts/" + os.path.basename(os.path.dirname(h)) + "/🦀️.rs", os.path.basename(os.path.dirname(h))) for h in sorted(glob.glob(t.plug + "/🪝️mounts/*/🦀️.rs"))]
    for rel, hub in files:
        fi = t.info(rel)
        base = os.path.dirname(rel)
        for m in fi.mounts:
            if m["path"]:
                res[m["name"]] = dict(kind="file", file=os.path.normpath(os.path.join(base, m["path"])), hub=hub, vis=m["vis"], attrs=m["attrs"])
        for m in re.finditer(r"(?m)^(?P<vis>pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(?P<lib>semio_framework_plugin_\w+)\s+as\s+(?P<name>\w+)\s*;", fi.clean):
            res[m.group("name")] = dict(kind="extern", lib=m.group("lib"), hub=hub, vis=(m.group("vis") or "").strip())
        for u in fi.uses:
            if not u["glob"] and u["path"] and u["path"][0].startswith("semio_framework_plugin_"):
                name = u["alias"] or u["path"][-1]
                if len(u["path"]) >= 2:
                    res.setdefault(name, dict(kind="reexport", lib=u["path"][0], path=u["path"][1:], hub=hub, vis=u["vis"]))
    for rel, hub in files:
        fi = t.info(rel)
        clean = fi.clean
        d = 0
        for m in re.finditer(r"(?m)^(?P<vis>pub(?:\([^)]*\))?\s+)?mod\s+(?P<name>\w+)\s*\{", clean):
            if m.start() and clean[:m.start()].count("{") != clean[:m.start()].count("}"):
                continue
            depth, j = 1, m.end()
            while j < len(clean) and depth:
                depth += (clean[j] == "{") - (clean[j] == "}")
                j += 1
            body = fi.text[m.end():j - 1]
            uses = []
            from rs import blank, expand_use
            for um in re.finditer(r"(?P<attrs>(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);", blank(body)):
                for path, alias, isglob in expand_use(um.group("body")):
                    uses.append(dict(path=path, alias=alias, glob=isglob, attrs=body[um.start("attrs"):um.end("attrs")]))
            res[m.group("name")] = dict(kind="shim", hub=hub, vis=(m.group("vis") or "").strip(), uses=uses, file=rel)
    return res

if __name__ == "__main__":
    mm = module_map()
    import collections
    print(collections.Counter(v["kind"] for v in mm.values()), len(mm))
    for k in ("child_content", "tool_cancellation", "media_kit", "reserved_jobs"):
        print(k, mm.get(k))

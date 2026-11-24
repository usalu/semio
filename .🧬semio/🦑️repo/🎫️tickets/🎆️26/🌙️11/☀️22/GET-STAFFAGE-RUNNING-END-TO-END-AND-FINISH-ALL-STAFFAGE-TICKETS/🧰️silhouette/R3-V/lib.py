"""🧰️ Helpers of the ops writers of batch R3-V: reads the catalog document of an object and writes `ops/<id>.json`, one op per line."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"


def asset(slug):
    """📖️ The asset of the catalog document, or of the document `ASSET_FILE` names (the state before the repair, see pristine.sh)."""
    return json.load(open(os.environ.get("ASSET_FILE") or f"{CAT}/{slug}/🔣️.json"))["asset"]


def shape(a, scale, view, sid):
    """🔎️ A drawn shape by id."""
    return next(s for s in a["representations"][str(scale)]["geometry"][view] if s["id"] == sid)


def solid(a, scale, sid, lst="body"):
    """🔎️ A solid by id."""
    return next(s for s in a["volumes"][str(scale)][lst] if s["id"] == sid)


def write(slug, ops):
    """📤️ Writes the ops of an object."""
    os.makedirs(f"{HERE}/ops", exist_ok=True)
    open(f"{HERE}/ops/{slug}.json", "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")


def _at(lst, sid, what):
    for i, x in enumerate(lst):
        if x.get("id") == sid:
            return i
    raise KeyError(f'{what} "{sid}" not found')


def apply_ops(a, ops):
    """🧷️ Applies repair ops to an asset in place, with the meaning of `ops.ts` (`applyOps`): idempotent, a removal whose target is gone is skipped."""
    for o in ops:
        kind = o["op"]
        if kind.startswith("shape"):
            lst = a["representations"][o["scale"]]["geometry"][o["view"]]
            has = lambda sid: any(x.get("id") == sid for x in lst)
            if kind == "shape-add":
                if has(o["shape"]["id"]):
                    lst[_at(lst, o["shape"]["id"], "shape")] = o["shape"]
                elif "after" not in o:
                    lst.insert(0, o["shape"])
                elif has(o["after"]):
                    lst.insert(_at(lst, o["after"], "shape") + 1, o["shape"])
                else:
                    lst.append(o["shape"])
            elif kind == "shape-remove":
                if has(o["id"]):
                    del lst[_at(lst, o["id"], "shape")]
            elif kind == "shape":
                lst[_at(lst, o["id"], "shape")] = o["shape"]
            elif kind == "shape-set":
                lst[_at(lst, o["id"], "shape")].update(o["set"])
            else:
                raise ValueError(kind)
        elif kind.startswith("solid"):
            lst = a["volumes"][o["scale"]][o.get("list", "body")]
            has = lambda sid: any(x.get("id") == sid for x in lst)
            if kind == "solid-add":
                if has(o["solid"]["id"]):
                    lst[_at(lst, o["solid"]["id"], "solid")] = o["solid"]
                else:
                    lst.insert(_at(lst, o["after"], "solid") + 1 if "after" in o else len(lst), o["solid"])
            elif kind == "solid-remove":
                if has(o["id"]):
                    del lst[_at(lst, o["id"], "solid")]
            elif kind == "solid":
                lst[_at(lst, o["id"], "solid")] = o["solid"]
            elif kind == "solid-set":
                lst[_at(lst, o["id"], "solid")].update(o["set"])
            else:
                raise ValueError(kind)
        elif kind == "dims":
            a["dimensions"].update(o["set"])
        elif kind == "set":
            node = a
            step = lambda n, s: n[_at(n, s["id"], "item")] if isinstance(s, dict) else n[s]
            for s in o["path"][:-1]:
                node = step(node, s)
            last = o["path"][-1]
            if isinstance(last, dict):
                node[_at(node, last["id"], "item")] = o["value"]
            elif "value" not in o:
                node.pop(last, None)
            else:
                node[last] = o["value"]
        else:
            raise ValueError(kind)


def repair_file(path, slug):
    """🩹️ The last step of the generator of an object: applies `ops/<slug>.json` of the silhouette batch R3-V to the document the generator wrote, in the indentation it wrote."""
    text = open(path).read()
    second = text.split("\n", 2)[1] if "\n" in text.rstrip("\n") else ""
    indent = len(second) - len(second.lstrip())
    doc = json.loads(text)
    ops = json.load(open(f"{HERE}/ops/{slug}.json"))
    apply_ops(doc["asset"], ops)
    one_line = "\n" not in text.rstrip("\n")
    open(path, "w").write((json.dumps(doc, ensure_ascii=False, separators=(",", ":")) if one_line else json.dumps(doc, ensure_ascii=False, indent=indent)) + ("\n" if text.endswith("\n") else ""))
    print(f"[DEBUG] {slug}: {len(ops)} silhouette repair ops of batch R3-V applied")

"""🧷️ The last step of the people of SH-2 that have no generator chain: applies the repair ops `ops/<slug>.json` of this folder (the op format of `ops.ts`; every op is idempotent) to the catalog file and rewrites only the text spans of the values it changes, so every untouched byte of the hand-laid-out file stays. `python3 apply.py <slug>… [--ops <file>] [--file <asset.json>]`"""
import json
import os
import sys

import sh2_doc

HERE = os.path.dirname(os.path.abspath(__file__))


def index_of(items, ident):
    """🔎️ The index of the item with an id, or -1."""
    return next((i for i, item in enumerate(items) if item.get("id") == ident), -1)


def apply(doc, ops):
    """🧷️ Applies the ops in order; returns how many changed the document and how many were already in place."""
    changed = skipped = 0
    for number, op in enumerate(ops):
        kind = op["op"]
        if kind.startswith("shape") or kind.startswith("solid"):
            base = ("asset", "representations", op["scale"], "geometry", op["view"]) if kind.startswith("shape") else ("asset", "volumes", op["scale"], op.get("list", "body"))
            word = kind[:5]
            items = doc.get(*base)
            if kind.endswith("-add"):
                new = op[word]
                at = index_of(items, new["id"])
                if at >= 0:
                    if items[at] == new:
                        skipped += 1
                    else:
                        doc.set(base + (at,), new)
                        changed += 1
                    continue
                after = index_of(items, op["after"]) if op.get("after") is not None else (-1 if word == "shape" else len(items) - 1)
                if op.get("after") is not None and after < 0:
                    after = len(items) - 1
                doc.insert(base, after + 1, new)
                changed += 1
                continue
            at = index_of(items, op["id"])
            if at < 0:
                later = any(o["op"] == f"{word}-remove" and o.get("scale") == op["scale"] and o.get("view") == op.get("view") and o.get("id") == op["id"] for o in ops[number + 1:])
                if kind.endswith("-remove") or later:
                    skipped += 1
                    continue
                raise SystemExit(f"{kind} {op['scale']} {op.get('view', op.get('list', 'body'))} {op['id']}: not found")
            if kind.endswith("-remove"):
                doc.remove(base, at)
                changed += 1
            elif kind.endswith("-set"):
                new = {**items[at], **op["set"]}
                if new == items[at]:
                    skipped += 1
                else:
                    doc.set(base + (at,), new)
                    changed += 1
            elif items[at] == op[word]:
                skipped += 1
            else:
                doc.set(base + (at,), op[word])
                changed += 1
        elif kind == "region-set":
            regions = doc.get("asset", "thermal", "regions")
            at = index_of(regions, op["id"])
            new = {**regions[at], **op["set"]}
            if new == regions[at]:
                skipped += 1
            else:
                doc.set(("asset", "thermal", "regions", at), new)
                changed += 1
        elif kind == "dims":
            new = {**doc.get("asset", "dimensions"), **op["set"]}
            if new == doc.get("asset", "dimensions"):
                skipped += 1
            else:
                doc.set(("asset", "dimensions"), new)
                changed += 1
        elif kind == "set":
            path = ["asset"]
            for step in op["path"]:
                path.append(index_of(doc.get(*path), step["id"]) if isinstance(step, dict) else step)
            if doc.get(*path) == op["value"]:
                skipped += 1
            else:
                doc.set(tuple(path), op["value"])
                changed += 1
        else:
            raise SystemExit(f"unknown op {kind}")
    return changed, skipped


def main(argv):
    """🏁️ Applies the ops of every named person."""
    slugs, ops_file, asset_file = [], None, None
    it = iter(argv)
    for arg in it:
        if arg == "--ops":
            ops_file = next(it)
        elif arg == "--file":
            asset_file = next(it)
        else:
            slugs.append(arg)
    for slug in slugs:
        ops = json.load(open(ops_file or os.path.join(HERE, "ops", f"{slug}.json"), encoding="utf8"))
        doc = sh2_doc.Doc(asset_file or sh2_doc.catalog_file(slug))
        changed, skipped = apply(doc, ops)
        doc.save()
        print(f"{slug}: {len(ops)} ops, {changed} applied, {skipped} already in place")


if __name__ == "__main__":
    main(sys.argv[1:])

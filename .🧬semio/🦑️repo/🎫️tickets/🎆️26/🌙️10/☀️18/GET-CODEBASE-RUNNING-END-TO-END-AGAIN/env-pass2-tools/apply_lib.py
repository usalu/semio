import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
FROZEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PATCHES = os.path.join(HERE, "staged", "patches")


def patched(rel, edits):
    text = open(os.path.join(ROOT, rel), encoding="utf-8").read()
    for edit in sorted((e for e in edits if "start" in e), key=lambda e: e["start"], reverse=True):
        start, end = edit["start"], edit["end"]
        if text[start:end] == edit["old"]:
            text = text[:start] + edit["new"] + text[end:]
        elif text[start:start + len(edit["new"])] != edit["new"]:
            raise SystemExit("span mismatch in %s at %d" % (rel, start))
    for edit in edits:
        if "start" in edit:
            continue
        have = text.count(edit["old"])
        if have == edit["count"]:
            text = text.replace(edit["old"], edit["new"])
        elif have == 0 and text.count(edit["new"]) >= edit["count"]:
            pass
        else:
            raise SystemExit("anchor mismatch (%d != %d) in %s: %r" % (have, edit["count"], rel, edit["old"][:80]))
    return text


def drain_direct():
    done = []
    for name in sorted(os.listdir(PATCHES)):
        if not name.endswith(".json"):
            continue
        path = os.path.join(PATCHES, name)
        specs = json.load(open(path, encoding="utf-8"))
        keep = []
        for spec in specs:
            if spec["file"].startswith(FROZEN):
                keep.append(spec)
                continue
            text = patched(spec["file"], spec["edits"])
            open(os.path.join(ROOT, spec["file"]), "w", encoding="utf-8").write(text)
            done.append(spec["file"])
        json.dump(keep, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return done


if __name__ == "__main__":
    for f in drain_direct():
        print("direct", f[-100:])

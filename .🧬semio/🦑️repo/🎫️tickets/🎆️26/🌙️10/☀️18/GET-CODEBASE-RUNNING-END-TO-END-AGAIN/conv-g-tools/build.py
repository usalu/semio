import sys, os, json, hashlib, shutil, importlib, re
import lib
HERE = os.path.dirname(os.path.abspath(__file__)); W = "/Users/ueli/Documents/semio/"
sys.path.insert(0, HERE)
PATHS = [p for p in open(os.path.join(HERE, "files.txt"), encoding="utf8").read().split("\n") if p]
sha = lambda b: hashlib.sha256(b).hexdigest()
def resolve(alias):
    hits = [p for p in PATHS if alias in p]
    assert len(hits) == 1, (alias, hits)
    return hits[0]
def apply_ops(path, text, ops):
    lines = text.split("\n")
    for ln, emoji in sorted(ops.get("emo", {}).items()):
        raw = lines[ln - 1]
        m = re.search(r"(/\*\*+\s*|///\s*|//!\s*|^\s*\*\s*)", raw)
        i = m.end() if m else 0
        if emoji.endswith("+"):
            lines[ln - 1] = raw[:i] + emoji[:-1].replace("\ufe0f", "") + "\ufe0f " + raw[i:]
            continue
        g = lib.first_grapheme(raw[i:])
        assert lib.MARK.match(raw[i:]), (path, ln, raw[:80])
        lines[ln - 1] = raw[:i] + emoji + "️" * (0 if emoji.endswith("️") else 1) + raw[i + len(g):]
    text = "\n".join(lines)
    for old, new, *cnt in ops.get("sub", []):
        n = cnt[0] if cnt else 1
        assert text.count(old) == n, (path, "sub", old[:80], text.count(old))
        text = text.replace(old, new)
    for anchor, doc, *occ in ops.get("doc", []):
        lines = text.split("\n")
        idx = [i for i, l in enumerate(lines) if l.lstrip().startswith(anchor)]
        k = occ[0] if occ else 0
        assert len(idx) > k and (len(idx) == 1 or occ), (path, "doc anchor", anchor[:80], len(idx))
        i = idx[k]
        prev = lines[i - 1].rstrip() if i else ""
        assert not prev.endswith("*/"), (path, "already documented", anchor[:80])
        pad = lines[i][:len(lines[i]) - len(lines[i].lstrip())]
        tail = "" if doc.endswith("️") else ""
        lines.insert(i, f"{pad}/** {doc} */")
        text = "\n".join(lines)
    return text
def main():
    ops_mod = importlib.import_module("ops"); importlib.reload(ops_mod)
    hashes = json.load(open(os.path.join(HERE, "base-hashes.json"))) if os.path.exists(os.path.join(HERE, "base-hashes.json")) else {}
    report = []
    for alias, ops in ops_mod.OPS.items():
        path = resolve(alias); key = sha(path.encode())[:10]
        bp = os.path.join(HERE, "base", key)
        live = open(W + path, "rb").read()
        if not os.path.exists(bp):
            open(bp, "wb").write(live); hashes[path] = sha(live)
        base = open(bp, "rb").read()
        hashes.setdefault(path, sha(base))
        if sha(live) != hashes[path]: report.append(f"LIVE-CHANGED {alias}")
        new = apply_ops(path, base.decode("utf8"), ops)
        open(os.path.join(HERE, "new", key), "w", encoding="utf8").write(new)
        r = lib.reuse(path, new)
        if r: report.append(f"STILL-DUP {alias} {r}")
        for ln, c in lib.openers(path, new):
            if not lib.MARK.match(c): report.append(f"NOT-EMOJI {alias}:{ln} {c[:40]}")
    for path, content in getattr(ops_mod, "NEW", {}).items():
        key = sha(path.encode())[:10]
        assert not os.path.exists(W + path) or open(W + path, encoding="utf8").read() == content or hashes.get(path) == "absent", ("new file exists", path)
        hashes[path] = "absent"
        open(os.path.join(HERE, "new", key), "w", encoding="utf8").write(content)
    json.dump(hashes, open(os.path.join(HERE, "base-hashes.json"), "w"), ensure_ascii=False, indent=1)
    print("\n".join(report) or "ok", len(ops_mod.OPS), "files")
if __name__ == "__main__": main()

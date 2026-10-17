"""🧱️ Edits the unowned-parser registry keeping its exact serialization. usage: from tt6_unowned import add, remove"""
import json
F = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧱️unowned/🔣️.json"
def load():
    raw = open(F, encoding="utf8", newline="").read()
    return raw, json.loads(raw)
def dump(d):
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"
def check_roundtrip():
    raw, d = load()
    return dump(d) == raw
def save(d):
    open(F, "w", encoding="utf8", newline="").write(dump(d))
def add(path, name, reason):
    raw, d = load()
    assert not any(r["path"] == path and r["name"] == name for r in d["parsers"]), "already listed"
    d["parsers"].append({"path": path, "name": name, "reason": reason})
    d["parsers"].sort(key=lambda r: (r["path"].encode("utf8"), r["name"].encode("utf8")))
    save(d)
def remove(path, name):
    raw, d = load()
    n = len(d["parsers"])
    d["parsers"] = [r for r in d["parsers"] if not (r["path"] == path and r["name"] == name)]
    assert len(d["parsers"]) == n - 1
    save(d)

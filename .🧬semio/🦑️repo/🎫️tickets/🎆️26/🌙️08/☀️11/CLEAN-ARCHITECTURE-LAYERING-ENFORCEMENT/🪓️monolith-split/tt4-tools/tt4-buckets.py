"""📊 Bucket a tsc diagnostics file by error code and by workspace area; prints markdown tables."""
import collections, os, re, sys

sys.path.insert(0, os.path.dirname(__file__))
from importlib import import_module

blocks = import_module("tt3-blocks".replace("-", "_")) if False else None

def read(path):
    out = []
    for line in open(path, encoding="utf8").read().split("\n"):
        m = re.match(r"(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)", line)
        if m:
            out.append((m.group(1), m.group(4), m.group(5), line))
    return out

def area(path):
    p = path
    fw = "🧰️framework/"
    if p.startswith(fw):
        p = p[len(fw):]
        parts = p.split("/")
        if p.startswith("🛍️products/🦑️repo/🔨️modules/"):
            return "repo/" + ("library" if "📚️library" in p else "other-modules")
        if p.startswith("🛍️products/💻️os/"):
            return "os/" + (parts[3] if len(parts) > 3 and parts[2] == "🔨️modules" else parts[2])
        if p.startswith("🛍️products/"):
            return "product/" + parts[1]
        return "framework/" + "/".join(parts[:2])
    if p.startswith("✏️s/🔌️plugins/"):
        return "plugin/" + p.split("/")[2]
    if p.startswith("✏️s/"):
        return "dev-or-tests/" + "/".join(p.split("/")[1:3])
    return p.split("/")[0]

if __name__ == "__main__":
    rows = read(sys.argv[1])
    by_code = collections.Counter(r[1] for r in rows)
    by_area = collections.Counter(area(r[0]) for r in rows)
    print("| Code | Count |\n| --- | --- |")
    for k, v in by_code.most_common(15): print(f"| {k} | {v} |")
    print("\n| Area | Count |\n| --- | --- |")
    for k, v in by_area.most_common(30): print(f"| {k} | {v} |")
    print(f"\ntotal {len(rows)}")

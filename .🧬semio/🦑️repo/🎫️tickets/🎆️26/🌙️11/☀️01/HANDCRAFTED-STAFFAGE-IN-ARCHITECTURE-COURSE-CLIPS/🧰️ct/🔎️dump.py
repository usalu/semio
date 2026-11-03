"""🔎️ Dump staffage call sites and nearby string literals of the modified clip files, plus catalog titles of their casts."""
import ast
import json
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio")
TICKET = ROOT / ".🧬semio" / "🦑️repo" / "🎫️tickets" / "🎆️26" / "🌙️11" / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
GEN = TICKET / "🗑️generated" / "ct"
STAFF = ROOT / "🎓️teaching" / "🏛️architecture" / "🧍️staffage"
CALLS = {"staffage", "staffage_motion", "fitted_staffage", "pendant_lamp", "staffage_height"}
NEAR = 30


def catalog_titles():
    out = {}
    for doc in (STAFF / "🗂️catalog").rglob("🔣️.json"):
        data = json.loads(doc.read_text(encoding="utf-8"))
        if "asset" in data:
            a = data["asset"]
            out[a["id"]] = (a["category"], a["title"].get("en"), a["title"].get("de"))
    return out


def casts():
    m = json.loads((ROOT / "🎓️teaching" / "🏛️architecture" / "⚡️energy" / "🏗️authoring" / "🎨️visuals" / "🧍️staffage" / "🔣️.json").read_text(encoding="utf-8"))
    return {k: v.get("asset") for k, v in m["casts"].items()}


def main():
    titles = catalog_titles()
    cmap = casts()
    files = [l.strip() for l in (GEN / "staffage-files.txt").read_text(encoding="utf-8").splitlines() if l.strip()]
    out = []
    for rel in files:
        src = (ROOT / rel).read_text(encoding="utf-8")
        tree = ast.parse(src)
        calls = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call):
                name = node.func.id if isinstance(node.func, ast.Name) else getattr(node.func, "attr", "")
                if name in CALLS:
                    cid = None
                    if node.args and isinstance(node.args[0], ast.Constant) and isinstance(node.args[0].value, str):
                        cid = node.args[0].value
                    calls.append((node.lineno, name, cid))
        lines_call = [c[0] for c in calls]
        strs = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str) and any(ch.isalpha() for ch in node.value):
                near = any(abs(node.lineno - l) <= NEAR for l in lines_call)
                strs.append((node.lineno, near, node.value))
        strs.sort()
        out.append(f"## {rel}")
        cids = sorted({c[2] for c in calls if c[2]})
        for ln, name, cid in sorted(calls, key=lambda c: c[0]):
            asset = cmap.get(cid)
            t = titles.get(asset, ("?", "?", "?")) if asset else ("?", "?", "?")
            out.append(f"CALL L{ln} {name}({cid!r}) asset={asset} en={t[1]!r} de={t[2]!r}")
        for ln, near, v in strs:
            if near:
                out.append(f"  L{ln} {repr(v)[:160]}")
        out.append("")
    (GEN / "dump.txt").write_text("\n".join(out), encoding="utf-8")
    print(len(out), "lines")


if __name__ == "__main__":
    sys.exit(main())

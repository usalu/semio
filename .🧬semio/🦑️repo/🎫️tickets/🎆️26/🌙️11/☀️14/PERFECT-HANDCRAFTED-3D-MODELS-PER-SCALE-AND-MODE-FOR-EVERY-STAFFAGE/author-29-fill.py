"""Sets `filled: true` on closed body shapes drawn as outline only (2D defect, C17-like repair at the root) by a textual patch that leaves every other byte of the asset file alone.
Usage: python3 author-29-fill.py <asset 🔣️.json> [--apply]   (prints the patched shapes; reads the closed unfilled body shapes itself, `--only scale:view:id,...` limits)"""
import json, re, sys
sys.path.insert(0, ".")

def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced")

class vi:
    member_end = staticmethod(member_end)

def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    f = args[0]
    only = None
    if "--only" in sys.argv:
        only = set(sys.argv[sys.argv.index("--only") + 1].split(","))
        args = [a for a in args if a != sys.argv[sys.argv.index("--only") + 1]]
    text = open(f, encoding="utf-8").read()
    asset = json.loads(text)["asset"]
    todo = []
    for sc, r in asset["representations"].items():
        if r.get("status") != "authored": continue
        for v in ("elevation", "front", "plan"):
            for s in r["geometry"][v]:
                if s.get("role") == "body" and s.get("filled") is False and s.get("closed", True) is not False:
                    key = f"{sc}:{v}:{s['id']}"
                    if only is None or key in only: todo.append((sc, v, s["id"]))
    print(len(todo), "shapes:", ", ".join(f"{a}:{b}:{c}" for a, b, c in todo))
    if "--apply" not in sys.argv: return
    for sc, v, sid in todo:
        text = open(f, encoding="utf-8").read() if False else text
        m = re.search(r'\n(\s*)"representations"\s*:\s*', text)
        rs, re_ = m.start() + 1, vi.member_end(text, m.end())
        m2 = re.compile(r'\n\s*"%s"\s*:\s*' % sc).search(text, rs, re_)
        s0, s1 = m2.start() + 1, vi.member_end(text, m2.end())
        m3 = re.compile(r'\n\s*"geometry"\s*:\s*').search(text, s0, s1)
        g0, g1 = m3.start() + 1, vi.member_end(text, m3.end())
        m4 = re.compile(r'\n\s*"%s"\s*:\s*' % v).search(text, g0, g1)
        v0, v1 = m4.start() + 1, vi.member_end(text, m4.end())
        pos = text.find('"id": "%s"' % sid, v0, v1)
        assert pos >= 0, (sc, v, sid)
        start = text.rfind("{", v0, pos)
        end = vi.member_end(text, start)
        obj = text[start:end]
        new = obj.replace('"filled": false', '"filled": true', 1)
        assert new != obj, (sc, v, sid)
        text = text[:start] + new + text[end:]
    open(f, "w", encoding="utf-8").write(text)
    json.loads(text)
main()

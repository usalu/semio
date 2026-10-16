# 🔓️ Auto-widening from oracle diagnostics: E0603 / E0616 / E0624 / E0451 name the private definition; widen exactly that declaration (or the private import that hides it) to `pub` in OUT.
import json, os, re, sys, shutil
from common import *

PLUGREL = PLUG_REL
CODES = {"E0603", "E0616", "E0624", "E0451", "E0364", "E0365"}


def rel_of(sb_path, sb_root):
    p = sb_path.replace("/📦️packages/🦀️rust/../../", "/")
    i = p.find("/" + PLUGREL + "/")
    return p[i + len(PLUGREL) + 2:] if i >= 0 else None


def definition_site(d):
    """-> (file, line, kind) of the declaration to widen, or None"""
    notes = [c for c in d.get("children", []) if c.get("spans")]
    if not notes:
        return None
    first = notes[0]["spans"][0]
    last = notes[-1]["spans"][-1]
    ft = (first.get("text") or [{}])[0].get("text", "")
    if len(notes) >= 2 and re.match(r"\s*(pub(\([^)]*\))?\s+)?use\b", ft) and not re.match(r"\s*pub\s+use\b", ft):
        return (first["file_name"], first["line_start"], "import")
    if len(notes) >= 2 and re.match(r"\s*pub\(", ft) and re.match(r"\s*pub\([^)]*\)\s+use\b", ft):
        return (first["file_name"], first["line_start"], "import")
    return (last["file_name"], last["line_start"], "def")


VIS = re.compile(r"^(\s*(?:#\[[^\]]*\]\s*)*)(pub(?:\s*\([^)]*\))?\s+)?")


def widen_line(line):
    m = VIS.match(line)
    if not m:
        return line
    cur = (m.group(2) or "").strip()
    if cur == "pub":
        return line
    return line[:m.end(1)] + "pub " + line[m.end():]


def run(log_path, sb_root, out_root, live_plug, tag=""):
    seen = set(); changed = []
    for raw in open(log_path, encoding="utf8", errors="ignore"):
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            d = json.loads(raw)
        except Exception:
            continue
        code = (d.get("code") or {}).get("code")
        if d.get("level") != "error" or code not in CODES:
            continue
        site = definition_site(d)
        if not site:
            continue
        f, line, kind = site
        rel = rel_of(f, sb_root)
        if not rel or (rel, line) in seen:
            continue
        seen.add((rel, line))
        out_p = out_root + "/" + rel
        if not os.path.exists(out_p):
            src = live_plug + "/" + rel
            if not os.path.exists(src):
                continue
            os.makedirs(os.path.dirname(out_p), exist_ok=True)
            shutil.copy(src, out_p)
        lines = open(out_p, encoding="utf8").read().split("\n")
        if line - 1 >= len(lines):
            continue
        new = widen_line(lines[line - 1])
        if new != lines[line - 1]:
            lines[line - 1] = new
            open(out_p, "w", encoding="utf8").write("\n".join(lines))
            changed.append((rel, line, kind))
    return changed


if __name__ == "__main__":
    log, sb, out, live = sys.argv[1:5]
    for c in run(log, sb, out, live):
        print(c)

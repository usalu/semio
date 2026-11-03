import json, re, subprocess, sys, glob, os

REPO = "C:/git/semio"
T = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
A = "🎓️teaching/🏛️architecture"
S = A + "/🧍️staffage/"
V = A + "/⚡️energy/🏗️authoring/🎨️visuals/"
PY = A + "/⚡️energy/🏗️authoring/📦️packages/🐍️python/"
KW = re.compile(r"staffage|fitted_staffage|staffage_motion|staffage_height|staffage_parts|person_glyph|object_silhouette|radiator\(|silhouette", re.I)


RENAME = re.compile(r"$^")
EXPLICIT = [
    ("⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🏃️activity/", "r2-tt-report.md (activity dir removed)"),
    ("⚡️energy/🏗️authoring/🎬️clip/👥️audience/🧪️tests/", "r2-fx-report.md"),
    ("⚡️energy/🏗️authoring/🎬️clip/📁️output/🧪️tests/", "r2-fx-report.md"),
    ("⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/", "r2-fx-report.md"),
    ("⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🧪️tests/", "r2-pt-report.md"),
]


def git(*args, raw=False):
    r = subprocess.run(["git", "-c", "core.quotepath=false", *args], cwd=REPO, capture_output=True)
    return r.stdout if raw else r.stdout.decode("utf-8", "replace")


def changed():
    out = git("status", "--porcelain", "-uall", "-z", "--", "🎓️teaching", raw=True).decode("utf-8", "replace")
    res = {}
    parts = out.split("\0")
    i = 0
    while i < len(parts):
        e = parts[i]
        i += 1
        if len(e) < 4:
            continue
        x, y, p = e[0], e[1], e[3:]
        if x in "RC":
            i += 1
        st = "??" if x + y == "??" else ("D" if "D" in (x + y) else ("A" if x == "A" else "M"))
        res[p] = (x + y, st)
    return res


def v2_sets():
    txt = open(f"{REPO}/{T}/r2-changed-files-v2.md", encoding="utf-8").read()
    sec = None
    sets = {"Created": set(), "Modified": set(), "Deleted": set()}
    for line in txt.splitlines():
        m = re.match(r"## (Created|Modified|Deleted|Uncertain)", line)
        if m:
            sec = m.group(1)
            continue
        m = re.match(r"- `([^`]+)`", line)
        if m and sec in sets:
            sets[sec].add(m.group(1))
    return sets


def mentions():
    toks = {}
    files = []
    for pat in ("*report*.md", "migration-*", "audit-*"):
        files += glob.glob(f"{REPO}/{T}/{pat}")
    for f in files:
        if os.path.isdir(f):
            continue
        try:
            txt = open(f, encoding="utf-8").read()
        except Exception:
            continue
        for tok in re.findall(r"`([^`\n]{4,260})`", txt):
            tok = tok.strip().strip("/").replace("\\", "/")
            if "/" in tok and not tok.startswith(("http", "-")):
                toks.setdefault(tok, set()).add(os.path.basename(f))
    return toks


def content(p, st):
    if st == "D":
        return git("show", f"HEAD:{p}")
    if st in ("??", "A") and not git("ls-tree", "HEAD", "--", p).strip():
        try:
            return open(f"{REPO}/{p}", encoding="utf-8", errors="replace").read()
        except Exception:
            return ""
    d = git("diff", "HEAD", "--", p)
    return "\n".join(l for l in d.splitlines() if l[:1] in "+-" and l[:3] not in ("+++", "---"))


def main():
    ch = changed()
    v2 = v2_sets()
    v2all = v2["Created"] | v2["Modified"] | v2["Deleted"]
    toks = mentions()
    dirtoks = {t: r for t, r in toks.items() if len(t.split("/")) >= 3}
    res = {"created": [], "modified": [], "deleted": []}
    excl, unc, why = [], [], {}
    for p, (code, st) in sorted(ch.items()):
        kind = "deleted" if st == "D" else ("created" if st in ("??",) else "modified")
        if st == "A" and not git("ls-tree", "HEAD", "--", p).strip():
            kind = "created"
        reason = None
        if p in v2all:
            reason = "v2 report-attributed"
        elif p.startswith(S):
            reason = "staffage archive"
        elif p.startswith(V):
            reason = "visuals module"
        elif p.startswith(PY) and p.endswith(("pyproject.toml", "uv.lock")):
            reason = "python package pyproject/uv.lock"
        elif p.startswith("🎓️teaching/") and not p.startswith(A + "/"):
            pass
        if not reason:
            c = content(p, st)
            hits = sorted(set(m.group(0).lower() for m in KW.finditer(c)))
            if hits:
                reason = "content: " + ",".join(hits)
        if not reason:
            for pre, rep in EXPLICIT:
                if (A + "/" + pre) in p:
                    reason = "report names path: " + rep
            if not reason and p.endswith("🧪️tests/🧭️cutter-winding.py"):
                reason = "report names path: r2-br-report.md"
            if not reason and not RENAME.search(c.replace("surface_finishes", "")) is None:
                pass
        if reason:
            res[kind].append(p)
            why[p] = reason
        else:
            area = p[len(A) + 1:].split("/")[0] if p.startswith(A + "/") else p.split("/")[0]
            excl.append((p, st, area))
    json.dump(res, open(f"{REPO}/{T}/🧰️cf3/files.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    json.dump({"why": why, "excluded": excl}, open(f"{REPO}/{T}/🧰️cf3/why.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print({k: len(v) for k, v in res.items()}, "excluded", len(excl), "total", len(ch))
    from collections import Counter
    pass
    for p, st, a in excl:
        pass


main()

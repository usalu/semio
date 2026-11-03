#!/usr/bin/env python3
"""🧾 Match source paths named in ticket reports against git status and write r2-changed-files-v2.md (read-only)."""
import glob
import os
import re
import subprocess
from collections import defaultdict

REPO = "C:/git/semio"
T = os.path.join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS")
GEN = os.path.join(T, "🗑️generated/cf2")
OUT = os.path.join(T, "r2-changed-files-v2.md")
TOKEN = re.compile(r"[^\s`|()\"'<>,;\[\]{}]+")
OTHER_AREAS = ("🏗️bim/", "🏢️nakagin/", "🎯️optimization/", "🦗️grasshopper/", "📊️excel/")


def git(*args):
    r = subprocess.run(["git", "-c", "core.quotepath=false", *args], cwd=REPO, capture_output=True, encoding="utf-8")
    return r.stdout


status = {}
for line in git("status", "--porcelain", "--untracked-files=all").splitlines():
    xy, path = line[:2], line[3:]
    if " -> " in path:
        old, path = path.split(" -> ")
        if old.startswith("🎓️teaching/🏛️architecture/"):
            status[old] = "D"
    if "🧬semio/" in path or "ðŸ§¬" in path or not path.startswith("🎓️teaching/🏛️architecture/"):
        continue
    if xy in ("??", "A ", "AM", "R "):
        status[path] = "??"
    elif "D" in xy:
        status[path] = "D"
    else:
        status[path] = "M"
src = sorted(status)
dirs_all = {os.path.dirname(p) for p in src}
indexed = git("ls-files", "--", "🎓️teaching/🏛️architecture").splitlines()

reports = sorted(set(glob.glob(os.path.join(T, "*-report.md")) + glob.glob(os.path.join(T, "migration-*"))))
named = defaultdict(set)
for rp in reports:
    base = os.path.basename(rp)
    with open(rp, encoding="utf-8") as f:
        text = f.read()
    for tok in TOKEN.findall(text):
        if tok.count("/") < 1 or any(x in tok for x in ("*", "…", "...", "http")):
            continue
        t = re.sub(r":\d+(-\d+)?$", "", tok).strip(".:").lstrip("./")
        if t.count("/") >= 1 and len(t) >= 8:
            named[t].add(base)

matched = {}
ambiguous = {}
dir_hits = defaultdict(set)
named_unchanged = {}
for t, refs in sorted(named.items()):
    hits = [p for p in src if p == t or p.endswith("/" + t)]
    if len(hits) == 1:
        matched.setdefault(hits[0], set()).update(refs)
        continue
    if len(hits) > 1:
        ambiguous[t] = (hits, refs)
        continue
    sub = [p for p in src if p.startswith(t.rstrip("/") + "/")]
    if sub and t.count("/") >= 3:
        for p in sub:
            dir_hits[p].update(refs)
        continue
    if any(p == t or p.endswith("/" + t) for p in indexed):
        named_unchanged[t] = refs

created = [p for p in src if status[p] == "??" and p in matched]
modified = [p for p in src if status[p] == "M" and p in matched]
deleted = [p for p in src if status[p] == "D" and p in matched]


def reason(p):
    if p in dir_hits:
        return "under directory named in " + ", ".join(sorted(dir_hits[p])) + "; no exact path"
    amb = [t for t, (hits, _) in ambiguous.items() if p in hits]
    if amb:
        return "short name `" + amb[0] + "` is ambiguous across " + str(len(ambiguous[amb[0]][0])) + " files"
    if p.startswith(OTHER_AREAS) or any(a in p for a in OTHER_AREAS):
        return "other area " + next(a for a in OTHER_AREAS if a in p).rstrip("/") + "; not named by path in any report"
    return "not named by path in any report"


uncertain = [p for p in src if p not in matched]
uncertain_pairs = [(status[p], p, reason(p)) for p in uncertain]

os.makedirs(GEN, exist_ok=True)
with open(os.path.join(GEN, "unmatched.txt"), "w", encoding="utf-8") as f:
    for c, p, r in uncertain_pairs:
        f.write(f"{c}\t{p}\t{r}\n")
with open(os.path.join(GEN, "named-unchanged.txt"), "w", encoding="utf-8") as f:
    for t, refs in sorted(named_unchanged.items()):
        f.write(f"{t}\t{','.join(sorted(refs))}\n")
with open(os.path.join(GEN, "ambiguous.txt"), "w", encoding="utf-8") as f:
    for t, (hits, refs) in sorted(ambiguous.items()):
        f.write(f"{t}\t{len(hits)}\t{','.join(sorted(refs))}\n")

print("created", len(created), "modified", len(modified), "deleted", len(deleted))
print("uncertain", len(uncertain_pairs), "status counts", {c: sum(1 for x in uncertain_pairs if x[0] == c) for c in ("??", "M", "D")})
print("named-unchanged", len(named_unchanged), "ambiguous", len(ambiguous), "dir-covered", len(dir_hits))


def section(title, paths, note):
    lines = [f"## {title} ({len(paths)})", "", note, ""]
    lines += [f"- `{p}`" for p in paths]
    lines.append("")
    return "\n".join(lines)


unc_lines = [f"- `{p}` ({c}): {r}" for c, p, r in uncertain_pairs]
body = [
    "# 🧾️ Round 2 Changed Files v2",
    "",
    "Read-only refresh by agent cf2 of `r2-changed-files.md`. Sources: every `*-report.md` and `migration-*` in this ticket, matched against `git -c core.quotepath=false status --porcelain --untracked-files=all` under `🎓️teaching/🏛️architecture`. A path counts as named only if a report names it exactly or with a unique trailing path. Ambiguous short names and directory mentions stay in Uncertain.",
    "",
    f"Counts: created {len(created)}, modified {len(modified)}, deleted {len(deleted)}, uncertain {len(uncertain_pairs)} (created {sum(1 for c,_,_ in uncertain_pairs if c=='??')}, modified {sum(1 for c,_,_ in uncertain_pairs if c=='M')}, deleted {sum(1 for c,_,_ in uncertain_pairs if c=='D')}). Total changed source paths {len(src)}.",
    "",
    "Scope check: `🎓️teaching/pyproject.toml` and `🎓️teaching/uv.lock` do not exist and have no changes. The pandas pin (`pandas==2.3.3`, from `3.1.0`) lives in `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml` (line 27, test extras) and its lock `uv.lock` (pandas 2.3.3 package entry). Both are modified and named by vf1/vf2/dl/dp/or reports.",
    "",
    section("Created", created, "Untracked, named exactly in a report."),
    section("Modified", modified, "Tracked, changed, named exactly in a report."),
    section("Deleted", deleted, "Removed from the working tree, named exactly in a report."),
    "## Uncertain attribution (" + str(len(uncertain_pairs)) + ")",
    "",
    "Changed under `🎓️teaching/🏛️architecture` but not named by an exact path in a report. Some are covered by a directory or category mention (for example the vegetation or animal catalog entries). Others are in areas like `🏗️bim`, `🏢️nakagin`, `🎯️optimization`, `🦗️grasshopper` or `📊️excel`, which may belong to other work. Decide per entry before the ticket close.",
    "",
] + unc_lines + [""]
with open(OUT, "w", encoding="utf-8") as f:
    f.write("\n".join(body))
print("wrote")

"""📦️ Plans (dry-run) or applies the Cargo manifest, workspace-dependency and Cargo.lock edits for the compute repoint."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
FACADE, COMPUTE = "semio-framework-2d", "semio-framework-2d-compute"
FAC_TAIL, COM_TAIL = "◻️2d/📦️packages/🦀️rust", "◻️2d/🧮️compute/📦️packages/🦀️rust"
apply = "--apply" in sys.argv
files = json.load(open(F + "/2dc-tools/candidates-rs.json"))
mapping = json.load(open(F + "/🗑️generated/2dc-before-manifest-of.json"))
def read(p): return open(os.path.join(ROOT, p), encoding="utf8").read()
unmapped, need = [], {}
for f in files:
    ms = mapping.get(f)
    if not ms: unmapped.append(f); continue
    for m in ms: need.setdefault(m, []).append(f)
sources_of = {}
for f, ms in mapping.items():
    for m in ms: sources_of.setdefault(m, []).append(f)
word = re.compile(r"(?<![A-Za-z0-9_])semio_framework_2d(?![A-Za-z0-9_])")
def remaining(m):
    n = 0
    for f in sources_of.get(m, []):
        try: t = read(f)
        except FileNotFoundError: continue
        for line in t.split("\n"):
            s = line.lstrip()
            if s.startswith("//"): continue
            if word.search(line): n += 1
    return n
dep_re = re.compile(r"^(semio-framework-2d)\s*=\s*(\{.*\})\s*$")
plan, problems = {}, []
for m, srcs in sorted(need.items()):
    text = read(m)
    lines = text.split("\n")
    section, hits = None, []
    for i, l in enumerate(lines):
        if l.startswith("["): section = l.strip()
        mm = dep_re.match(l)
        if mm: hits.append((i, section, mm.group(2)))
        elif re.match(r"^\[(.*dependencies)\.semio-framework-2d\]", l): hits.append((i, l.strip(), "TABLE"))
    other = len(re.findall(r"semio-framework-2d(?![A-Za-z0-9_-])", text)) - len(hits)
    if COMPUTE in text: plan[m] = {"action": "already", "files": len(srcs)}; continue
    if len(hits) != 1 or hits[0][1] not in ("[dependencies]",) or hits[0][2] == "TABLE":
        problems.append((m, "hits", hits)); continue
    i, sec, body = hits[0]
    rem = remaining(m)
    plan[m] = {"action": "replace" if rem == 0 and other == 0 else "add", "line": i, "body": body, "files": len(srcs), "remaining": rem, "other": other}
print("manifests", len(need), "unmapped", len(unmapped))
for u in unmapped: print("UNMAPPED", u)
for p in problems: print("PROBLEM", p)
from collections import Counter
print(Counter(v["action"] for v in plan.values()))
json.dump({"plan": plan, "unmapped": unmapped, "problems": problems}, open(F + "/2dc-tools/manifest-plan.json", "w"), ensure_ascii=False, indent=0)

# ---------- apply phase ----------
def pkg_name(text):
    mm = re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"', text, re.S | re.M)
    return mm.group(1) if mm else None
def workspace_root(m, text):
    mm = re.search(r'^workspace\s*=\s*"([^"]+)"', text, re.M)
    d = os.path.dirname(m)
    if mm:
        return os.path.normpath(os.path.join(d, mm.group(1), "Cargo.toml"))
    while d:
        d = os.path.dirname(d)
        c = os.path.join(d, "Cargo.toml") if d else "Cargo.toml"
        if os.path.exists(os.path.join(ROOT, c)) and re.search(r"^\[workspace[\].]", read(c), re.M): return c
    return "Cargo.toml"
def compute_body(body, m):
    if "path" in body:
        mm = re.search(r'path\s*=\s*"([^"]*)"', body)
        p = mm.group(1)
        if not p.endswith(FAC_TAIL): raise Exception(("odd path", m, p))
        return body.replace(p, p[: -len(FAC_TAIL)] + COM_TAIL)
    return body
def lock_insert(lock, name, add=None, remove=None):
    pat = re.compile(r'(\[\[package\]\]\nname = "%s"\nversion = "[^"]*"\n(?:source = "[^"]*"\n)?(?:checksum = "[^"]*"\n)?dependencies = \[\n)((?: "[^"]*",?\n)*)(\]\n)' % re.escape(name))
    ms = list(pat.finditer(lock))
    if len(ms) != 1: return None
    mm = ms[0]
    items = [x.strip().strip(",").strip('"') for x in mm.group(2).split("\n") if x.strip()]
    if add and add not in items: items.append(add)
    if remove and remove in items: items.remove(remove)
    items = sorted(items, key=lambda s: s.encode())
    body = "".join(' "%s",\n' % i for i in items)
    return lock[: mm.start(2)] + body + lock[mm.end(2):]
ws_done, report = {}, []
fac_line = re.compile(r"^semio-framework-2d\s*=", re.M)
com_line = re.compile(r"^semio-framework-2d-compute\s*=.*$", re.M)
if apply:
    for m, pl in sorted(plan.items()):
        text = read(m)
        if not com_line.search(text):
            lines = text.split("\n"); i = pl["line"]; body = pl["body"]
            assert dep_re.match(lines[i]), m
            nb = compute_body(body, m)
            if pl["action"] == "replace": lines[i] = "%s = %s" % (COMPUTE, nb)
            else: lines.insert(i + 1, "%s = %s" % (COMPUTE, nb))
            text = "\n".join(lines)
            open(os.path.join(ROOT, m), "w", encoding="utf8").write(text)
        wr = workspace_root(m, text)
        if "workspace = true" in com_line.search(text).group(0):
            if wr not in ws_done:
                wt = read(wr)
                if COMPUTE in wt: ws_done[wr] = None
                else:
                    wl = wt.split("\n"); done = False
                    for k, l in enumerate(wl):
                        mm = dep_re.match(l)
                        if mm and "path" in mm.group(2):
                            wl.insert(k + 1, "%s = %s" % (COMPUTE, compute_body(mm.group(2), wr))); done = True; break
                        if l.strip() == "[workspace.dependencies.semio-framework-2d]":
                            j = k + 1
                            while j < len(wl) and wl[j].strip(): j += 1
                            tbl = wl[k + 1: j]
                            nt = ["[workspace.dependencies.%s]" % COMPUTE] + [compute_body(x, wr) if x.startswith("path") else x for x in tbl]
                            wl[j: j] = [""] + nt; done = True; break
                    if not done: report.append(("NO-WORKSPACE-STANZA", wr, m)); ws_done[wr] = None
                    else: ws_done[wr] = "\n".join(wl)
        lockp = os.path.join(os.path.dirname(wr), "Cargo.lock")
        if os.path.exists(os.path.join(ROOT, lockp)):
            lock = read(lockp)
            if 'name = "%s"' % COMPUTE not in lock: report.append(("LOCK-LACKS-COMPUTE", lockp)); continue
            keep = fac_line.search(text) or re.search(r"^\[[a-z-]*dependencies\.semio-framework-2d\]", text, re.M)
            res = lock_insert(lock, pkg_name(text), add=COMPUTE, remove=None if keep else FACADE)
            if res is None: report.append(("LOCK-ENTRY-NOT-FOUND", lockp, pkg_name(text)))
            elif res != lock: open(os.path.join(ROOT, lockp), "w", encoding="utf8").write(res)
        else: report.append(("NO-LOCK", lockp, m))
    for wr, t in ws_done.items():
        if t: open(os.path.join(ROOT, wr), "w", encoding="utf8").write(t)
    for r in report: print("REPORT", r)
    print("applied", len(plan))

# ---------- dev-only / missing-dependency manifests: the gate requires a normal provider ----------
if apply and "--problems" in sys.argv:
    for m, why, hits in problems:
        text = read(m)
        if com_line.search(text): continue
        lines = text.split("\n")
        start = next(k for k, l in enumerate(lines) if l.strip() == "[dependencies]")
        end = next((k for k in range(start + 1, len(lines)) if lines[k].startswith("[")), len(lines))
        while end > start + 1 and not lines[end - 1].strip(): end -= 1
        if hits:
            i, sec, body = hits[0]
            assert sec == "[dev-dependencies]" and dep_re.match(lines[i]), m
            newline = "%s = %s" % (COMPUTE, compute_body(body, m))
            rem = remaining(m)
            lines.insert(end, newline)
            if rem == 0:
                del lines[i + (1 if i >= end else 0)]
            text = "\n".join(lines)
        else: continue
        open(os.path.join(ROOT, m), "w", encoding="utf8").write(text)
        wr = workspace_root(m, text)
        lockp = os.path.join(os.path.dirname(wr), "Cargo.lock")
        print("problem-fixed", m[-70:], "lock", lockp[-40:], "workspace-true" if "workspace = true" in com_line.search(text).group(0) else "path")

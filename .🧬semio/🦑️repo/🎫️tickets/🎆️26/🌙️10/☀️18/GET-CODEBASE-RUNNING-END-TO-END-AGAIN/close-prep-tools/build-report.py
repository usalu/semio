import os, json, subprocess, datetime
ROOT = "/Users/ueli/Documents/semio"
TICKET_NAME = "GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TICKET_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/" + TICKET_NAME
TICKET = os.path.join(ROOT, TICKET_REL)
GEN = os.path.join(TICKET, "🗑️generated")
S = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad"
SINCE = "2026-10-22 19:00"
LIVE = {"e2e-orch", "slice-play3", "course-rest2", "rust-health-logs", "slice-demo2", "apps-b-1022", "apps-a2-x",
        "ts-verify", "rust-reds-b-logs", "rust-reds-a-logs", "demo-verify", "plugin-chain2-1022", "rust-port-logs"}
OWNER = {"course-rest-stale-framework-cache2", "course-rest-new-framework-cache", "course-rest-stale-framework-cache3"}

rows = json.load(open(os.path.join(GEN, "close-prep-resolved.json"), encoding="utf-8"))
keep, missing, ambiguous, unresolved = set(), [], [], []
for r in rows:
    if not r["edit_in"]:
        continue
    if r["kind"].startswith("ambiguous"):
        ambiguous.append(r["ref"])
    elif r["kind"] == "unresolved":
        unresolved.append(r["ref"])
    elif r["kind"] in ("exact", "suffix"):
        t = r["target"]
        if t.startswith(".🧬semio/"):
            continue
        if not os.path.exists(os.path.join(ROOT, t)):
            missing.append(t)
            continue
        keep.add(t)
files = sorted(keep)
files = ["package.json"] + [f for f in files if f != "package.json"]
open(os.path.join(GEN, "close-prep-files.json"), "w", encoding="utf-8").write(json.dumps(files, ensure_ascii=False, indent=1))
open(os.path.join(GEN, "close-prep-files.txt"), "w", encoding="utf-8").write("\n".join(files) + "\n")

entries = []
for line in open(os.path.join(GEN, "close-prep-du.txt"), encoding="utf-8"):
    kb, name = line.rstrip("\n").split("\t", 1)
    entries.append((name.rstrip("/"), int(kb)))
rows_gen = []
for name, kb in entries:
    p = os.path.join(GEN, name)
    recent = subprocess.run(["find", p, "-type", "f", "-newermt", SINCE, "-print", "-quit"], capture_output=True, text=True).stdout.strip() != ""
    if name in OWNER:
        st = "CANDIDATE, owner check (framework cache, not live)"
    elif name in LIVE or recent:
        st = "KEEP (live or touched after 19:00)"
    else:
        st = "CANDIDATE"
    rows_gen.append((name, kb, st))
loose_kb = {"KEEP (live or touched after 19:00)": 0, "CANDIDATE": 0}
loose_n = {"KEEP (live or touched after 19:00)": 0, "CANDIDATE": 0}
with os.scandir(GEN) as it:
    for e in it:
        if e.is_file(follow_symlinks=False):
            st = "KEEP (live or touched after 19:00)" if datetime.datetime.fromtimestamp(e.stat().st_mtime) >= datetime.datetime(2026, 10, 22, 19, 0) else "CANDIDATE"
            loose_kb[st] += e.stat().st_size / 1024
            loose_n[st] += 1
with open(os.path.join(GEN, "close-prep-generated-inventory.txt"), "w", encoding="utf-8") as f:
    for name, kb, st in sorted(rows_gen, key=lambda x: -x[1]):
        f.write(f"{kb/1024:10.1f} MiB\t{st}\t{name}/\n")
agg = {}
for name, kb, st in rows_gen:
    a = agg.setdefault(st, [0, 0]); a[0] += kb; a[1] += 1

staged = [l.strip() for l in open(os.path.join(GEN, "close-prep-staged-entries.txt")) if l.strip()]
sfiles = 0; skb = 0
for e in staged:
    for dp, dn, fn in os.walk(os.path.join(TICKET, e)):
        for f in fn:
            sfiles += 1
            skb += os.path.getsize(os.path.join(dp, f)) / 1024
dup = open(os.path.join(GEN, "close-prep-staged-dupes.txt")).read().strip() if os.path.exists(os.path.join(GEN, "close-prep-staged-dupes.txt")) else "n/a"
stats = {"files": files, "missing": missing, "ambiguous": len(ambiguous), "unresolved": len(unresolved),
         "agg": agg, "loose_kb": loose_kb, "loose_n": loose_n, "staged_entries": len(staged), "staged_files": sfiles,
         "staged_mib": skb / 1024, "dup": dup,
         "top_staged": []}
json.dump(stats, open(os.path.join(GEN, "close-prep-stats.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1, default=str)
print("files", len(files), "missing", len(missing), "ambiguous refs", len(ambiguous), "unresolved refs", len(unresolved))
print({k: (round(v[0]/1024/1024, 2), v[1]) for k, v in agg.items()}, "GiB, count")
print("loose", {k: (round(v/1024/1024,2), loose_n[k]) for k, v in loose_kb.items()})
print("staged", len(staged), sfiles, round(skb/1024, 1), "MiB")

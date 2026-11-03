"""🧪 Idempotence proof: every generator twice in the sandbox; first run must only add, second run must change nothing.

usage: python3 prove.py [--only SUBSTR] [--out FILE] [--timeout S]   (results: JSON lines)
"""
import json, os, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sandbox as S

SCRIPTS = S.TICKET + "/🧰️scripts"
VENV = "/home/deck/semio/.venv/bin/python"
SKIP = {"tools-hardening", "__pycache__"}


def entries():
    out = []
    for root, dirs, files in os.walk(SCRIPTS):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for f in sorted(files):
            if f.endswith((".py", ".sh")) and not f.startswith("_"):
                out.append(os.path.relpath(os.path.join(root, f), SCRIPTS))
    ticket_top = [f for f in sorted(os.listdir(S.TICKET)) if f.endswith(".py")]
    return sorted(out) + ["../" + f for f in ticket_top]


def matrix():
    here = os.path.dirname(os.path.abspath(__file__))
    out = []
    for c in json.load(open(here + "/cases.json", encoding="utf-8")):
        rel = c["script"]
        out.append((rel, ["bash", rel] + c["args"] if rel.endswith(".sh") else [VENV, rel] + c["args"], c["args"]))
    return out


def variants(rel):
    src = open(os.path.join(SCRIPTS, rel), encoding="utf-8").read()
    interp = ["bash"] if rel.endswith(".sh") else [VENV]
    vs = [[]]
    if "--apply" in src:
        vs.append(["--apply"])
    return [(interp + [rel] + a, a) for a in vs]


def main():
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
    out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else S.BASE + "/results.jsonl"
    tmo = int(sys.argv[sys.argv.index("--timeout") + 1]) if "--timeout" in sys.argv else 120
    S.reset()
    S.sync_scripts()
    f = open(out, "a")
    lo, hi = (int(x) for x in sys.argv[sys.argv.index("--slice") + 1].split(":")) if "--slice" in sys.argv else (0, 10**6)
    todo = [(rel, v) for rel in entries() for v in variants(rel)] if "--matrix" not in sys.argv else [(rel, (cmd, a)) for rel, cmd, a in matrix()]
    for i, (rel, (cmd, a)) in enumerate(todo):
        if only not in rel or not lo <= i < hi:
            continue
        if True:
            S.sync_scripts()
            if "--safe" in sys.argv and cmd[0] == VENV:
                cmd = [VENV, "safe_run.py"] + cmd[1:]
            rec = {"script": rel, "args": a, "safe": "--safe" in sys.argv}
            mark = S.marker()
            d1 = {}
            has_check = ("--safe" in sys.argv and cmd[0] == VENV) or ('"--check"' in open(os.path.join(SCRIPTS, rel), encoding="utf-8").read() and "--check" not in a)
            chk = ([x for x in cmd if x != "--check"] + ["--check"]) if "--check" not in a else cmd
            try:
                if has_check:
                    c0 = S.run(chk, tmo)
                    rec["check_before"] = c0.returncode
                    rec["check_before_wrote"] = len(S.delta(mark))
                t = time.time()
                r1 = S.run(cmd, tmo)
                d1 = S.delta(mark)
                rec.update(rc1=r1.returncode, secs=round(time.time() - t, 1), err1=(r1.stderr or r1.stdout)[-300:])
                rec.update(added=[k for k, v in d1.items() if v[0] == "added"][:8], deleted=[k for k, v in d1.items() if v[0] == "deleted"][:8], modified=sum(v[0] == "modified" for v in d1.values()), removed_lines={k: v[2] for k, v in d1.items() if v[2]})
                rec["nadded"] = sum(v[0] == "added" for v in d1.values())
                rec["ndeleted"] = sum(v[0] == "deleted" for v in d1.values())
                r2 = S.run(cmd, tmo)
                d2 = S.delta(mark)
                rec.update(rc2=r2.returncode, second=[k for k in set(d1) | set(d2) if d1.get(k, (0, 0))[:2] != d2.get(k, (0, 0))[:2]][:10])
                rec["nsecond"] = len([k for k in set(d1) | set(d2) if d1.get(k, (0, 0))[:2] != d2.get(k, (0, 0))[:2]])
                if has_check:
                    m3 = S.marker()
                    c1 = S.run(chk, tmo)
                    rec["check_after"] = c1.returncode
                    rec["check_after_wrote"] = len(S.delta(m3))
                d1.update(d2)
            except Exception as e:
                rec["exception"] = repr(e)
                d1 = S.delta(mark)
            S.restore(d1)
            f.write(json.dumps(rec, ensure_ascii=False) + "\n")
            f.flush()


if __name__ == "__main__":
    main()

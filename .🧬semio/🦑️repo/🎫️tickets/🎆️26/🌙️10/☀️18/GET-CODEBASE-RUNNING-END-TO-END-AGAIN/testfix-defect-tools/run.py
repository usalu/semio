#!/usr/bin/env python3
"""Genuine-birth bun runner: python3 run.py <suites.txt> <out.jsonl> [--root DIR] [--jobs N] [--cap SECONDS] [--birth ID]

Every file runs through the repository harness `genuine-birth/📜️script.ts` (declared birth ladder in its 🔣️.json):
first birth unconditionally, further births only when the previous failure output matches the declared `after`.
A failing file is re-run test by test (one process per failing test name, since a rejected issued operation consumes the process birth).
"""
import json, os, re, shutil, subprocess, sys, tempfile, time
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ANSI = re.compile(r"\x1b\[[0-9;]*m")
HARNESS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth"


def escape(name):
    return "^" + re.sub(r"[.*+?^${}()|\[\]\\]", lambda m: "\\" + m.group(0), name) + "$"


def main():
    args = sys.argv[1:]
    opt = lambda n, d=None: args[args.index(n) + 1] if n in args else d
    lst, out = args[0], args[1]
    root = opt("--root", "/Users/ueli/Documents/semio")
    jobs, cap, first = int(opt("--jobs", "2")), opt("--cap", "300"), opt("--birth")
    policy = json.load(open(root + "/" + HARNESS + "/🔣️.json", encoding="utf-8"))
    births = policy["births"]
    files = [l.rstrip("\n") for l in open(lst, encoding="utf-8") if l.strip()]
    done = set()
    if os.path.exists(out):
        for l in open(out, encoding="utf-8"):
            try: done.add(json.loads(l)["file"])
            except Exception: pass
    logs = HERE + "/../🗑️generated/tfd/logs"
    os.makedirs(logs, exist_ok=True)
    artifacts = root + "/.🧬semio/🦑️repo/⚡️cache/🧪️tests/tfd"
    os.makedirs(artifacts, exist_ok=True)
    base = dict(os.environ, NO_COLOR="1", SEMIO_WORKSPACE_ROOT=root, SEMIO_REPO_ROOT=root, REPO_ROOT=root, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")

    def one(file, birth, pattern=None):
        art = tempfile.mkdtemp(dir=artifacts, prefix="")
        cmd = ["perl", "-e", "alarm " + cap + "; exec @ARGV", "--", "bun", root + "/" + HARNESS + "/📜️script.ts", "--file=" + file, "--birth=" + birth["id"]]
        if pattern: cmd.append("--pattern=" + pattern)
        t0 = time.time()
        try:
            p = subprocess.run(cmd, cwd=root, env=dict(base, SEMIO_TEST_ARTIFACT_DIR=art), capture_output=True)
            txt = ANSI.sub("", (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace"))
            rc = p.returncode
        except Exception as ex:
            txt, rc = "RUNNER " + repr(ex), -1
        shutil.rmtree(art, ignore_errors=True)
        num = lambda k: (lambda m: int(m[-1]) if m else 0)(re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M))
        return dict(rc=rc, pass_=num("pass"), fail=num("fail"), err=num("error"), secs=round(time.time() - t0, 1), text=txt,
                    fails=[re.sub(r"\s*\[[0-9.]+m?s\]$", "", l[len("(fail) "):]).strip() for l in txt.splitlines() if l.startswith("(fail) ")])

    def nxt(birth, text):
        i = next(k for k, b in enumerate(births) if b["id"] == birth["id"])
        return next((b for b in births[i + 1:] if "after" in b and re.search(b["after"], text)), None)

    def run(file):
        birth = next(b for b in births if b["id"] == first) if first else births[0]
        ladder, r = [], None
        while True:
            r = one(file, birth)
            ladder.append(birth["id"])
            if r["rc"] == 0: break
            nb = nxt(birth, r["text"])
            if not nb: break
            birth = nb
        rec = dict(file=file, rc=r["rc"], birth=birth["id"], ladder=ladder, pass_=r["pass_"], fail=r["fail"], secs=r["secs"], isolated=None)
        if r["rc"] != 0:
            lp = logs + "/" + re.sub(r"[^A-Za-z0-9]+", "_", file)[-70:] + ".log"
            open(lp, "w", encoding="utf-8").write(r["text"][-60000:])
            rec["log"] = lp
            rec["fails"] = r["fails"][:12]
            rec["errs"] = [l.strip()[:300] for l in r["text"].splitlines() if l.lstrip().startswith(("error:", "Error:", "TypeError", "ReferenceError", "SyntaxError"))][:6]
            names = list(dict.fromkeys(r["fails"]))[:40]
            if names and r["fail"] > 0:
                iso = {}
                for n in names:
                    q = one(file, birth, escape(n))
                    iso[n] = q["rc"] == 0 and q["pass_"] >= 1
                rec["isolated"] = iso
        open(out, "a", encoding="utf-8").write(json.dumps(rec, ensure_ascii=False) + "\n")
        return rec

    todo = [f for f in files if f not in done]
    with ThreadPoolExecutor(jobs) as ex:
        res = list(ex.map(run, todo))
    green = sum(1 for r in res if r["rc"] == 0)
    iso = sum(1 for r in res if r["rc"] != 0 and r["isolated"] and all(r["isolated"].values()))
    print(f"ran {len(res)} green {green} isolated-green {iso} red {len(res)-green-iso}")


main()

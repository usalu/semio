#!/usr/bin/env python3
"""Round-6 apply series (integrate2): the 25 staged sets of the last wave as ONE ordered, backed-up, undoable run (steps.py is the single table; sim.py proves the same table on an overlay).

  apply-all.sh                         dry run: prepare (merged copies) + check of every step on the live tree (read only; later steps may read `refused` only because earlier ones are not applied yet: the overlay sim is the chain proof)
  apply-all.sh --yes [--only a,b]      apply in order; every step is re-checked right before its write; backup of the files it touches; syntax check; STOPS on the first failure
  apply-all.sh --sim [--fresh]         overlay simulation of the whole chain (sandboxed: nothing below the checkout can be written)
  apply-all.sh --undo <step>           restore a step from its backup while the checkout still holds its result (--force: also if later steps touched the same files)
options: --list, --skip a,b, --from step, --to step, --ignore-running, --no-verify, --reapply, --preflight
Preflight (always before --yes): no `repo show|run|exec|dependencies|tools setup` process (the pinned graph build must have ended), load <= 30, `.nx/installation` resolves, disk >= 60 GB.
Ledger and backups: C/🗑️generated/round6-apply/ (ledger.json, backup/<step>/)."""
import hashlib, json, os, re, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
GEN = C + "/🗑️generated/round6-apply"
LEDGER = GEN + "/ledger.json"
PY, BUN = sys.executable, shutil.which("bun") or "bun"
args = sys.argv[1:]
flag = lambda n: n in args
opt = lambda n, d="": args[args.index(n) + 1] if n in args else d
csv = lambda n: [x for x in opt(n).split(",") if x]
sha = lambda b: hashlib.sha256(b).hexdigest()
sys.path.insert(0, HERE)
import steps as STEPS  # noqa: E402

TOUCH = json.load(open(HERE + "/touched.json", encoding="utf8")) if os.path.isfile(HERE + "/touched.json") else {}


def sh(argv, timeout=3600, env=None):
    run = subprocess.run(argv, capture_output=True, text=True, encoding="utf8", errors="replace", cwd=LIVE, timeout=timeout, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_ROOT": LIVE, **(env or {})})
    return run.returncode, run.stdout + run.stderr


def run_all(cmds, env):
    out, rc = "", 0
    for cmd in cmds:
        rc, text = sh(cmd, env=env)
        out += "$ " + " ".join(cmd)[:200] + "\n" + text
        if rc not in (0, 1) or (rc == 1 and cmd is cmds[-1] and False):
            break
    return rc, out


def classify(step, rc, out):
    if rc not in step.get("ok", (0,)) and not (rc == 1 and re.search(r"(?i)\b(pending|dry run|would be written|to apply)\b", out) and not re.search(r"(?i)refus|drift", out)):
        return "refused"
    if re.search(r"(?i)\bREFUSED\b|\bDRIFT\b|Traceback", out):
        return "refused"
    if re.search(r"(?i)0 pending|already applied|all applied|^applied|nothing to (do|write)|0 file\(s\) (would|pending)|, 0 pending", out) and not re.search(r"(?i)[1-9]\d* (file\(s\) )?pending|would be written|would write|\bpending\b (?!0)", out):
        return "applied"
    return "pending"


def load_ledger():
    return json.load(open(LEDGER, encoding="utf8")) if os.path.isfile(LEDGER) else {}


def save_ledger(l):
    os.makedirs(GEN, exist_ok=True)
    json.dump(l, open(LEDGER + ".tmp", "w", encoding="utf8"), ensure_ascii=False, indent=1)
    os.replace(LEDGER + ".tmp", LEDGER)


def fsha(rel):
    p = os.path.join(LIVE, rel)
    return sha(open(p, "rb").read()) if os.path.isfile(p) else None


def running():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout
    return [l.strip()[:150] for l in out.split("\n") if re.search(r"\brepo (show|setup|run|exec|dependencies)\b|dependencies (sync|lock|ensure)\b|repo tools setup|\bbun repo\b", l) and "apply-all" not in l and "grep" not in l and "ps -axo" not in l]


def preflight():
    problems = []
    busy = running()
    if busy and not flag("--ignore-running"):
        problems.append("repo processes run (a write below the build library aborts them):\n    " + "\n    ".join(busy))
    load = os.getloadavg()[0]
    if load > 30:
        problems.append("load %.1f > 30" % load)
    if not os.path.exists(os.path.join(LIVE, ".nx", "installation")):
        problems.append(".nx/installation does not resolve (run `bun repo tools setup nx`)")
    free = shutil.disk_usage(LIVE).free / 1e9
    if free < 60:
        problems.append("only %.0f GB free" % free)
    return problems


def touched(step):
    return TOUCH.get(step["id"], [])


def backup(step, rels):
    folder = os.path.join(GEN, "backup", step["id"])
    shutil.rmtree(folder, ignore_errors=True)
    os.makedirs(folder)
    rows = {}
    for i, rel in enumerate(sorted(set(rels))):
        b = fsha(rel)
        name = None
        if b is not None:
            name = "%05d.orig" % i
            shutil.copyfile(os.path.join(LIVE, rel), os.path.join(folder, name))
            os.chmod(os.path.join(folder, name), 0o644)
        rows[rel] = {"before": b, "backup": name, "mode": os.lstat(os.path.join(LIVE, rel)).st_mode & 0o777 if b else None}
    return rows


def syntax(rels):
    bad, batch = [], []
    for rel in rels:
        p = os.path.join(LIVE, rel)
        if not os.path.isfile(p):
            continue
        if rel.endswith(".json"):
            try:
                json.load(open(p, encoding="utf8"))
            except ValueError as e:
                bad.append("%s: %s" % (rel, e))
        elif rel.endswith((".ts", ".tsx", ".mjs", ".cjs")):
            batch.append(p)
    out = os.path.join(GEN, "syntax-out")
    for i in range(0, len(batch), 150):
        shutil.rmtree(out, ignore_errors=True)
        rc, text = sh(["nice", "-n", "10", BUN, "build", "--no-bundle", "--outdir", out, *batch[i:i + 150]], timeout=900)
        if rc:
            bad.append(text.strip()[:400])
    shutil.rmtree(out, ignore_errors=True)
    return bad


def prepare_and_check(step):
    cmds = [*step.get("prepare", []), *step["check"]]
    rc, out = run_all(cmds, step.get("env"))
    return classify(step, rc, out), out


def apply_step(step, ledger):
    rels = touched(step)
    rows = backup(step, rels)
    for cmd in step.get("prepare", []):
        rc, out = sh(cmd, env=step.get("env"))
        if rc:
            print("   PREPARE FAILED:", out.strip()[-600:])
            return False
    rc, out = 0, ""
    for cmd in step["write"]:
        rc, text = sh(cmd, env=step.get("env"))
        out += text
        if rc not in step.get("ok", (0,)):
            print("   apply rc=%d FAILED\n   %s" % (rc, text.strip()[-1500:].replace("\n", "\n   ")))
            return False
    for rel, row in rows.items():
        row["after"] = fsha(rel)
    changed = {r: row for r, row in rows.items() if row["after"] != row["before"]}
    ledger[step["id"]] = {"time": time.strftime("%Y-%m-%d %H:%M:%S"), "files": changed, "listed": len(rels)}
    save_ledger(ledger)
    print("   applied: %d of %d listed files changed" % (len(changed), len(rels)))
    return True


def undo(name):
    ledger = load_ledger()
    e = ledger.get(name)
    if not e:
        sys.exit("nothing recorded for " + name)
    later = [k for k, v in ledger.items() if v["time"] > e["time"] and set(v["files"]) & set(e["files"])]
    if later and not flag("--force"):
        sys.exit("refused: later steps touched the same files: %s" % ", ".join(later))
    for rel, row in e["files"].items():
        if row.get("after") and fsha(rel) != row["after"] and not flag("--force"):
            sys.exit("refused: %s changed after the step was applied" % rel)
    folder = os.path.join(GEN, "backup", name)
    for rel, row in e["files"].items():
        t = os.path.join(LIVE, rel)
        if row["before"] is None:
            if os.path.exists(t):
                os.remove(t)
        else:
            tmp = t + ".r6undo"
            shutil.copyfile(os.path.join(folder, row["backup"]), tmp)
            if row.get("mode") is not None:
                os.chmod(tmp, row["mode"])
            os.replace(tmp, t)
    del ledger[name]
    save_ledger(ledger)
    print("restored", name)


def main():
    if flag("--undo"):
        return undo(opt("--undo"))
    if flag("--sim"):
        return subprocess.run([PY, HERE + "/sim.py", *(["--fresh"] if flag("--fresh") else []), *(["--continue"] if flag("--continue") else [])]).returncode
    table = STEPS.steps(LIVE, C, LIVE)
    ids = [s["id"] for s in table]
    chosen = [s for s in table if (not csv("--only") or s["id"] in csv("--only")) and s["id"] not in csv("--skip")]
    if opt("--from"):
        chosen = [s for s in chosen if ids.index(s["id"]) >= ids.index(opt("--from"))]
    if opt("--to"):
        chosen = [s for s in chosen if ids.index(s["id"]) <= ids.index(opt("--to"))]
    if flag("--list"):
        for s in chosen:
            print(s["id"], "-", s.get("title", ""))
        return 0
    problems = preflight()
    if flag("--preflight") or (flag("--yes") and problems):
        print("PREFLIGHT:", "ok" if not problems else "\n  " + "\n  ".join(problems))
        return 0 if not problems else 2
    ledger = load_ledger()
    print("%s on %s\n" % ("APPLY" if flag("--yes") else "DRY RUN", LIVE))
    if problems:
        print("note (preflight):\n  " + "\n  ".join(problems) + "\n")
    for s in chosen:
        if s["id"] in ledger and not flag("--reapply"):
            e = ledger[s["id"]]
            changed = [r for r, row in e["files"].items() if row.get("after") and fsha(r) != row["after"]]
            print("%-28s applied (ledger%s)" % (s["id"], "; %d file(s) changed since" % len(changed) if changed else ""))
            continue
        st, out = prepare_and_check(s)
        print("%-28s %s" % (s["id"], st))
        if not flag("--yes"):
            if st == "refused":
                print("   " + out.strip()[-500:].replace("\n", "\n   "))
            continue
        if st == "applied":
            continue
        if st != "pending":
            print("   STOP: %s\n   %s" % (st, out.strip()[-1200:].replace("\n", "\n   ")))
            return 3
        if not apply_step(s, ledger):
            print("STOPPED at %s (undo with --undo %s)" % (s["id"], s["id"]))
            return 4
        if not flag("--no-verify"):
            bad = syntax(list(ledger[s["id"]]["files"]))
            print("   syntax/json of %d file(s): %s" % (len(ledger[s["id"]]["files"]), "ok" if not bad else "FAILED"))
            for b in bad:
                print("      " + b)
            if bad:
                print("STOPPED: syntax of %s failed (undo with --undo %s)" % (s["id"], s["id"]))
                return 5
    return 0


sys.exit(main())

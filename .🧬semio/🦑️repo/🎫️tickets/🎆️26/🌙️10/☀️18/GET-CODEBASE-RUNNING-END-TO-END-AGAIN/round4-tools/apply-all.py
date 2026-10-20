#!/usr/bin/env python3
"""Round-4 apply series (integrate2): the concurrency / granularity / graph sets in one ordered, backed-up, undoable run.

  apply-all.sh                         dry run: state of every group on the live tree (read only)
  apply-all.sh --yes [--only a,b]      apply the groups in order; each group is re-checked right before its write; backup of the files the group touches; syntax check; light verification; STOPS on the first failure
  apply-all.sh --sim                   overlay simulation of the whole chain under /private/tmp/i3 (never touches the checkout) and refresh of touched.json
  apply-all.sh --undo <group>          restore a group from its backup while the checkout still holds exactly its result (--force: also if later groups touched the same files)
options: --list, --skip a,b, --verify-only (run only the verification of groups already in the ledger), --no-verify, --reapply
Backups and ledger: C/🗑️generated/round4-apply/ (ledger.json, backup/<group>/). Groups that were applied outside this tool are `adopted` in the ledger (see result-integrate2.md round 3)."""
import hashlib, json, os, re, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
GEN = C + "/🗑️generated/round4-apply"
LEDGER = GEN + "/ledger.json"
PY, BUN = sys.executable, shutil.which("bun") or "bun"
NICE = ["nice", "-n", "10"]
args = sys.argv[1:]
flag = lambda n: n in args
opt = lambda n, d="": args[args.index(n) + 1] if n in args else d
csv = lambda n: [x for x in opt(n).split(",") if x]
sha = lambda b: hashlib.sha256(b).hexdigest()
TOUCH = json.load(open(HERE + "/touched.json", encoding="utf8")) if os.path.isfile(HERE + "/touched.json") else {}
TEST_RE = re.compile(r"/🧪️tests/(?:[^/]+/)*🟦️\.ts$")


def sh(argv, timeout=1800, env=None):
    run = subprocess.run(argv, capture_output=True, text=True, encoding="utf8", errors="replace", cwd=LIVE, timeout=timeout, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_ROOT": LIVE, **(env or {})})
    return run.returncode, run.stdout + run.stderr


def lines(out, prefix):
    return [l for l in out.split("\n") if l.lower().startswith(prefix)]


def c_rc(rc, out):
    return {0: "applied", 1: "pending"}.get(rc, "refused")


def c_pending_words(*words):
    def f(rc, out):
        if rc not in (0, 1):
            return "refused"
        return "pending" if any(re.search(r"(?m)^(%s)\b" % "|".join(words), out) for _ in [0]) else "applied"
    return f


def c_leases(rc, out):
    m = re.search(r"check ok: (\d+) patch, (\d+) new, (\d+) already applied", out)
    if rc or not m:
        return "refused"
    return "pending" if int(m.group(1)) + int(m.group(2)) else "applied"


def c_cargo(rc, out):
    m = re.search(r"(\d+) write\(s\), (\d+) error", out)
    if rc or not m or int(m.group(2)):
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_ops(rc, out):
    if rc == 2 or "DRIFT" in out and rc:
        return "refused"
    return {0: "applied", 1: "pending"}.get(rc, "refused")


def c_bazel(rc, out):
    m = re.search(r'summary \{"new": (\d+), "pending": (\d+)(?:, "upgrade": (\d+))?', out)
    if rc or not m:
        return "refused"
    return "pending" if int(m.group(1)) + int(m.group(2)) + int(m.group(3) or 0) else "applied"


def c_dryrun_files(rc, out):
    m = re.search(r"(\d+) file\(s\) would be written", out)
    if rc or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_gk(rc, out):
    m = re.search(r"set: (\d+) file operation", out)
    if rc or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_course(rc, out):
    if rc not in (0, 1) or lines(out, "refused"):
        return "refused"
    return "pending" if lines(out, "pending") else "applied"


def c_default(rc, out):
    if rc:
        return "refused"
    m = re.search(r'"edit": (\d+)', out)
    return "pending" if m and int(m.group(1)) else "applied"


def c_spec(rc, out):
    m = re.search(r"ops (\d+) \(pending (\d+)\)", out)
    if rc not in (0, 1) or not m:
        return "refused" if rc not in (0, 1) else "applied"
    return "pending" if int(m.group(2)) else "applied"


def c_counts(rc, out):
    m = re.search(r"files (\d+), pending (\d+), applied (\d+), drift (\d+)", out)
    if not m or int(m.group(4)):
        return "refused"
    return "pending" if int(m.group(2)) else "applied"


def c_fns(rc, out):
    m = re.search(r"pending=(\d+) already-applied=(\d+) drift=(\d+)", out)
    if rc or not m or int(m.group(3)):
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_clizt(rc, out):
    try:
        d = json.loads(out[out.index("{"):])
    except Exception:
        return "refused"
    if d.get("drift"):
        return "refused"
    return "pending" if d.get("create", 0) + d.get("replace", 0) + d.get("edit", 0) + d.get("edit-anchored", 0) else "applied"


def c_docs(rc, out):
    if rc:
        return "refused"
    return "pending" if re.search(r"(?m)^  [a-z][a-z0-9-]+$", out) else "applied"


def c_ci(rc, out):
    m = re.search(r'summary \{([^}]*)\} problems (\d+)', out)
    if rc or not m or int(m.group(2)):
        return "refused"
    return "pending" if re.search(r'"(edit|new|replace)": [1-9]', m.group(1)) else "applied"


def c_btr(rc, out):
    m = re.search(r"(\d+) files to write, (\d+) already in place", out)
    if rc or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_xplat(rc, out):
    m = re.search(r"dry run: (\d+) pending operation", out)
    if rc or not m:
        return "applied" if not rc and "dry run: 0 pending" in out else "refused"
    return "pending" if int(m.group(1)) else "applied"


def c_ts(rc, out):
    if re.search(r"[1-9]\d* to create", out):
        return "pending"
    return "applied" if not rc and "0 to create" in out else "refused"


def groups():
    a = lambda *x: [PY, *x]
    G = []
    S = HERE + "/staged"

    def add(**kw):
        kw.setdefault("touch", kw["id"])
        for k, v in (("prepare", []), ("post", []), ("verify", "tests"), ("ok_rcs", (0,)), ("adopted", False), ("rebase", None), ("note", ""), ("tests_extra", []), ("extra_touched", [])):
            kw.setdefault(k, v)
        G.append(kw)

    L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
    mg = C + "/mem-gov-tools"
    add(id="agents-fix", title="agents-fix: comment discipline, localised host-capacity errors, registry/pointer/debug/legacy cleanups (MERGED COPY on round-3 mem-gov; join-bases file left to fnsplit4a-finish)",
        prepare=[a(S + "/make-agents.py"), [BUN, S + "/agents-fix/build.mjs"], a(S + "/prune-agents.py")], check=a(S + "/agents-fix/apply.py", "check", "--root", LIVE), classify=c_counts, apply=a(S + "/agents-fix/apply.py", "apply", "--write", "--root", LIVE))
    fn = S + "/fnsplit4a-finish/staged/apply.py"
    add(id="fnsplit4a-finish", title="fnsplit4a-finish: 116 ops (64 modify, 52 create) emoji/shape/function splits", check=a(C + "/fnsplit4a-finish-tools/staged/apply.py", "--root", LIVE), classify=c_fns,
        apply=a(C + "/fnsplit4a-finish-tools/staged/apply.py", "--root", LIVE, "--write"), post=[["zsh", C + "/fnsplit4a-finish-tools/check-live.sh"]])
    cz = C + "/cli-zt-tools/apply.py"
    add(id="cli-zt2", title="cli-zt2: no default language, bilingual notes, `ensure` (MERGED PLAN: help fixture regenerated, cache notes/status adapted to cache-gc)", prepare=[a(S + "/make-cli-zt2.py")],
        check=a(cz, "--root", LIVE, "--plan", S + "/cli-zt2-plan.json", "--allow-anchored"), classify=c_clizt, apply=a(cz, "--root", LIVE, "--plan", S + "/cli-zt2-plan.json", "--apply", "--allow-anchored", "--ignore-running"),
        post=[[BUN, mg + "/regen-help.ts", LIVE]], extra_touched=[L + "/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"])
    dd = C + "/docs-1020e-tools"
    laws = ["❄️frozen-markdown-coordinates", "🏠️owner-projection-policy", "👀️readme-reviewed-fixture-inputs", "🔖️readme-current-source-revision", "🗺️testing-readme-coordinates", "🚚️readme-move-source-authority", "🟢️readme-current-source-activation"]
    add(id="docs-1020e", title="docs-1020e: concurrency / incremental / shared-cache build guide EN+DE (refresh.sh --write regenerates the edits from the feature states first), then the 7 doc laws", check=["sh", dd + "/refresh.sh"], classify=c_docs,
        apply=["sh", dd + "/refresh.sh", "--write"], tests_extra=[L + "/🧪️tests/%s/🟦️.ts" % law for law in laws], verify="tests")
    cb = C + "/ci-bazel-tools/apply.py"
    add(id="ci-bazel", title="ci-bazel READY2: CI on Bazel (workflow, scripts, dependabot, report)", check=a(cb), classify=c_ci, apply=a(cb, "--write"))
    bt = S + "/bzl-tests-rest"
    add(id="bzl-tests-rest", title="bzl-tests-rest: every test task is a real `bazel test` (MERGED COPY: adapter anchor 12 ends at `groups:`)", prepare=[a(S + "/make-btr.py"), a(bt + "/build-staged.py")], check=a(bt + "/apply.py"), classify=c_btr, apply=a(bt + "/apply.py", "--write"))
    xp = S + "/xplat-linux-fix"
    add(id="xplat-linux-fix", title="xplat-linux-fix: all 12 groups (MERGED COPY: g3 sampling edit re-based on agents-fix' line; repo-command guard skipped for chain16)", prepare=[a(S + "/make-xplat.py", "--root", LIVE)],
        check=a(xp + "/apply.py", "--root", LIVE), classify=c_xplat, apply=a(xp + "/apply.py", "--root", LIVE, "--group", "all", "--write", "--backup", GEN + "/backup/xplat-own"))
    ct = C + "/conc-ts-tools"
    add(id="conc-ts", title="conc-ts: per-package TypeScript checks (engine, 10 source fixes, typecheck wiring)", check=["sh", ct + "/go.sh"], classify=c_ts, apply=["sh", ct + "/go.sh", "--apply"])
    return G


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
    out = subprocess.run(["ps", "-axo", "command"], capture_output=True, text=True).stdout
    return [l[:140] for l in out.split("\n") if re.search(r"\brepo (show|setup|run|exec|dependencies)\b|dependencies (sync|lock|ensure)\b|repo tools setup", l) and "apply-all" not in l and "grep" not in l]


def touched(g):
    if g.get("touched_cmd"):
        rc, out = sh(g["touched_cmd"])
        try:
            return json.loads(out[out.index("{"):])["files"]
        except Exception:
            return []
    if g.get("fixed"):
        return g["fixed"]
    return (TOUCH.get(g["touch"], []) if g["touch"] else []) + list(g["extra_touched"])


def backup(g, rels):
    folder = os.path.join(GEN, "backup", g["id"])
    shutil.rmtree(folder, ignore_errors=True)
    os.makedirs(folder)
    rows = {}
    for i, rel in enumerate(sorted(set(rels))):
        b = fsha(rel)
        name = None
        if b is not None:
            name = "%04d.orig" % i
            shutil.copyfile(os.path.join(LIVE, rel), os.path.join(folder, name))
        rows[rel] = {"before": b, "backup": name}
    return rows


def status(g, ledger):
    if g["id"] in ledger and not flag("--reapply"):
        e = ledger[g["id"]]
        d = [r for r, row in e["files"].items() if row.get("after") and fsha(r) != row["after"]]
        return "applied (%s%s)" % ("adopted" if e.get("adopted") else "ledger", "; %d file(s) changed since by later groups" % len(d) if d else ""), ""
    if flag("--yes") or flag("--fresh"):
        for cmd in g["prepare"]:
            rc, out = sh(cmd)
            if rc:
                return "refused", "prepare failed: " + out.strip()[-500:]
    rc, out = sh(g["check"], env=None)
    st = g["classify"](rc, out)
    if st == "refused" and g["rebase"] and (flag("--yes") or flag("--fresh")):
        rrc, rout = sh(g["rebase"])
        rc, out = sh(g["check"])
        st = g["classify"](rc, out)
        out = "(re-based: %s) " % ("ok" if not rrc else "rebase failed " + rout.strip()[-200:]) + out
    return st, out


def syntax(rels):
    bad = []
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
            rc, out = sh(NICE + [BUN, "build", "--no-bundle", p], timeout=120)
            if rc:
                bad.append("%s: %s" % (rel, out.strip()[:160]))
    return bad


HARNESS = C + "/testfix-defect-tools/run.py"


def run_tests(rels):
    results = []
    for rel in rels:
        listing = "/private/tmp/i4-one.txt"
        open(listing, "w", encoding="utf8").write(rel + "\n")
        out = "/private/tmp/i4-one.jsonl"
        if os.path.exists(out):
            os.remove(out)
        started = time.time()
        sh(NICE + [PY, HARNESS, listing, out, "--jobs", "1", "--cap", "60"], timeout=1200)
        rec = json.loads(open(out, encoding="utf8").read().strip().split("\n")[-1]) if os.path.isfile(out) else {"rc": -9}
        results.append((rel, rec.get("rc"), rec.get("pass_"), rec.get("fail"), round(time.time() - started)))
    return results


def verify(g, ledger):
    e = ledger.get(g["id"], {})
    rels = list(e.get("files", {}))
    bad = syntax(rels)
    print("   syntax/json of %d file(s): %s" % (len(rels), "ok" if not bad else "FAILED"))
    for b in bad:
        print("      " + b)
    if bad:
        return False
    if g["verify"] == "tests" and not flag("--no-verify"):
        tests = g["tests_extra"] + [r for r in rels if TEST_RE.search(r) and not any(h in r for h in ("/🏃️step/", "🔬️probes", "📥️worker", "⏳️slow", "🤝️shared", "📥️reader"))]
        for rel, rc, p, f, secs in run_tests(tests):
            print("   test %-5s pass=%s fail=%s %4ss %s" % ("GREEN" if rc == 0 else "RED", p, f, secs, rel[-80:]))
            if rc != 0:
                e.setdefault("verify_red", []).append(rel)
        save_ledger(ledger)
    return True


def chain16():
    """State of the chain16 `show projects` run (the live test of conc-graph under edits): process alive, log size, and the markers that would show a restart/settle."""
    log = GEN.replace("round4-apply", "chain16") + "/show-projects.err"
    alive = subprocess.run(["pgrep", "-f", "repo show projects"], capture_output=True, text=True).stdout.split()
    if not os.path.isfile(log):
        return {"alive": bool(alive), "lines": 0, "markers": 0, "last": ""}
    text = open(log, encoding="utf8", errors="replace").read()
    lines = text.split("\n")
    marks = [l for l in lines if re.search(r"restart|graph-settle|settle|Original Script process terminated|stale|changed during|retry|round \d+", l, re.I)]
    return {"alive": bool(alive), "lines": len(lines), "markers": len(marks), "last": (lines[-2] if len(lines) > 1 else "")[:140]}


def apply_group(g, ledger):
    rels = touched(g)
    if not rels and not g["adopted"]:
        print("   REFUSED: no touched list for %s (run --sim)" % g["id"])
        return False
    rows = backup(g, rels)
    before16 = chain16()
    for cmd in g["prepare"]:
        rc, out = sh(cmd)
        if rc:
            print("   PREPARE FAILED:", out.strip()[-500:])
            return False
    rc, out = sh(g["apply"], timeout=3600)
    ok = rc in g["ok_rcs"]
    print("   apply rc=%d %s" % (rc, "ok" if ok else "FAILED"))
    if not ok:
        print("   " + out.strip()[-1500:].replace("\n", "\n   "))
        return False
    for cmd in g["post"]:
        prc, pout = sh(cmd)
        print("   post rc=%d %s" % (prc, os.path.basename(cmd[1] if len(cmd) > 1 else cmd[0])))
        if prc:
            print("   " + pout.strip()[-600:].replace("\n", "\n   "))
            return False
    for rel, row in rows.items():
        row["after"] = fsha(rel)
    changed = {r: row for r, row in rows.items() if row["after"] != row["before"]}
    after16 = chain16()
    ledger[g["id"]] = {"time": time.strftime("%Y-%m-%d %H:%M:%S"), "files": changed, "listed": len(rels), "chain16": {"before": before16, "after": after16}}
    save_ledger(ledger)
    print("   chain16: alive %s->%s, log lines %d->%d, restart/settle markers %d->%d" % (before16["alive"], after16["alive"], before16["lines"], after16["lines"], before16["markers"], after16["markers"]))
    print("   %d of %d listed files changed" % (len(changed), len(rels)))
    return True


def undo(name):
    ledger = load_ledger()
    e = ledger.get(name)
    if not e:
        sys.exit("nothing recorded for " + name)
    later = [k for k, v in ledger.items() if v["time"] > e["time"] and set(v["files"]) & set(e["files"])]
    if later and not flag("--force"):
        sys.exit("refused: later groups touched the same files: %s" % ", ".join(later))
    for rel, row in e["files"].items():
        if row.get("after") and fsha(rel) != row["after"] and not flag("--force"):
            sys.exit("refused: %s changed after the group was applied" % rel)
    folder = os.path.join(GEN, "backup", name) if not e.get("adopted") else e.get("backup_folder", "")
    for rel, row in e["files"].items():
        t = os.path.join(LIVE, rel)
        if row["before"] is None:
            if os.path.exists(t):
                os.remove(t)
        elif row.get("backup") and row["before"] != "unknown":
            tmp = t + ".r3undo"
            shutil.copyfile(os.path.join(folder, row["backup"]), tmp)
            os.replace(tmp, t)
    del ledger[name]
    save_ledger(ledger)
    print("restored", name)


def main():
    if flag("--undo"):
        return undo(opt("--undo"))
    if flag("--sim"):
        return subprocess.run([PY, HERE + "/sim.py", "--tag", "touch", "--continue"]).returncode
    only, skip = csv("--only"), csv("--skip")
    G = [g for g in groups() if (not only or g["id"] in only) and g["id"] not in skip]
    if flag("--list"):
        for g in G:
            print(g["id"], "-", g["title"])
        return 0
    ledger = load_ledger()
    busy = running()
    if busy:
        print("NOTE: these run (a write may abort them):\n  " + "\n  ".join(busy))
        if flag("--yes") and not flag("--ignore-running"):
            return 2
    print("%s on %s\n" % ("APPLY" if flag("--yes") else "DRY RUN", LIVE))
    for g in G:
        st, out = status(g, ledger)
        print("%-24s %-40s %s" % (g["id"], st, g["title"][:90]))
        if st.startswith("applied") and flag("--verify-only"):
            verify(g, ledger)
            continue
        if not flag("--yes"):
            if st == "refused":
                print("   " + out.strip()[-400:].replace("\n", "\n   "))
            continue
        if st.startswith("applied"):
            continue
        if st != "pending":
            print("   STOP: %s\n   %s" % (st, out.strip()[-900:].replace("\n", "\n   ")))
            return 3
        if not apply_group(g, ledger):
            print("STOPPED at %s (undo with --undo %s)" % (g["id"], g["id"]))
            return 4
        if not verify(g, ledger):
            print("STOPPED: syntax/json of %s failed (undo with --undo %s)" % (g["id"], g["id"]))
            return 5
    return 0


sys.exit(main())

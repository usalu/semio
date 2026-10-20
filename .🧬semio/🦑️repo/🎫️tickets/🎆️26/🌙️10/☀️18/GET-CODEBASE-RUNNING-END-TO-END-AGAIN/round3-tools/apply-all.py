#!/usr/bin/env python3
"""Round-3 apply series (integrate2): the concurrency / granularity / graph sets in one ordered, backed-up, undoable run.

  apply-all.sh                         dry run: state of every group on the live tree (read only)
  apply-all.sh --yes [--only a,b]      apply the groups in order; each group is re-checked right before its write; backup of the files the group touches; syntax check; light verification; STOPS on the first failure
  apply-all.sh --sim                   overlay simulation of the whole chain under /private/tmp/i3 (never touches the checkout) and refresh of touched.json
  apply-all.sh --undo <group>          restore a group from its backup while the checkout still holds exactly its result (--force: also if later groups touched the same files)
options: --list, --skip a,b, --verify-only (run only the verification of groups already in the ledger), --no-verify, --reapply
Backups and ledger: C/🗑️generated/round3-apply/ (ledger.json, backup/<group>/). Groups that were applied outside this tool are `adopted` in the ledger (see result-integrate2.md round 3)."""
import hashlib, json, os, re, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
GEN = C + "/🗑️generated/round3-apply"
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


def groups():
    a = lambda *x: [PY, *x]
    G = []

    def add(**kw):
        kw.setdefault("touch", kw["id"])
        for k, v in (("prepare", []), ("post", []), ("verify", []), ("ok_rcs", (0,)), ("adopted", False), ("rebase", None), ("note", "")):
            kw.setdefault(k, v)
        G.append(kw)

    mg = C + "/mem-gov-tools"
    add(id="mem-gov", title="mem-gov round-2 combined (hotfix ramp/backfill + decision/gate/policy), --tolerate-drift", check=a(mg + "/apply.py", "--tolerate-drift"), classify=c_dryrun_files,
        apply=a(mg + "/apply.py", "--tolerate-drift", "--write", "--ignore-runs"), post=[[BUN, mg + "/regen-help.ts", LIVE]], verify="tests")
    bd = C + "/bzl-default-tools/apply.py"
    add(id="bzl-default-probe-setup", title="bzl-default READY2: Bazel cache probe + the 3 setup repair edits (NOT closure/manifest/manifest2/nx/flip: lock-sync round); placed right after mem-gov: after conc-bazel its import-line anchor no longer exists", check=a(bd, "--root", LIVE, "--group", "probe", "--group", "setup"), classify=c_default,
        apply=a(bd, "--root", LIVE, "--group", "probe", "--group", "setup", "--write", "--ignore-running"), verify="tests")
    cl = HERE + "/staged/conc-leases"
    add(id="conc-leases", title="conc-leases (lease order observe-mode, fair multi-lease, `leases status`); NOT --with-inheritance; MERGED COPY under conc-harness' traceLeaseV1", prepare=[a(HERE + "/staged/make-leases.py"), a(cl + "/stage.py")],
        check=a(cl + "/apply.py", "check"), classify=c_leases, apply=a(cl + "/apply.py", "apply", "--write", "--ignore-sync"), verify="tests")
    cc = C + "/conc-cargo-tools/apply.py"
    add(id="conc-cargo-1", title="conc-cargo set 1: lease, lane, admit, registry, config", check=a(cc, "check"), classify=c_cargo, apply=a(cc, "apply", "--write", "--ignore-sync"), verify="tests")
    add(id="conc-cargo-2", title="conc-cargo group 2: buildscript, broker (no-op when set 1 already carried the final versions)", check=a(cc, "apply", "--groups", "buildscript,broker", "--ignore-sync"), classify=lambda rc, out: "refused" if rc else ("pending" if re.search(r"(?m)^(rewrite|write|new|patch)", out) else "applied"),
        apply=a(cc, "apply", "--write", "--ignore-sync", "--groups", "buildscript,broker"), verify=[])
    nx = C + "/conc-nx-tools"
    for gid, grp in (("conc-nx-1", "1"), ("conc-nx-2d", "2-daemon"), ("conc-nx-2p", "2-put")):
        add(id=gid, title="conc-nx group %s" % grp, rebase=a(nx + "/ops_gen.py"), check=a(nx + "/apply.py", "check", "--group", grp), classify=c_ops, apply=a(nx + "/apply.py", "apply", "--write", "--group", grp, "--ignore-running"), verify="tests")
    fx = HERE + "/staged/fixups/apply.py"
    L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
    NXT = L + "/⚡️caching/💾️storage/🔒️enforcement/🔌️nx"
    add(id="fixup-nx-tests", touch=None, touched_cmd=None, title="merge fixup (tests only): conc-nx pool-order path (one `../` too many) and put-slots with one control per worker (conc-leases order guard)", check=a(fx, "--op", "nx-tests"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx, "--op", "nx-tests", "--write"), fixed=[NXT + "/📦️graph/🧪️tests/🔓️pool-order/🟦️.ts", NXT + "/🎟️put-slots/🧪️tests/🟦️.ts"], verify="tests")
    dp = C + "/conc-deps-tools"
    add(id="conc-deps", title="conc-deps group all (side-by-side generation protocol, generation-scoped maintenance)", rebase=a(dp + "/mkops.py"), check=a(dp + "/apply.py", "check", "--group", "all"), classify=c_ops,
        apply=a(dp + "/apply.py", "apply", "--group", "all", "--write", "--ignore-running"), ok_rcs=(0, 1), verify="tests")
    bz = C + "/conc-bazel-tools/apply.py"
    add(id="conc-bazel", title="conc-bazel group all (build/service slots, shared caches)", check=a(bz, "--root", LIVE, "--group", "all"), classify=c_bazel,
        apply=a(bz, "--root", LIVE, "--group", "all", "--write", "--backup", GEN + "/backup/conc-bazel-own"), verify="tests")
    add(id="fixup-bazel-tests", touch=None, title="merge fixup (tests only): the Bazel platform suite extracts STARLARK_RULES through mem-gov's bazelResourceSetsV1 prefix", check=a(fx, "--op", "bazel-tests"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx, "--op", "bazel-tests", "--write"), fixed=[L + "/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️platform/🟦️.ts"], verify="tests")
    gc = HERE + "/staged/cache-gc"
    add(id="cache-gc", title="cache-gc (cache ledger + `cache gc` command; do NOT run `repo cache gc`); MERGED COPY (list-independent --json flag edit)", prepare=[a(HERE + "/staged/make-cachegc.py")], check=a(gc + "/apply.py"), classify=c_dryrun_files,
        apply=a(gc + "/apply.py", "--write"), post=[[BUN, gc + "/regen-help.ts", LIVE]], verify="tests")
    add(id="conc-harness", title="conc-harness core + trace (ADOPTED: applied early by the overlay simulation at 22:10, see report)", adopted=True, check=a(C + "/conc-harness-tools/apply.py", "--group", "all"), classify=lambda rc, out: "applied", apply=[], verify="tests")
    add(id="gran-keys", title="gran-keys READY2, 174 ops (ADOPTED: applied early by the overlay simulation at 22:10, backup adopted)", adopted=True, check=a(C + "/gran-keys-tools/apply.py"), classify=lambda rc, out: "applied", apply=[], verify="tests")
    cg = C + "/conc-graph-tools"
    add(id="conc-graph", title="conc-graph READY3 (generations, per-project records, settle-free snapshots, fact table)", rebase=a(cg + "/ops_gen.py"), check=a(cg + "/apply.py", "check"), classify=c_ops,
        apply=a(cg + "/apply.py", "apply", "--write", "--ignore-running"), verify="tests")
    lh = C + "/loop-hunt-tools/apply.py"
    ops = json.load(open(C + "/loop-hunt-tools/staged/ops.json", encoding="utf8")) if os.path.isfile(C + "/loop-hunt-tools/staged/ops.json") else {"edits": [], "newFiles": []}
    add(id="loop-hunt", touch=None, fixed=sorted({e["file"] for e in ops["edits"]} | {n["file"] for n in ops["newFiles"]}), title="loop-hunt: settlement rethrows repeated/unchanged errors naming the failing project, `[graph-settle]` rows, repeat law (settle wrapper still present after conc-graph)",
        check=a(lh, "check"), classify=lambda rc, out: "refused" if rc or "drift" in out and not re.search(r" 0 drift", out) else ("pending" if re.search(r"[1-9]\d* file\(s\) pending", out) else "applied"), apply=a(lh, "apply", "--write"), verify="tests")
    add(id="fixup-loop-hunt-test", touch=None, title="merge fixup (test only): loop-hunt settle-repeat suite path to the observation module", check=a(fx, "--op", "loop-hunt-test"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx, "--op", "loop-hunt-test", "--write"), fixed=[L + "/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/🟦️.ts"], verify="tests")
    gs = C + "/graph-speed-tools"
    add(id="graph-speed-followup", title="graph-speed follow-up (progress.json input, shared fact table pieces) - applied only if not redundant after conc-graph", prepare=[a(gs + "/followup_ops_gen.py")], check=a(gs + "/apply.py", "apply", "--spec", "followup"),
        classify=c_spec, apply=a(gs + "/apply.py", "apply", "--spec", "followup", "--write"), verify="tests")
    ac = C + "/course-bzl-tools/staged/apply-course.py"
    add(id="course-B", title="course-B (rebased series: x-inputs, x-site, x-release, x-paths, x-proctor, reconcile; no x-graph, no cleanup)", check=a(ac, "--check"), classify=c_course, apply=a(ac, "--live"), touch=None,
        touched_cmd=a(ac, "--check", "--json"), verify="tests")
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
    return TOUCH.get(g["touch"], []) if g["touch"] else []


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
        listing = "/private/tmp/i3-one.txt"
        open(listing, "w", encoding="utf8").write(rel + "\n")
        out = "/private/tmp/i3-one.jsonl"
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
        tests = [r for r in rels if TEST_RE.search(r) and not any(h in r for h in ("/🏃️step/", "🔬️probes", "📥️worker", "⏳️slow", "🤝️shared", "📥️reader"))]
        for rel, rc, p, f, secs in run_tests(tests):
            print("   test %-5s pass=%s fail=%s %4ss %s" % ("GREEN" if rc == 0 else "RED", p, f, secs, rel[-80:]))
            if rc != 0:
                e.setdefault("verify_red", []).append(rel)
        save_ledger(ledger)
    return True


def apply_group(g, ledger):
    rels = touched(g)
    if not rels and not g["adopted"]:
        print("   REFUSED: no touched list for %s (run --sim)" % g["id"])
        return False
    rows = backup(g, rels)
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
    ledger[g["id"]] = {"time": time.strftime("%Y-%m-%d %H:%M:%S"), "files": changed, "listed": len(rels)}
    save_ledger(ledger)
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

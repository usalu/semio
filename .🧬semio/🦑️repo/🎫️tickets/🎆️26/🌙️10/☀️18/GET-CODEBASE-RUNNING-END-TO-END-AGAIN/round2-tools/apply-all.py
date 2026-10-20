#!/usr/bin/env python3
"""Round-2 apply series (integrate2): every staged-but-unapplied set in one ordered, verified, undoable run.

  apply-all.sh                          dry run (default): gates, readiness and the read-only check of every group on the live tree; writes nothing
  apply-all.sh --yes                    apply the enabled groups in order, back up the files each group touches, verify, stop on the first failure
  apply-all.sh --sim                    prove the whole chain on a private overlay under /private/tmp/i2 (never touches the checkout) and refresh touched.json
  apply-all.sh --undo <group>           restore a group from its backup while the checkout still holds exactly its result
  options: --only a,b  --skip a,b  --from <group>  --with-gated (course-B, own-output are gated: see round2-plan.md)  --include a,b (optional groups)
           --include-unready (also run sets whose result file does not say READY)  --no-verify  --verify-heavy  --keep-going (continue after a failed verification)
           --ignore-running (do not refuse while `show projects` / dependency sync runs)  --list
Cross-platform: Python standard library only; every write is a temp file + rename done by the set's own tool, the ledger and backups live under C/🗑️generated/round2-apply/."""
import glob, hashlib, json, os, re, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = HERE
while LIVE != os.path.dirname(LIVE) and not (os.path.isfile(os.path.join(LIVE, "bunfig.toml")) and os.path.isfile(os.path.join(LIVE, "bun.lock"))):
    LIVE = os.path.dirname(LIVE)
P = (glob.glob(LIVE + "/.🧬semio/*/*/*/*/*/ARCHITECTURE-COURSE-ON-BAZEL") or [""])[0]
GEN = C + "/🗑️generated/round2-apply"
LEDGER = GEN + "/ledger.json"
TOUCHED = HERE + "/touched.json"
PY, BUN = sys.executable, shutil.which("bun") or "bun"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
HELP_FIXTURE = L + "/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
TEST_RE = re.compile(r"/🧪️tests/(?:[^/]+/)*🟦️\.ts$")
args = sys.argv[1:]
flag = lambda name: name in args
opt = lambda name, default="": args[args.index(name) + 1] if name in args else default
csv = lambda name: [x for x in opt(name).split(",") if x]
sha = lambda data: hashlib.sha256(data).hexdigest()


def sh(argv, cwd=LIVE, timeout=1800, env=None):
    run = subprocess.run(argv, capture_output=True, text=True, encoding="utf8", errors="replace", cwd=cwd, timeout=timeout, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1", **(env or {})})
    return run.returncode, (run.stdout or "") + (run.stderr or "")


def tool_flag(script, candidates=("--write", "--apply")):
    try:
        text = open(script, encoding="utf8").read()
    except OSError:
        return None
    return next((c for c in candidates if '"%s"' % c in text or "'%s'" % c in text), None)


def ready(result):
    path = os.path.join(C, result)
    if not os.path.isfile(path):
        return False
    return any(re.search(r"\bREADY\b", line) for line in open(path, encoding="utf8").read().split("\n")[:40])


def lines(out, prefix):
    return [l for l in out.split("\n") if l.lower().startswith(prefix)]


def classify_pending_lines(prefixes):
    def f(rc, out):
        if rc != 0:
            return "refused"
        return "pending" if any(lines(out, p) for p in prefixes) else "applied"
    return f


def classify_course(rc, out):
    if rc not in (0, 1) or lines(out, "refused"):
        return "refused"
    return "pending" if lines(out, "pending") else "applied"


def classify_docs(rc, out):
    m = re.search(r"(\d+) pending, (\d+) applied", out)
    if rc != 0 or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def classify_hub(rc, out):
    if rc != 0:
        return "refused"
    return "applied" if '"adapter": "already patched"' in out and '"module": "present"' in out else "pending"


def classify_rc(rc, out):
    return {0: "applied", 1: "pending"}.get(rc, "refused")


def classify_testfix(rc, out):
    if rc != 0:
        return "refused"
    return "pending" if re.search(r"(?m)^ready\b", out) else "applied"


def classify_law(rc, out):
    m = re.search(r"(\d+) operations; (\d+) already applied; (\d+) drifted", out)
    if rc != 0 or not m or int(m.group(3)):
        return "refused"
    return "applied" if m.group(1) == m.group(2) else "pending"


def classify_precision(rc, out):
    m = re.search(r"(\d+) file operation", out)
    if rc != 0 or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def classify_incr(rc, out):
    m = re.search(r"(\d+) change\(s\)", out)
    if rc != 0 or not m:
        return "refused"
    return "pending" if int(m.group(1)) else "applied"


def classify_default(rc, out):
    if rc != 0:
        return "refused"
    m = re.search(r'"edit": (\d+)', out)
    return "pending" if m and int(m.group(1)) else "applied"


def classify_nocache(rc, out):
    m = re.search(r"dry run: (\d+) manifest\(s\), (\d+) target\(s\), (\d+) pending-requires", out)
    if rc != 0 or not m:
        return "refused"
    return "pending" if int(m.group(2)) or int(m.group(3)) else "applied"


def classify_play(rc, out):
    if rc == 0:
        return "pending" if lines(out, "dry-run") and any(re.search(r"bytes (\d+) -> (?!\1\b)", l) for l in lines(out, "dry-run")) else "applied"
    return "applied?" if "occurs 0 times" in out else "refused"


def classify_parity(rc, out):
    if rc == 0:
        return "pending" if lines(out, "would write") else "applied"
    return "applied?" if "drift" in out.lower() else "refused"


def parity_prepare():
    return [[PY, C + "/parity-tools/build-staged.py"]]


def tests(group, extra=()):
    rows = [p for p in TOUCH.get(group, []) if TEST_RE.search(p) and "/🏃️step/" not in p]
    return [[BUN, "test", "./" + p] for p in rows] + [list(x) for x in extra]


def guarded(directory, name, old, new):
    """With --ignore-running the tools that refuse by themselves while `show projects` runs are executed from a copy whose guard line is replaced (coordinator's limited GO 18:25: the queued show builds its graph once from the post-round-2 tree)."""
    if not flag("--ignore-running"):
        return directory
    target = os.path.join(HERE, "staged", "live-" + os.path.basename(directory))
    shutil.rmtree(target, ignore_errors=True)
    shutil.copytree(directory, target, ignore=shutil.ignore_patterns("__pycache__", "backup"))
    path = os.path.join(target, name)
    text = open(path, encoding="utf8").read()
    assert text.count(old) == 1, "guard line not found in " + path
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    return target


def groups():
    a = lambda *x: [PY, *x]
    G = []

    def add(**kw):
        kw.setdefault("sim", kw["id"])
        kw.setdefault("gated", False)
        kw.setdefault("optional", False)
        kw.setdefault("result", None)
        kw.setdefault("prepare", [])
        kw.setdefault("post", [])
        kw.setdefault("verify", [])
        kw.setdefault("heavy", [])
        kw.setdefault("extra_touched", [])
        kw.setdefault("known_red", [])
        kw.setdefault("needs", None)
        kw.setdefault("env", None)
        G.append(kw)

    mg = C + "/mem-gov-tools"
    add(id="mem-gov", title="host-capacity admission (first: must be live before any heavy Bazel/play run)", result="result-mem-gov.md",
        check=a(mg + "/apply.py"), classify=classify_pending_lines(("patch", "new", "update")), apply=a(mg + "/apply.py", "--write", *(["--ignore-runs"] if flag("--ignore-running") else [])),
        post=[[BUN, mg + "/regen-help.ts", LIVE]], verify=tests("mem-gov") + [[BUN, mg + "/regen-help.ts", LIVE, "--check"]], extra_touched=[HELP_FIXTURE])
    oc = guarded(C + "/orphan-child-tools", "apply.py", "if live:", "if False:")
    add(id="orphan-child", title="persistent-state child tolerated by the parent recheck", check=a(oc + "/apply.py", "--dry-run", "--root", LIVE + "/"),
        classify=classify_pending_lines(("would write",)), apply=a(oc + "/apply.py"), verify=lambda: tests("orphan-child"))
    bs = guarded(C + "/boot-sweep-tools", "apply.py", "if live and write:", "if False:") + "/apply.py"
    add(id="boot-sweep", title="reboot/crash recovery of the native storage partitions (device-independent identities, boot sweep, abandoned-image heal)", result="result-boot-sweep.md", check=a(bs, "--root", LIVE),
        classify=classify_pending_lines(("would write",)), apply=a(bs, "--root", LIVE, "--write"), verify=lambda: tests("boot-sweep"))
    td = C + "/testfix-defect-tools"
    add(id="testfix-defect", title="genuine-birth harness and real test defects", check=a(td + "/apply.py"), classify=classify_testfix, apply=a(td + "/apply.py", "--write"),
        verify=lambda: tests("testfix-defect"), heavy=[["sh", td + "/live.sh", "real"]])
    hb = C + "/hub-bzl-tools"
    add(id="hub-bzl", title="Go toolchain digest in the Bazel action key", check=a(hb + "/apply.py"), classify=classify_hub, apply=a(hb + "/apply.py", "--write"), verify=lambda: tests("hub-bzl"))
    rbe = HERE + "/staged/bzl-rbe"
    add(id="bzl-rbe", title="local Buildbarn remote execution (merged copy: order-independent catalogue anchors)", check=a(rbe + "/apply.py", "--dry-run", "--allow-drift"),
        classify=classify_pending_lines(("would write",)), apply=a(rbe + "/apply.py", "--allow-drift"), verify=lambda: tests("bzl-rbe"))
    db = C + "/demo-bzl-tools"
    add(id="demo-bzl", title="demonstrator + mit-bestand print on Bazel", check=a(db + "/apply.py", "--status"), classify=classify_pending_lines(("pending",)), apply=a(db + "/apply.py", "--write", "--accept-drift"),
        verify=lambda: [a(db + "/verify-independent.py")] + [[BUN, db + "/law-check.ts"]])
    nc = C + "/bzl-nocache-tools"
    add(id="bzl-nocache", sim="bzl-nocache", title="no-cache census conversions: 19 authored targets become cached actions (MERGED COPY without the course build op, owned by course-B/x-site; after demo-bzl: bericht:build waits for its output fix; supersedes cache-law for the 3 retire/cargo targets)", result="result-bzl-nocache.md",
        check=[BUN, HERE + "/staged/bzl-nocache/apply.ts"], classify=classify_nocache, apply=[BUN, HERE + "/staged/bzl-nocache/apply.ts", "--write"], prepare=[a(HERE + "/staged/make-nocache.py")], verify=lambda: [[BUN, nc + "/validate.ts"], a(nc + "/verify-independent.py")],
        heavy=[["bash", nc + "/prove.sh", phase] for phase in ("light", "bericht", "course", "wasm", "storybook")], env={"SEMIO_ROOT": LIVE})
    pb = C + "/play-bzl-tools"
    add(id="play-bzl", title="semio-tech play on Bazel (manifests)", check=a(pb + "/apply.py", "--accept-drift"), classify=classify_play, apply=a(pb + "/apply.py", "--write", "--accept-drift"), verify=lambda: [[BUN, pb + "/law-check.ts"]])
    cl = HERE + "/staged/cache-law"
    add(id="cache-law", title="cache decisions for the three peer cache:false targets (cache-correctness law findings 3 -> 0)", check=a(cl + "/apply.py"), classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"),
        apply=a(cl + "/apply.py", "--write"), verify=lambda: [[BUN, "test", "./" + L + "/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts"]])
    xf = P + "/x-ingress-tools/followup"
    add(id="x-followup", title="x-ingress follow-up (shared-only miss path, replay instead of re-walk)", check=a(xf + "/apply.py", "--check"), classify=classify_rc, apply=a(xf + "/apply.py", "--live"), verify=lambda: tests("x-followup"))
    ac = C + "/course-bzl-tools/staged/apply-course.py"
    add(id="course-A", sim="course-A", title="course series, ungated part: x-dev, x-keys, course-bzl", check=a(ac, "--check", "--only", "x-dev,x-keys,course-bzl"), classify=classify_course,
        apply=a(ac, "--live", "--only", "x-dev,x-keys,course-bzl"), verify=lambda: tests("course-A"), known_red=["🧱️local-stack/"])
    dbd = C + "/docs-build-tools"
    add(id="docs-build", title="EN/DE build guide in the product README (after bzl-rbe, soft anchors)", check=a(dbd + "/apply.py", "--rebase"), classify=classify_docs, apply=a(dbd + "/apply.py", "--write", "--rebase"),
        verify=lambda: [a(dbd + "/check.py")])
    bd = C + "/bzl-default-tools/apply.py"
    add(id="bzl-default", sim="bzl-default", title="Bazel-default preparation: tests pin their tool, narrow `tools setup bazel` storage, README rows (groups prep+setup+commands, behaviour neutral)", result="result-bzl-default.md",
        check=a(bd, "--root", LIVE, "--group", "all"), classify=classify_default, apply=a(bd, "--root", LIVE, "--group", "all", "--write", *(["--ignore-running"] if flag("--ignore-running") else [])), verify=lambda: tests("bzl-default"))
    fx = HERE + "/staged/fixups/apply.py"
    add(id="fixup-rbe-default", sim=None, title="merge fixup: bzl-default's installSkipped fixture row carries `remote-execution` (bzl-rbe added it to the entry script)", check=a(fx, "--op", "rbe-default"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx, "--op", "rbe-default", "--write"),
        extra_touched=[L + "/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🧫️fixtures/🔣️.json"], verify=[[BUN, "test", "./" + L + "/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts"]])
    si = guarded(C + "/sync-incr-tools", "apply.py", "running = busy() if guard else []", "running = []")
    add(id="sync-incr", title="incremental dependency sync", result="result-sync-incr.md", check=a(si + "/apply.py", "check", "--root", LIVE), classify=classify_rc, apply=a(si + "/apply.py", "apply", "--write", "--root", LIVE),
        verify=lambda: tests("sync-incr"), ok_rcs=(0, 1))
    tl = C + "/testfix-law-tools"
    add(id="testfix-law", title="law-drift suites and fixtures", result="result-testfix-law.md", check=a(tl + "/apply.py"), classify=classify_law, apply=a(tl + "/apply.py", "--write"), verify=lambda: tests("testfix-law"), heavy=[["bash", tl + "/live.sh"]])
    bi = HERE + "/staged/bzl-incr/apply.py"
    add(id="bzl-incr", sim="bzl-incr", title="incremental conformance harness (`repo incremental`) for nx and bazel (MERGED COPY without the generated help fixture edit; regen-help after)", result="result-bzl-incr.md", check=a(bi, "--root", LIVE), classify=classify_incr,
        apply=a(bi, "--root", LIVE, "--write"), prepare=[a(HERE + "/staged/make-incr.py")], post=[[BUN, mg + "/regen-help.ts", LIVE]], extra_touched=[HELP_FIXTURE],
        verify=lambda: tests("bzl-incr") + [[BUN, mg + "/regen-help.ts", LIVE, "--check"]])
    pr = C + "/parity-tools"
    add(id="parity", title="nx vs Bazel output parity verifier (whole-file results are rebuilt from the then-current files right before the write)", result="result-parity.md", check=a(pr + "/apply.py"),
        classify=classify_parity, apply=a(pr + "/apply.py", "--write"), prepare=parity_prepare(), verify=lambda: tests("parity"))
    add(id="regen-help", sim=None, title="help snapshots regenerated from the integrated catalogue (mem-gov, bzl-rbe and parity all add operations)", check=[BUN, mg + "/regen-help.ts", LIVE, "--check"],
        classify=lambda rc, out: "applied" if rc == 0 else "pending", apply=[BUN, mg + "/regen-help.ts", LIVE], verify=lambda: [[BUN, mg + "/regen-help.ts", LIVE, "--check"]], extra_touched=[HELP_FIXTURE])
    ip = C + "/input-precision-tools"
    add(id="input-precision", title="derived inputs for whole-repository input tasks (after every manifest change)", result="result-input-precision.md", check=a(ip + "/apply.py"), classify=classify_precision,
        apply=a(ip + "/apply.py", "--apply"), verify=lambda: tests("input-precision"))
    pgc = C + "/pg-chain-tools/apply.py"
    add(id="pg-chain", sim="pg-chain", title="plugin playground pipeline cached: component targets (store receipt), prepare/build lanes, Vite bundles (after play-bzl, demo-bzl, bzl-nocache; --rebase: two files drifted by earlier groups, anchors still exactly once)", result="result-pg-chain.md",
        check=a(pgc, "apply", "--rebase"), classify=lambda rc, out: "refused" if rc else ("pending" if re.search(r"(?m)^would (edit|write)", out) else "applied"), apply=a(pgc, "apply", "--rebase", "--write"), verify=lambda: tests("pg-chain"))
    fx2 = HERE + "/staged/fixups/apply.py"
    for op, test in (("capacity-split-cli", "🚀️bootstrap/⌨️cli"), ("capacity-split-birth", "📥️caller")):
        add(id="fixup-" + op, sim=None, title="merge fixup: the suite extracting the build entry by function name follows mem-gov's capacity split", check=a(fx2, "--op", op),
            classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx2, "--op", op, "--write"),
            extra_touched=[L + "/🏗️build/" + ("🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts" if "cli" in op else "📥️caller/🧪️tests/📥️birth/🟦️.ts")], verify=[])
    add(id="fixup-rbe-probes", sim=None, title="merge fixup: cache decision for the five bzl-rbe hermetic probe targets (cache-correctness law: unclassified-target x5, authored-cache-overridden x5)", check=a(fx2, "--op", "rbe-probes"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx2, "--op", "rbe-probes", "--write"), extra_touched=[L + "/⚡️caching/📋️project.json"],
        verify=lambda: [[BUN, "test", "./" + L + "/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts"]])
    add(id="fixup-precision-storybook", sim=None, title="merge fixup: input precision rule for build-storybook (cached by bzl-nocache, declares the whole repository)", check=a(fx2, "--op", "precision-storybook"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx2, "--op", "precision-storybook", "--write"), extra_touched=[L + "/🏗️build/📥️inputs/🎯️precision/🔣️.json"],
        verify=lambda: [[BUN, "test", "./" + L + "/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts"]])
    add(id="fixup-policy-driver", sim=None, title="follow-up: remove the obsolete exact `driver-owned` row (prepare-prototype, activate-prototype, prepare-release, activate-lanes-prototype) from the caching policy; play-bzl and demo-bzl classify these targets per manifest", check=a(fx2, "--op", "policy-driver"),
        classify=lambda rc, out: "refused" if rc else ("pending" if out.startswith("pending") else "applied"), apply=a(fx2, "--op", "policy-driver", "--write"), extra_touched=[L + "/⚡️caching/🔣️policy.json"],
        verify=lambda: [[BUN, C + "/bzl-nocache-tools/law.ts", "/private/tmp/i2/lawroot-empty"]])
    cb = C + "/course-bzl-tools/staged/apply-course.py"
    add(id="course-B", sim="course-B", gated=True, title="course series, gated part (inference/adapter/admission files frozen by the Codex ticket 26/10/02): x-inputs, x-site, x-release, x-paths, x-proctor, reconcile, x-graph",
        check=a(cb, "--check", "--with-graph", "--skip", "x-dev,x-keys,course-bzl"), classify=classify_course, apply=a(cb, "--live", "--with-graph", "--skip", "x-dev,x-keys,course-bzl"),
        verify=lambda: tests("course-B"), heavy=[["sh", P + "/x-integrate-tools/check-all.sh", "live"]])
    add(id="lock-sync", sim="bzl-default-manifest", needs="--with-lock", title="needs dependency lock + sync: remove the closed-recipe violation from the tools package.json and bun.lock (bzl-default manifest = frozen part of zt-e2e-fix), `dependencies lock --all`, then sync-incr's cold sync, a second sync (must be current)",
        result="result-bzl-default.md", check=a(bd, "--root", LIVE, "--group", "manifest"), classify=classify_default, apply=a(bd, "--root", LIVE, "--group", "manifest", "--write"),
        post=[[BUN, "repo", "dependencies", "lock", "--all"], [BUN, "repo", "dependencies", "sync"], [BUN, "repo", "dependencies", "sync"]])
    add(id="flip", sim="bzl-default-flip", gated=True, optional=True, needs="--flip-gate-passed", title="Bazel becomes the default tool (13 files): ONLY after the architecture course, semio-tech play and the mit-bestand demonstrator pass end to end on Bazel",
        result="result-bzl-default.md", check=a(HERE + "/staged/bzl-default-flip/apply.py", "--root", LIVE, "--group", "flip"), classify=classify_default,
        apply=a(HERE + "/staged/bzl-default-flip/apply.py", "--root", LIVE, "--group", "flip", "--flip-gate-passed", "--write"), prepare=[a(HERE + "/staged/make-flip.py")],
        post=[[BUN, mg + "/regen-help.ts", LIVE]], extra_touched=[HELP_FIXTURE], verify=lambda: tests("bzl-default-flip") + [[BUN, mg + "/regen-help.ts", LIVE, "--check"]])
    oo = C + "/own-output-tools/staged/apply.sh"
    add(id="own-output", gated=True, optional=True, title="shell typegen freshness remainder (Codex repaired the refusal live and froze the shell manifest; apply only if still needed)", check=["sh", oo], classify=classify_pending_lines(("pending",)),
        apply=["sh", oo, "--yes"])
    for sid, tool in (("cli-zt", "cli-zt-tools/apply.py"), ("bzl-hermetic", "bzl-hermetic-tools/apply.py"), ("graph-speed", "graph-speed-tools/apply.py")):
        script = C + "/" + tool
        w = tool_flag(script, ("--write", "--apply"))
        add(id=sid, sim=None, title="%s (no result file yet: placeholder, activates when its apply tool and a READY result exist)" % sid, result="result-%s.md" % sid, optional=False, exists=script,
            check=a(script), classify=classify_pending_lines(("would write", "write", "create", "edit", "pending")), apply=a(script, w) if w else a(script), verify=lambda sid=sid: tests(sid))
    return G


TOUCH = {}
if os.path.isfile(TOUCHED):
    TOUCH = json.load(open(TOUCHED, encoding="utf8"))


def load_ledger():
    return json.load(open(LEDGER, encoding="utf8")) if os.path.isfile(LEDGER) else {}


def save_ledger(ledger):
    os.makedirs(GEN, exist_ok=True)
    json.dump(ledger, open(LEDGER + ".tmp", "w", encoding="utf8"), ensure_ascii=False, indent=1)
    os.replace(LEDGER + ".tmp", LEDGER)


def running():
    out = sh(["ps", "-axo", "command"])[1]
    return [l[:160] for l in out.split("\n") if re.search(r"\brepo (show projects|dependencies (sync|lock))\b|dependencies (sync|lock)\b", l) and "apply-all" not in l and "grep" not in l]


def codex_gate():
    out = sh(["ps", "-axo", "pid,command"])[1]
    return [l.strip()[:200] for l in out.split("\n") if re.search(r"test-workspace-build|native-build-regression|workspace:test-workspace-build", l) and "apply-all" not in l and "grep" not in l]


def file_sha(rel):
    path = os.path.join(LIVE, rel)
    return sha(open(path, "rb").read()) if os.path.isfile(path) else None


def backup(group, rels):
    folder = os.path.join(GEN, "backup", group["id"])
    shutil.rmtree(folder, ignore_errors=True)
    os.makedirs(folder)
    rows = {}
    for index, rel in enumerate(sorted(set(rels))):
        before = file_sha(rel)
        name = None
        if before is not None:
            name = "%04d.orig" % index
            shutil.copyfile(os.path.join(LIVE, rel), os.path.join(folder, name))
        rows[rel] = {"before": before, "backup": name}
    return rows


def touched_of(group):
    rels = list(TOUCH.get(group["sim"], [])) if group.get("sim") else []
    return rels + list(group["extra_touched"])


def status(group, ledger):
    entry = ledger.get(group["id"])
    if entry and not flag("--reapply"):
        drifted = [rel for rel, row in entry["files"].items() if row.get("after") and file_sha(rel) != row["after"]]
        return "applied (ledger%s)" % ("; %d file(s) changed since by later groups" % len(drifted) if drifted else ""), ""
    if flag("--yes"):
        for command in group["prepare"]:
            prc, pout = sh(command)
            if prc:
                return "refused", "prepare failed: " + pout.strip()[-600:]
    rc, out = sh(group["check"], timeout=900, env=group["env"])
    state = group["classify"](rc, out)
    return state, out


def enabled(group, only, skip):
    if only and group["id"] not in only:
        return False, "not selected"
    if group["id"] in skip:
        return False, "skipped"
    if group.get("exists") and not os.path.isfile(group["exists"]):
        return False, "no apply tool yet"
    if group["optional"] and group["id"] not in csv("--include"):
        return False, "optional (use --include %s)" % group["id"]
    if group["needs"] and not flag(group["needs"]):
        return False, "needs %s" % group["needs"]
    if group["gated"] and not flag("--with-gated"):
        return False, "gated (use --with-gated after the Codex sessions 82722, 63416, 86151, 96264 are terminal)"
    if group["result"] and not ready(group["result"]) and not flag("--include-unready"):
        return False, "not READY (%s)" % group["result"]
    return True, ""


def run_group(group, ledger, ok_state_apply=("pending",)):
    rels = touched_of(group)
    if not rels:
        print("   REFUSED: no touched-file list for %s (add it to sim.py and run --sim); nothing could be backed up" % group["id"])
        return False
    rows = backup(group, rels)
    for command in group["prepare"]:
        rc, out = sh(command)
        if rc:
            print("   PREPARE FAILED:", out.strip()[-600:])
            return False
    rc, out = sh(group["apply"], timeout=3600, env=group["env"])
    ok = rc in group.get("ok_rcs", (0,))
    print("   apply rc=%d %s" % (rc, "ok" if ok else "FAILED"))
    if not ok:
        print("   " + out.strip()[-1200:].replace("\n", "\n   "))
        return False
    for command in group["post"]:
        prc, pout = sh(command)
        print("   post rc=%d %s" % (prc, " ".join(command[-2:])[-80:]))
        if prc:
            print("   " + pout.strip()[-800:].replace("\n", "\n   "))
            return False
    for rel, row in rows.items():
        row["after"] = file_sha(rel)
    changed = {rel: row for rel, row in rows.items() if row["after"] != row["before"]}
    ledger[group["id"]] = {"time": time.strftime("%Y-%m-%d %H:%M:%S"), "files": changed, "listed": len(rels), "apply": group["apply"][-3:]}
    save_ledger(ledger)
    print("   %d of %d listed files changed (backups: %s)" % (len(changed), len(rels), os.path.relpath(os.path.join(GEN, "backup", group["id"]), C)))
    return True


DEFER = ["🏗️preparation", "⌨️cli/🧪️tests", "portable-cli", "📥️birth", "🔬️probes", "📦️resolution", "🚮️absence", "🌱️genuine-birth", "🩺️heal", "💥️crash", "🐳️service", "🧱️local-stack", "live.sh", "prove.sh", "check-all.sh"]


def syntax_check(group, ledger):
    bad = []
    for rel in ledger.get(group["id"], {}).get("files", {}):
        path = os.path.join(LIVE, rel)
        if not os.path.isfile(path):
            continue
        if rel.endswith(".json"):
            try:
                json.load(open(path, encoding="utf8"))
            except ValueError as error:
                bad.append("%s: %s" % (rel, error))
        elif rel.endswith((".ts", ".tsx", ".mjs", ".cjs")):
            rc, out = sh([BUN, "build", "--no-bundle", path], timeout=120)
            if rc:
                bad.append("%s: %s" % (rel, out.strip()[:200]))
    return bad


def verify_group(group, ledger=None):
    plan = group["verify"]() if callable(group["verify"]) else group["verify"]
    if ledger is not None:
        bad = syntax_check(group, ledger)
        print("   syntax/json check of %d changed file(s): %s" % (len(ledger.get(group["id"], {}).get("files", {})), "ok" if not bad else "FAILED"))
        for row in bad:
            print("      " + row)
        if bad:
            return [["syntax"]]
    if flag("--defer-heavy"):
        keep, deferred = [], []
        for command in plan:
            (deferred if group["id"] == "testfix-law" or any(mark in " ".join(command) for mark in DEFER) else keep).append(command)
        if deferred:
            os.makedirs(GEN, exist_ok=True)
            path = GEN + "/deferred-verification.json"
            known = json.load(open(path, encoding="utf8")) if os.path.isfile(path) else {}
            known[group["id"]] = [" ".join(c) for c in deferred + group["heavy"]]
            json.dump(known, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
            print("   deferred verification: %d command(s) recorded" % len(deferred + group["heavy"]))
        plan = keep
    if flag("--verify-heavy"):
        plan = plan + group["heavy"]
    failures = []
    for command in plan:
        started = time.time()
        rc, out = sh(command, timeout=600)
        print("   verify rc=%d %5.0fs %s" % (rc, time.time() - started, " ".join(os.path.basename(x) if i == 0 else x[-70:] for i, x in enumerate(command))[-110:]))
        if rc and any(mark in " ".join(command) for mark in group["known_red"]):
            print("      known red before the set (result-course-bzl G9: two Vite-proxy cases under Bun); not counted")
        elif rc:
            failures.append(command)
            print("      " + out.strip()[-500:].replace("\n", "\n      "))
    return failures


def undo(name):
    ledger = load_ledger()
    entry = ledger.get(name)
    if not entry:
        sys.exit("nothing recorded for " + name)
    later = [g for g, e in ledger.items() if e["time"] > entry["time"] and set(e["files"]) & set(entry["files"])]
    if later and not flag("--force"):
        sys.exit("refused: later groups touched the same files: %s (undo them first, newest first)" % ", ".join(later))
    for rel, row in entry["files"].items():
        if file_sha(rel) != row["after"]:
            sys.exit("refused: %s changed after the group was applied" % rel)
    folder = os.path.join(GEN, "backup", name)
    for rel, row in entry["files"].items():
        target = os.path.join(LIVE, rel)
        if row["before"] is None:
            os.remove(target)
            parent = os.path.dirname(target)
            while parent != LIVE and not os.listdir(parent):
                os.rmdir(parent)
                parent = os.path.dirname(parent)
        else:
            temporary = target + ".r2undo"
            shutil.copyfile(os.path.join(folder, row["backup"]), temporary)
            os.replace(temporary, target)
    del ledger[name]
    save_ledger(ledger)
    print("restored %d file(s) of %s" % (len(entry["files"]), name))


def simulate(selected):
    order = ",".join(g["sim"] for g in selected if g.get("sim"))
    extra = []
    for g in selected:
        if g["id"] == "docs-build":
            extra.append("--arg-docs-build=--rebase")
        if g["id"] == "play-bzl":
            extra.append("--arg-play-bzl=--accept-drift")
        if g["id"] == "bzl-rbe":
            extra.append("--arg-bzl-rbe=--allow-drift")
    subprocess.run([PY, HERE + "/seeds.py"], check=True)
    subprocess.run([PY, HERE + "/staged/make-flip.py"], check=True)
    subprocess.run([PY, HERE + "/staged/make-nocache.py"], check=True)
    subprocess.run([PY, HERE + "/staged/make-incr.py"], check=True)
    rc = subprocess.run([PY, HERE + "/sim.py", "--tag", "touch", "--seed-extra", HERE + "/seed-extra.txt", "--continue", "--order", order, *extra]).returncode
    result = json.load(open(GEN + "/sim-touch/result.json", encoding="utf8"))["result"]
    mapping = {row["set"]: row["changed"] for row in result}
    json.dump(mapping, open(TOUCHED, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("touched.json refreshed:", {k: len(v) for k, v in mapping.items()})
    return rc


def main():
    if flag("--undo"):
        return undo(opt("--undo"))
    only, skip = csv("--only"), csv("--skip")
    sh([PY, HERE + "/staged/make-flip.py"])
    sh([PY, HERE + "/staged/make-nocache.py"])
    sh([PY, HERE + "/staged/make-incr.py"])
    all_groups = groups()
    if flag("--from"):
        names = [g["id"] for g in all_groups]
        start = names.index(opt("--from"))
        skip += names[:start]
    ledger = load_ledger()
    selected, listing = [], []
    for g in all_groups:
        ok, why = enabled(g, only, skip)
        (selected if ok else listing).append(g if ok else (g, why))
    if flag("--list"):
        for g in selected:
            print("ON  ", g["id"], "-", g["title"])
        for g, why in listing:
            print("OFF ", g["id"], "-", why)
        return 0
    if flag("--sim"):
        return simulate(selected)
    blockers = [] if flag("--ignore-running") else running()
    if blockers:
        print("REFUSED while a dependency sync or `show projects` runs (any write aborts it):\n  " + "\n  ".join(blockers))
        if flag("--yes"):
            return 2
    if flag("--with-gated"):
        gate = codex_gate()
        if gate:
            print("GATE CLOSED: the Codex regression is still running:\n  " + "\n  ".join(gate))
            if flag("--yes"):
                return 2
    print("%s on %s\n" % ("APPLY" if flag("--yes") else "DRY RUN", LIVE))
    outcome = []
    for g in selected:
        state, out = status(g, ledger)
        print("%-16s %-18s %s" % (g["id"], state, g["title"]))
        if not flag("--yes"):
            if state.startswith("refused") or state == "applied?":
                print("   " + out.strip()[-500:].replace("\n", "\n   "))
            outcome.append((g["id"], state))
            continue
        if state.startswith("applied") and state != "applied?":
            if flag("--verify-only"):
                failures = verify_group(g, ledger)
                if failures and not flag("--keep-going"):
                    print("STOPPED: verification of %s failed" % g["id"])
                    return 5
            outcome.append((g["id"], state))
            continue
        if state == "refused" or state == "applied?":
            print("   STOP: " + state + "\n   " + out.strip()[-800:].replace("\n", "\n   "))
            return 3
        if not run_group(g, ledger):
            print("STOPPED at %s (nothing later was applied; undo with --undo %s)" % (g["id"], g["id"]))
            return 4
        if not flag("--no-verify"):
            failures = verify_group(g, ledger)
            if failures and not flag("--keep-going"):
                print("STOPPED: verification of %s failed (undo with --undo %s)" % (g["id"], g["id"]))
                return 5
        outcome.append((g["id"], "applied now"))
    for g, why in listing:
        if why != "not selected":
            print("%-16s %-18s %s" % (g["id"], "off", why))
    os.makedirs(GEN, exist_ok=True)
    json.dump({"time": time.strftime("%H:%M:%S"), "outcome": outcome}, open(GEN + "/last-run.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
    return 0


sys.exit(main())

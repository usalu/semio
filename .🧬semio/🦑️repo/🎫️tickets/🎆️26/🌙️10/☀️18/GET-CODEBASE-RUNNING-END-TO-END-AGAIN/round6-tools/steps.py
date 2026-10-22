"""Round-6 step table: the ordered chain (check = dry run, write = apply). A command is an argv list or {cmd, env, cwd}; `ok` lists accepted return codes."""
import os

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"

TITLES = {
    "assert-no-repo-repo": "assertion: the finished tree has 0 `repo repo` (command-form codemod idempotence)",
    "ledger-root-budget": "cache ledger: repository cache root budget 24 GiB (remote-cache 3 GiB + remote-execution 512 MiB do not fit into the 0.25 GiB that is left of 20 GiB)",
    "catalogue-neutral": "catalogue rows added after backend-contract (units, bench, service) carry no `implementations` property (backend-contract law)",
    "cache-gc-4": "cache-gc staged4: incremental write-time closure proof (per-group member indexes; 150 s / 10 GiB -> about 3 s); the maximumExpandedBytes 256 MiB drift is accepted",
    "backend-contract-recut": "backend-contract READY3: ratchet re-cut at the END of the chain (recut.sh)",
    "graph-parallel": "graph-parallel: multi-core inference farm (after graph-mem and conc-graph; COPY with ops re-derived from the tree)",
    "pm-seam-3": "pm-seam READY3 (round 3, LAST step: script.ts, cache-gc launch, managers files, re-pinned fixtures; needs pm-seam + pm-seam-2)",
    "rexec-inhouse": "rexec-inhouse: in-house Bazel remote execution `repo remote-execution serve|status|workers` (needs rcache-server)",
    "repo-doctor": "repo-doctor: `repo doctor` one-shot diagnostics + safe --fix (after repo-launcher; the help snapshot is the one regeneration)",
    "cache-gc-3": "cache-gc staged3 (subsumes staged2): graph image budget, background reclaim (--recreate-graph only in the final cold step, not here)",
    "legacy-purge": "legacy-purge READY2: legacy readers/aliases removed (right after tool-iface), 137 old tickets converted",
    "agent-session": "agent-session: core + routing + service (COPY without its product-README edit: docs-1021a documents agent sessions; service needs hot-cache)",
    "perf-slo": "perf-slo: SLO table and `repo bench` (core + ci; the help snapshot is the one regeneration)",
    "mem-gov": "mem-gov READY3: host-capacity governor (5 files, foreign load never refuses)",
    "revision-fix": "conc-graph revision-fix: stale-reader self check (revision module + law test)",
    "cli-latency-hooks": "cli-latency: phase profile hooks (MERGED COPY: anchors carry the live implementation pin)",
    "cli-latency": "cli-latency: command custody, child-door validation, revision stamp, sweep interval, lazy contract (MERGED COPY: script edits of door-validate dropped, revision stamp on the revision-fix text)",
    # triage-r4 is applied live (dropped from round 6)
    "rcache-server": "rcache-server: `repo remote-cache serve|status|gc` + CI workflow line (MERGED COPY on cache-gc staged2: `--deep` flag)",
    "lock-sync": "lock-sync ROUND 5 + ide-free manifest rows (MERGED PLAN: closure edit re-anchored on the profiled door)",
    "sync-incr-3": "sync-incr staged3: install-equivalent generation adoption",
    "pm-seam": "pm-seam: package-manager seam, round 1 (after round 5)",
    "pm-seam-2": "pm-seam READY2: neutral install/generation schemas filled by manager descriptors (round 2 on top of round 1)",
    "launcher-install": "live only: build and install the full `repo` (launcher + mcp) and prove the pinned bun 1.4.2 script route",
    "repo-launcher-cargo": "repo-launcher group cargo: rustc wrapper = repo (after the launcher is installed)",
    "repo-launcher": "repo-launcher groups taxonomy, launcher, rust, bootstrap, docs: native launcher, catalogue-derived routing, bootstrap, cargo, docs (COPY: plan rebuilt from the tree by make-plan.py)",
    "manifest-neutral": "repo-launcher codemod: runtime-neutral manifest commands (874 manifests)",
    "command-form": "repo-launcher codemod: consumers, tests, docs (about 5.7k files)",
    "tool-iface": "tool-iface: tool registry (COPY rebuilt from the tree: catalogue anchors on `remote-cache`, door anchor on the profiled door)",
    "runtime-seam": "runtime-seam: Bun APIs behind the runtime interface (COPY: TLS serve option for the remote-cache server, regenerated manual edits and law ledger)",
    "storage-seam": "storage-seam: storage backend seam (MERGED COPY: platform image `opening` scope)",
    "backend-contract": "backend-contract: build backend registry + conformance (MERGED COPY: ledger owner enum after rcache-server)",
    "polyglot-units": "polyglot-units READY2: `repo units` (MERGED COPY: help snapshot left to the one regeneration)",
    "hot-cache": "hot-cache groups 1-3: resident repo service (MERGED COPY: three schema/catalogue anchors from the tree; run twice: the waiting part)",
    "ide-free": "ide-free READY2: no IDE extension, client plans archived (COPY rebuilt from the tree; manifest rows travel with lock-sync)",
    "langs-conc": "langs-conc READY2: pinned toolchains, isolated environments (MERGED COPY: Windows bootstrap header after repo-launcher; the live apply downloads the pinned Go)",
    "graph-mem": "graph-mem: bounded graph memory (MERGED COPY: collector loop, implementation URL list)",
    "conc-cargo-scheduler-test": "conc-cargo scheduler-test group (the scheduler fix itself is live)",
    "regen-help": "one help snapshot regeneration from the final catalogue",
    "docs-1021a": "docs-1021a READY2 (successor of docs-1020e): concurrency / incremental / shared-cache build guide EN+DE (COPY keeping the round-4 apply state; edits re-derived from the pristine README backup, README rebased)",
}


def steps(OV, COV, LIVE):
    PY = "python3"
    BUN = "bun"
    t = lambda name: f"{COV}/{name}-tools"
    S = COV + "/round6-tools/staged"
    st = []

    def add(id, check, write, **kw):
        st.append({"id": id, "check": check, "write": write, **kw})

    add("mem-gov", [[PY, t("mem-gov") + "/apply.py", "--tolerate-drift"]], [[PY, t("mem-gov") + "/apply.py", "--tolerate-drift", "--write", "--ignore-runs"]])
    add("revision-fix", [[PY, t("conc-graph") + "/revision-fix.py", "--root", OV]], [[PY, t("conc-graph") + "/revision-fix.py", "--root", OV, "--write"]], ok=(0,))
    CL = S + "/cli-latency"
    add("cli-latency-hooks", [[PY, CL + "/apply-profile-hooks.py"]], [[PY, CL + "/apply-profile-hooks.py", "--write"]], prepare=[[PY, S + "/make-cli-latency.py"]])
    add("cli-latency", [[PY, CL + "/staged/apply.py"]], [[PY, CL + "/staged/apply.py", "--write"]])
    add("cache-gc-3", [[PY, t("cache-gc") + "/apply3.py", "--tolerate-drift"]], [[PY, t("cache-gc") + "/apply3.py", "--tolerate-drift", "--write", "--ignore-runs"]])
    CG = S + "/cache-gc"
    add("cache-gc-4", [[PY, CG + "/apply4.py", "--tolerate-drift"]], [[PY, CG + "/apply4.py", "--tolerate-drift", "--write", "--ignore-runs"]], prepare=[[PY, S + "/make-cache-gc.py"]], env={"CACHE_GC_ROOT": OV})
    FX = S + "/fixups"
    RS = S + "/rcache-server"
    add("rcache-server", [[PY, RS + "/apply.py", "--root", OV], [PY, FX + "/rcache-workflow.py", "--root", OV]], [[PY, RS + "/apply.py", "--root", OV, "--write"], [PY, FX + "/rcache-workflow.py", "--root", OV, "--write"]], prepare=[[PY, S + "/make-rcache-server.py"]])
    RX = S + "/rexec-inhouse/apply.py"
    add("rexec-inhouse", [[PY, RX, "--root", OV]], [[PY, RX, "--root", OV, "--write", "--no-help"]], prepare=[[PY, S + "/make-rexec-inhouse.py"]])
    add("ledger-root-budget", [[PY, FX + "/ledger-root-budget.py", OV]], [[PY, FX + "/ledger-root-budget.py", OV, "--write"]], ok=(0, 1))
    LP = S + "/lock-sync-plan.json"
    add("lock-sync", [[PY, t("lock-sync") + "/apply.py", "--root", OV, "--plan", LP]], [[PY, t("lock-sync") + "/apply.py", "--root", OV, "--plan", LP, "--write", "--ignore-running"]], prepare=[[PY, S + "/make-lock-sync.py"]])
    PMS = S + "/pm-seam/apply.py"
    add("pm-seam", [[PY, PMS, "check", "--root", OV]], [[PY, PMS, "apply", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, S + "/make-pm-seam.py"], [PY, FX + "/pm-seam-rebase-rewrites.py", S + "/pm-seam", OV, "staged"]])
    add("pm-seam-2", [[PY, PMS, "apply", "--round", "2", "--root", OV]], [[PY, PMS, "apply", "--round", "2", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, FX + "/pm-seam-rebase-rewrites.py", S + "/pm-seam", OV, "staged2"]])
    SI = S + "/sync-incr-3/apply.py"
    add("sync-incr-3", [[PY, SI, "check", "--root", OV]], [[PY, SI, "apply", "--write", "--root", OV]], ok=(0, 1), prepare=[[PY, S + "/make-sync-incr-3.py"]])
    RL = COV + "/r6-repo-launcher-tools"
    G1 = "taxonomy,launcher,rust,bootstrap,docs"
    add("repo-launcher", [[PY, RL + "/apply.py", "--root", OV, "--ignore-running", "--group", G1]], [[PY, RL + "/apply.py", "--root", OV, "--apply", "--ignore-running", "--group", G1]], prepare=[[PY, S + "/make-repo-launcher.py"]], env={"SEMIO_ROOT": OV})
    add("launcher-install", [["true"]], [["sh", S + "/launcher-install.sh"]], env={"SEMIO_ROOT": OV})
    add("repo-launcher-cargo", [[PY, RL + "/apply.py", "--root", OV, "--ignore-running", "--group", "cargo"]], [[PY, RL + "/apply.py", "--root", OV, "--apply", "--ignore-running", "--group", "cargo"]], env={"SEMIO_ROOT": OV})
    RD = S + "/repo-doctor/apply.py"
    add("repo-doctor", [[PY, RD, "--root", OV, "--ignore-running"]], [[PY, RD, "--root", OV, "--write", "--ignore-running"]], prepare=[[PY, S + "/make-repo-doctor.py"]])
    add("manifest-neutral", [[PY, COV + "/r6-repo-launcher-tools/manifest-neutral.py", "--root", OV]], [[PY, COV + "/r6-repo-launcher-tools/manifest-neutral.py", "--root", OV, "--apply"]])
    add("command-form", [[PY, COV + "/r6-repo-launcher-tools/command-form-codemod.py", "--root", OV]], [[PY, COV + "/r6-repo-launcher-tools/command-form-codemod.py", "--root", OV, "--apply"]])
    RT = COV + "/r6-runtime-seam-tools"
    add("runtime-seam", [[BUN, RT + "/apply.ts", "--root", OV]], [[BUN, RT + "/apply.ts", "--root", OV, "--write"]], prepare=[[PY, S + "/make-runtime-seam.py"]], ok=(0, 1), refuse_if=r"(?m)^(REFUSED|NOTHING WRITTEN)")
    TI = COV + "/r6-tool-iface-tools"
    add("tool-iface", [["sh", S + "/tool-iface-regen.sh"], [PY, TI + "/apply.py", "apply", "--root", OV]], [["sh", S + "/tool-iface-regen.sh"], [PY, TI + "/apply.py", "apply", "--write", "--root", OV]], env={"R6_SCRATCH": "/private/tmp/i6/scratch"}, prepare=[[PY, S + "/make-tool-iface.py"]])
    LP2 = COV + "/r6-legacy-purge-tools"
    add("legacy-purge", [[PY, LP2 + "/apply.py", "check", "--root", OV]], [[PY, LP2 + "/apply.py", "apply", "--write", "--root", OV]], ok=(0, 1), prepare=[[PY, S + "/make-legacy-purge.py"]])
    SS = S + "/storage-seam"
    add("storage-seam", [[PY, SS + "/apply.py", "--root", OV]], [[PY, SS + "/apply.py", "--root", OV, "--write", "--backup", OV + "-storage-backup"]], prepare=[[PY, S + "/make-storage-seam.py"], [PY, FX + "/storage-seam-rebase-rewrites.py", SS, OV, LIVE]])
    BC = S + "/backend-contract"
    add("backend-contract", [[PY, BC + "/apply.py", "--root", OV]], [[PY, BC + "/apply.py", "--root", OV, "--write", "--force-running"]], prepare=[[PY, S + "/make-backend-contract.py"]])
    PU = S + "/polyglot-units"
    add("polyglot-units", [[PY, PU + "/apply.py", "--root", OV]], [[PY, PU + "/apply.py", "--root", OV, "--apply"]], prepare=[[PY, S + "/make-polyglot-units.py"]])
    HC = S + "/hot-cache"
    add("hot-cache", [[PY, HC + "/apply.py", "--root", OV]], [[PY, HC + "/apply.py", "--root", OV, "--write"], [PY, HC + "/apply.py", "--root", OV, "--write"]], prepare=[[PY, S + "/make-hot-cache.py"]], env={"SEMIO_ROOT": OV})
    AS = S + "/agent-session/apply.py"
    add("agent-session", [[PY, AS, "--root", OV, "--group", "core"], [PY, AS, "--root", OV, "--group", "routing"], [PY, AS, "--root", OV, "--group", "service"]], [[PY, AS, "--root", OV, "--group", "core", "--write", "--backup", OV + "-as-core"], [PY, AS, "--root", OV, "--group", "routing", "--write", "--backup", OV + "-as-routing"], [PY, AS, "--root", OV, "--group", "service", "--write", "--backup", OV + "-as-service"]], prepare=[[PY, S + "/make-agent-session.py"]])
    PS = t("perf-slo") + "/apply.py"
    add("perf-slo", [[PY, PS, "--root", OV, "--group", "core", "--dry-run"], [PY, PS, "--root", OV, "--group", "ci", "--dry-run"]], [[PY, PS, "--root", OV, "--group", "core", "--write"], [PY, PS, "--root", OV, "--group", "ci", "--write"]])
    add("catalogue-neutral", [[PY, FX + "/catalog-drop-implementations.py", OV]], [[PY, FX + "/catalog-drop-implementations.py", OV, "--write"]], ok=(0, 1))
    IF = COV + "/r6-ide-free-tools"
    add("ide-free", [[PY, IF + "/apply.py", "--root", OV, "--without", "manifests"]], [[PY, IF + "/apply.py", "--root", OV, "--write", "--without", "manifests", "--allow-frozen", "--ignore-running"]], prepare=[[PY, S + "/make-ide-free.py"]])
    LC = S + "/langs-conc"
    add("langs-conc", [[PY, LC + "/apply.py"]], [[PY, LC + "/apply.py", "--apply", "--coordinator-go"]], prepare=[[PY, S + "/make-langs-conc.py"]], sim_prepare=[[PY, FX + "/sim-langs-conc.py", LC + "/apply.py"]])
    GM = S + "/graph-mem"
    add("graph-mem", [[PY, GM + "/apply.py", "check", "--root", OV]], [[PY, GM + "/apply.py", "apply", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, S + "/make-graph-mem.py"]])
    GP = S + "/graph-parallel/apply.py"
    add("graph-parallel", [[PY, GP, "check", "--root", OV]], [[PY, GP, "apply", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, S + "/make-graph-parallel.py"]], env={"SEMIO_ROOT": OV})
    add("conc-cargo-scheduler-test", [[PY, t("conc-cargo") + "/apply.py", "apply", "--groups", "scheduler-test", "--ignore-sync"]], [[PY, t("conc-cargo") + "/apply.py", "apply", "--write", "--groups", "scheduler-test", "--ignore-sync"]])
    add("regen-help", [[BUN, t("mem-gov") + "/regen-help.ts", OV, "--check"]], [[BUN, t("mem-gov") + "/regen-help.ts", OV]])
    DD = COV + "/r6-docs-1021a-tools"
    add("docs-1021a", [[PY, "-I", DD + "/make-edits.py"], [PY, "-I", DD + "/check.py"], [PY, "-I", DD + "/apply.py", "--rebase"]], [[PY, "-I", DD + "/make-edits.py"], [PY, "-I", DD + "/check.py"], [PY, "-I", DD + "/apply.py", "--write", "--rebase"]], prepare=[[PY, S + "/make-docs.py"]])
    add("pm-seam-3", [[PY, PMS, "apply", "--round", "3", "--root", OV, "--ignore-running"]], [[PY, PMS, "apply", "--round", "3", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, FX + "/pm-seam-rebase-rewrites.py", S + "/pm-seam", OV, "staged3"]])
    add("backend-contract-recut", [["sh", "-c", "test -f " + FX + "/bkc-ratchet.sh"]], [["sh", FX + "/bkc-ratchet.sh", BC, OV]], env={"BKC_ROOT": OV})
    add("assert-no-repo-repo", [[PY, FX + "/assert-no-repo-repo.py", OV]], [[PY, FX + "/assert-no-repo-repo.py", OV]], env={"R6_MARKER": "/private/tmp/i6/ov-born"})
    early = [r for r in st if r["id"] in ("repo-launcher", "launcher-install", "repo-launcher-cargo")]
    st = [r for r in st if r not in early]
    at = [r["id"] for r in st].index("revision-fix") + 1
    st[at:at] = early
    skipped = {"cache-gc-3", "cache-gc-4", "langs-conc", "docs-1021a", "mem-gov", "revision-fix", "repo-launcher", "launcher-install", "graph-parallel"}
    st = [r for r in st if r["id"] not in skipped]
    for row in st:
        row["title"] = TITLES.get(row["id"], "")
        row["view"] = row["id"] not in ("runtime-seam", "command-form", "manifest-neutral", "legacy-purge", "ide-free", "regen-help", "docs-1021a", "assert-no-repo-repo", "backend-contract-recut", "ledger-root-budget", "catalogue-neutral")
    return st

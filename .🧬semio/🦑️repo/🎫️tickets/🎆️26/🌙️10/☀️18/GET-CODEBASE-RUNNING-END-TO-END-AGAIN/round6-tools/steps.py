"""Round-6 step table: the ordered chain (check = dry run, write = apply). A command is an argv list or {cmd, env, cwd}; `ok` lists accepted return codes."""
import os

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"


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
    add("cache-gc-2", [[PY, t("cache-gc") + "/apply2.py", "--tolerate-drift"]], [[PY, t("cache-gc") + "/apply2.py", "--tolerate-drift", "--write", "--ignore-runs"]])
    add("triage-r4", [[PY, t("triage-r4") + "/stage-source-demand.py"]], [["sh", t("triage-r4") + "/apply-staged.sh"]])
    FX = S + "/fixups"
    RS = S + "/rcache-server"
    add("rcache-server", [[PY, RS + "/apply.py", "--root", OV], [PY, FX + "/rcache-workflow.py", "--root", OV]], [[PY, RS + "/apply.py", "--root", OV, "--write"], [PY, FX + "/rcache-workflow.py", "--root", OV, "--write"]], prepare=[[PY, S + "/make-rcache-server.py"]])
    LP = S + "/lock-sync-plan.json"
    add("lock-sync", [[PY, t("lock-sync") + "/apply.py", "--root", OV, "--plan", LP]], [[PY, t("lock-sync") + "/apply.py", "--root", OV, "--plan", LP, "--write", "--ignore-running"]], prepare=[[PY, S + "/make-lock-sync.py"]])
    add("sync-incr-3", [[PY, t("sync-incr") + "/staged3/apply.py", "check", "--root", OV]], [[PY, t("sync-incr") + "/staged3/apply.py", "apply", "--write", "--root", OV]], ok=(0, 1))
    add("pm-seam", [[PY, t("pm-seam") + "/apply.py", "check", "--root", OV]], [[PY, t("pm-seam") + "/apply.py", "apply", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1))
    add("repo-launcher", [[PY, t("repo-launcher") + "/apply.py", "--root", OV, "--ignore-running"]], [[PY, t("repo-launcher") + "/apply.py", "--root", OV, "--apply", "--ignore-running"]])
    add("manifest-neutral", [[PY, t("repo-launcher") + "/manifest-neutral.py", "--root", OV]], [[PY, t("repo-launcher") + "/manifest-neutral.py", "--root", OV, "--apply"]])
    add("command-form", [[PY, t("repo-launcher") + "/command-form-codemod.py", "--root", OV]], [[PY, t("repo-launcher") + "/command-form-codemod.py", "--root", OV, "--apply"]])
    TI = COV + "/r6-tool-iface-tools"
    add("tool-iface", [["sh", S + "/tool-iface-regen.sh"], [PY, TI + "/apply.py", "apply", "--root", OV]], [["sh", S + "/tool-iface-regen.sh"], [PY, TI + "/apply.py", "apply", "--write", "--root", OV]], env={"R6_SCRATCH": "/private/tmp/i6/scratch"}, prepare=[[PY, S + "/make-tool-iface.py"]])
    RT = COV + "/r6-runtime-seam-tools"
    add("runtime-seam", [[BUN, RT + "/apply.ts", "--root", OV]], [[BUN, RT + "/apply.ts", "--root", OV, "--write"]], prepare=[[PY, S + "/make-runtime-seam.py"]])
    add("storage-seam", [[PY, t("storage-seam") + "/apply.py", "--root", OV]], [[PY, t("storage-seam") + "/apply.py", "--root", OV, "--write", "--backup", OV + "-storage-backup"]])
    BC = S + "/backend-contract"
    add("backend-contract", [[PY, BC + "/apply.py", "--root", OV]], [[PY, BC + "/apply.py", "--root", OV, "--write", "--force-running"]], prepare=[[PY, S + "/make-backend-contract.py"]])
    PU = S + "/polyglot-units"
    add("polyglot-units", [[PY, PU + "/apply.py", "--root", OV]], [[PY, PU + "/apply.py", "--root", OV, "--apply"]], prepare=[[PY, S + "/make-polyglot-units.py"]])
    add("hot-cache", [[PY, t("hot-cache") + "/staged/apply.py", "--root", OV]], [[PY, t("hot-cache") + "/staged/apply.py", "--root", OV, "--write"], [PY, t("hot-cache") + "/staged/apply.py", "--root", OV, "--write"]])
    IF = COV + "/r6-ide-free-tools"
    add("ide-free", [[PY, IF + "/apply.py", "--root", OV, "--without", "manifests"]], [[PY, IF + "/apply.py", "--root", OV, "--write", "--without", "manifests", "--allow-frozen", "--ignore-running"]], prepare=[[PY, S + "/make-ide-free.py"]])
    add("langs-conc", [[PY, t("langs-conc") + "/apply.py"]], [[PY, t("langs-conc") + "/apply.py", "--apply", "--coordinator-go"]], sim_prepare=[[PY, FX + "/sim-langs-conc.py", t("langs-conc") + "/apply.py"]])
    GM = S + "/graph-mem"
    add("graph-mem", [[PY, GM + "/apply.py", "check", "--root", OV]], [[PY, GM + "/apply.py", "apply", "--write", "--root", OV, "--ignore-running"]], ok=(0, 1), prepare=[[PY, S + "/make-graph-mem.py"]])
    add("conc-cargo-scheduler-test", [[PY, t("conc-cargo") + "/apply.py", "apply", "--groups", "scheduler-test", "--ignore-sync"]], [[PY, t("conc-cargo") + "/apply.py", "apply", "--write", "--groups", "scheduler-test", "--ignore-sync"]])
    add("regen-help", [[BUN, t("mem-gov") + "/regen-help.ts", OV, "--check"]], [[BUN, t("mem-gov") + "/regen-help.ts", OV]])
    add("docs-1020e", [["sh", t("docs-1020e") + "/refresh.sh"]], [["sh", t("docs-1020e") + "/refresh.sh", "--write"]])
    return st

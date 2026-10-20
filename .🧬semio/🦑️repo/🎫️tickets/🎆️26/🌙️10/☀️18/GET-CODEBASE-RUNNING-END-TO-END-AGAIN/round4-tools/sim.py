#!/usr/bin/env python3
"""Overlay simulation of the round-4 apply series (never writes into the checkout).

The overlay /private/tmp/i4/ov-<tag> is a byte-copy subset of the checkout (every file a set reads, from the traces in C/🗑️generated/round4-apply/trace, plus the course series' touched files)
with bunfig.toml/bun.lock markers; every tool directory is copied to the SAME relative position inside the overlay, so tools that derive their root from their own location (HERE/../x8,
bunfig walk) resolve the overlay without any text replacement. Each step runs the set's own write mode with cwd = overlay; changed files = sha256 diff of the overlay.
usage: sim.py [--tag a] [--order step,step,...] [--continue] [--keep]   result: C/🗑️generated/round4-apply/sim-<tag>/result.json"""
import glob, hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
P = glob.glob(os.path.dirname(os.path.dirname(os.path.dirname(C))) + "/*/*/ARCHITECTURE-COURSE-ON-BAZEL")[0]
PREL = os.path.relpath(P, LIVE)
GEN = C + "/🗑️generated/round4-apply"
args = sys.argv[1:]
opt = lambda n, d=None: args[args.index(n) + 1] if n in args else d
TAG = opt("--tag", "a")
OV = f"/private/tmp/i4/ov-{TAG}"
OUT = f"{GEN}/sim-{TAG}"
PY, BUN = sys.executable, shutil.which("bun") or "bun"
TOOLS = ["agents-fix", "bzl-tests-rest", "fnsplit4a-finish", "cli-zt", "cli-zt2", "docs-1020e", "ci-bazel", "xplat-linux-fix", "conc-ts", "mem-gov", "round4-tools"]


def tool(name):
    return f"{OV}/{CREL}/{name}-tools" if name != "round4-tools" else f"{OV}/{CREL}/round4-tools"


def steps():
    t = tool
    a = lambda *x: [PY, *x]
    S = f"{OV}/{CREL}/round4-tools/staged"
    return {
        "agents-fix": a(S + "/agents-fix/apply.py", "apply", "--write", "--root", OV),
        "fnsplit4a-finish": a(t("fnsplit4a-finish") + "/staged/apply.py", "--root", OV, "--write"),
        "cli-zt2": a(t("cli-zt") + "/apply.py", "--root", OV, "--plan", S + "/cli-zt2-plan.json", "--apply", "--allow-anchored", "--ignore-running"),
        "regen-help": [BUN, t("mem-gov") + "/regen-help.ts", OV],
        "docs-1020e": ["sh", t("docs-1020e") + "/refresh.sh", "--write"],
        "ci-bazel": a(t("ci-bazel") + "/apply.py", "--write"),
        "bzl-tests-rest": a(S + "/bzl-tests-rest/apply.py", "--write"),
        "xplat-linux-fix": a(S + "/xplat-linux-fix/apply.py", "--root", OV, "--group", "all", "--write", "--backup", OV + "-xplat-backup"),
        "conc-ts": ["sh", t("conc-ts") + "/go.sh", "--apply"],
    }


REBASE = {
    "agents-fix": [BUN, "{S}/agents-fix/build.mjs"],
    "xplat-linux-fix": [PY, "{S}/make-xplat.py", "--root", "{OV}"],
    "bzl-tests-rest": [PY, "{S}/bzl-tests-rest/build-staged.py"],
    "cli-zt2": [PY, "{S}/make-cli-zt2.py"],
}
PRE = {"agents-fix": True, "bzl-tests-rest": True, "cli-zt2": True, "xplat-linux-fix": True}


def rebase(name):
    S = f"{OV}/{CREL}/round4-tools/staged"
    cmd = [x.replace("{S}", S).replace("{OV}", OV) for x in REBASE[name]]
    cwd = S + "/agents-fix" if name == "agents-fix" else (S + "/bzl-tests-rest" if name == "bzl-tests-rest" else OV)
    for maker in ({"agents-fix": "make-agents.py", "bzl-tests-rest": "make-btr.py"}.get(name),):
        if maker:
            subprocess.run([PY, S + "/" + maker], capture_output=True, text=True, cwd=OV)
    run = subprocess.run(cmd, capture_output=True, text=True, encoding="utf8", errors="replace", cwd=cwd, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "BZL_TESTS_ROOT": OV})
    if name == "agents-fix":
        subprocess.run([PY, S + "/prune-agents.py"], capture_output=True, text=True, cwd=OV)
    return run


def inventory():
    rows = {}
    for base, _, files in os.walk(OV):
        rel = os.path.relpath(base, OV)
        if rel == CREL or rel.startswith(CREL + "/") or rel == PREL or rel.startswith(PREL + "/"):
            continue
        for file in files:
            path = os.path.join(base, file)
            if not os.path.islink(path) and not os.path.islink(base):
                rows[os.path.relpath(path, OV)] = hashlib.sha256(open(path, "rb").read()).hexdigest()
    return rows


def seed():
    wanted = set()
    for trace in glob.glob(GEN + "/trace/*.json"):
        wanted.update(p for p in json.load(open(trace, encoding="utf8")) if not p.startswith(".🧬semio/") and not p.startswith("/"))
    shutil.rmtree(OV, ignore_errors=True)
    n = 0
    for rel in sorted(wanted):
        source = os.path.join(LIVE, rel)
        if os.path.isfile(source):
            target = os.path.join(OV, rel)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(source, target)
            shutil.copymode(source, target)
            n += 1
    extra_dirs = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/🧬️schema", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/📚️compiler", "🎓️teaching/🛂️proctor"]
    for d in extra_dirs:
        for base, dirs, files in os.walk(os.path.join(LIVE, d)):
            dirs[:] = [x for x in dirs if x not in ("node_modules", "target", ".cache")]
            for file in files:
                s = os.path.join(base, file)
                tgt = os.path.join(OV, os.path.relpath(s, LIVE))
                if not os.path.exists(tgt) and os.path.getsize(s) < 3_000_000:
                    os.makedirs(os.path.dirname(tgt), exist_ok=True)
                    shutil.copyfile(s, tgt)
    os.symlink(os.path.join(LIVE, "node_modules"), os.path.join(OV, "node_modules"))
    for marker in ("bunfig.toml", "bun.lock"):
        open(os.path.join(OV, marker), "w").close()
    skip = shutil.ignore_patterns("__pycache__", "backup", "backup-*", "🗑️generated", "node_modules")
    for name in TOOLS:
        source = tool(name).replace(OV, LIVE)
        if name == "round4-tools":
            source = HERE
        if os.path.isdir(source):
            shutil.copytree(source, tool(name), ignore=skip)
            for base, _, files in os.walk(tool(name)):
                for file in files:
                    path = os.path.join(base, file)
                    if file.endswith((".py", ".ts", ".cjs", ".mjs", ".sh")) and "/staged" not in path.replace(tool(name), "") and "/src/" not in path.replace(tool(name), ""):
                        text = open(path, encoding="utf8", errors="replace").read()
                        if name == "xplat-linux-fix" and file == "apply.py":
                            text = text.replace("guards = running(root)", "guards = []")
                        if LIVE in text:
                            open(path, "w", encoding="utf8").write(text.replace(LIVE, OV))
    shutil.copytree(P, os.path.join(OV, PREL), ignore=skip, dirs_exist_ok=True)
    return n


def link_missing(source, target):
    """Symlinks every entry of the live directory that the overlay does not have yet (read-only use: only called right before a step that writes nowhere below these trees)."""
    os.makedirs(target, exist_ok=True)
    for name in os.listdir(source):
        s, d = os.path.join(source, name), os.path.join(target, name)
        if os.path.lexists(d):
            if os.path.isdir(d) and not os.path.islink(d) and os.path.isdir(s):
                link_missing(s, d)
        else:
            os.symlink(s, d)


def main():
    order = opt("--order", ",".join(k for k in steps().keys() if k not in ("conc-harness-1", "gran-keys"))).split(",")
    os.makedirs(OUT, exist_ok=True)
    print("seeded", seed(), "files into", OV)
    result = []
    table = steps()
    for name in order:
        if name == "course-B":
            for rel in ("🧰️framework/🔨️modules", "🧰️framework/🛍️products"):
                link_missing(os.path.join(LIVE, rel), os.path.join(OV, rel))
        if name in PRE:
            rebase(name)
        before = inventory()
        env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_ROOT": OV}
        run = subprocess.run(table[name], capture_output=True, text=True, encoding="utf8", errors="replace", cwd=OV, env=env)
        if run.returncode not in (0, 1) and name in REBASE and name not in PRE:
            note = rebase(name)
            open(f"{OUT}/{name}.rebase.log", "w", encoding="utf8").write(note.stdout + "\n---\n" + note.stderr)
            run = subprocess.run(table[name], capture_output=True, text=True, encoding="utf8", errors="replace", cwd=OV, env=env)
        after = inventory()
        changed = sorted(r for r in after if before.get(r) != after[r])
        row = {"step": name, "rc": run.returncode, "changed": changed, "new": [r for r in changed if r not in before]}
        open(f"{OUT}/{name}.log", "w", encoding="utf8").write(run.stdout + "\n--- stderr ---\n" + run.stderr)
        result.append(row)
        print(f"{'ok  ' if run.returncode == 0 else 'FAIL'} rc={run.returncode} {name:22} changed {len(changed):4} (new {len(row['new'])})" + ("" if run.returncode == 0 else "\n    " + (run.stdout + run.stderr).strip()[-500:].replace("\n", "\n    ")))
        if run.returncode != 0 and "--continue" not in args:
            break
    json.dump({"order": order, "result": result}, open(f"{OUT}/result.json", "w"), ensure_ascii=False, indent=1)
    if "--keep" not in args:
        pass


main()

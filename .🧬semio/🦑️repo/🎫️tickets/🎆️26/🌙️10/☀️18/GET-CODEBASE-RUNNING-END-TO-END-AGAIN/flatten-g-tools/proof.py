#!/usr/bin/env python3
"""🔬 proof.py: proves a converted batch tool behaves like its old layout on a C/mirror-tools/mirror.py mirror (check -> apply -> check -> revert -> check, exit codes + sha256 of every file that differs from the mirror's pre-run snapshot).
usage: python3 -I proof.py record <lane> old|new --only <subtree|alias>... [--cwd DIR] -- <command template with {mode} and {root}>
       python3 -I proof.py compare <lane>
`record` creates the mirror `flg-<phase>` (dropped first), runs the template for modes check, apply, check, revert, check inside the mirror (sealed; the ticket generated folder stays writable) and stores C/flatten-g-tools/proof/<lane>-<phase>.json.
`compare` exits 0 when exit codes, the apply difference set with sha256 per file, and and an empty post-revert difference set are identical for old and new."""
import hashlib, json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
MM = os.path.join(HERE, "..", "mirror-tools", "mirror.py")
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()


def mm(*args, check=True):
    r = subprocess.run([sys.executable, "-I", MM, *args], capture_output=True, text=True)
    if check and r.returncode:
        raise SystemExit("mirror failed: %s\n%s" % (args, r.stderr[-2000:]))
    return r


def mirror_root(name):
    for line in mm("list").stdout.splitlines():
        parts = line.split("\t")
        if parts[0] == name:
            return parts[-1]
    raise SystemExit("mirror not found " + name)


def snapshot(root):
    out = {}
    for base, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for name in names:
            path = os.path.join(base, name)
            out[os.path.relpath(path, root)] = "L" if os.path.islink(path) else sha(path)
    return out


def differences(before, root):
    now = snapshot(root)
    return {k: now.get(k, "-") for k in sorted(set(before) | set(now)) if before.get(k, "-") != now.get(k, "-")}


def record(lane, phase, only, cwd, template):
    name = "flg-" + phase
    mm("drop", name, "--sync", check=False)
    mm("create", name, "--only", *only, "--no-nm", "--no-closure")
    root = mirror_root(name)
    before = snapshot(root)
    result = {"baseline": {}, "codes": {}, "apply": None, "revert": None}
    steps = ["check", "apply", "check", "revert", "check"]
    for i, mode in enumerate(steps):
        cmd = template.replace("{mode}", mode).replace("{root}", "'" + root + "'")
        argv = ["run", name, "--seal-allow", TICKET + "/🗑️generated", "--peers-ok"] + (["--cwd", cwd] if cwd else []) + ["--", "sh", "-c", cmd]
        r = mm(*argv, check=False)
        result["codes"]["%d-%s" % (i, mode)] = r.returncode
        if r.returncode not in (0, 1, 2, 3):
            sys.stderr.write("[%s %s] exit %d\n%s\n" % (phase, mode, r.returncode, (r.stdout + r.stderr)[-1500:]))
        if mode == "apply":
            result["apply"] = differences(before, root)
        if mode == "revert":
            result["revert"] = differences(before, root)
    os.makedirs(os.path.join(HERE, "proof"), exist_ok=True)
    json.dump(result, open(os.path.join(HERE, "proof", "%s-%s.json" % (lane, phase)), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    changed = result["apply"]
    print("%s %s: codes %s, apply changes %d files, revert == baseline: %s" % (lane, phase, list(result["codes"].values()), len(changed), result["revert"] == {}))
    mm("drop", name, "--sync", check=False)


def compare(lane):
    a, b = [json.load(open(os.path.join(HERE, "proof", "%s-%s.json" % (lane, p)), encoding="utf8")) for p in ("old", "new")]
    problems = []
    if a["codes"] != b["codes"]:
        problems.append("exit codes differ %s vs %s" % (a["codes"], b["codes"]))
    if a["apply"] != b["apply"]:
        problems.append("apply differences differ: %s" % sorted(set(a["apply"].items()) ^ set(b["apply"].items()))[:6])
    if a["revert"] or b["revert"]:
        problems.append("revert is not byte-exact to the baseline")
    if not a["apply"]:
        problems.append("apply changed nothing (vacuous proof)")
    print(("FAIL " if problems else "PASS ") + lane + " " + "; ".join(problems))
    return 1 if problems else 0


if __name__ == "__main__":
    a = sys.argv[1:]
    if a and a[0] == "record":
        cut = a.index("--")
        only = a[a.index("--only") + 1:cut] if "--only" in a else []
        only = [o for o in only if o != "--cwd"]
        cwd = a[a.index("--cwd") + 1] if "--cwd" in a else None
        if cwd:
            only = [o for o in only if o != cwd]
        record(a[1], a[2], only, cwd, " ".join(a[cut + 1:]))
    elif a and a[0] == "compare":
        sys.exit(compare(a[1]))
    else:
        raise SystemExit(__doc__)

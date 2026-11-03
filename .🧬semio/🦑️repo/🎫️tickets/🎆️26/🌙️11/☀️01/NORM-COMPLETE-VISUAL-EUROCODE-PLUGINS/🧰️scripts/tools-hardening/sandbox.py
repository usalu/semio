"""🧪 Generator sandbox: runs a command with scratch copies of every tree the generators write to, bind-mounted over the real paths inside a read-only bubblewrap namespace, so a generator can never touch the source tree.

usage: python3 sandbox.py setup | reset | run [--] CMD...   (also importable: `run(cmd, timeout)`, `diff()`)
"""
import os, shutil, subprocess, sys

SEMIO = "/home/deck/semio"
TICKET = SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
BASE = TICKET + "/🗑️generated/tools-hardening/scratch"
LOW = ["nice", "-n", "19", "ionice", "-c3"]
SCRIPTS_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SLOT = os.environ.get("TH_SLOT", "")
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm"
DIRS = {"norm": NORM, "ticket": TICKET}
FILES = {"build-owners.json": SEMIO + "/📋️build-owners.json", "dependencies.json": SEMIO + "/🔒️dependencies.json", "hub-Cargo.lock": SEMIO + "/🌎️hub/Cargo.lock", "root-Cargo.lock": SEMIO + "/Cargo.lock", "root-Cargo.toml": SEMIO + "/Cargo.toml"}
TICKET_KEEP = ["🧰️scripts", "🧰️ndp", "📐️specs", "📌️important", "🛠️asset-laws", "🎫️ticket.json", "fieldcov.json"]


def sh(*a):
    subprocess.run(LOW + list(a), check=True)


def setup():
    for k, src in DIRS.items():
        dst = BASE + "/base/" + k
        os.makedirs(dst, exist_ok=True)
        if k == "ticket":
            for n in TICKET_KEEP:
                s = src + "/" + n
                if os.path.isdir(s):
                    sh("rsync", "-a", "--delete", "--exclude=__pycache__", s + "/", dst + "/" + n + "/")
                elif os.path.exists(s):
                    sh("cp", "-p", s, dst + "/" + n)
            os.makedirs(dst + "/🗑️generated", exist_ok=True)
        else:
            sh("rsync", "-a", "--delete", "--exclude=target", src + "/", dst + "/")
    os.makedirs(BASE + "/base/files", exist_ok=True)
    for k, src in FILES.items():
        shutil.copy2(src, BASE + "/base/files/" + k)
    reset()


def reset():
    for k in DIRS:
        os.makedirs(BASE + "/work" + SLOT + "/" + k, exist_ok=True)
    os.makedirs(BASE + "/work" + SLOT + "/files", exist_ok=True)
    for k in DIRS:
        sh("rsync", "-a", "--delete", BASE + "/base/" + k + "/", BASE + "/work" + SLOT + "/" + k + "/")
    sh("rsync", "-a", "--delete", BASE + "/base/files/", BASE + "/work" + SLOT + "/files/")


def sync_scripts():
    sh("rsync", "-a", "--delete", "--exclude=__pycache__", TICKET + "/🧰️scripts/", BASE + "/work" + SLOT + "/ticket/🧰️scripts/")
    sh("rsync", "-a", "--delete", "--exclude=__pycache__", TICKET + "/🧰️scripts/", BASE + "/base/ticket/🧰️scripts/")


def argv(cmd):
    a = LOW + ["bwrap", "--ro-bind", "/", "/", "--dev-bind", "/dev", "/dev", "--proc", "/proc", "--tmpfs", "/tmp", "--die-with-parent", "--unshare-pid", "--setenv", "PYTHONDONTWRITEBYTECODE", "1"]
    for k, dst in FILES.items():
        a += ["--bind", BASE + "/work" + SLOT + "/files/" + k, dst]
    for k in ("norm", "ticket"):
        a += ["--bind", BASE + "/work" + SLOT + "/" + k, DIRS[k]]
    return a + ["--chdir", TICKET + "/🧰️scripts"] + cmd


def run(cmd, timeout=300):
    return subprocess.run(argv(cmd), capture_output=True, text=True, timeout=timeout)


def _find(root, kind, marker):
    r = subprocess.run(["find", root, "-type", kind, "-newer", marker], capture_output=True, text=True).stdout
    return [x for x in r.split("\n") if x and "__pycache__" not in x]


def _files(root):
    return [os.path.join(d, f) for d, _, fs in os.walk(root) for f in fs if "__pycache__" not in d]


def _skip(k, rel):
    return k == "ticket" and (rel.startswith("🗑️generated/") or rel == "🗑️generated")


def subsumes(old, new):
    """True when `new` contains everything `old` had (dict keys with subsuming values, list items in order or as a superset, equal scalars)."""
    if isinstance(old, dict):
        return isinstance(new, dict) and all(k in new and subsumes(v, new[k]) for k, v in old.items())
    if isinstance(old, list):
        return isinstance(new, list) and all(any(subsumes(o, n) for n in new) for o in old)
    return old == new


def removed_lines(bf, f):
    """Lines the new file really lost, by the same rule `safe_run.lost` enforces (JSON files are compared semantically first)."""
    if bf.endswith(".json"):
        try:
            import json
            if subsumes(json.load(open(bf, encoding="utf-8")), json.load(open(f, encoding="utf-8"))):
                return 0
        except ValueError:
            pass
    sys.path.insert(0, SCRIPTS_DIR)
    import safe_run
    return len(safe_run.lost(open(bf, encoding="utf-8", errors="replace").read(), open(f, encoding="utf-8", errors="replace").read()))


def marker():
    m = BASE + "/marker" + SLOT
    open(m, "w").close()
    os.utime(m, None)
    return m


def delta(mark):
    """Everything that differs from the base copies since `marker()`: {path: (kind, md5, removed_lines)}; kinds added|deleted|modified. Only touched files and changed directories are inspected."""
    import hashlib
    out = {}
    for k in ("norm", "ticket"):
        w, b = BASE + "/work" + SLOT + "/" + k, BASE + "/base/" + k
        for d in _find(w, "d", mark):
            rel = os.path.relpath(d, w)
            if _skip(k, rel):
                continue
            bd = os.path.join(b, rel)
            if os.path.isdir(bd):
                for n in set(os.listdir(bd)) - set(os.listdir(d)):
                    p = os.path.join(bd, n)
                    for f in (_files(p) if os.path.isdir(p) else [p]):
                        r = os.path.relpath(f, b)
                        if not _skip(k, r) and "__pycache__" not in r:
                            out[k + ":" + r] = ("deleted", "", 0)
        for f in _find(w, "f", mark):
            rel = os.path.relpath(f, w)
            if _skip(k, rel):
                continue
            data = open(f, "rb").read()
            h = hashlib.md5(data).hexdigest()
            bf = os.path.join(b, rel)
            if not os.path.exists(bf):
                out[k + ":" + rel] = ("added", h, 0)
            elif open(bf, "rb").read() != data:
                out[k + ":" + rel] = ("modified", h, removed_lines(bf, f))
    for k in FILES:
        w, b = BASE + "/work" + SLOT + "/files/" + k, BASE + "/base/files/" + k
        if open(w, "rb").read() != open(b, "rb").read():
            d = subprocess.run(["diff", b, w], capture_output=True, text=True).stdout
            out["files:" + k] = ("modified", hashlib.md5(open(w, "rb").read()).hexdigest(), sum(1 for x in d.splitlines() if x.startswith("< ")))
    return out


def restore(d):
    """Undo a `delta` by copying the touched paths back from base (cheap, no tree scan)."""
    for key in d:
        k, rel = key.split(":", 1)
        if k == "files":
            shutil.copy2(BASE + "/base/files/" + rel, BASE + "/work" + SLOT + "/files/" + rel)
            continue
        w, b = BASE + "/work" + SLOT + "/" + k + "/" + rel, BASE + "/base/" + k + "/" + rel
        if os.path.exists(b):
            os.makedirs(os.path.dirname(w), exist_ok=True)
            shutil.copy2(b, w)
        elif os.path.exists(w):
            os.remove(w)
            dd = os.path.dirname(w)
            while dd != BASE + "/work" + SLOT + "/" + k and not os.listdir(dd):
                os.rmdir(dd)
                dd = os.path.dirname(dd)


if __name__ == "__main__":
    c = sys.argv[1]
    if c == "setup":
        setup()
    elif c == "reset":
        reset()
    elif c == "run":
        r = run([x for x in sys.argv[2:] if x != "--"])
        sys.stdout.write(r.stdout)
        sys.stderr.write(r.stderr)
        sys.exit(r.returncode)

"""🚦 Post-apply pipeline: schema twin generation, the mutation tree, build-owner sync and the gates."""
import os, subprocess, sys
from core import SEMIO

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.abspath(os.path.join(HERE, "..", ".."))
HEAVY = TICKET + "/🗑️generated/build-runner/heavy.sh"
SCRIPTS = TICKET + "/🧰️scripts"
PY = os.environ.get("EVOLVE_PYTHON") or sys.executable


def sh(cmd, cwd=SEMIO, check=True):
    print("[schema-evolve] $", " ".join(c if " " not in c else repr(c) for c in cmd))
    done = subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL)
    if check and done.returncode:
        raise SystemExit(f"[schema-evolve] failed ({done.returncode}): {' '.join(cmd)}")
    return done.returncode


def heavy(*args, check=True):
    return sh([HEAVY, "repo", "./📜️script.ts", *args], check=check)


def under(ctx):
    return ctx.layout.dir[len(SEMIO) + 1:]


def generate(ctx):
    heavy("schema", "twins", "generate", "--under", under(ctx))


def owners_sync():
    heavy("verify", "build-owners", "sync")


def gates(ctx):
    results = {}
    results["twins-check-strict"] = heavy("schema", "twins", "check", "--strict", "--under", under(ctx), check=False)
    results["schema-sync"] = sh(["nice", "-n", "19", PY, "-I", SCRIPTS + "/schema-sync/check.py", "--artifact", ctx.short, "--summary"], check=False)
    results["handle-paths"] = sh(["nice", "-n", "19", PY, "-I", SCRIPTS + "/handle-paths-check.py"], check=False)
    results["cargo-authority"] = sh(["nice", "-n", "19", PY, "-I", SCRIPTS + "/check-cargo-authority.py"], check=False)
    results["build-owners-sync"] = heavy("verify", "build-owners", "sync", check=False)
    print("[schema-evolve] gates:", ", ".join(f"{k}={'ok' if v == 0 else 'FAIL(' + str(v) + ')'}" for k, v in results.items()))
    return 0 if all(v == 0 for v in results.values()) else 1


def run(ctx):
    import l_mutations
    generate(ctx)
    l_mutations.apply(ctx)
    generate(ctx)
    owners_sync()
    return gates(ctx)

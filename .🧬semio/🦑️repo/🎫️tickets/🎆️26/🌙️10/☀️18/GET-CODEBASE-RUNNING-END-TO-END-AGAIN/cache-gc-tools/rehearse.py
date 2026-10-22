#!/usr/bin/env python3
"""🎭️ Rehearses the apply flows on a throwaway root: unrecorded bases refuse, apply3 then apply4 write and are identical to the mirror, a changed file blocks `--restore`, a lease that cannot be taken writes nothing, a failed recreate rolls the files back. Never touches the real root."""
import contextlib, hashlib, io, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
SIM = os.environ.get("REHEARSE_ROOT") or tempfile.mkdtemp(prefix="rehearse-", dir="/private/tmp/cache-gc")
os.environ["CACHE_GC_ROOT"] = SIM
sys.path.insert(0, HERE)
import apply, apply3, apply4, patches, recreate  # noqa: E402
from paths import J, NEW, NEW3, NEW4, TARGET, TARGET4  # noqa: E402

REAL = os.environ.get("REHEARSE_REAL") or subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, cwd=HERE).stdout.strip()


def populate():
    shutil.rmtree(SIM, ignore_errors=True)
    for table in (TARGET, TARGET4, NEW, NEW3, NEW4):
        for rel in table.values():
            source = J(REAL, rel)
            if os.path.exists(source) and not os.path.exists(J(SIM, rel)):
                os.makedirs(os.path.dirname(J(SIM, rel)), exist_ok=True)
                shutil.copyfile(source, J(SIM, rel))


def sha(path):
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def run(module, argv):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        try:
            code = module.main(argv)
        except SystemExit as error:
            code = error.code
    return code, out.getvalue()


def snapshot():
    return {rel: sha(J(SIM, rel)) for table in (TARGET, TARGET4, NEW3, NEW4) for rel in table.values() if os.path.exists(J(SIM, rel))}


def expect(label, condition, detail=""):
    print(("ok   " if condition else "FAIL ") + label + (" " + detail if detail and not condition else ""))
    if not condition:
        raise SystemExit(1)


populate()
before = snapshot()
for module in (apply3, apply4):
    module.configure()
    path = module.apply.BASES
    if os.path.exists(path):
        os.rename(path, path + ".rehearse")
try:
    code, out = run(apply3, [])
    expect("a set without recorded bases refuses", code == 3 and "no recorded base" in out, out)
    expect("--tolerate-drift no longer exists", "tolerate" not in open(J(HERE, "apply.py"), encoding="utf-8").read())
    run(apply3, ["--record"])
    expect("apply3 dry run passes once recorded", run(apply3, [])[0] == 0)
    real_holding, real_preflight = recreate.holding, recreate.preflight

    @contextlib.contextmanager
    def taken(name, directory=None):
        raise recreate.Busy("BUSY native-fixed-image: taken")
        yield

    recreate.holding = taken
    recreate.preflight = lambda name: (J(SIM, ".cache/b/g"), J(SIM, ".cache/b/g/m"), [])
    mid = snapshot()
    code, out = run(apply3, ["--write", "--ignore-runs", "--recreate-graph"])
    expect("a lease that cannot be taken refuses before any write", code == 2 and snapshot() == mid, out)

    @contextlib.contextmanager
    def held(name, directory=None):
        yield

    recreate.holding = held
    real_run = recreate.run
    recreate.run = lambda name, yes, held=False, lease_directory=None: 3
    code, out = run(apply3, ["--write", "--ignore-runs", "--recreate-graph"])
    expect("a failure after the write rolls every file back", code == 3 and snapshot() == mid and "rolling the files back" in out, out)
    recreate.run = lambda name, yes, held=False, lease_directory=None: 0
    code, out = run(apply3, ["--write", "--ignore-runs", "--recreate-graph"])
    expect("apply3 with the image step succeeding writes", code == 0 and snapshot() != mid, out)
    recreate.holding, recreate.preflight, recreate.run = real_holding, real_preflight, real_run
    run(apply4, ["--record"])
    code, out = run(apply4, ["--write", "--ignore-runs"])
    expect("apply4 writes on top of apply3", code == 0, out)
    expect("apply4 is idempotent", run(apply4, [])[1].count("same") >= 15)
    victim = J(SIM, TARGET4["closures-ts"])
    with open(victim, "a", encoding="utf-8") as handle:
        handle.write("\n")
    code, out = run(apply4, ["--restore"])
    expect("--restore refuses a file that changed since it was written, naming it", code == 5 and "closures" in out or "🟦️.ts" in out, out)
    with open(victim, "rb") as handle:
        data = handle.read()
    with open(victim, "wb") as handle:
        handle.write(data[:-1])
    expect("--restore works once the file is as written again", run(apply4, ["--restore"])[0] == 0)
    expect("--restore of apply3 works too", run(apply3, ["--restore"])[0] == 0 and snapshot() == before)
    print("rehearsal passed on", SIM)
finally:
    for module in (apply3, apply4):
        module.configure()
        path = module.apply.BASES
        if os.path.exists(path + ".rehearse"):
            os.replace(path + ".rehearse", path)
    for name in ("backup3", "backup4"):
        shutil.rmtree(J(HERE, name), ignore_errors=True)
    shutil.rmtree(SIM, ignore_errors=True)

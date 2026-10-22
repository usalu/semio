#!/usr/bin/env python3
"""Proves the doctor tests bite: each mutation edits one decision of the overlay copy of the implementation, the suites must then fail, and the file is restored.

usage: mutation-check.py [OVERLAY_DIR]   (default /private/tmp/repo-doctor/ov; run overlay.py first)"""
import os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import paths  # noqa: E402

J = os.path.join
OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/repo-doctor/ov"
DOCTOR = J(OVERLAY, paths.D)
FILES = {"analysis": J(DOCTOR, paths.E(0x1F9EE) + "analysis", paths.TS), "index": J(DOCTOR, paths.TS), "runner": J(DOCTOR, paths.E(0x1F3C3) + "runner", paths.TS),
         "leases": J(paths.PROBES.replace(paths.D, DOCTOR), paths.E(0x1F512) + "leases", paths.TS), "caches": J(paths.PROBES.replace(paths.D, DOCTOR), paths.E(0x26A1) + "caches", paths.TS),
         "storage": J(paths.PROBES.replace(paths.D, DOCTOR), paths.E(0x1F4BE) + "storage", paths.TS), "dependencies": J(paths.PROBES.replace(paths.D, DOCTOR), paths.E(0x1F4E6) + "dependencies", paths.TS)}
MUTATIONS = [
    ("analysis", "holder.heldSeconds>=policy.leases.holdWarnSeconds", "holder.heldSeconds>policy.leases.holdWarnSeconds"),
    ("analysis", "if(waiter.waitedSeconds<policy.leases.waitWarnSeconds)continue;", "if(waiter.waitedSeconds<=policy.leases.waitWarnSeconds)continue;"),
    ("analysis", "host.availableMegabytes<=host.floorMegabytes", "host.availableMegabytes<host.floorMegabytes"),
    ("analysis", "else if(percent<policy.storage.freePercentWarn)", "else if(percent<=policy.storage.freePercentWarn)"),
    ("analysis", "part.freeBytes<floor)add(\"storage.image-below-floor\"", "part.freeBytes<=floor)add(\"storage.image-below-floor\""),
    ("analysis", "grant.observedMegabytes*100<grant.reservedMegabytes*policy.capacity.idleObservedPercent", "grant.observedMegabytes*100<=grant.reservedMegabytes*policy.capacity.idleObservedPercent"),
    ("analysis", "hop<8", "hop<1"),
    ("analysis", 'state=deadHolders.has(root.pid)?"gone":', 'state=false?"gone":'),
    ("analysis", 'kind==="out-of-order"?"info":"warn"', 'kind==="out-of-order"?"warn":"info"'),
    ("analysis", "if(!building&&dependencies.unready>0", "if(dependencies.unready>0"),
    ("analysis", "const parentGone=row.parent<=1;", "const parentGone=row.parent<1;"),
    ("analysis", 'longest>=policy.leases.waitWarnSeconds?"warn":"info"', 'longest>policy.leases.waitWarnSeconds?"warn":"info"'),
    ("analysis", "host.attachedImages>=policy.host.imagesError", "host.attachedImages>policy.host.imagesError"),
    ("analysis", "if(percent>=policy.host.openFilesWarnPercent)", "if(percent>policy.host.openFilesWarnPercent)"),
    ("analysis", "counts.error>0?\"blocked\"", "counts.error>1?\"blocked\""),
    ("analysis", "if(sessions.gone>0)", "if(sessions.gone>1)"),
    ("index", "if(WRAPPERS.has(program))return false;", ""),
    ("index", "rest[0]===\"repo\"?rest.slice(1):rest", "rest"),
    ("runner", "ports.signal.addEventListener(\"abort\",()=>fail(new DoctorCancelledV1()),{once:true});", ""),
    ("runner", "if(outcome===\"timeout\"){controller.abort();return", "if(false){controller.abort();return"),
    ("leases", "!directory.startsWith(root+sep)&&!existsSync(directory)", "!existsSync(directory)"),
    ("leases", "deadHolders:[...deadPids]", "deadHolders:[]"),
    ("caches", "ports.alive(pid)||ports.now()-info.mtimeMs", "ports.now()-info.mtimeMs"),
    ("caches", "real!==boundary&&!real.startsWith(boundary+sep)", "false"),
    ("storage", "inner.dev!==outer.dev", "inner.dev===outer.dev"),
    ("dependencies", "time>published+2000", "time>published-2000"),
    ("dependencies", "referenced.has(entry.name)", "false"),
]


def run():
    tests = paths.TESTS.split(os.sep)[-1]
    result = subprocess.run(["bun", "test", "./" + tests + "/" + paths.TS, "./" + tests + "/" + paths.E(0x1F50E) + "probes/" + paths.TS, "./" + tests + "/" + paths.E(0x2328) + "command/" + paths.TS], cwd=DOCTOR, capture_output=True, text=True, env={**os.environ, "LC_ALL": "en_US.UTF-8"})
    return result.returncode


def main():
    if run() != 0:
        print("baseline fails; fix the suites first")
        return 2
    survivors, killed = [], 0
    for index, (name, old, new) in enumerate(MUTATIONS):
        path = FILES[name]
        with open(path, encoding="utf-8") as handle:
            original = handle.read()
        if old == new or original.count(old) < 1:
            survivors.append((index, "anchor missing or no-op: " + old[:70]))
            continue
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(original.replace(old, new, 1))
        try:
            failed = run() != 0
        finally:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(original)
        if failed:
            killed += 1
            print("killed   %2d %s: %s" % (index, name, old[:70]))
        else:
            survivors.append((index, "SURVIVED " + name + ": " + old[:70]))
            print("SURVIVED %2d %s: %s" % (index, name, old[:70]))
    print("%d of %d mutations killed" % (killed, len(MUTATIONS)))
    for row in survivors:
        print(row)
    return 0 if not survivors else 1


if __name__ == "__main__":
    raise SystemExit(main())

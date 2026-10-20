#!/usr/bin/env python3
"""One ordered, conflict-free apply series for all staged sets of the architecture-course-on-Bazel fleet (x-integrate).

  apply-all.py --check                       read only: runs every step on a private overlay seeded from the checkout, verifies every anchor, prints the plan; exit 0 = every step resolves, 2 = a step refuses
  apply-all.py --live                        applies all steps or none: the overlay result is computed first; files are written (temp file + rename, so no file ever keeps a second link) only if no step refused and
                                             no touched file changed since it was read; a failed write restores the files already replaced
  options  --root <dir>                      another checkout or byte-copy mirror (default: this checkout)
           --only a,b / --skip a            select steps by name (see STEPS); later steps still see the result of the earlier ones
           --with-cleanup                   include the final x-proctor cleanup (deletes buildProctor and PROCTOR_ACQUISITION_*): only after the live verification of x-proctor
           --json                           machine readable plan on stdout

Steps are idempotent and applied in this order; every step runs against the text the earlier steps produced, which is what resolves the anchor conflicts documented in result-x-integrate.md."""
import argparse, hashlib, json, os, shutil, subprocess, sys, tempfile

TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(TOOLS, "steps"))
from lib import *
import cleanup_proctor, proctor_step, reconcile, release_step

BUN = shutil.which("bun") or "bun"
PYTHON = sys.executable
PEER = os.path.join(TOOLS, "steps", "peer-ops", "apply-manifests.ts")
STAGED = {name: os.path.join(TICKET, f"x-{name}-tools") for name in ("inputs", "site", "dev", "release", "paths", "proctor", "keys", "graph")}

SEED = json.load(open(os.path.join(TOOLS, "touched.json"), encoding="utf8"))
TOUCHED = SEED["files"] + SEED["added"] + SEED["readonly"]


def sh(command, env=None):
    run = subprocess.run(command, capture_output=True, text=True, env={**os.environ, **(env or {})})
    if run.returncode:
        refuse(f"{os.path.basename(command[-1] if command[-1].endswith('.py') else command[1])} exited {run.returncode}: " + (run.stderr.strip() or run.stdout.strip())[-600:])


def step_peer_ops(root): sh([BUN, PEER], {"SEMIO_ROOT": root})
def step_inputs(root): sh([PYTHON, os.path.join(STAGED["inputs"], "staged", "apply.py"), "--root", root])
def step_site(root): sh([PYTHON, os.path.join(STAGED["site"], "staged", "apply.py")], {"X_SITE_ROOT": root})
def step_dev(root): sh([PYTHON, os.path.join(STAGED["dev"], "staged", "apply.py")], {"X_DEV_ROOT": root})
def step_release(root): release_step.run(root)
def step_paths(root): sh([PYTHON, os.path.join(STAGED["paths"], "apply.py"), "--root", root])
def step_proctor(root): proctor_step.run(root)
def step_keys(root): sh([PYTHON, os.path.join(STAGED["keys"], "staged", "apply.py"), root])
def step_reconcile(root): reconcile.run(root)
def step_cleanup(root): cleanup_proctor.run(root)


def step_graph(root):
    script = os.path.join(STAGED["graph"], "staged", "apply.py")
    if not os.path.exists(script):
        raise Skipped("x-graph has no staged apply.py yet")
    sh([PYTHON, script, "--root", root])


class Skipped(Exception):
    pass


def contains(rel, needle):
    return lambda root: os.path.isfile(os.path.join(root, rel)) and needle in read(root, rel)


SCRIPT_INPUTS = f"{BUILD}/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
SITE_BUILDER = f"{SITE}/🚀️deploy/🌐️site/🟦️.ts"


STEPS = [
    ("peer-ops", "uncached-owners manifest rows of the peer (course and proctor only; idempotent, never overwrites authored check inputs)", step_peer_ops, None),
    ("x-inputs", "exact inputs: declared closure (modules, TypeScript projects), scanner, schema, hermetic law", step_inputs, contains(SCRIPT_INPUTS, "declaredModuleGlobsV1")),
    ("x-site", "cached site build (receipt), publish on top of build, deploy-check as graph tasks", step_site, contains(SITE_BUILDER, "export const COURSE_SITE_OWNER")),
    ("x-dev", "stack parameter on dev-site and proctor dev, manifest test", step_dev, None),
    ("x-release", "build mode axis in the Bazel adapter, dry-run plan, README, 📥️mode suite", step_release, None),
    ("x-paths", "short source aliases and per-OS path budget in the Bazel adapter", step_paths, None),
    ("x-proctor", "proctor as a cached product: manifests, receipt input, acquisition flip", step_proctor, None),
    ("x-keys", "action-key regression contract (tests only)", step_keys, None),
    ("reconcile", "R1 check keys on the receipt, R2 build closure over the site tsconfig", step_reconcile, None),
    ("x-graph", "selected-closure graph (only when staged)", step_graph, None),
    ("cleanup", "x-proctor step 5: delete buildProctor and PROCTOR_ACQUISITION_*", step_cleanup, None),
]


def inventory(directory):
    rows = {}
    for base, _, names in os.walk(directory):
        for name in names:
            path = os.path.join(base, name)
            if os.path.islink(path):
                continue
            with open(path, "rb") as handle:
                rows[os.path.relpath(path, directory)] = hashlib.sha256(handle.read()).hexdigest()
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--live", action="store_true")
    parser.add_argument("--root", default=LIVE)
    parser.add_argument("--only", default="")
    parser.add_argument("--skip", default="")
    parser.add_argument("--with-cleanup", action="store_true")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--inplace", action="store_true", help=argparse.SUPPRESS)
    options = parser.parse_args()
    root = os.path.abspath(options.root)
    names = [row[0] for row in STEPS]
    only = [x for x in options.only.split(",") if x]
    skip = {x for x in options.skip.split(",") if x}
    for name in only + sorted(skip):
        if name not in names:
            parser.error(f"unknown step {name}; steps: {', '.join(names)}")
    selected = [row for row in STEPS if (not only or row[0] in only) and row[0] not in skip and (row[0] != "cleanup" or options.with_cleanup or "cleanup" in only)]

    scratch = None
    if options.inplace:
        overlay = root
    else:
        scratch = tempfile.mkdtemp(prefix="x-integrate-")
        overlay = os.path.join(scratch, "overlay")
        os.makedirs(overlay)
        for rel in TOUCHED:
            source = os.path.join(root, rel)
            if os.path.isfile(source):
                target = os.path.join(overlay, rel)
                os.makedirs(os.path.dirname(target), exist_ok=True)
                with open(source, "rb") as reader, open(target, "wb") as writer:
                    writer.write(reader.read())
                shutil.copymode(source, target)
    seeded = inventory(overlay) if not options.inplace else {}
    report, refused = [], False
    try:
        for name, label, function, done in selected:
            before = inventory(overlay) if not options.inplace else None
            try:
                if done is not None and done(overlay):
                    raise Skipped("already applied (marker found); the step is not run again because later steps rewrote its anchors")
                function(overlay)
                state = "ok"
            except Skipped as skipped:
                state, detail = ("applied" if done is not None and str(skipped).startswith("already") else "skipped"), str(skipped)
            except (SystemExit, FileNotFoundError, AssertionError, KeyError, ValueError) as error:
                state, detail = "refused", (error.code if isinstance(error, SystemExit) else f"{type(error).__name__}: {error}")
                refused = True
            row = {"step": name, "label": label, "state": state}
            if state in ("skipped", "refused"):
                row["detail"] = str(detail)
            if before is not None:
                after = inventory(overlay)
                row["changed"] = sorted(path for path in after if before.get(path) != after[path])
                if state == "ok":
                    row["state"] = "pending" if row["changed"] else "applied"
            report.append(row)
        plan = {}
        if not options.inplace:
            final = inventory(overlay)
            plan = {path: digest for path, digest in final.items() if seeded.get(path) != digest}
        if options.json:
            print(json.dumps({"root": root, "steps": report, "files": sorted(plan), "refused": refused}, ensure_ascii=False, indent=1))
        else:
            print(f"series on {root}")
            for row in report:
                print(f"{row['state']:9} {row['step']:10} {len(row.get('changed', [])):3} file(s)  {row['label']}" + (f"\n          {row['detail']}" if "detail" in row else ""))
            print(f"{len(plan)} file(s) differ from the checkout" + ("; REFUSED: nothing would be written" if refused else ""))
        if refused:
            return 2
        if options.check or options.inplace:
            return 0
        current = {rel: (hashlib.sha256(open(os.path.join(root, rel), "rb").read()).hexdigest() if os.path.isfile(os.path.join(root, rel)) else None) for rel in plan}
        for rel in plan:
            seed = seeded.get(rel)
            if current[rel] != seed:
                refuse(f"{rel} changed in the checkout since it was read; run again")
        saved, written = {}, []
        try:
            for rel in sorted(plan):
                target = os.path.join(root, rel)
                if os.path.isfile(target):
                    saved[rel] = (open(target, "rb").read(), os.stat(target).st_mode & 0o777)
                os.makedirs(os.path.dirname(target), exist_ok=True)
                temporary = target + ".tmp-x-integrate"
                shutil.copyfile(os.path.join(overlay, rel), temporary)
                if rel in saved:
                    os.chmod(temporary, saved[rel][1])
                os.replace(temporary, target)
                written.append(rel)
        except BaseException:
            for rel in written:
                target = os.path.join(root, rel)
                if rel in saved:
                    temporary = target + ".tmp-x-integrate"
                    with open(temporary, "wb") as handle:
                        handle.write(saved[rel][0])
                    os.chmod(temporary, saved[rel][1])
                    os.replace(temporary, target)
                else:
                    os.remove(target)
            raise
        print(f"applied: {len(written)} file(s) written")
        return 0
    finally:
        if scratch:
            shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())

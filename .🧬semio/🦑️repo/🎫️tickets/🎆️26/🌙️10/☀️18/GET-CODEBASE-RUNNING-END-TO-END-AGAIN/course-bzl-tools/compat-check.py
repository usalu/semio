#!/usr/bin/env python3
"""Does the course-B series still apply after another staged set that supports --root? Copies the course files plus the other set's targets (read from its dry-run) into <dir>/root, applies the other set there, then runs the course series --check and --live.
usage: compat-check.py <dir> <pre-apply command with {root}> [--extra relpath ...]     e.g. compat-check.py /private/tmp/cb "python3 -I conc-bazel-tools/apply.py --root {root} --group all --write" --extra <rel> ..."""
import importlib.util, os, shlex, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
directory, command, extra = os.path.abspath(sys.argv[1]), sys.argv[2], sys.argv[4:] if len(sys.argv) > 3 and sys.argv[3] == "--extra" else []
script = os.path.join(HERE, "staged", "apply-course.py")
sys.argv = [sys.argv[0]]
spec = importlib.util.spec_from_file_location("apply_course", script)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
live, root = module.series.LIVE, os.path.join(directory, "root")
shutil.rmtree(root, ignore_errors=True)
for rel in [*module.series.TOUCHED, *extra]:
    source = os.path.join(live, rel)
    if os.path.isfile(source):
        target = os.path.join(root, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(source, "rb") as reader, open(target, "wb") as writer:
            writer.write(reader.read())
        shutil.copymode(source, target)
pre = subprocess.run(shlex.split(command.replace("{root}", root)), capture_output=True, text=True, cwd=os.path.join(HERE, ".."))
print(f"== pre-apply rc={pre.returncode}\n{(pre.stdout + pre.stderr)[-900:]}")
for mode in ("--check", "--live", "--check"):
    run = subprocess.run([sys.executable, "-I", script, mode, "--root", root, "--with-cleanup"], capture_output=True, text=True)
    lines = [line for line in run.stdout.splitlines() if not line.startswith("applied ")]
    print(f"== course {mode} rc={run.returncode}\n" + "\n".join(lines[-14:])[-1500:] + run.stderr[-600:])

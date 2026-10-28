# coding: utf-8
"""🛂️ Negative controls of the acceptance laws, on a mirror only: every control plants one violation in a file of the applied batch, the acceptance laws must fail with the named test, and the batch is written back (`forceapply.py`) before the next control. Prints RED-AS-REQUIRED per control and exits 1 when a control stays green.
usage: python3 -I negative.py MIRROR_ROOT"""
import os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(sys.argv[1])
if "/sm/" not in root + "/":
    sys.exit("refusing: not a mirror")
VS = "️"
ACC = "🧰" + VS + "framework/🛍" + VS + "products/🦑" + VS + "repo/🔨" + VS + "modules/📚" + VS + "library/🏗" + VS + "build/📏" + VS + "incremental/🏎" + VS + "bench/🛂" + VS + "acceptance"
OPS = "🧰" + VS + "framework/🛍" + VS + "products/🦑" + VS + "repo/🔨" + VS + "modules/📚" + VS + "library/🏗" + VS + "build/⚙" + VS + "operations/🔣" + VS + "catalog.json"
LOGIC, RUNNER = ACC + "/🟦" + VS + ".ts", ACC + "/🏃" + VS + "runner/🟦" + VS + ".ts"
BUDGETS = ACC + "/🎯" + VS + "budgets/🔣" + VS + ".json"
TEST = ACC + "/🧪" + VS + "tests/🟦" + VS + ".ts"
FLOW = ".github/workflows/acceptance.yml"
CONTROLS = [
    ("a budget loosened", BUDGETS, '"id": "course-noop-wall",\n   "title"', None, "the budgets are the acceptance targets of the brief"),
    ("a budget that drifted from the SLO table", BUDGETS, '"maximum": 1536,', '"maximum": 1500,', "restates its number exactly"),
    ("a judge that treats the maximum as exclusive", LOGIC, "if(value<=budget.maximum)return", "if(value<budget.maximum)return", "recorded runs are judged as written down"),
    ("a restore that leaves the edit in place", LOGIC, "return Object.freeze({bytes:original,exact:true,merged:false});", "return Object.freeze({bytes:edited.bytes,exact:true,merged:false});", "an edit appends exactly one line"),
    ("a journal that is never cleared", RUNNER, "   await ports.journal.clear();\n   return restored=", "   return restored=", "an edit scenario edits one line"),
    ("a nightly job that cannot fail", FLOW, "        run: repo bench acceptance", "        continue-on-error: true\n        run: repo bench acceptance", "the nightly workflow runs the acceptance"),
    ("a catalogue without --in-place", OPS, '"flags": ["--scenario", "--in-place", "--markdown", "--file", "--json"]', '"flags": ["--scenario", "--markdown", "--file", "--json"]', "the command line options are closed and shared with the catalogue"),
]


def run():
    done = subprocess.run(["bun", "test", "./" + TEST], cwd=root, capture_output=True, text=True)
    return done.stdout + done.stderr


def restore():
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "forceapply.py"), root], check=True, capture_output=True)


bad = 0
restore()
for name, rel, old, new, expected in CONTROLS:
    path = os.path.join(root, rel)
    text = open(path, encoding="utf8").read()
    if new is None:
        new = old.replace("course-noop-wall", "course-noop-wall").replace("\n   \"title\"", "\n   \"title\"")
        text2 = text.replace('"maximum": 10000,\n   "enforcement": "blocking",\n   "source": "brief-wave-1026g targets: no-op < 10 s"', '"maximum": 11000,\n   "enforcement": "blocking",\n   "source": "brief-wave-1026g targets: no-op < 10 s"', 1)
    else:
        if text.count(old) < 1:
            print("DEAD CONTROL", name)
            bad += 1
            continue
        text2 = text.replace(old, new, 1)
    if text2 == text:
        print("DEAD CONTROL", name)
        bad += 1
        continue
    open(path, "w", encoding="utf8").write(text2)
    try:
        output = run()
    finally:
        restore()
    failed = [line for line in output.splitlines() if line.startswith("(fail)")]
    hit = any(expected in line for line in failed)
    print(("RED-AS-REQUIRED" if hit else "STILL-GREEN"), "|", name, "|", len(failed), "failing:", expected[:60])
    bad += 0 if hit else 1
sys.exit(1 if bad else 0)

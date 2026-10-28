"""📓️ docs3-g: the doc batches that build on a code batch's own guide text (quietfc) and the foreign anchors, on a MIRROR root.

usage: python3 -I verify_codebatch.py <mirror root>
1. applies quietfc's own guide edit pairs (read with `ast` from quietfc-g-tools/edit_code.py, same rule as its `edit()`: skip when the new text is there) to the mirror guide,
2. quietfc doc batch: check pending, apply, check applied, the three 'staged' words are gone, revert restores the post-code-batch bytes,
3. applies every doc batch and checks that the foreign anchors (foreign.py) still match exactly once."""
import ast
import hashlib
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
root = os.path.abspath(sys.argv[1])
GUIDE = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
guide = os.path.join(root, GUIDE)
text = open(guide, encoding="utf8").read()
pristine_text = text
pristine_sha = hashlib.sha256(text.encode("utf8")).hexdigest()
tree = ast.parse(open(os.path.join(C, "quietfc-g-tools/edit_code.py"), encoding="utf8").read())
pairs = []
for node in ast.walk(tree):
    if isinstance(node, ast.Call) and getattr(node.func, "id", "") == "edit" and node.args and "build-guide.md" in ast.unparse(node.args[0]):
        pairs = ast.literal_eval(node.args[1])
for old, new in pairs:
    if new in text:
        continue
    assert text.count(old) == 1, old[:60]
    text = text.replace(old, new)
open(guide, "w", encoding="utf8").write(text)
post_code = hashlib.sha256(text.encode("utf8")).hexdigest()
print("quietfc code batch guide edits applied:", len(pairs), "pairs; guide sha", post_code[:12])


def run(name, mode):
    process = subprocess.run([sys.executable, "-I", os.path.join(HERE, "staged", name, "stage.py"), mode, "--root", root], capture_output=True, text=True)
    return process.returncode, process.stdout


failures = []


def expect(condition, message):
    print("ok  " if condition else "FAIL", message, flush=True)
    if not condition:
        failures.append(message)


code, out = run("quietfc", "check")
expect(code == 0 and "result: pending" in out, "quietfc doc batch is pending after the code batch")
code, out = run("quietfc", "apply")
expect(code == 0, "quietfc doc batch applies")
after = open(guide, encoding="utf8").read()
expect("(staged, quietfc-g)" not in after and "(vorgemerkt, quietfc-g)" not in after, "no 'staged' word of quietfc remains in either language")
expect("Quiet follow-up** (live)" in after and "Ruhige Anschlusszusage** (live)" in after, "both follow-up bullets say live")
code, out = run("quietfc", "revert")
expect(code == 0 and hashlib.sha256(open(guide, "rb").read()).hexdigest() == post_code, "revert restores the post-code-batch bytes")
for name in [row for row in sorted(os.listdir(os.path.join(HERE, "staged"))) if os.path.isdir(os.path.join(HERE, "staged", row))]:
    code, out = run(name, "apply")
    expect(code == 0, "apply %s on top of the code batch text (exit %d)" % (name, code))
proc = subprocess.run([sys.executable, "-I", os.path.join(HERE, "foreign.py"), root, root], capture_output=True, text=True)
print(proc.stdout)
expect(proc.returncode == 0, "foreign anchors still match after every doc batch")
open(guide, "w", encoding="utf8").write(pristine_text)
print("FAILURES: %d" % len(failures))
sys.exit(1 if failures else 0)

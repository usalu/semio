# 🧩 Appends the scratch host to a copy of the staged adapter beside it in the overlay, exporting the two private helpers the host needs.
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib

root = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/bworkers/r"
source = os.path.join(root, lib.BAZEL, lib.SCRIPT)
text = open(source, encoding="utf8").read()
e = lib.Edit(text, "scratch")
e.once("async function writeBazelRuntimeV1(", "export async function writeBazelRuntimeV1(")
e.once("async function bazelCancellationControl(", "export async function bazelCancellationControl(")
e.text += open(os.path.join(lib.TOOLS, "harness", "scratch.part.ts"), encoding="utf8").read()
target = os.path.join(root, lib.BAZEL, "\U0001F9EA️scratch.ts")
open(target, "w", encoding="utf8").write(e.text)
print("scratch adapter", len(e.text), target[-60:])

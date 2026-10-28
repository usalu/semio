"""📓️ docs5: which candidate identifiers are in the live source tree right now (one streamed `rg -F -f` over the framework, the documents themselves excluded)."""
import os
import subprocess
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isfile(os.path.join(ROOT, "AGENTS.md")):
    ROOT = os.path.dirname(ROOT)


def live(needles):
    root = os.environ.get("DOCS5_LIVE") or ROOT
    with tempfile.NamedTemporaryFile("w", delete=False, suffix=".txt", encoding="utf8") as handle:
        handle.write("\n".join(sorted(needles, key=lambda word: (-len(word), word))) + "\n")
        name = handle.name
    command = ["rg", "-F", "-o", "-N", "--no-filename", "--no-messages", "-g", "!*.log", "-g", "!*.jsonl", "-g", "!node_modules", "-g", "!🗑️generated", "-g", "!📓️build-guide.md", "-g", "!README.md", "-f", name, os.path.join(root, "🧰️framework")]
    seen = set()
    process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True, encoding="utf8")
    for line in process.stdout:
        seen.add(line.rstrip("\n"))
    process.wait()
    os.unlink(name)
    return seen

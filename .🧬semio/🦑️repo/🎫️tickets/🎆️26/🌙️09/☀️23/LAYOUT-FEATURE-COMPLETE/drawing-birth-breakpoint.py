from pathlib import Path
import re
import signal
import subprocess
import sys

lines = Path(sys.argv[1]).read_text().splitlines()
binary = next(re.search(r"\((\.cache/.*)\)$", row)[1] for row in lines if row.startswith("     Running") and "drawing_birth" in row)
args = [
    "lldb", "--batch",
    "-o", "settings set target.disable-aslr false",
    "-o", "breakpoint set --name main",
    "-o", "run parent_reference_copy_preserves_the_exact_original_birth_and_physical_ledger --exact",
    "-o", "breakpoint set --file 🧪️tests/🦀️.rs --line 134",
    "-o", "breakpoint modify --ignore-count 125 2",
    "-o", "continue",
    "-o", "breakpoint set --func-regex 'DrawingBirthPreparation.*step_selected'",
    "-o", "continue",
    "-o", "bt 8",
    "-o", "frame variable action",
    "-o", "register read x0 x1 x2 x3 x4 x5 x6 x7 x8",
    "-o", "disassemble --frame --count 16",
    str(Path(binary).resolve()),
]
debugger = subprocess.Popen(args)
try:
    sys.exit(debugger.wait(timeout=140))
except subprocess.TimeoutExpired:
    debugger.send_signal(signal.SIGINT)
    try:
        debugger.wait(timeout=5)
    except subprocess.TimeoutExpired:
        debugger.kill()
        debugger.wait()
    sys.exit("[DEBUG] The native debugger did not reach the selected breakpoint")

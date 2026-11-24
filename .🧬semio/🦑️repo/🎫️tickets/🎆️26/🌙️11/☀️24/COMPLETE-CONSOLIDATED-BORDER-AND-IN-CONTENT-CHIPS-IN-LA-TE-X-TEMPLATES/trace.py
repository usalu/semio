"""🔬 Builds a traced copy of a fixture and prints the body's vertical list of one shipped page.

usage: python trace.py <fixture dir> <main tex> [page] [extra levels]
"""
import io, os, re, subprocess, sys

here = os.path.dirname(os.path.abspath(__file__))
directory, main = sys.argv[1], sys.argv[2]
page = int(sys.argv[3]) if len(sys.argv) > 3 else 1
levels = int(sys.argv[4]) if len(sys.argv) > 4 else 1
source = io.open(os.path.join(directory, main), encoding="utf-8").read()
begin = r"\begin{document}"
traced = source.replace(begin, r"\tracingoutput=1 \showboxbreadth=10000 \showboxdepth=13 " + begin, 1)
name = "trace-" + main
io.open(os.path.join(directory, name), "w", encoding="utf-8").write(traced)
try:
    subprocess.run(["bun", os.path.join(here, "build.ts"), directory, name, "trace", "40"], check=False, capture_output=True)
finally:
    os.remove(os.path.join(directory, name))
log = io.open(os.path.join(os.environ["SEMIO_PRINT_SCRATCH"], "out", "trace", name.replace(".tex", ".log")), encoding="utf-8", errors="replace").read()
marker = "Completed box being shipped out [%d]" % page
start = log.index(marker)
end = log.find("Completed box being shipped out", start + len(marker))
lines = log[start:end if end > 0 else len(log)].split("\n")
box = re.compile(r"^(\.+)\\vbox\(([\d.]+)\+[\d.]+\)x")
node = re.compile(r"^(\.+)(\\.*)$")
body, depth = 0, 0
for index, line in enumerate(lines):
    match = box.match(line)
    if match and float(match.group(2)) > 500 and len(match.group(1)) > depth:
        body, depth = index, len(match.group(1))
for line in lines[body + 1:]:
    match = node.match(line)
    if not match:
        continue
    if len(match.group(1)) <= depth:
        break
    if len(match.group(1)) <= depth + levels:
        print("  " * (len(match.group(1)) - depth - 1) + match.group(2)[:110])

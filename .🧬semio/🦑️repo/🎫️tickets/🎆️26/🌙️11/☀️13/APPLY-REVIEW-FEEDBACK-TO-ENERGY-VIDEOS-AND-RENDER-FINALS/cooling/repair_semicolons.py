import glob
import re
import subprocess
from pathlib import Path

ROOT = Path(r"C:\git\semio")
FILES = ["solar-radiation", "heating-comparison", "summer-strategy", "internal-gains", "transmission-humidity", "system-sizing"]
PATTERNS = {name: str(ROOT / "*teaching" / "*architecture" / "*energy" / "*cooling" / "*clip" / ("*" + name) / "*.py") for name in FILES}
PATTERNS["clip-tests"] = str(ROOT / "*teaching" / "*architecture" / "*energy" / "*cooling" / "*clip" / "*tests" / "*.py")


def head(path: Path) -> str:
    rel = path.relative_to(ROOT).as_posix()
    out = subprocess.run(["git", "-c", "core.quotepath=false", "show", "HEAD:" + rel], cwd=ROOT, capture_output=True)
    return out.stdout.decode("utf-8")


for name, pattern in PATTERNS.items():
    matches = [Path(p) for p in glob.glob(pattern) if Path(p).name.endswith(".py")]
    if name == "clip-tests":
        matches = [p for p in matches if p.parent.parent.name.endswith("clip")]
    assert len(matches) == 1, (name, matches)
    path = matches[0]
    raw = path.read_bytes().decode("utf-8")
    crlf = "\r\n" in raw
    cur = raw.replace("\r\n", "\n").split("\n")
    heads = head(path).replace("\r\n", "\n").split("\n")
    table = {}
    for line in heads:
        table.setdefault(line.replace(";", " "), []).append(line)
    fixed, unmatched = [], []
    for number, line in enumerate(cur, 1):
        spaced = line.replace(";", " ")
        if spaced in table and len(set(table[spaced])) == 1:
            fixed.append(table[spaced][0])
            continue
        fixed.append(re.sub(r"\\ ", lambda m: "\\;", spaced))
        if spaced.strip():
            unmatched.append(number)
    text = "\n".join(fixed)
    if crlf:
        text = text.replace("\n", "\r\n")
    path.write_bytes(text.encode("utf-8"))
    print(name, "lines", len(cur), "unmatched", len(unmatched), "crlf", crlf)

from pathlib import Path
import re
puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())

def fix(path: Path):
    lines = path.read_text().splitlines()
    out = []
    in_c = False
    for line in lines:
        if line.startswith("kind-compatibility ["):
            in_c = True
            out.append(line)
            continue
        if in_c:
            if line.strip() == "}":
                in_c = False
                out.append(line)
                continue
            if "false general" in line or "true general" in line:
                out.append(line)
                continue
            # append defaults if row ends with true/false
            m = re.match(r'^(.*\s)(true|false)\s*$', line)
            if m:
                out.append(f"{m.group(1)}{m.group(2)} false general")
                continue
        out.append(line)
    path.write_text("\n".join(out) + "\n")
    # count
    t = path.read_text()
    print(path.name, "false general", t.count("false general"))

for rel in [
    "🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio",
    "🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio",
]:
    fix(puzzle / rel)

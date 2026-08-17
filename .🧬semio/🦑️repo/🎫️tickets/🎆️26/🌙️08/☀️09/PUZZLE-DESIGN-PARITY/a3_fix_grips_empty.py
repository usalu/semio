from pathlib import Path
puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
for rel in [
    "🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio",
    "🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio",
]:
    path = puzzle / rel
    lines = path.read_text().splitlines()
    out = []
    in_g = False
    for line in lines:
        if line.startswith("grips [id:TEXT code:TEXT"):
            in_g = True
            out.append(line)
            continue
        if in_g:
            if line.strip() == "}":
                in_g = False
                out.append(line)
                continue
            line = line.replace('[] "" ""', "[] - -")
        out.append(line)
    path.write_text("\n".join(out) + "\n")
    print("fixed", path.name)

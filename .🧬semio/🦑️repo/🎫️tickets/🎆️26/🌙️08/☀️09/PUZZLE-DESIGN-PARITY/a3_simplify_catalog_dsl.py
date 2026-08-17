from pathlib import Path
import re
puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())

def fix(path: Path):
    text = path.read_text()
    # Fix empty description in representations
    text = text.replace('description=""', 'description=none')
    # Fix remaining empty quoted strings in catalog parts header rows
    text = text.replace(' "" "" "" "" false', ' none none none none false')
    # tags=[mesh] is fine
    path.write_text(text)
    print(path.name, 'description=none', path.read_text().count('description=none'))

for rel in [
    "🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio",
    "🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio",
]:
    fix(puzzle / rel)

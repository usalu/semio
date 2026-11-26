"""🧹 Remove the artifact-wide PDF dependency from BIM crates that never use it."""
from pathlib import Path
import re

root = Path("✏️s/🔌️plugins/🏙️bim")
owners = {"semio-s-artifact-bim-model-sheets", "semio-s-artifact-bim-model-tests-5", "semio-s-artifact-bim-model-families-tests"}
changed = []
for path in root.rglob("Cargo.toml"):
    before = path.read_text()
    name = re.search(r'^name\s*=\s*"([^"]+)"', before, re.M)
    if not name or name.group(1) in owners:
        continue
    after, count = re.subn(r'^semio-s-artifact-stdio-pdf\s*=.*\n', '', before, flags=re.M)
    if count:
        if path.read_text() != before:
            raise RuntimeError(f"manifest changed during edit: {path}")
        path.write_text(after)
        changed.append(name.group(1))
print(f"Removed {len(changed)} unused dependencies")

for path in [Path("🌎️hub/Cargo.lock"), root / "🏭️bridge/Cargo.lock", root / "🗿️artifacts/🏢️model/Cargo.lock", Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.lock")]:
    before = path.read_text()
    blocks = before.split("[[package]]")
    removed = 0
    for index in range(1, len(blocks)):
        block = blocks[index]
        name = re.search(r'^name = "([^"]+)"', block, re.M)
        if name and name.group(1).startswith("semio-s-artifact-bim-model") and name.group(1) not in owners:
            blocks[index], count = re.subn(r'^ "semio-s-artifact-stdio-pdf",\n', '', block, flags=re.M)
            removed += count
    after = "[[package]]".join(blocks)
    if after != before:
        if path.read_text() != before:
            raise RuntimeError(f"lock changed during edit: {path}")
        path.write_text(after)
    print(f"{path}: removed {removed} lock edges")

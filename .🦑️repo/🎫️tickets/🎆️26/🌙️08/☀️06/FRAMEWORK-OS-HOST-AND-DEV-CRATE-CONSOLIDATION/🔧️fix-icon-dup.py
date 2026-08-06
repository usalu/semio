
from pathlib import Path
import re
import sys

root = Path("/Users/ueli/Documents/semio")
fw = next(p for p in root.iterdir() if p.is_dir() and "framework" in p.name)
idx = next(p for p in fw.rglob("📦️index.tsx") if "🖱️ui" in str(p) and "🎯️targets" in str(p) and "⚛️react" in str(p))
text = idx.read_text()

# Find Icon region import/export that includes both `Icon` value and `type Icon`
# Replace `type Icon,` with nothing in that Icons import/export pair; add `export type { Icon } from same path` once.

pattern = re.compile(
    r'(// #region 🔖️Icon\nimport \{)(.*?)(\} from ("[^"]*Icons[^"]*");\nexport \{)(.*?)(\} ;\n)',
    re.S,
)

# Simpler line-based approach
lines = text.splitlines(keepends=True)
out = []
i = 0
fixed = False
while i < len(lines):
    line = lines[i]
    if (
        not fixed
        and line.startswith("import {")
        and "Icons" in "".join(lines[i : i + 3])
        and "createIconComponent" in line
        and "type Icon," in line
        and ", Icon," in line.replace("type Icon,", "")
    ):
        # single-line import
        new_import = line.replace("type Icon, ", "").replace(", type Icon,", ",").replace(", type Icon }", " }")
        # also handle `type Icon,` specifically
        new_import = new_import.replace("type Icon, ", "")
        out.append(new_import)
        i += 1
        # next should be export {
        if i < len(lines) and lines[i].startswith("export {"):
            exp = lines[i]
            # keep Icon value export; remove type Icon from value export list
            new_exp = exp.replace("type Icon, ", "").replace(", type Icon,", ",").replace(", type Icon }", " }")
            out.append(new_exp)
            i += 1
            # insert export type for Icon from same module
            m = re.search(r'from ("[^"]+")', new_import)
            if not m:
                raise SystemExit("no from path")
            path = m.group(1)
            out.append(f"export type {{ Icon }} from {path};\n")
            fixed = True
            continue
        raise SystemExit("expected export after import")
    # multi-line? handle if import spans - in this file it's one long line
    out.append(line)
    i += 1

if not fixed:
    # try alternate: find line with both
    for j, line in enumerate(lines):
        if "createIconComponent" in line and "type Icon" in line and "Icon," in line and line.strip().startswith("import"):
            print("FOUND candidate", j + 1, "len", len(line))
            print(line[line.find("WorkbenchIcon") : line.find("WorkbenchIcon") + 80])
    raise SystemExit("Icon dup fix not applied")

idx.write_text("".join(out))
print("fixed Icon type/value clash in", idx)

# Also check for other value+type same name in same import that babel hates: Cursor already ok
# Verify no `type Icon` left next to value Icon in that region
text2 = idx.read_text()
# count
print("type Icon remaining near Icons:", len(re.findall(r'type Icon,', text2)))
print("export type { Icon }", "export type { Icon }" in text2)

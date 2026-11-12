"""Switches the python-oracle test files of the staffage suite to the locked oracle sets (OracleSet) and the new support module."""
import re, sys
from pathlib import Path

root = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests")
documents = {"pymupdf", "lxml", "ezdxf", "reportlab"}
changed = []
for path in sorted(root.glob("*/🟦️.ts")):
    text = path.read_text(encoding="utf8")
    if "python(" not in text or "🧰️support/🧰️.ts" not in text:
        continue
    if path.parent.name in ("📕️pdf-reopen", "🗜️zip-reopen"):
        continue
    original = text

    def group(match):
        names = set(re.findall(r'"([a-z]+)"', match.group(2)))
        return match.group(1) + ('"documents"' if names & documents else '"geometry"')

    text = re.sub(r'(python\([^\[\n]*?, )(\[[^\]]*\]|ORACLE_PACKAGES)', group, text)
    text = re.sub(r'const ORACLE_PACKAGES = \[[^\]]*\];\n', "", text)
    m = re.search(r'import \{([^}]*)\} from "\.\./🧰️support/🧰️\.ts";', text)
    names = [n.strip() for n in m.group(1).split(",") if n.strip()]
    names.remove("python")
    new = 'import { %s } from "../🧰️support/🧰️.ts";\nimport { python } from "../🧰️support/🐍️.ts";' % ", ".join(names)
    text = text.replace(m.group(0), new)
    if text != original:
        path.write_text(text, encoding="utf8")
        changed.append(path.parent.name)
print(len(changed), changed)

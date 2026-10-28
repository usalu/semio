"""🔗 Check that every relative link in the BIM clip README resolves."""

import re
from pathlib import Path
from urllib.parse import unquote

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
README = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/README.md"
targets = [target for target in re.findall(r"\]\(([^)]+)\)", README.read_text(encoding="utf-8")) if not target.startswith("http")]
missing = [target for target in targets if not (README.parent / unquote(target.split("#")[0])).exists()]
print(len(targets), "relative links,", len(missing), "missing", missing)

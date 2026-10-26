"""🔗 Audit live repository references to moved teaching coordinates."""
import json
import re
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "AGENTS.md").is_file())
PLAN = json.loads((TICKET / "taxonomy-paths.cases.json").read_text(encoding="utf-8-sig"))
needles = set()
for move in PLAN["moves"]:
    old = move["old"]
    needles.add(old)
    relative = old.removeprefix("🎓️teaching/")
    if "/" in relative:
        needles.add(relative)
    elif old.endswith((".py", ".tests.json", ".schema.json")) or relative == "video_manifest.json":
        needles.add(relative)
patterns = TICKET / "🗑️generated/taxonomy-reference-patterns.txt"
patterns.write_text("\n".join(sorted(needles)) + "\n", encoding="utf-8")
command = ["rg", "--files-with-matches", "--null", "--fixed-strings", "--file", str(patterns), "--hidden"]
for excluded in ["**/.[0-9a-f][0-9a-f][0-9a-f][0-9a-f]*", "**/.git/**", "**/.🧬semio/**", "**/node_modules/**", "**/__pycache__/**", "**/.venv/**", "**/dist/**", "**/target/**", "**/🗑️generated/**", "compose/**", "temp/compose/**", "♻️mit-bestand/🔎️recherche/**"]:
    command.extend(["--glob", "!" + excluded])
process = subprocess.run([*command, "."], cwd=ROOT, capture_output=True, check=False)
if process.returncode not in (0, 1):
    raise RuntimeError(process.stderr.decode("utf-8", errors="replace"))
paths = set(process.stdout.decode("utf-8").split("\0")) | {m["new"] for m in PLAN["moves"] if m["new"]}
matcher = re.compile("|".join(re.escape(needle) for needle in sorted(needles)))
text_extensions = {".py", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".json", ".jsonc", ".toml", ".md", ".sh", ".ps1", ".rs", ".go", ".cs", ".yaml", ".yml", ".txt", ".cmake"}
hits = []
read = 0
for relative in sorted(paths):
    path = Path(relative)
    parts = path.parts
    if not relative or relative.startswith(("compose/", "temp/compose/", "♻️mit-bestand/🔎️recherche/")):
        continue
    if any(part in {".🧬semio", ".git", "node_modules", "__pycache__", ".venv", "dist", "target", "🗑️generated"} for part in parts):
        continue
    if path.suffix not in text_extensions:
        continue
    try:
        data = (ROOT / path).read_text(encoding="utf-8-sig")
    except (OSError, UnicodeError):
        continue
    read += 1
    for number, line in enumerate(data.splitlines(), 1):
        matched = sorted(set(matcher.findall(line.replace("\\", "/"))))
        if matched:
            hits.append({"path": relative, "line": number, "coordinates": matched})
result = {"read_text_files": read, "hits": hits}
(TICKET / "🗑️generated/taxonomy-reference-audit.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
print(json.dumps(result, ensure_ascii=True))

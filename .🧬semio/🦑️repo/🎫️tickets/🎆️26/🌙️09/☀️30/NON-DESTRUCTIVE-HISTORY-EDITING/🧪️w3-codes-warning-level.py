"""🚦️ W3-CODES follow-up: the outcome level is spelled `warning` (the `Severity` wire name) everywhere — committed `🎯️outcome`
documents, the Python second implementations that emit outcome dicts, the remodel TypeScript twin, the per-case Rust
readers, and the feature adapters, whose `level_of` alias normaliser is deleted. Idempotent; `--dry` reports only."""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
DRY = "--dry" in sys.argv
LEVEL_OF = re.compile(r"\n(?:[ \t]*///[^\n]*\n)*([ \t]*)fn level_of\(word: &str\) -> String \{\n\s*if word == \"warn\" \{\n\s*\"warning\"\.to_string\(\)\n\s*\} else \{\n\s*word\.to_string\(\)\n\s*\}\n\1\}\n")
RULES = {
    "outcome": [(re.compile(r'("level":\s*)"warn"'), r'\1"warning"')],
    "rust": [
        (re.compile(r'"warn" => protocol::Severity::Warning'), '"warning" => protocol::Severity::Warning'),
        (re.compile(r'Some\("warn"\)'), 'Some("warning")'),
        (LEVEL_OF, "\n"),
        (re.compile(r'level_of\(&(\w+\.str\("level"\))\)'), r"\1"),
    ],
    "python": [(re.compile(r'("level":\s*)"warn"'), r'\1"warning"')],
    "twin": [(re.compile(r'"warn"'), '"warning"')],
    "feature": [(re.compile(r"\{level: warn,"), "{level: warning,")],
}


def tracked(*patterns):
    out = subprocess.run(["git", "ls-files", "-z", "--", *patterns], cwd=ROOT, capture_output=True, check=True).stdout
    return [ROOT / p.decode() for p in out.split(b"\0") if p and not p.startswith(".🧬semio/".encode())]


TARGETS = {
    "outcome": tracked("*🎯️outcome/🔣️.json"),
    "rust": [path for path in tracked("✏️s/*.rs") if "/🧪️tests/" in str(path)],
    "python": [path for path in tracked("✏️s/*🐍️.py") if "/🧪️tests/" in str(path)],
    "twin": tracked("✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts"),
    "feature": tracked("✏️s/*🥒️.feature"),
}

for kind, paths in TARGETS.items():
    for path in paths:
        if not path.exists():
            continue
        text = original = path.read_text(encoding="utf-8")
        for pattern, replacement in RULES[kind]:
            text = pattern.sub(replacement, text)
        if text != original:
            print(f"[w3-codes] {kind} {path.relative_to(ROOT)}")
            if not DRY:
                path.write_text(text, encoding="utf-8")

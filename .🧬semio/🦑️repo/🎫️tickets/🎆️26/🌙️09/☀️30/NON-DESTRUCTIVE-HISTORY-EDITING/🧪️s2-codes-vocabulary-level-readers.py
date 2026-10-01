"""🎚️ S2-CODES: per-case outcome readers took a rejected outcome's level from a second hand-written rule (`invariant` ⇒
Fatal, else Error), which mis-levels `duplicate-id` and `mutation.apply.*`. Every reader now asks the one vocabulary table
(`protocol::outcome_code_level`). Rewrites the layout per-case readers, their author template and the lowpoly paint reader.
Idempotent; `--dry` reports only."""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
DRY = "--dry" in sys.argv
LAYOUT = re.compile(
    r'let fatal = outcome\["code"\]\.as_str\(\) == Some\("mutation\.invariant"\);\n(\s*)return vec!\[\(if fatal \{\{? protocol::Severity::Fatal \}\}? else \{\{? protocol::Severity::Error \}\}?, outcome\["code"\]\.as_str\(\)\.expect\("a code"\)\.to_string\(\), strings\(&outcome\["path"\]\)\)\];'
)
LOWPOLY = re.compile(r'Some\("rejected"\) => vec!\[\(if outcome\["code"\] == "mutation\.invariant" \{ "fatal" \} else \{ "error" \}\.to_string\(\), outcome\["code"\]\.as_str\(\)\.unwrap_or_default\(\)\.to_string\(\)\)\],')


def layout(match: re.Match) -> str:
    return f'let code = outcome["code"].as_str().expect("a code");\n{match.group(1)}return vec![(protocol::outcome_code_level(code).expect("a vocabulary code"), code.to_string(), strings(&outcome["path"]))];'


def lowpoly(_: re.Match) -> str:
    return 'Some("rejected") => { let code = outcome["code"].as_str().unwrap_or_default(); vec![(to_json(&protocol::outcome_code_level(code).expect("a vocabulary code")).as_str().unwrap_or_default().to_string(), code.to_string())] }'


tracked = subprocess.run(["git", "ls-files", "-z", "--", "✏️s/🔌️plugins/📏️layout/*.rs", "✏️s/🔌️plugins/💠️lowpoly/*.rs"], cwd=ROOT, capture_output=True, check=True).stdout
paths = [ROOT / p.decode() for p in tracked.split(b"\0") if p] + [TICKET / "🧪️w3-t-layout-author-vectors.py"]
for path in paths:
    text = original = path.read_text(encoding="utf-8")
    text = LAYOUT.sub(layout, text)
    text = LOWPOLY.sub(lowpoly, text)
    if text != original:
        print(f"[s2-codes] vocabulary level reader {path.relative_to(ROOT)}")
        if not DRY:
            path.write_text(text, encoding="utf-8")

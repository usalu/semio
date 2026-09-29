"""🧯️ S20 F2a (faults overlay): every `Fault::from("<code-like literal>")` in the non-test sources of the app crates
(`✏️s/🔌️plugins/**`) becomes `app_fault("<code>")` — the literal WAS a code (it rode as the message of the untyped
`app.message`). `app_fault` joins the file's existing `use …::{…, Fault, …}` import when there is one, else the call is
qualified by the crate that file already names `Fault` through. Writes the per-plugin list of converted codes (the en/de
declarations F3 owes) to <report>. Idempotent. Usage: python3 f2-code-like.py <root> <report.json> [--dry-run]"""
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(sys.argv[1])
REPORT = Path(sys.argv[2])
DRY = "--dry-run" in sys.argv
PLUGINS = ROOT / "✏️s/🔌️plugins"
CALL = re.compile(r'(?<![\w:])(?:[A-Za-z_]\w*::)*Fault::from\(\s*"([a-z0-9]+(?:[.\-/][a-z0-9]+)+)"\s*\)')
USE = re.compile(r"use ((?:::)?[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)::\{([^}]*)\}\s*;")


def crate_prefix(path: Path) -> str:
    for parent in path.parents:
        manifest = parent / "📦️packages/🦀️rust/Cargo.toml"
        if manifest.exists():
            deps = manifest.read_text()
            if re.search(r"^semio-framework-plugin\s*=", deps, re.M):
                return "semio_framework_plugin"
            return "semio_framework"
    return "semio_framework_plugin"


def main() -> None:
    codes: dict[str, set[str]] = defaultdict(set)
    converted = files = 0
    for path in PLUGINS.rglob("*.rs"):
        rel = path.relative_to(PLUGINS)
        if "🧪️tests" in rel.parts or "target" in rel.parts:
            continue
        text = path.read_text()
        calls = CALL.findall(text)
        if not calls:
            continue
        for code in calls:
            codes[rel.parts[0]].add(code)
        importer = next((match for match in USE.finditer(text) if re.search(r"(^|[\s,])Fault([\s,]|$)", match.group(2)) and not match.group(2).strip().startswith("self")), None)
        if importer is not None:
            new = CALL.sub(lambda match: f'app_fault("{match.group(1)}")', text)
            names = importer.group(2)
            if not re.search(r"(^|[\s,])app_fault([\s,]|$)", names):
                start = importer.start(2)
                new = new[:start] + "app_fault, " + new[start:] if new[start:start + len(names)] == names else new
        else:
            prefix = crate_prefix(path)
            new = CALL.sub(lambda match: f'{prefix}::app_fault("{match.group(1)}")', text)
        converted += len(calls)
        files += 1
        if not DRY:
            path.write_text(new)
    REPORT.write_text(json.dumps({plugin: sorted(found) for plugin, found in sorted(codes.items())}, ensure_ascii=False, indent=1))
    print(f"{'dry-run' if DRY else 'applied'}: {converted} Fault::from code-like calls in {files} files; {sum(len(found) for found in codes.values())} distinct codes")


if __name__ == "__main__":
    main()

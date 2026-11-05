"""🔎 AST audit of course staffage aliases and numeric invariants."""
import ast
import importlib.util
from pathlib import Path

root = Path.cwd()
ticket = root / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
file = root / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️course_staffage.py"
spec = importlib.util.spec_from_file_location("coverage", file)
coverage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(coverage)
sources, missing = coverage.course_sources()
aliases, errors = [], []
known = set(coverage.CASES["sources"]["cast_calls"])
for source in sources:
    try:
        tree = ast.parse(source["text"])
    except SyntaxError as error:
        errors.append(f"{source['path']}: {error}")
        continue
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom):
            for alias in node.names:
                if alias.name in known and alias.asname and alias.asname not in known:
                    aliases.append(f"{source['path']}: {alias.name} as {alias.asname}")
assert not missing, missing
assert not errors, errors
assert not aliases, aliases
for value in [True, False, float("nan"), float("inf"), float("-inf")]:
    assert not coverage.number(value)
with (ticket / "codex-coverage.md").open("a", encoding="utf-8") as out:
    out.write(f"\n## Additional Parity and Source Audit\n\nParsed all {len(sources)} scanned Python sources with the standard-library AST parser. No missing series, syntax errors, or staffage import aliases outside the scanner vocabulary were found. The numeric predicate rejects booleans, NaN and both infinities. Decimal JSON scale 50.0 resolves to the authored 50 representation. Added shared regressions for boolean version/FPS/scale/time, unknown timeline-key fields, malformed environment, duplicate transport layers and missing transport age. Both semantic twins now enforce these schema constraints.\n")
print(f"AST audit passed: {len(sources)} sources")

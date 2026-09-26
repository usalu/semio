#!/usr/bin/env python3
"""🧫️ Python oracle sweep (session 12): nine `🧿️semio` subset references each re-parsed their scenario steps for ONE
hard-coded fixture scheme (`asset://`, `local://`), and the features moved their vectors to `shared://` — so the
references found no vector (`not enough values to unpack (expected 3, got 0)`, `names no local:// fixture`). The host
Context now answers every fixture URI a scenario's steps name through the platform's one grammar (`FIXTURE_URI_RE`,
`🧪️test/🟦️.ts`), and the references use it instead of their own scans. Usage: semio-step-fixture-uris.py [--write]"""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HOST = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
SUBSETS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
write = "--write" in sys.argv
edits, problems = {}, []


def text(path):
    return edits.get(path) or path.read_text(encoding="utf-8")


def replace(path, old, new, count=1):
    source = text(path)
    if source.count(old) != count:
        problems.append(f"{path.relative_to(ROOT)}: expected {count}× {old[:70]!r}, found {source.count(old)}")
        return
    edits[path] = source.replace(old, new)


def drop_function(path, name):
    source = text(path)
    match = re.search(r"^def " + name + r"\(.*?(?=^\S)", source, re.S | re.M)
    if match is None:
        problems.append(f"{path.relative_to(ROOT)}: def {name} not found")
        return
    edits[path] = source[: match.start()] + source[match.end():]


replace(HOST, "import os\n", "import os\nimport re\n")
replace(HOST, "class Context:\n", '''#: 🧫️ The platform's one fixture-URI grammar — the Python twin of `FIXTURE_URI_RE` in `🧪️test/🟦️.ts`.
FIXTURE_URI = re.compile(r"\\b(shared|local|asset|schema)://([^\\s\\"'`,;)\\]]+)")


class Context:
''')
replace(HOST, "    def fixture_bytes(self, uri: str) -> bytes:\n", '''    def step_fixture_uris(self) -> List[str]:
        """🔗️ Every fixture URI the scenario's steps name — step text and data-table cells, in step order, whatever
        scheme the feature uses. The feature is the single place a vector path is written down."""
        return [match.group(0) for step in self.scenario["steps"] for text in [step.get("text", "")] + [cell for row in (step.get("dataTable") or []) for cell in row] for match in FIXTURE_URI.finditer(text)]

    def fixture_bytes(self, uri: str) -> bytes:
''')

for case, helper, call, new_call in [
    ("🧰️kit/🧪️tests/🧰️mutate-semio-kit", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("🧊️brep/🧪️tests/🧊️mutate-semio-brep", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("🕸️graph/🧪️tests/🌳️mutate-semio-graph", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("📦️object/🧪️tests/📦️mutate-semio-object", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("📊️table/🧪️tests/📊️mutate-semio-table", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("🔤️text/🧪️tests/🔤️mutate-semio-text", "step_assets", "step_assets(ctx)", "ctx.step_fixture_uris()"),
    ("🏛️model/🧪️tests/🏛️mutate-semio-model", "step_fixtures", 'step_fixtures(ctx, "local")', "ctx.step_fixture_uris()"),
    ("🔢️value/🧪️tests/🔢️mutate-semio-value", "step_fixtures", "step_fixtures(ctx)", "ctx.step_fixture_uris()"),
    ("🌊️flow/🧪️tests/🌊️mutate-semio-flow", "step_fixture", 'step_fixture(ctx, "local")', "ctx.step_fixture_uris()[0]"),
]:
    path = SUBSETS / case / "🐍️.py"
    drop_function(path, helper)
    replace(path, call, new_call)

host_source = text(HOST)
if "from typing import" in host_source and "List" not in host_source.split("from typing import", 1)[1].split("\n", 1)[0]:
    problems.append("host: typing.List not imported")
for path in edits:
    print(("write " if write and not problems else "plan  ") + str(path.relative_to(ROOT))[-90:])
print(f"files={len(edits)} problems={len(problems)}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, source in edits.items():
        path.write_text(source, encoding="utf-8")

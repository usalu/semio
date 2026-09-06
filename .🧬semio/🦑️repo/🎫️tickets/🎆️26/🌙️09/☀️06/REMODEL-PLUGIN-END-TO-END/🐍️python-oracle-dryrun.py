#!/usr/bin/env python3
"""🐍️ Executes `📸️mutate-remodeling-1`'s Python oracle adapter offline, without cargo, bun or the
generated host, so the harness rewrite is validated by RUNNING it rather than by reading it.

It reproduces the three things the real host gives an adapter — the expanded scenario list (doc
strings with `<placeholder>` tokens substituted per Examples row), fixture resolution by URI scheme,
and the `Adapter`/`Context`/`Outcome` surface of
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` — and then calls every
registered oracle handler. A handler that raises is reported with its scenario id.

Usage: python3 🐍️python-oracle-dryrun.py
"""
import importlib.util
import json
import os
import sys
import types

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
OWNER_REL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
CASE_REL = f"{OWNER_REL}/🧪️tests/📸️mutate-remodeling-1"
BASE_REL = {"asset": OWNER_REL, "local": f"{CASE_REL}/🧫️fixtures", "shared": f"{OWNER_REL}/🧫️fixtures"}


class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None, artifacts=None, production_dispatch=None):
        self.projection, self.raw = projection, raw


class Context:
    def __init__(self, scenario):
        self.scenario = scenario

    def fixture(self, uri):
        scheme, name = uri.split("://", 1)
        return os.path.join(REPO_ROOT, BASE_REL[scheme], name)

    def fixture_bytes(self, uri):
        with open(self.fixture(uri), "rb") as handle:
            return handle.read()


class Adapter:
    def __init__(self, implementation="python"):
        self.implementation, self.scenarios = implementation, {}

    def oracle(self, scenario, handler):
        self.scenarios.setdefault(scenario, {})["oracle"] = handler
        return self

    def subject(self, scenario, handler):
        self.scenarios.setdefault(scenario, {})["subject"] = handler
        return self


def install_host_stub():
    """🧭️ Stands in for the installed `semio_repo_test` package the generated host provides."""
    module = types.ModuleType("semio_repo_test")
    module.Adapter, module.Context, module.Outcome = Adapter, Context, Outcome
    sys.modules["semio_repo_test"] = module


def scenarios_from(feature_text):
    """🥒️ Expands the feature's blocks the way `materializeScenario` does: `@id-<base>` plus the
    Examples row's own `id` column, with `<placeholder>` substituted into every step and doc string."""
    lines = feature_text.splitlines()
    blocks, index = [], 0
    while index < len(lines):
        stripped = lines[index].strip()
        if not stripped.startswith("@"):
            index += 1
            continue
        tags = []
        while index < len(lines) and lines[index].strip().startswith("@"):
            tags.append(lines[index].strip())
            index += 1
        if index >= len(lines) or "Scenario" not in lines[index]:
            continue
        index += 1
        steps, doc, in_doc = [], None, False
        while index < len(lines) and "Examples:" not in lines[index] and not lines[index].strip().startswith("@"):
            stripped = lines[index].strip()
            if stripped == '"""':
                if in_doc:
                    steps[-1]["docString"] = "\n".join(doc)
                    doc, in_doc = None, False
                else:
                    doc, in_doc = [], True
            elif in_doc:
                doc.append(stripped)
            elif stripped.startswith(("Given ", "And ", "When ", "Then ")):
                steps.append({"text": stripped})
            index += 1
        table = []
        if index < len(lines) and "Examples:" in lines[index]:
            index += 1
            while index < len(lines) and lines[index].strip().startswith("|"):
                table.append([cell.strip() for cell in lines[index].strip().strip("|").split("|")])
                index += 1
        base = next(tag[len("@id-"):] for tag in tags if tag.startswith("@id-"))
        if not table:
            blocks.append({"id": base, "steps": steps})
            continue
        header, rows = table[0], table[1:]
        for row in rows:
            mapping = dict(zip(header, row))
            expanded = []
            for step in steps:
                copy = {"text": step["text"]}
                if "docString" in step:
                    copy["docString"] = step["docString"]
                for column, value in mapping.items():
                    copy["text"] = copy["text"].replace(f"<{column}>", value)
                    if "docString" in copy:
                        copy["docString"] = copy["docString"].replace(f"<{column}>", value)
                expanded.append(copy)
            blocks.append({"id": f"{base}-{mapping['id']}", "steps": expanded})
    return blocks


def main():
    install_host_stub()
    spec = importlib.util.spec_from_file_location("remodel_oracle", os.path.join(REPO_ROOT, CASE_REL, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    adapter = module.adapter()
    planned = scenarios_from(open(os.path.join(REPO_ROOT, CASE_REL, "🥒️.feature"), encoding="utf-8").read())
    unregistered = [scenario["id"] for scenario in planned if "oracle" not in adapter.scenarios.get(scenario["id"], {})]
    unknown = sorted(set(adapter.scenarios) - {scenario["id"] for scenario in planned})
    failures = []
    for scenario in planned:
        handler = adapter.scenarios.get(scenario["id"], {}).get("oracle")
        if handler is None:
            continue
        try:
            outcome = handler(Context(scenario))
            assert outcome.projection is not None
        except Exception as error:
            failures.append((scenario["id"], f"{type(error).__name__}: {error}"))
    print(f"[dry-run] {len(planned)} planned scenario(s), {len(adapter.scenarios)} registered")
    for scenario_id in unregistered:
        print(f"[UNREGISTERED] {scenario_id}")
    for scenario_id in unknown:
        print(f"[UNKNOWN] adapter registers {scenario_id}, which the feature does not plan")
    for scenario_id, message in failures:
        print(f"[FAIL] {scenario_id}\n       {message[:600]}")
    print(f"[dry-run] {len(planned) - len(unregistered) - len(failures)} passed, {len(failures)} failed, {len(unregistered)} unregistered, {len(unknown)} unknown")
    return 1 if failures or unregistered or unknown else 0


if __name__ == "__main__":
    sys.exit(main())

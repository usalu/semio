#!/usr/bin/env python3
"""🔍️ Census of every s-plugin artifact, its standards, subsets, mutations, fixtures and tests.

Emits a JSON census plus a markdown violation report so the fleet can be sharded by artifact.
Violations tracked:
  - ARTIFACT_LEVEL_TESTS: a `🧪️tests/<case>` folder sitting at artifact level instead of subset level.
  - SUBSET_NO_IMPL: a subset folder with no own `🚪️io`/`🧬️schema` implementation.
  - SUBSET_NO_TESTS: a subset with mutations but no own `🧪️tests`.
  - MUTATION_NO_FIXTURE: a declared mutation carrying NEITHER form of committed evidence.

⚠️ This was the ticket's FIRST-HOUR reconnaissance script and its fixture heuristic was wrong: it
looked only for a `🧫️fixtures/<mutation>` folder, which is not how most of the repository declares
evidence, and so reported 2168 false positives. Evidence takes two legitimate forms — a v1 physical
vector bundle at `<mutation>/🧪️tests/<case>/` (`🦠️mutation`, `📸️snapshot/⬅️before`,
`📸️snapshot/➡️after`, `🔺️diff`, `🎯️outcome`) or a v2 `fixtureManifests` entry in the owning subset's
`🧪️oracle/🔣️.json`. Both are now checked below.

The AUTHORITY on both of this ticket's laws is the repository's own gate, not this script:
`bun ./📜️script.ts test contract`, whose `mutation-without-fixture` and `case-above-subset` rules
were written and hand-calibrated for exactly this question. Use this script for shape reconnaissance
only.
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
PLUGINS = ROOT / "✏️s" / "🔌️plugins"

ARTIFACTS_DIR = "🗿️artifacts"
STANDARDS_DIR = "🏅️standards"
SUBSETS_DIR = "🪆️subsets"
TESTS_DIR = "🧪️tests"
FIXTURES_DIR = "🧫️fixtures"
SCHEMA_DIR = "🧬️schema"
MUTATIONS_DIR = "🧬️mutations"
IO_DIR = "🚪️io"
ORACLE_DIR = "🧪️oracle"
EXAMPLES_DIR = "📚️examples"
GENERATOR_DIR = "🏭️generator"

SKIP_DIR_PARTS = {"target", "node_modules", "dist", "build", ".git"}


def subdirs(path: Path) -> list[Path]:
    if not path.is_dir():
        return []
    return sorted((p for p in path.iterdir() if p.is_dir() and p.name not in SKIP_DIR_PARTS), key=lambda p: p.name)


def bare_name(directory: str) -> str:
    """🏷️ The kebab-case mutation id inside a leaf directory name, whose emoji prefix may or may not
    carry a U+FE0F variation selector — splitting on the selector alone silently keeps the emoji."""
    return "".join(ch for ch in directory if ch.isascii() and (ch.isalnum() or ch == "-")).strip("-")


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def scan_mutations(schema: Path) -> list[dict]:
    mutations_root = schema / MUTATIONS_DIR
    out = []
    for mutation in subdirs(mutations_root):
        descriptor = mutation / "🔣️.json"
        if not descriptor.is_file():
            continue
        try:
            if "owner" not in json.loads(descriptor.read_text(encoding="utf-8")):
                continue
        except (json.JSONDecodeError, OSError):
            continue
        tests = [t.name for t in subdirs(mutation / TESTS_DIR)]
        out.append(
            {
                "name": mutation.name,
                "path": rel(mutation),
                "has_json": (mutation / "🔣️.json").is_file(),
                "has_schema_json": (mutation / "🔣️.schema.json").is_file(),
                "has_rs": (mutation / "🦀️.rs").is_file(),
                "has_ts": (mutation / "🟦️.ts").is_file(),
                "tests": tests,
            }
        )
    return out


def oracle_fixture_ids(manifest: Path) -> list[str]:
    """🧫️ Every fixture id the owner contribution declares, the v2 form of committed evidence."""
    if not manifest.is_file():
        return []
    try:
        data = json.loads(manifest.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return []
    return [str(entry.get("id", "")) for entry in data.get("fixtureManifests", [])]


def scan_subset(subset: Path) -> dict:
    schema = subset / SCHEMA_DIR
    fixtures = subset / FIXTURES_DIR
    return {
        "name": subset.name,
        "path": rel(subset),
        "has_io": (subset / IO_DIR).is_dir(),
        "has_schema": schema.is_dir(),
        "has_oracle": (subset / ORACLE_DIR).is_dir(),
        "has_generator": (subset / GENERATOR_DIR).is_dir(),
        "has_examples": (subset / EXAMPLES_DIR).is_dir(),
        "fixtures": [f.name for f in subdirs(fixtures)],
        "oracle_fixture_ids": oracle_fixture_ids(subset / ORACLE_DIR / "🔣️.json"),
        "tests": [t.name for t in subdirs(subset / TESTS_DIR)],
        "mutations": scan_mutations(schema),
    }


def scan_artifact(artifact: Path) -> dict:
    standards = []
    for standard in subdirs(artifact / STANDARDS_DIR):
        subsets = [scan_subset(s) for s in subdirs(standard / SUBSETS_DIR)]
        standards.append({"name": standard.name, "path": rel(standard), "subsets": subsets})
    return {
        "name": artifact.name,
        "path": rel(artifact),
        "artifact_tests": [t.name for t in subdirs(artifact / TESTS_DIR)],
        "artifact_fixtures": [f.name for f in subdirs(artifact / FIXTURES_DIR)],
        "artifact_mutations": scan_mutations(artifact / SCHEMA_DIR),
        "standards": standards,
    }


def collect() -> list[dict]:
    plugins = []
    for plugin in subdirs(PLUGINS):
        artifacts = [scan_artifact(a) for a in subdirs(plugin / ARTIFACTS_DIR)]
        if artifacts:
            plugins.append({"name": plugin.name, "path": rel(plugin), "artifacts": artifacts})
    return plugins


def violations(plugins: list[dict]) -> list[dict]:
    found = []
    for plugin in plugins:
        for artifact in plugin["artifacts"]:
            where = f"{plugin['name']}/{artifact['name']}"
            for test in artifact["artifact_tests"]:
                found.append({"kind": "ARTIFACT_LEVEL_TESTS", "where": where, "detail": test, "path": artifact["path"]})
            for mutation in artifact["artifact_mutations"]:
                found.append({"kind": "ARTIFACT_LEVEL_MUTATION", "where": where, "detail": mutation["name"], "path": mutation["path"]})
            for standard in artifact["standards"]:
                # 🪆️Evidence is looked up by DECLARED ownership, pooled across the standard's subsets:
                # a leaf directory must sit beside its aggregate (validate_mutation_leaf_source), so its
                # physical subset is often NOT the subset whose manifest owns and fixtures it.
                pooled = {str(fixture) for subset in standard["subsets"] for fixture in subset["oracle_fixture_ids"]}
                pooled |= {fixture for subset in standard["subsets"] for fixture in subset["fixtures"]}
                for subset in standard["subsets"]:
                    tag = f"{where}/{standard['name']}/{subset['name']}"
                    if not subset["has_io"] and not subset["has_schema"]:
                        found.append({"kind": "SUBSET_NO_IMPL", "where": tag, "detail": "no 🚪️io and no 🧬️schema", "path": subset["path"]})
                    if subset["mutations"] and not subset["tests"]:
                        found.append({"kind": "SUBSET_NO_TESTS", "where": tag, "detail": f"{len(subset['mutations'])} mutations, 0 subset tests", "path": subset["path"]})
                    for mutation in subset["mutations"]:
                        bare = bare_name(mutation["name"])
                        vectored = bool(mutation["tests"])
                        manifested = any(bare in fixture for fixture in pooled)
                        if not vectored and not manifested:
                            found.append({"kind": "MUTATION_NO_FIXTURE", "where": tag, "detail": mutation["name"], "path": mutation["path"]})
    return found


def main() -> int:
    out_dir = Path(__file__).resolve().parent / "🗑️generated"
    out_dir.mkdir(exist_ok=True)
    plugins = collect()
    (out_dir / "census.json").write_text(json.dumps(plugins, ensure_ascii=False, indent=2), encoding="utf-8")
    found = violations(plugins)
    (out_dir / "violations.json").write_text(json.dumps(found, ensure_ascii=False, indent=2), encoding="utf-8")

    counts: dict[str, int] = {}
    for violation in found:
        counts[violation["kind"]] = counts.get(violation["kind"], 0) + 1
    total_subsets = sum(len(s["subsets"]) for p in plugins for a in p["artifacts"] for s in a["standards"])
    total_mutations = sum(len(ss["mutations"]) for p in plugins for a in p["artifacts"] for s in a["standards"] for ss in s["subsets"])
    print(f"plugins={len(plugins)} artifacts={sum(len(p['artifacts']) for p in plugins)} subsets={total_subsets} subset_mutations={total_mutations}")
    for kind, count in sorted(counts.items(), key=lambda kv: -kv[1]):
        print(f"  {kind}: {count}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

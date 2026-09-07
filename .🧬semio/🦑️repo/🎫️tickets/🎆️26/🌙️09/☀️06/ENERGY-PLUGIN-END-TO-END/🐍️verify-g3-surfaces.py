"""🧭️ G3 — asserts that every G3 kind reached every surface the scaffolding recipe names.

The generator writes twelve surfaces per kind and nine artefacts per fixture vector. A kind that is
declared in the spec table but silently missing from, say, the crate mount block or the protobuf
`oneof` is invisible until a full crate build — and the build is contended for hours. This reads the
emitted files back and checks the whole fan-out from the spec table, in seconds.

Run from anywhere: `python3 <this file>`.
"""

from __future__ import annotations

import importlib.util
import json
import os

ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END")
SUBSET = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
CRATE = "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs"
CASE_LEAVES = ("📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🦠️mutation/🔣️.json", "🔺️diff/🔣️.json", "🎯️outcome/🔣️.json", "🦀️.rs")


def read(relative: str) -> str:
    return open(os.path.join(ROOT, relative), encoding="utf-8").read()


def main() -> int:
    os.chdir(ROOT)
    spec = importlib.util.spec_from_file_location("generator", os.path.join(TICKET, "🐍️generate-mutation-leaves.py"))
    generator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(generator)
    kinds = [k for k in generator.KINDS if 500 <= k.number <= 699]
    crate = read(CRATE)
    aggregate = read(f"{SUBSET}/🧬️schema/🧬️mutations/🦀️.rs")
    proto = read(f"{SUBSET}/🧬️schema/🧬️mutations/🛰️.proto")
    typescript = read(f"{SUBSET}/🧬️schema/🧬️mutations/🟦️.ts")
    graphql = read(f"{SUBSET}/🧬️schema/🧬️mutations/🔗️.graphql")
    grammar = read(f"{SUBSET}/🧬️schema/🧬️mutations/📖️.grammar.semio")
    feature = read(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🥒️.feature")
    adapter = read(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🦀️.rs")
    second = read(f"{SUBSET}/🧪️tests/🏛️mutate-energy-model-1/🐍️.py")
    oracle = json.loads(read(f"{SUBSET}/🔮️oracle/🔣️.json"))
    catalog = set(oracle["mutationCatalogs"][0]["kinds"])
    manifest = {row["id"] for row in oracle["mutationManifests"][0]["mutations"]}
    problems = []
    for kind in kinds:
        for label, haystack, needle in (
            ("crate mount", crate, f"pub mod {kind.module}"),
            ("aggregate variant", aggregate, f"    {kind.variant}({kind.variant}),"),
            ("aggregate KINDS row", aggregate, f'    "{kind.slug}",'),
            ("aggregate DIRECTORIES row", aggregate, f'("{kind.slug}", "{kind.dir}")'),
            ("protobuf message", proto, f"message {kind.variant} "),
            ("protobuf field number", proto, f"= {kind.number};"),
            ("typescript interface", typescript, f"export interface {kind.variant}"),
            ("graphql type", graphql, f"type {kind.variant} "),
            ("grammar production", grammar, kind.slug),
            ("python vocabulary row", second, f'"{kind.slug}": {kind.module},'),
        ):
            if needle not in haystack:
                problems.append(f"{kind.slug}: missing {label} ({needle!r})")
        if kind.slug not in catalog:
            problems.append(f"{kind.slug}: missing from the oracle mutation catalog")
        if kind.slug not in manifest:
            problems.append(f"{kind.slug}: missing from the oracle mutation manifest")
        for emoji, name, _description, _scenario in kind.cases:
            vector = f"{kind.slug}-{name}"
            if f"| {vector} | {kind.dir} | {emoji}{name} |" not in feature:
                problems.append(f"{vector}: missing its 🥒️.feature Examples row")
            if f'id: "{vector}"' not in adapter:
                problems.append(f"{vector}: missing its Rust subject-adapter Vector")
            if f'"{vector}": "asset://' not in second:
                problems.append(f"{vector}: missing its python VECTOR_ROOTS entry")
            for leaf in CASE_LEAVES:
                if not os.path.exists(os.path.join(SUBSET, "🧬️schema/🧬️mutations", kind.dir, "🧪️tests", f"{emoji}{name}", leaf)):
                    problems.append(f"{vector}: missing {leaf}")
    vectors = sum(len(kind.cases) for kind in kinds)
    print(f"{len(kinds)} G3 kinds across 12 surfaces, {vectors} vectors across 9 artefacts")
    for problem in problems:
        print("MISSING", problem)
    print("PASS" if not problems else f"{len(problems)} PROBLEMS")
    return 1 if problems else 0


if __name__ == "__main__":
    raise SystemExit(main())

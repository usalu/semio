#!/usr/bin/env python3
"""🔏️ ST2 item 1b: the registry catalog chain replays only sound cache entries, so `--skip-nx-cache` leaves the rebuild chain.

Root cause (measured, `📓️wp-st2.md` 20:0x): `repo:generator-inputs` digests every plugin descriptor, example and the
registry implementation closure into a receipt, but was cached on membership inputs only, so Nx replayed a receipt older
than the bytes it digests and `plugin-registry:generate` replayed a stale catalog (chain run 4, 2026-09-27 13:0x).
Fix: the producer re-digests on every run (uncached, dropped from `cachedExact`); `generate` already keys on the receipt
through `dependentTasksOutputFiles`; the registry `check` gate now does too; the rebuild chain drops `--skip-nx-cache`.
Law: `⚡️caching/🔏️inputs/🧪️tests/🔏️receipt` gains an Nx replay oracle (throwaway git + Nx workspaces, fixture `replay`).

usage: python3 nx1b-apply.py [--dry-run | --write] [--root <repo root>] [--part all|now|rebuild]   (anchored, all-or-nothing,
idempotent refusal; `now` = everything but the rebuild chain's `--skip-nx-cache` drop, `rebuild` = only that drop, window 3)
"""
import argparse
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "nx1b-payload")
CACHING = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
REGISTRY = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry"
RECEIPT = ".🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/📇️registry/🔣️.json"

EDITS = [
    (f"{CACHING}/🔣️policy.json",
     '  "cachedExact": [\n    "generator-inputs",\n    "hub-live-catalog-check",\n',
     '  "cachedExact": [\n    "hub-live-catalog-check",\n'),
    (f"{CACHING}/📋️project.json",
     '''    "generator-inputs": {
      "executor": "nx:run-commands",
      "cache": true,
      "outputs": [
        "{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/📇️registry/🔣️.json"
      ],
      "options": {
        "command": "bun \\"./🔏️inputs/📜️script.ts\\" registry-catalog"
      },
      "inputs": [
        "{projectRoot}/🔏️inputs/**/*",
        "{projectRoot}/🔣️policy.json",
        "{projectRoot}/📋️project.json",
        "{workspaceRoot}/bun.lock",
        "{workspaceRoot}/**/Cargo.toml",
        "{workspaceRoot}/**/📋️project.json",
        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/**/*"
      ]
    },
''',
     '''    "generator-inputs": {
      "executor": "nx:run-commands",
      "cache": false,
      "outputs": [
        "{workspaceRoot}/.🧬semio/🦑️repo/⚡️cache/🔏️generator-inputs/📇️registry/🔣️.json"
      ],
      "options": {
        "command": "bun \\"./🔏️inputs/📜️script.ts\\" registry-catalog"
      }
    },
'''),
    (f"{CACHING}/🧫️fixtures/nx-contract/🔣️.json",
     '''    {
      "target": "generator-inputs",
      "cache": true,
      "continuous": false
    },
''',
     '''    {
      "target": "generator-inputs",
      "cache": true,
      "continuous": false,
      "authored": true
    },
'''),
    (f"{CACHING}/🧫️fixtures/nx-contract/🥒️.feature",
     '''  Scenario: Generator discovery fingerprint is cacheable
    Given generator-inputs or a required checkTarget freshness guard
    When Nx resolves the project
    Then generator-inputs stays cacheable with declared catalog inputs
''',
     '''  Scenario: Generator discovery fingerprint re-digests on every run
    Given generator-inputs, whose receipt digests bytes its own cache key cannot name
    When Nx resolves the project
    Then no policy classifies generator-inputs and its authored cache false keeps it uncached
    And every consumer keys on the receipt through dependentTasksOutputFiles
'''),
    (f"{REGISTRY}/📋️project.json",
     '''    "check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",
        "command": "bun ./📜️script.ts check",
        "forwardAllArgs": true
      }
    },
''',
     '''    "check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry",
        "command": "bun ./📜️script.ts check",
        "forwardAllArgs": true
      },
      "dependsOn": [
        "repo:generator-inputs"
      ],
      "inputs": [
        "default",
        "^default",
        {
          "dependentTasksOutputFiles": "''' + RECEIPT + '''"
        }
      ]
    },
'''),
]
EDITS.append((f"{CACHING}/🧪️tests/⚡️cache-contracts/🟦️.ts",
     "assert.ok(target.dependsOn.includes(fingerprint.target)); assert.equal(guard.cache, true);",
     "assert.ok(target.dependsOn.includes(fingerprint.target)); assert.equal(guard.cache, false, `${id} discovery receipt producer re-digests on every run: its receipt digests bytes its own cache key cannot name`);"))
EDITS += [(f"{CACHING}/🧪️tests/⚡️cache-contracts/🟦️.ts", f'for (const name of ["test", "lint", "build", "generate", "verify", "test-exhaustive"]) assert.equal({owner}.cache, true, name);', f'for (const name of ["test", "lint", "build", "verify", "test-exhaustive"]) assert.equal({owner}.cache, true, name);') for owner in ("authoredWorkspace.targets[name]", "workspace.targets[name]?")]
REBUILD = f"{REGISTRY}/🔁️rebuild/🔣️.json"
REBUILD_EDITS = [
    ('"@semio-tech/plugin-registry:generate", "--skip-nx-cache"]', '"@semio-tech/plugin-registry:generate"]'),
    ('"@semio-tech/plugin-registry:check", "--skip-nx-cache"]', '"@semio-tech/plugin-registry:check"]'),
    ('"@semio-tech/framework-os-dev:activate-s-react-dev", "--skip-nx-cache"]', '"@semio-tech/framework-os-dev:activate-s-react-dev"]'),
    ('"--variant", "s", "--skip-nx-cache"]', '"--variant", "s"]'),
]
REBUILD_EDIT_ROWS = [(REBUILD, old, new) for old, new in REBUILD_EDITS]
REPLACEMENTS = [
    (f"{CACHING}/🔏️inputs/🧫️fixtures/🔏️receipt/🔣️.json", "38fc25ce21a6ded7f99d9427fb04509c261925d2a558055e8fc6e6c6371956c7", "receipt-fixture.json"),
    (f"{CACHING}/🔏️inputs/🧪️tests/🔏️receipt/🟦️.ts", "5dd1a74ec2906b5b763fed519635726547fe70b50e49c9fcf59282db947d9548", "receipt-test.ts"),
]


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--write", action="store_true")
    parser.add_argument("--root", default="/Users/ueli/Documents/semio")
    parser.add_argument("--part", choices=["all", "now", "rebuild"], default="all")
    args = parser.parse_args()
    edits = (EDITS if args.part != "rebuild" else []) + (REBUILD_EDIT_ROWS if args.part != "now" else [])
    replacements = REPLACEMENTS if args.part != "rebuild" else []
    texts, problems = {}, []
    for rel, old, new in edits:
        path = os.path.join(args.root, rel)
        text = texts.get(rel) or open(path, encoding="utf-8").read()
        count = text.count(old)
        if count != 1:
            problems.append(f"{rel}: anchor found {count}x: {old.splitlines()[0][:100]!r}")
            continue
        texts[rel] = text.replace(old, new, 1)
    for rel, sha, payload in replacements:
        current = open(os.path.join(args.root, rel), "rb").read()
        if hashlib.sha256(current).hexdigest() != sha:
            problems.append(f"{rel}: content differs from the reviewed base (sha256 {hashlib.sha256(current).hexdigest()})")
            continue
        texts[rel] = open(os.path.join(PAYLOAD, payload), encoding="utf-8").read()
    for rel, text in texts.items():
        if rel.endswith(".json"):
            try:
                json.loads(text)
            except json.JSONDecodeError as error:
                problems.append(f"{rel}: result is not JSON: {error}")
    for rel in sorted(texts):
        print(f"edit  {rel}")
    print(f"nx1b-apply: part {args.part}: {len(texts)} files, {len(problems)} problems, root={args.root}")
    for problem in problems:
        print(f"PROBLEM {problem}")
    if problems:
        sys.exit(1)
    if not args.write:
        print("dry run: nothing written")
        return
    for rel, text in texts.items():
        path = os.path.join(args.root, rel)
        with open(path + ".nx1b-tmp", "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(path + ".nx1b-tmp", path)
    print("nx1b-apply: WRITTEN")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""🚚️ S5-STORE: lands a staged law set of the store (`🗑️generated/s5-store/stage-<set>/🏪️store/**`) into
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**` and registers it.

    python3 🧪️s5-store-land-laws.py <viewer-head|supersede-law> [--rust] [--check]

Without `--rust` only the language-agnostic half lands: the schema, the corpus, the TS twin, and its registration in the os
`test-store-oracles` script and nx inputs. `--rust` also lands the Rust law and mounts it in the store unit-test module (the
kernel test target must compile for it to be verified). `--check` prints what is pending and writes nothing. Idempotent:
a file already equal to its staged copy and a registration already present are skipped.
"""
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OS = ROOT / "🧰️framework/🛍️products/💻️os"
STORE = OS / "🔨️modules/🏪️store"
SETS = {"viewer-head": "stage-p1", "supersede-law": "stage-fw"}
SCRIPT = OS / "📦️packages/🟦️typescript/📜️script.ts"
PROJECT = OS / "📦️packages/🟦️typescript/📋️project.json"
UNIT = STORE / "🧪️tests/🔬️unit/🦀️.rs"


def main():
    arguments = [argument for argument in sys.argv[1:] if not argument.startswith("--")]
    if len(arguments) != 1 or arguments[0] not in SETS:
        raise SystemExit(__doc__)
    name = arguments[0]
    rust, check = "--rust" in sys.argv[1:], "--check" in sys.argv[1:]
    staged = TICKET / "🗑️generated/s5-store" / SETS[name] / "🏪️store"
    files = [f"🧬️schema/🔣️{name}/🔣️.json", f"🧫️fixtures/🧫️{name}/🔣️.json", f"🧪️tests/🧪️{name}/🟦️.ts"] + ([f"🧪️tests/🧪️{name}/🦀️.rs"] if rust else [])
    writes, pending = [], []
    for relative in files:
        source = (staged / relative).read_bytes()
        target = STORE / relative
        if not target.exists() or target.read_bytes() != source:
            writes.append((target, source))
            pending.append(f"file:{relative}")

    script = SCRIPT.read_text(encoding="utf-8")
    if f'"🧪️{name}"' not in script:
        anchor = '"].map((oracle) => join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests", oracle, "🟦️.ts"));'
        if script.count(anchor) != 1:
            raise SystemExit("the oracle list anchor is not unique in the os script: re-derive")
        script = script.replace(anchor, f'", "🧪️{name}{anchor}')
        writes.append((SCRIPT, script.encode("utf-8")))
        pending.append("register:script")
    project = PROJECT.read_text(encoding="utf-8")
    prefix = '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
    if f"🧪️tests/🧪️{name}/**/*" not in project:
        for kind, mark in (("🧪️tests/🧪️", "🧪️tests/🧪️"), ("🧫️fixtures/🧫️", "🧫️fixtures/🧫️"), ("🧬️schema/🔣️", "🧬️schema/🔣️")):
            lines = [line for line in project.split("\n") if line.startswith(prefix + mark)]
            if not lines:
                raise SystemExit(f"no `{mark}` input line in the os project: re-derive")
            last = lines[-1]
            if project.count(last + "\n") != 1:
                raise SystemExit(f"the last `{mark}` input line is not unique in the os project: re-derive")
            added = f'{prefix}{kind}{name}/**/*"'
            project = project.replace(last + "\n", (last if last.endswith(",") else last + ",") + "\n" + added + ("," if last.endswith(",") else "") + "\n")
        writes.append((PROJECT, project.encode("utf-8")))
        pending.append("register:project")
    if rust:
        unit = UNIT.read_text(encoding="utf-8")
        module = name.replace("-", "_") + "_tests"
        if f"mod {module};" not in unit:
            anchor = '#[cfg(test)]\n#[path = "../⚡️hot-path/🦀️.rs"]\nmod hot_path_tests;\n'
            if unit.count(anchor) != 1:
                raise SystemExit("the unit-test mount anchor is not unique: re-derive")
            unit = unit.replace(anchor, anchor + f'\n#[cfg(test)]\n#[path = "../🧪️{name}/🦀️.rs"]\nmod {module};\n')
            writes.append((UNIT, unit.encode("utf-8")))
            pending.append("mount:unit")
    print("pending: " + (", ".join(pending) if pending else "none"))
    if check:
        return
    for target, content in writes:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(content)
    if writes:
        print("applied")


main()

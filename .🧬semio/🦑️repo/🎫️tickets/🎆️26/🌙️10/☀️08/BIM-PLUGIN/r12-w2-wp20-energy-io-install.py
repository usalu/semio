#!/usr/bin/env python3
"""Installs the drafts of `w2-wp20-energy-io` from `🗑️generated/w2-wp20-energy-io/draft` into the BIM crate (new files only; never overwrites a file that exists).

Run it once the coordinator's `r13-blessed.flag` exists:  python r12-w2-wp20-energy-io-install.py [--force-tests]
Files are written next to the target as `*.tmp` and moved with `os.replace`.
"""
import os
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
D = T / "🗑️generated" / "w2-wp20-energy-io" / "draft"
S = T.parents[6] / "✏️s" / "🔌️plugins" / "🏙️bim" / "🗿️artifacts" / "🏢️model" / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any"
EXPORT = S / "🚪️io" / "📤️export"
IMPORT = S / "🚪️io" / "📥️import" / "🏗️ifc"

PLAN = [
    ("holders.rs", EXPORT / "🧱️holders" / "🦀️.rs"),
    ("tests/holders_tests.rs", EXPORT / "🧱️holders" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("gbxml.rs", EXPORT / "🌿️gbxml" / "🦀️.rs"),
    ("codec.rs", EXPORT / "🌿️gbxml" / "🧱️codec" / "🦀️.rs"),
    ("tests/codec_tests.rs", EXPORT / "🌿️gbxml" / "🧱️codec" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("geometry.rs", EXPORT / "🌿️gbxml" / "📐️geometry" / "🦀️.rs"),
    ("tests/geometry_tests.rs", EXPORT / "🌿️gbxml" / "📐️geometry" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("pairing.rs", EXPORT / "🌿️gbxml" / "🔗️pairing" / "🦀️.rs"),
    ("tests/pairing_tests.rs", EXPORT / "🌿️gbxml" / "🔗️pairing" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("tables.rs", EXPORT / "🌿️gbxml" / "📊️tables" / "🦀️.rs"),
    ("tests/tables_tests.rs", EXPORT / "🌿️gbxml" / "📊️tables" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("tests/gbxml_tests.rs", EXPORT / "🌿️gbxml" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("energy_ifc_export.rs", EXPORT / "🏗️ifc" / "🔥️energy" / "🦀️.rs"),
    ("tests/energy_ifc_export_tests.rs", EXPORT / "🏗️ifc" / "🔥️energy" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("energy_ifc_import.rs", IMPORT / "🔥️energy" / "🦀️.rs"),
    ("tests/energy_ifc_import_tests.rs", IMPORT / "🔥️energy" / "🧪️tests" / "🔬️unit" / "🦀️.rs"),
    ("cases/gbxml.feature", S / "🧪️tests" / "🌿️export-bim-1-gbxml" / "🥒️.feature"),
    ("cases/gbxml.rs", S / "🧪️tests" / "🌿️export-bim-1-gbxml" / "🦀️.rs"),
    ("gbxml_oracle.py", S / "🧪️tests" / "🌿️export-bim-1-gbxml" / "🐍️.py"),
    ("cases/ifc-energy.feature", S / "🧪️tests" / "🔥️export-bim-1-ifc-energy" / "🥒️.feature"),
    ("cases/ifc-energy.rs", S / "🧪️tests" / "🔥️export-bim-1-ifc-energy" / "🦀️.rs"),
    ("ifc_energy_oracle.py", S / "🧪️tests" / "🔥️export-bim-1-ifc-energy" / "🐍️.py"),
]


def main():
    written = 0
    for source, target in PLAN:
        path = D / source
        if not path.exists():
            print("missing draft", source)
            continue
        if target.exists():
            print("exists, kept", target.relative_to(S))
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_name(target.name + ".tmp")
        temporary.write_bytes(path.read_bytes())
        os.replace(temporary, target)
        written += 1
        print("installed", target.relative_to(S))
    print("%d file(s) installed" % written)


if __name__ == "__main__":
    sys.exit(main())

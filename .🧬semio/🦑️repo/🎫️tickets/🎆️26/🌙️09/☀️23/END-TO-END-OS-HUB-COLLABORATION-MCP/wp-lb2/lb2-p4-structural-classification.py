#!/usr/bin/env python3
"""🧵️ LB2 prepared patch p4 — the stdio structural table verbs are classified retained routes.

Measured 14:25 (`cargo test -p semio-s-plugin-stdio --test shipped_fleet`, `generated/i5-test-shipped-2.txt`): stdio `plugin()`
panics `app-definition.interactive-job-classification: unclassified interactive command 'framework.window.table:add-column'`
(+ add-row, remove-column, remove-row, set-header). `structural_table_window_kind()` appends five `bounded_catalog` rows AFTER
the SDK kit scaffold (`window_kind_definition`) stamped its own rows `Migrated`, so the appended rows stay `Unclassified` and
every app declaring the structural table (csv, tsv — both shipped — and wav) fails assembly. They ARE retained routes: csv,
tsv and wav register bounded tool-job factories for exactly these ids. The fix stamps them `Migrated` where they are declared.

Usage: lb2-p4-structural-classification.py [--dry-run | --write] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs"
OLD_USE = """    use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, LocalizedLabel};
    let revision = || ActionArgDef::text("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")).required();
    let mut definition = revision_addressed_table_window_kind();
    definition.actions.extend([
"""
NEW_USE = """    use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, InteractiveJobClassification, LocalizedLabel};
    let revision = || ActionArgDef::text("revision", LocalizedLabel::native("Document revision", "Dokumentrevision")).required();
    let mut definition = revision_addressed_table_window_kind();
    let structural = [
"""
OLD_TAIL = """            .with_args(vec![ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(), revision(), ActionArgDef::text("value", LocalizedLabel::native("Header", "Spaltenkopf")).min_length(0).required()])
            .in_palette(false),
    ]);
    definition
}
"""
NEW_TAIL = """            .with_args(vec![ActionArgDef::number("column", LocalizedLabel::native("Column", "Spalte")).required(), revision(), ActionArgDef::text("value", LocalizedLabel::native("Header", "Spaltenkopf")).min_length(0).required()])
            .in_palette(false),
    ];
    definition.actions.extend(structural.map(|mut action| {
        action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        action
    }));
    definition
}
"""
DOC_OLD = "/// 🧱️ Declares direct structural table edits beside revision-addressed cell editing.\n"
DOC_NEW = "/// 🧱️ Declares direct structural table edits beside revision-addressed cell editing — retained routes, classified\n/// `Migrated` here like the kit scaffold classifies its own rows, so every app declaring the table assembles.\n"
path = ROOT / CONTRACT
text = path.read_text(encoding="utf-8")
problems = []
if "definition.actions.extend(structural.map(|mut action| {" in text:
    print("already applied")
    sys.exit(0)
for label, old in [("structural docstring", DOC_OLD), ("structural use/extend head", OLD_USE), ("structural extend tail", OLD_TAIL)]:
    if text.count(old) != 1:
        problems.append(f"{CONTRACT}: expected 1x {label}, found {text.count(old)}")
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={0 if problems else 1} problems={len(problems)} write={WRITE}")
if WRITE and not problems:
    path.write_text(text.replace(DOC_OLD, DOC_NEW, 1).replace(OLD_USE, NEW_USE, 1).replace(OLD_TAIL, NEW_TAIL, 1), encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)

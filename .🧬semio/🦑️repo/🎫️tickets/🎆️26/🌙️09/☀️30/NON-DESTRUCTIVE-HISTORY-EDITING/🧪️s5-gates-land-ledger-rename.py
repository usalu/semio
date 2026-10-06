"""📒️ Renames the guest fault code `ledger-not-replayable` to `history.ledger-not-replayable` and labels it (`📓️s4-gates-report.md` § S5.8).

The single-segment code fitted no notice table grammar. It is raised at ONE site (`replay_envelopes_fault`, plugin SDK) and asserted
by ONE law; nothing matches it as a string. The hub check-in refusal `DocumentCheckInRefusalV1::LedgerNotReplayable` (wire value
`ledger-not-replayable` in the directory schema, its TypeScript twin, the hub bootstrap span and the React check-in label) is a
different vocabulary and is NOT touched. The new code gets its en/de row at the end of the kernel history notice table (Rust,
TypeScript twin, fixture — the table every shell resolves a guest fault through). Anchors are counted; nothing is written unless
every anchor holds.

    python3 🧪️s5-gates-land-ledger-rename.py --root <repository root> [--apply | --restore]

`--apply` first copies the five files to `🗑️generated/s5-gates/exec/pre-ledger/` beside this script; `--restore` copies them back.
"""
import json
import os
import shutil
import sys

KERNEL = "🧰️framework/🔨️modules/🎠️kernel"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUST = f"{KERNEL}/🦀️.rs"
TWIN = f"{KERNEL}/🟦️.ts"
FIXTURE = f"{KERNEL}/🧫️fixtures/🧫️history-notices/🔣️.json"
PLUGIN = f"{SDK}/🦀️.rs"
LAW = f"{SDK}/🧪️tests/🧪️time-travel/🦀️.rs"
PATHS = (RUST, TWIN, FIXTURE, PLUGIN, LAW)
BACKUP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "s5-gates", "exec", "pre-ledger")
OLD, NEW = "ledger-not-replayable", "history.ledger-not-replayable"
EN = "This document's history does not replay without errors yet — fix or withdraw the mutations that end with errors, then check in again."
DE = "Der Verlauf dieses Dokuments lässt sich noch nicht fehlerfrei neu anwenden — die Mutationen mit Fehlern beheben oder zurückziehen, dann erneut einchecken."


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[land-ledger-rename] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def swap(texts, path, old, new, expected):
    """✍️ Replaces the `expected` occurrences of `old` in the planned text of `path`."""
    found = texts[path].count(old)
    if found != expected:
        refuse(f"{path} holds {found} occurrence(s) of {old[:80]!r}, expected {expected}")
    texts[path] = texts[path].replace(old, new)


def main():
    arguments = sys.argv[1:]
    root = (arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else "").rstrip("/")
    if root == "" or any(not os.path.isfile(os.path.join(root, path)) for path in PATHS):
        refuse("--root must name a directory that holds the kernel notice tables and the plugin SDK")
    if "--restore" in arguments:
        if any(not os.path.isfile(os.path.join(BACKUP, path)) for path in PATHS):
            refuse(f"no complete pre-wave copy under {BACKUP}")
        for path in PATHS:
            shutil.copyfile(os.path.join(BACKUP, path), os.path.join(root, path))
        print(f"[land-ledger-rename] {len(PATHS)} file(s) restored under {root}")
        return
    texts = {path: open(os.path.join(root, path), encoding="utf-8").read() for path in PATHS}
    if any(NEW in text for text in texts.values()):
        refuse(f"{NEW} already stands in one of the files")
    rows = json.loads(texts[FIXTURE])["notices"]
    last, count = rows[-1]["code"], len(rows)
    length = f"pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); {count}] = ["
    swap(texts, RUST, length, length.replace(f"; {count}]", f"; {count + 1}]"), 1)
    rust_last = next((line for line in texts[RUST].split("\n") if line.startswith(f'    ("{last}", ')), None)
    twin_last = next((line for line in texts[TWIN].split("\n") if line.startswith(f'  {{ code: "{last}", ')), None)
    fixture_last = next((line for line in texts[FIXTURE].split("\n") if line.startswith(f'    {{ "code": "{last}", ')), None)
    if rust_last is None or twin_last is None or fixture_last is None or fixture_last.endswith(","):
        refuse(f"the row {last} is not the last row of every history notice table")
    swap(texts, RUST, rust_last + "\n];", rust_last + f'\n    ("{NEW}", "{EN}", "{DE}"),\n];', 1)
    swap(texts, TWIN, twin_last + "\n] as const;", twin_last + f'\n  {{ code: "{NEW}", en: "{EN}", de: "{DE}" }},\n] as const;', 1)
    swap(texts, FIXTURE, fixture_last + "\n  ]", fixture_last + f',\n    {{ "code": "{NEW}", "en": "{EN}", "de": "{DE}" }}\n  ]', 1)
    swap(texts, PLUGIN, f'FaultCode::new("{OLD}")', f'FaultCode::new("{NEW}")', 1)
    swap(texts, PLUGIN, f"typed `{OLD}` naming its first blocking messages", f"typed `{NEW}` naming its first blocking messages", 1)
    swap(texts, LAW, f'FaultCode::new("{OLD}")', f'FaultCode::new("{NEW}")', 2)
    swap(texts, LAW, f"typed `{OLD}` naming only its blocking messages", f"typed `{NEW}` naming only its blocking messages", 1)
    if [row["code"] for row in json.loads(texts[FIXTURE])["notices"]][-1] != NEW:
        refuse("the fixture did not take the row at its end")
    if "--apply" in arguments:
        for path in PATHS:
            os.makedirs(os.path.dirname(os.path.join(BACKUP, path)), exist_ok=True)
            shutil.copy2(os.path.join(root, path), os.path.join(BACKUP, path))
        for path, text in texts.items():
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    print(f"[land-ledger-rename] 1 row × 3 history tables ({count} → {count + 1}), 1 fault site, 1 law in {len(PATHS)} file(s) under {root}: {'WRITTEN' if '--apply' in arguments else 'planned (check only; pass --apply)'}")


main()

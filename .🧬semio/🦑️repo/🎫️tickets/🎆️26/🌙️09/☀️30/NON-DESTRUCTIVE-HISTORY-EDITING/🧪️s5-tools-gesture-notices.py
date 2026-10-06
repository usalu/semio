"""📢️ Framework notices of the three refusals a gesture dispatch raises (`toolTransaction.closed`, `toolTransaction.unclosed`,
`toolTransaction.slot-poisoned` — renamed from `toolGesture.…` by `🧪️s5-tools-gesture-notice-family.py`): one row each, English and German, in the kernel table, its TS twin and their shared fixture
(ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, report `📓️s4-tools-a-report.md` § S5.14). Counted anchors, idempotent.

Usage (cwd: repo root): python3 🧪️s5-tools-gesture-notices.py            dry run
                        python3 🧪️s5-tools-gesture-notices.py --apply    hold `landing`, then `serve` (TS + fixture)
                        python3 🧪️s5-tools-gesture-notices.py --restore  take the three rows out again
"""

import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
KERNEL = "🧰️framework/🔨️modules/🎠️kernel"
ROWS = [
    ("toolTransaction.closed", "The tool's recording had already ended — this step was not recorded.", "Die Aufzeichnung des Werkzeugs war bereits beendet — dieser Schritt wurde nicht aufgezeichnet."),
    ("toolTransaction.unclosed", "The tool stopped without finishing its recording — this step was not recorded.", "Das Werkzeug hat angehalten, ohne seine Aufzeichnung abzuschließen — dieser Schritt wurde nicht aufgezeichnet."),
    ("toolTransaction.slot-poisoned", "The tool could not continue this gesture — release and try again.", "Das Werkzeug konnte diese Geste nicht fortsetzen — loslassen und erneut versuchen."),
]
AFTER = ("app.command.tool-mismatch", "This action does not belong to the active tool.", "Diese Aktion gehört nicht zum aktiven Werkzeug.")
SHAPES = {
    f"{KERNEL}/🦀️.rs": lambda code, en, de: f'    ("{code}", "{en}", "{de}"),\n',
    f"{KERNEL}/🟦️.ts": lambda code, en, de: f'  {{ code: "{code}", en: "{en}", de: "{de}" }},\n',
    f"{KERNEL}/🧫️fixtures/🧫️framework-notices/🔣️.json": lambda code, en, de: f'    {{ "code": "{code}", "en": "{en}", "de": "{de}" }},\n',
}
SIZE = re.compile(r"(pub const FRAMEWORK_FAULT_NOTICE_LABELS: \[\(&str, &str, &str\); )(\d+)(\] = \[)")


def main():
    restore, staged, moved = "--restore" in sys.argv, [], 0
    for relative, row in SHAPES.items():
        path = REPO / relative
        text = path.read_text(encoding="utf-8")
        anchor, block = row(*AFTER), "".join(row(*entry) for entry in ROWS)
        present = text.count(block)
        if text.count(anchor) != 1 or present > 1:
            raise SystemExit(f"{relative}: anchor row moved")
        if restore and present == 1:
            text, delta = text.replace(block, ""), -len(ROWS)
        elif not restore and present == 0:
            if any(text.count(f'"{code}"') for code, _, _ in ROWS):
                raise SystemExit(f"{relative}: a row for one of the codes exists elsewhere")
            text, delta = text.replace(anchor, anchor + block), len(ROWS)
        else:
            delta = 0
        if delta and relative.endswith("🦀️.rs"):
            if len(SIZE.findall(text)) != 1:
                raise SystemExit("the table declaration moved")
            text = SIZE.sub(lambda match: f"{match.group(1)}{int(match.group(2)) + delta}{match.group(3)}", text)
        moved += abs(delta)
        staged.append((path, text))
    if "--apply" in sys.argv or restore:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'restored' if restore else 'applied' if '--apply' in sys.argv else 'would apply'} {moved} row(s) in {len(staged)} files")


if __name__ == "__main__":
    main()

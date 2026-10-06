#!/usr/bin/env python3
"""📢 S5-NESTED: the owned-child emission's retirement refusal gets a framework code and its en/de notice.

Usage: python3 🧪️s5-nested-notice-child-emission.py check|land|restore
The SDK fault `framework.child-emission.retirement-refusal` had no notice row and its `framework.` namespace is outside the notice
schema's closed code pattern. It becomes `interactive-job.child-emission-retirement-refused` — the namespace of every sibling bounded
child-ownership refusal — with one row in the kernel table (86 → 87), the fixture of record and the TS twin, in the same position.
Locks: `landing` (2 Rust + 1 Rust test) then `serve` (fixture JSON + TS twin). Counted anchors, fails closed before the first write.
"""
import pathlib
import shutil
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TICKET = pathlib.Path(__file__).resolve().parent
PRE = TICKET / "🗑️generated/s5-nested/pre-notice"
KERNEL = "🧰️framework/🔨️modules/🎠️kernel"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
OLD_CODE = "framework.child-emission.retirement-refusal"
NEW_CODE = "interactive-job.child-emission-retirement-refused"
EN = "A change to a part could not be released cleanly — try again."
DE = "Eine Änderung an einem Teil konnte nicht sauber freigegeben werden — erneut versuchen."
PREVIOUS_EN = "A command of this app lacks its registered editing step."
PREVIOUS_DE = "Einem Befehl dieser App fehlt sein registrierter Bearbeitungsschritt."

FILES = [
    (
        f"{KERNEL}/🦀️.rs",
        [
            ("pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); 86] = [", "pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); 87] = [", 1),
            (
                f'    ("interactive-job.catalog-incomplete", "{PREVIOUS_EN}", "{PREVIOUS_DE}"),\n',
                f'    ("interactive-job.catalog-incomplete", "{PREVIOUS_EN}", "{PREVIOUS_DE}"),\n    ("{NEW_CODE}", "{EN}", "{DE}"),\n',
                1,
            ),
        ],
    ),
    (
        f"{KERNEL}/🧫️fixtures/🧫️framework-notices/🔣️.json",
        [
            (
                f'    {{ "code": "interactive-job.catalog-incomplete", "en": "{PREVIOUS_EN}", "de": "{PREVIOUS_DE}" }},\n',
                f'    {{ "code": "interactive-job.catalog-incomplete", "en": "{PREVIOUS_EN}", "de": "{PREVIOUS_DE}" }},\n    {{ "code": "{NEW_CODE}", "en": "{EN}", "de": "{DE}" }},\n',
                1,
            ),
        ],
    ),
    (
        f"{KERNEL}/🟦️.ts",
        [
            (
                f'  {{ code: "interactive-job.catalog-incomplete", en: "{PREVIOUS_EN}", de: "{PREVIOUS_DE}" }},\n',
                f'  {{ code: "interactive-job.catalog-incomplete", en: "{PREVIOUS_EN}", de: "{PREVIOUS_DE}" }},\n  {{ code: "{NEW_CODE}", en: "{EN}", de: "{DE}" }},\n',
                1,
            ),
        ],
    ),
    (f"{PLUGIN}/🧩️composition/📨️emission/📦️preparation/🦀️.rs", [(f'"{OLD_CODE}"', f'"{NEW_CODE}"', 1)]),
    (f"{PLUGIN}/🧪️tests/🧩️composition/📨️emission/🦀️.rs", [(f'"{OLD_CODE}"', f'"{NEW_CODE}"', 1)]),
]


def pre_image(relative: str) -> pathlib.Path:
    return PRE / relative.replace("/", "__")


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "restore":
        for relative, _ in FILES:
            if pre_image(relative).exists():
                shutil.copyfile(pre_image(relative), REPO / relative)
                print(f"restored {relative}")
        return
    results = []
    for relative, replacements in FILES:
        text = (REPO / relative).read_text(encoding="utf-8")
        if NEW_CODE in text:
            print(f"already landed: {relative}")
            continue
        result = text
        for old, new, count in replacements:
            if result.count(old) != count:
                raise SystemExit(f"{relative}: anchor drift: expected {count}, found {result.count(old)}: {old[:110]!r}")
            result = result.replace(old, new)
        results.append((relative, text, result))
        print(f"{relative}: {len(replacements)} replacement(s)")
    if mode == "land":
        PRE.mkdir(parents=True, exist_ok=True)
        for relative, text, result in results:
            pre_image(relative).write_text(text, encoding="utf-8")
            (REPO / relative).write_text(result, encoding="utf-8")
            print(f"landed {relative}")


if __name__ == "__main__":
    main()

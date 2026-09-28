#!/usr/bin/env python3
"""🧪️ U6 set A — stdio lib-test compile drift (test-only, rule 22): bcf, svg, semio, txt, wav.

Each hunk moves a `🧪️tests/` file onto the CURRENT production API (never production code):
* bcf  `render(document, locale)` gained its locale → the default-document render runs in en + de and must assemble.
* svg  `serde_json` is no dependency of the crate any more → the neutral XML boundary fixture is read with the crate's own
       `pack::parse_json` (the framework's serde_json replacement; same `Index`/`PartialEq<&str>`/`Display` surface).
* semio `Fault` deliberately has no `Display` → `Fault::describe()` (the documented one-line rendering).
* txt  `Mutation::apply` is gone → `MutationDiff::apply(Mutation::diff(..).diff(), ..)` (the sibling mp4/wav idiom).
* wav  `set_snapshot` is no longer imported by the editor → the test imports the mutation leaf itself;
       `UiMap` iterates owned `(UiText, UiValue)` pairs → owned argument reads; `LocalizedLabel` is a locale×terminology
       matrix → `resolve(Terminology::Native, locale)`; tree-window coordinates are `u32`.

Usage: u6-test-drift.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path(__file__).resolve().parent / "w3-backup" / "test-drift"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
WAV = ART + "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/"

SETS = {
    ART + "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [
        (
            "    let document = BcfSnapshot::default();\n    let _node = render(&document);\n}\n",
            "    let document = BcfSnapshot::default();\n    for locale in [semio_framework_plugin::Locale::En, semio_framework_plugin::Locale::De] {\n        render(&document, locale).expect(\"default document table renders\");\n    }\n}\n",
            1,
        ),
    ],
    ART + "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs": [
        ("    let fixture: serde_json::Value = serde_json::from_str(include_str!(", "    let fixture = pack::parse_json(include_str!(", 1),
    ],
    ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        ('.expect_err("ambiguous target").to_string().contains("ambiguous")', '.expect_err("ambiguous target").describe().contains("ambiguous")', 1),
    ],
    ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        ('.expect_err("duplicate mesh id").to_string().contains("ambiguous")', '.expect_err("duplicate mesh id").describe().contains("ambiguous")', 1),
        ('.expect_err("duplicate primitive id").to_string().contains("ambiguous")', '.expect_err("duplicate primitive id").describe().contains("ambiguous")', 1),
    ],
    ART + "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        (
            "        next = <TxtMutation as protocol::Mutation<TxtSnapshot>>::apply(mutation, &next).expect(\"native mutation applies\");\n",
            "        next = protocol::MutationDiff::apply(<TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next).diff(), &next).expect(\"native mutation applies\");\n",
            1,
        ),
    ],
    WAV + "🧪️tests/🔬️unit/🦀️.rs": [
        ("use super::*;\n\n#[semio_framework_async_macros::async_test]\nasync fn create_editor_builds", "use super::*;\nuse crate::standards::riff_pcm::subsets::any::schema::mutations::set_snapshot;\n\n#[semio_framework_async_macros::async_test]\nasync fn create_editor_builds", 1),
    ],
    WAV + "🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [
        ("    if node.key.as_str() == key { return Some(node); }\n", "    if node.key.as_str() == key {\n        return Some(node);\n    }\n", 1),
        (".and_then(|value| match value { UiValue::Number(value) => Some(*value), _ => None })\n", ".and_then(|value| match value {\n        UiValue::Number(value) => Some(value),\n        _ => None,\n    })\n", 1),
        ("fn text_arg<'a>(binding: &'a ActionBinding, key: &str) -> Option<&'a str> {", "fn text_arg(binding: &ActionBinding, key: &str) -> Option<String> {", 1),
        (".and_then(|value| match value { UiValue::Text(value) => Some(value.as_str()), _ => None })\n", ".and_then(|value| match value {\n        UiValue::Text(value) => Some(value.as_str().to_owned()),\n        _ => None,\n    })\n", 1),
        ('    assert_eq!(append.label.en, "Append frame");\n    assert_eq!(append.label.de, "Frame anhängen");\n', '    assert_eq!(append.label.resolve(semio_framework_plugin::Terminology::Native, Locale::En), "Append frame");\n    assert_eq!(append.label.resolve(semio_framework_plugin::Terminology::Native, Locale::De), "Frame anhängen");\n', 1),
        ('assert_eq!(text_arg(binding, "revision"), Some(revision), "{action_id} revision binding");', 'assert_eq!(text_arg(binding, "revision").as_deref(), Some(revision), "{action_id} revision binding");', 1),
        ("    let channels = 64usize;\n", "    let channels = 64u32;\n", 1),
        (
            "        fmt: crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt {\n            channels: channels as u16,\n            sample_rate: 1_000,\n            byte_rate: (channels * 1_000) as u32,\n            block_align: channels as u16,\n            bits_per_sample: 8,\n            ..Default::default()\n        },\n",
            "        fmt: crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt { channels: channels as u16, sample_rate: 1_000, byte_rate: channels * 1_000, block_align: channels as u16, bits_per_sample: 8, ..Default::default() },\n",
            1,
        ),
        ('assert_eq!(text_arg(sample, "revision"), Some(revision));', 'assert_eq!(text_arg(sample, "revision").as_deref(), Some(revision));', 1),
        ('assert_eq!(text_arg(insert_channel, "revision"), Some(revision));', 'assert_eq!(text_arg(insert_channel, "revision").as_deref(), Some(revision));', 1),
        ('.expect("sample-rate control"), "revision"), Some(revision));', '.expect("sample-rate control"), "revision").as_deref(), Some(revision));', 1),
        ('.expect("append-channel control"), "revision"), Some(revision));', '.expect("append-channel control"), "revision").as_deref(), Some(revision));', 1),
    ],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if not backup.exists():
                print(f"SKIP no backup: {rel}")
                continue
            (ROOT / rel).write_bytes(backup.read_bytes())
            backup.unlink()
            print(f"REVERTED {rel}")
        return 0
    problems = 0
    planned = []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        next_text = text
        for old, new, count in hunks:
            found = next_text.count(old)
            if found != count:
                state = "already applied" if next_text.count(new) == count else f"anchor count {found} != {count}"
                print(f"PROBLEM {rel}: {state}: {old[:70]!r}")
                problems += 1
                continue
            next_text = next_text.replace(old, new)
        planned.append((rel, path, next_text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, next_text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(next_text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())

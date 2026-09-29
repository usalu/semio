"""🧯️ S20 faults overlay, session-15 rebase fix-ups: the raise sites and declarations the post-round-3a live tree brought in
(`verify faults` on the rebased overlay, census `.🧬semio/🌐hub/s14-s20-sets/rebase/census-1.txt`) — new flag-value refusals
(cad, puzzle 2d/3d/5d), trinity jack's query-size refusal + declarations of codes jack's query-in-document rewrite retired,
the SDK's presence read, P9's no-effect verb and example codes (framework catalog texts live in `wp-fh1/catalog_texts.py`).
Idempotent: an edit whose result is present is skipped; its anchor must match exactly once otherwise.
Usage: python3 s20-rebase-fixes.py [--dry-run]"""
from __future__ import annotations

import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
PLUGINS = "✏️s/🔌️plugins"
CAD = f"{PLUGINS}/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
JACK = f"{PLUGINS}/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"


def puzzle(dimension: str, emoji: str, anchor_indent: str) -> list[tuple[str, str, str]]:
    path = f"{PLUGINS}/🧩️puzzle/🗿️artifacts/{emoji}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
    code = f"puzzle{dimension}.action.flag-value-required"
    label = {"2d": "2D", "3d": "3D", "5d": "5D"}[dimension]
    unknown = f'.fault("puzzle{dimension}.action-unknown", LocalizedLabel::native("The action {{action}} is not available in the {label} puzzle editor.", "Die Aktion {{action}} ist im {label}-Puzzleeditor nicht verfügbar."))\n'
    return [
        (path,
         f'Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("{code}"), format!("action \'{{action}}\' requires the boolean \'{{flag}}\' it sets"))',
         f'app_fault("{code}").with_parameter("action", action).with_parameter("flag", flag)'),
        (path, anchor_indent + unknown,
         anchor_indent + unknown + anchor_indent + f'.fault("{code}", LocalizedLabel::native("The action {{action}} needs {{flag}} set to on or off; choose it and try again.", "Die Aktion {{action}} benötigt für {{flag}} den Wert ein oder aus; wählen Sie ihn und versuchen Sie es erneut."))\n'),
    ]


RETIRED_JACK = ["jack.editor-window-required", "jack.format.editor-window-config-missing", "jack.format.editor-window-required",
                "jack.format.window-context-required", "jack.text-edit.editor-window-required", "jack.query.editor-window-config-missing"]


def jack_declarations(text: str) -> str:
    lines = text.split("\n")
    kept = [line for line in lines if not any(line.lstrip().startswith(f'.fault("{code}",') for code in RETIRED_JACK)]
    text = "\n".join(kept)
    anchor = next(line for line in kept if line.lstrip().startswith('.fault("jack.query.too-large",'))
    indent = anchor[: len(anchor) - len(anchor.lstrip())]
    added = indent + '.fault("jack.query-too-large", LocalizedLabel::native("A query holds at most {maximum} bytes; shorten it.", "Eine Abfrage enthält höchstens {maximum} Bytes; kürzen Sie sie."))'
    return text if added in text else text.replace(anchor, anchor + "\n" + added, 1)


EDITS: list[tuple[str, str, str]] = [
    (CAD,
     'Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("cad.action.flag-value-required"), format!("action \'{action}\' requires the boolean \'{flag}\' it sets"))',
     'app_fault("cad.action.flag-value-required").with_parameter("action", action).with_parameter("flag", flag)'),
    (CAD,
     '            .fault("cad.action-unknown", LocalizedLabel::native("The action {action} is not available in CAD.", "Die Aktion {action} ist in CAD nicht verfügbar."))\n',
     '            .fault("cad.action-unknown", LocalizedLabel::native("The action {action} is not available in CAD.", "Die Aktion {action} ist in CAD nicht verfügbar."))\n'
     '            .fault("cad.action.flag-value-required", LocalizedLabel::native("The action {action} needs {flag} set to on or off; choose it and try again.", "Die Aktion {action} benötigt für {flag} den Wert ein oder aus; wählen Sie ihn und versuchen Sie es erneut."))\n'),
    *puzzle("2d", "◻️2d", "            "),
    *puzzle("3d", "🧊️3d", "            "),
    *puzzle("5d", "🖐️5d", "    "),
    (SDK,
     "let presence_local = self.presence_store.local_read().map_err(Fault::from)?;",
     'let presence_local = self.presence_store.local_read().map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.presence.local-read"), reason))?;'),
    (SDK,
     'let mut fault = Fault::new(FaultOrigin::App, FaultCode::new("app.command.no-effect"), format!("action \'{verb}\' changes nothing here"));',
     'let mut fault = Fault::new(FaultOrigin::App, FaultCode::new("app.command.no-effect"), format!("action \'{verb}\' changes nothing here")).with_parameter("action", verb);'),
    (SDK,
     'None => Err(Fault::new(FaultOrigin::App, FaultCode::new("app.example.unknown"), format!("no registered example \'{example_id}\'"))),',
     'None => Err(Fault::new(FaultOrigin::App, FaultCode::new("app.example.unknown"), format!("no registered example \'{example_id}\'")).with_parameter("example", example_id)),'),
]


def main(dry_run: bool) -> None:
    for rel, old, new in EDITS:
        path = OVERLAY / rel
        text = path.read_text()
        if new in text and (old in new or old not in text):
            print("present ", rel.split("/")[-8:][0], new[:70])
            continue
        assert text.count(old) == 1, (rel, old[:80], text.count(old))
        print("apply   ", rel.split("/")[-8:][0], new[:70])
        if not dry_run:
            path.write_text(text.replace(old, new))
    text = (OVERLAY / JACK).read_text()
    result = jack_declarations(text)
    print("jack    ", "unchanged" if result == text else "declarations updated")
    if not dry_run and result != text:
        (OVERLAY / JACK).write_text(result)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])

#!/usr/bin/env python3
from __future__ import annotations
"""🔍️ CA1 source lexicon scan — asks the plugin SOURCE the two questions `semio-os-mcp audit` asks a
committed descriptor, so a re-describe cannot surprise this slice with findings that were always in
the source but invisible while the descriptor was stale.

It re-implements the gateway's own word-run matcher (`🌉️mcp/🗂️catalog/🦀️.rs`, `verb_id_words` +
`matching_lexicon_word`) over the ids a plugin's rust declares, and reports every id that matches a
lexicon and carries no `action_destructive` / `action_audience` / `.destructive()` / `.input_event()`
/ `.chrome()` declaration anywhere in the same plugin.

It is a PREDICTOR, not the gate: it cannot see `ActionKind` behind a helper, nor an id assembled from
a const, so it over- and under-reports. The gate is `bun ./📜️script.ts capability-audit-check`.

Usage: 🐍️ca1-source-lexicon-scan.py <plugin-dir-name> ...
"""
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[7]
PLUGINS = REPO / "✏️s/🔌️plugins"

GESTURE_ROUTE_WORDS = [
    "pointerdown", "pointerup", "pointermove", "pointercancel", "pointerenter", "pointerleave",
    "mousedown", "mouseup", "mousemove", "doubleclick", "dblclick", "dragstart", "dragmove",
    "dragend", "dragover", "dragenter", "dragleave", "drop", "wheel", "keydown", "keyup",
    "keypress", "escape", "hover", "touchstart", "touchmove", "touchend", "gesture",
    "engagementinput", "engagementsubmit", "engagementcancel", "engagementabort",
    "engagementcommit", "commitdraft", "canceldraft", "updatedraft", "applyevents",
]
DESTRUCTIVE_VERB_WORDS = [
    "delete", "remove", "clear", "discard", "purge", "wipe", "erase", "truncate",
    "setactiveexample", "setfixturejson", "setspecjson", "setsnapshot", "loaddocument",
    "setdocument", "replacedocument",
]

DECLARATION = re.compile(
    r'\.mutation\("([^"]+)"'
    r'|ActionDefinition::new\("([^"]+)"'
    r'|ActionDefinition::new_catalog\("([^"]+)"'
    r'|ActionDefinition::bounded_catalog\("([^"]+)"'
    r'|\.view_action\("([^"]+)"'
    r'|\.shell_action\("([^"]+)"'
)


def verb_id_words(identifier: str) -> list[str]:
    words: list[str] = []
    current = ""
    for character in identifier:
        if character in ".:_-":
            if current:
                words.append(current)
                current = ""
            continue
        if character.isupper() and current:
            words.append(current)
            current = ""
        current += character.lower()
    if current:
        words.append(current)
    return words


def matching_lexicon_word(identifier: str, lexicon: list[str]) -> str | None:
    words = verb_id_words(identifier)
    for start in range(len(words)):
        run = ""
        for word in words[start:]:
            run += word
            if run in lexicon:
                return run
    return None


def main() -> int:
    for plugin in sys.argv[1:]:
        root = PLUGINS / plugin
        sources = [path for path in root.rglob("🦀️.rs") if "/target" not in str(path)]
        declared: dict[str, list[str]] = {}
        marked: set[str] = set()
        for path in sources:
            if "🧪️tests" in str(path):
                continue
            for number, line in enumerate(path.read_text(encoding="utf8", errors="replace").splitlines(), start=1):
                for match in DECLARATION.finditer(line):
                    identifier = next(group for group in match.groups() if group is not None)
                    kind = "Mutation" if ".mutation(" in line else ("View" if "ActionKind::View" in line else ("Shell" if "ActionKind::Shell" in line else ("Mutation" if "ActionKind::Mutation" in line else "?")))
                    declared.setdefault(identifier, []).append(f"{path.relative_to(REPO)}:{number} kind={kind}")
                    if ".input_event()" in line or ".chrome()" in line or ".destructive()" in line:
                        marked.add(identifier)
                for marker in re.finditer(r'action_(?:destructive|audience)\("([^"]+)"', line):
                    marked.add(marker.group(1))
        open_rows = []
        for identifier, sites in sorted(declared.items()):
            if identifier in marked:
                continue
            gesture = matching_lexicon_word(identifier, GESTURE_ROUTE_WORDS)
            destructive = matching_lexicon_word(identifier, DESTRUCTIVE_VERB_WORDS)
            if not gesture and not destructive:
                continue
            open_rows.append((identifier, gesture, destructive, sites))
        print(f"## {plugin} — {len(declared)} declared id(s), {len(open_rows)} unmarked lexicon match(es)")
        for identifier, gesture, destructive, sites in open_rows:
            reason = "gesture=" + gesture if gesture else ""
            reason += (" " if reason else "") + ("destructive=" + destructive if destructive else "")
            print(f"- {identifier} [{reason}]")
            for site in sites[:3]:
                print(f"    {site}")
        print("")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

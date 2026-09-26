#!/usr/bin/env python3
"""⌨️ F1 — generates the language-agnostic typing-run fixture (≥ 1000 typed characters with pauses, caret moves back into
the run and Backspace/Delete corrections) and its expected text under the caret-relative text-input model."""
import json, random

random.seed(20260926)
words = ["MATCH", "(a:Piece)", "RETURN", "a.name", "WHERE", "b.label", "=", "'core'", "AND", "ORDER", "BY", "LIMIT", "25", "alpha", "beta_2", "gamma", "Zürich", "Straße"]
keys = []
typed = 0
while typed < 1100:
    word = random.choice(words)
    for ch in " " + word:
        keys.append(ch)
        typed += 1
    roll = random.random()
    if roll < 0.18:
        keys.extend(["Backspace"] * random.randint(1, 3))
    elif roll < 0.30:
        back = random.randint(1, 4)
        keys.extend(["ArrowLeft"] * back + [random.choice("xyz")] + ["ArrowRight"] * back)
    elif roll < 0.36:
        keys.extend(["ArrowLeft", "ArrowLeft", "Delete", "End"])
    elif roll < 0.40:
        keys.extend(["Enter"])
    if random.random() < 0.08:
        keys.append("pause")
    if random.random() < 0.05:
        keys.extend(["Home", "q", "End"])

def apply(text, caret, key):
    if key == "pause":
        return text, caret
    if key == "Backspace":
        return (text[:caret - 1] + text[caret:], caret - 1) if caret > 0 else (text, caret)
    if key == "Delete":
        return (text[:caret] + text[caret + 1:], caret) if caret < len(text) else (text, caret)
    if key == "ArrowLeft":
        return text, max(0, caret - 1)
    if key == "ArrowRight":
        return text, min(len(text), caret + 1)
    if key == "Home":
        return text, text.rfind("\n", 0, caret) + 1
    if key == "End":
        end = text.find("\n", caret)
        return text, len(text) if end < 0 else end
    ch = "\n" if key == "Enter" else key
    return text[:caret] + ch + text[caret:], caret + 1

initial = "MATCH (n) RETURN n"
text, caret = initial, len(initial)
edits = 0
for key in keys:
    before = text
    text, caret = apply(text, caret, key)
    edits += before != text
fixture = {
    "schema": "semio.editor.typing-run/v1",
    "provenance": {
        "what": "One long typing run into a text editor: every key is a real keystroke of the caret-relative text-input model (typing inserts at the caret, Backspace/Delete remove one character, arrows move one character, Home/End reach the line edges, Enter inserts a line break); `pause` is a typing pause (a unit law ignores it, a live run waits 500 ms). Offsets are Unicode scalar values.",
        "law": "Every family that edits text through the TextEditor replays this run as the host does — one full-text `textEdit` per changed text — and must SAVE every keystroke (no refusal; the store's applied-edit ledger is fixed at 64, so a family that spends one ledger edit per keystroke fails at the 65th), end at `expect.text`, revert the WHOLE run with ONE undo (`initial`) and restore it with ONE redo. Chromium's native <textarea> answers the same keys with the same `expect.text` (third-party oracle).",
        "generator": ".tmp-ticket/wp-f1/patches/typing-run/f1-generate-typing-run.py (seed 20260926)"
    },
    "initial": initial,
    "caret": len(initial),
    "keys": keys,
    "typedCharacters": sum(1 for k in keys if len(k) == 1 or k == "Enter"),
    "textEdits": edits,
    "expect": {"text": text, "caret": caret},
}
open("typing-run.json", "w", encoding="utf-8").write(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n")
print("keys", len(keys), "typed", fixture["typedCharacters"], "edits", edits, "final length", len(text))

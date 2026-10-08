#!/usr/bin/env python
"""Round 2 slice T-C: language-neutral terminal fixtures and their third-party oracles.

  generate   write the three fixtures under ui/🧫️fixtures from the oracles
  verify     recompute every oracle result and compare it with the committed fixtures

Oracles (none of them is first-party code):
  screens  pyte 0.8.2         a Python VT100/xterm screen emulator; fixture streams -> screen grid
  keys     ncurses terminfo   `infocmp -x -1 xterm-256color`: the bytes xterm-256color promises for every key
  mouse    prompt_toolkit     its xterm SGR / X10 / urxvt decode tables; encoded report -> button, kind, modifiers

Run: python r2-tc-oracle.py verify --libs <dir holding pyte and prompt_toolkit>
"""
import argparse
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, ".git")):
    ROOT = os.path.dirname(ROOT)
UI = os.path.join(ROOT, "🧰️framework", "🔨️modules", "🖱️ui")
FIXTURES = os.path.join(UI, "🧫️fixtures")
STREAMS = os.path.join(FIXTURES, "⌨️tui-terminal-streams", "🔣️.json")
KEYS = os.path.join(FIXTURES, "⌨️tui-terminal-keys", "🔣️.json")
MOUSE = os.path.join(FIXTURES, "⌨️tui-terminal-mouse", "🔣️.json")

ESC = "\x1b"

STREAM_CASES = [
    ("text-and-crlf", 10, 3, "hello\r\nworld"),
    ("linefeed-keeps-the-column", 10, 3, "ab\ncd\nef"),
    ("autowrap-at-the-right-edge", 5, 3, "abcdefgh"),
    ("scroll-at-the-bottom", 4, 2, "a\r\nb\r\nc\r\nd"),
    ("cursor-moves", 10, 5, f"{ESC}[3;4HX{ESC}[2AY{ESC}[3BZ{ESC}[2C!{ESC}[3D?"),
    ("column-and-row-absolute", 10, 4, f"{ESC}[2;5HA{ESC}[1GB{ESC}[4dC"),
    ("cursor-stays-inside-the-screen", 6, 3, f"{ESC}[99;99H{ESC}[99A{ESC}[99D"),
    ("erase-display-below", 6, 3, f"aaaaaa\r\nbbbbbb\r\ncccccc{ESC}[2;3H{ESC}[J"),
    ("erase-display-above", 6, 3, f"aaaaaa\r\nbbbbbb\r\ncccccc{ESC}[2;3H{ESC}[1J"),
    ("erase-display-all", 6, 3, f"aaaaaa\r\nbbbbbb\r\ncccccc{ESC}[2;3H{ESC}[2J"),
    ("erase-line-modes", 6, 3, f"aaaaaa\r\nbbbbbb\r\ncccccc{ESC}[1;3H{ESC}[K{ESC}[2;3H{ESC}[1K{ESC}[3;3H{ESC}[2K"),
    ("insert-and-delete-characters", 10, 2, f"abcdef{ESC}[1;3H{ESC}[2@XY{ESC}[1;7H{ESC}[2P"),
    ("erase-characters", 10, 1, f"abcdef{ESC}[1;2H{ESC}[3X"),
    ("insert-and-delete-lines", 4, 4, f"aaaa\r\nbbbb\r\ncccc\r\ndddd{ESC}[2;1H{ESC}[L{ESC}[3;1H{ESC}[M"),
    ("scroll-region-linefeed", 5, 5, f"{ESC}[1;1HAAAAA{ESC}[2;1HBBBBB{ESC}[3;1HCCCCC{ESC}[4;1HDDDDD{ESC}[5;1HEEEEE{ESC}[2;4r{ESC}[4;1H\n"),
    ("reverse-index-scrolls-the-region-down", 5, 5, f"{ESC}[1;1HAAAAA{ESC}[2;1HBBBBB{ESC}[3;1HCCCCC{ESC}[2;4r{ESC}[2;1H{ESC}M"),
    ("index-and-reverse-index", 20, 4, f"A{ESC}DB{ESC}MC"),
    ("tab-stops-set-and-clear", 30, 2, f"a\tb{ESC}H{ESC}[3g\r\n{ESC}[1;5H{ESC}H{ESC}[2;1H\tX"),
    ("default-tab-stops", 24, 1, "a\tb\tc"),
    ("origin-mode-confines-the-cursor", 10, 5, f"{ESC}[2;4r{ESC}[?6h{ESC}[1;1HX{ESC}[2;3HY"),
    ("save-and-restore-cursor", 10, 5, f"{ESC}[2;3HA{ESC}7{ESC}[5;1HB{ESC}8C"),
    ("utf8-text", 20, 1, "héllo wörld €"),
    ("wide-characters", 10, 2, "a好b世界c"),
    ("backspace-overwrites", 10, 1, "abc\b\bX"),
    ("alignment-test", 4, 2, f"{ESC}#8"),
    ("sgr-leaves-text-untouched", 24, 1, f"{ESC}[1;31mred{ESC}[0m {ESC}[38;5;196mx{ESC}[48;2;1;2;3my{ESC}[0mz"),
    ("osc-title-is-not-text", 10, 1, f"{ESC}]0;title\x07text{ESC}]2;two{ESC}\\!"),
    ("unknown-sequences-are-swallowed", 10, 1, f"a{ESC}[?9999hb{ESC}[>cc{ESC}[1;2;3;4;5xd"),
    ("carriage-return-rewrite", 10, 2, "hello\rHE\r\nx"),
    ("decstbm-homes-the-cursor", 6, 4, f"abc{ESC}[2;3r{ESC}[Hx"),
]


def feed_pyte(cols, rows, data):
    import pyte

    screen = pyte.Screen(cols, rows)
    pyte.ByteStream(screen).feed(data.encode("utf-8"))
    display = [line.rstrip() for line in screen.display]
    return display, [screen.cursor.x, screen.cursor.y]


def stream_cases():
    cases = []
    for name, cols, rows, data in STREAM_CASES:
        display, cursor = feed_pyte(cols, rows, data)
        entry = {"id": name, "cols": cols, "rows": rows, "stream": data, "screen": display}
        entry["cursor"] = cursor if cursor[0] < cols else None
        cases.append(entry)
    return {
        "provenance": {
            "what": "Byte streams a child writes to its terminal and the screen grid (rows, trailing blanks trimmed) and cursor they leave behind, plus the language-neutral oracle for the embedded terminal's VT parser.",
            "why": "The dashboard's embedded terminal replaced ad-hoc parsing with a table-driven VT screen; every stream here is replayed by pyte 0.8.2 (a third-party VT emulator) and by the Rust screen, and both must show the same grid.",
            "oracle": "pyte 0.8.2 `Screen.display` and `Screen.cursor`; a null cursor marks streams that end in a pending wrap, where pyte reports column == width and xterm reports the last column.",
            "regenerate": "python r2-tc-oracle.py generate --libs <dir holding pyte>",
        },
        "cases": cases,
    }


MODIFIER_NAMES = [(1, "shift"), (2, "alt"), (4, "ctrl")]
CURSOR_KEYS = {"kcuu1": "Up", "kcud1": "Down", "kcuf1": "Right", "kcub1": "Left", "khome": "Home", "kend": "End"}
EDIT_KEYS = {"kich1": "Insert", "kdch1": "Delete", "kpp": "PageUp", "knp": "PageDown", "kbs": "Backspace", "kcbt": "BackTab"}
MODIFIED = {"UP": "Up", "DN": "Down", "LFT": "Left", "RIT": "Right", "HOM": "Home", "END": "End", "IC": "Insert", "DC": "Delete", "NXT": "PageDown", "PRV": "PageUp"}
KEYPAD = {"kpZRO": "Digit0", "kc1": "Digit1", "kc2": "Digit2", "kc3": "Digit3", "kb1": "Digit4", "kb2": "Digit5", "kb3": "Digit6", "ka1": "Digit7", "ka2": "Digit8", "ka3": "Digit9", "kpADD": "Plus", "kpSUB": "Minus", "kpMUL": "Multiply", "kpDIV": "Divide", "kpDOT": "Decimal", "kent": "Enter", "kpCMA": "Separator"}


def unescape(text):
    out = []
    i = 0
    while i < len(text):
        c = text[i]
        if c == "\\" and i + 1 < len(text):
            n = text[i + 1]
            if n == "E":
                out.append("\x1b")
            elif n == "n":
                out.append("\n")
            elif n == "r":
                out.append("\r")
            elif n == "t":
                out.append("\t")
            elif n == "s":
                out.append(" ")
            elif n in "\\^,:":
                out.append(n)
            else:
                out.append(n)
            i += 2
        elif c == "^" and i + 1 < len(text):
            n = text[i + 1]
            out.append("\x7f" if n == "?" else chr(ord(n) & 0x1F))
            i += 2
        else:
            out.append(c)
            i += 1
    return "".join(out)


def terminfo():
    raw = subprocess.run(["infocmp", "-x", "-1", "xterm-256color"], capture_output=True, text=True, check=True).stdout
    caps = {}
    for line in raw.splitlines():
        line = line.strip().rstrip(",")
        if "=" in line and not line.startswith("#"):
            name, value = line.split("=", 1)
            caps[name] = unescape(value)
    return caps


def mask_modifiers(parameter):
    bits = parameter - 1
    return [name for bit, name in MODIFIER_NAMES if bits & bit]


def key_cases():
    caps = terminfo()
    cases = []

    def add(capability, key, modifiers, modes):
        cases.append({"capability": capability, "key": key, "mods": modifiers, "modes": modes, "bytes": caps[capability]})

    for capability, key in CURSOR_KEYS.items():
        add(capability, key, [], ["app_cursor"])
    for capability, key in EDIT_KEYS.items():
        add(capability, key, [], [])
    for number in range(1, 64):
        capability = f"kf{number}"
        if capability in caps:
            add(capability, f"F{number}", [], [])
    for short, key in MODIFIED.items():
        if f"k{short}" in caps:
            add(f"k{short}", key, ["shift"], ["app_cursor"])
        for parameter in range(3, 8):
            capability = f"k{short}{parameter}"
            if capability in caps:
                add(capability, key, mask_modifiers(parameter), ["app_cursor"])
    for capability, key in KEYPAD.items():
        add(capability, "Keypad" + key, [], ["app_keypad"])
    cases.append({"capability": "kxIN", "key": "FocusGained", "mods": [], "modes": ["focus_reporting"], "bytes": caps["kxIN"]})
    cases.append({"capability": "kxOUT", "key": "FocusLost", "mods": [], "modes": ["focus_reporting"], "bytes": caps["kxOUT"]})
    return {
        "provenance": {
            "what": "Keys, the child modes in force, and the exact bytes the child reads, for the embedded terminal's key encoder.",
            "why": "The embedded terminal used to forward a fixed table that dropped Delete, Insert, function keys, Alt and modified arrows; a child that asked for application cursor keys or modified keys got the wrong bytes.",
            "oracle": "ncurses terminfo entry xterm-256color read with `infocmp -x -1 xterm-256color` (kcuu1 .. kf63, kUP5 .. kPRV7, keypad k*, kxIN/kxOUT): the strings every terminfo-driven program matches the terminal against.",
            "regenerate": "python r2-tc-oracle.py generate",
        },
        "cases": cases,
    }


BUTTONS = {"left": 0, "middle": 1, "right": 2}


def mouse_code(kind, button, modifiers):
    code = 0
    if kind == "down" or kind == "up":
        code = BUTTONS[button]
    elif kind == "drag":
        code = BUTTONS[button] + 32
    elif kind == "move":
        code = 35
    elif kind == "wheel_up":
        code = 64
    elif kind == "wheel_down":
        code = 65
    for name, bits in (("shift", 4), ("alt", 8), ("ctrl", 16)):
        if name in modifiers:
            code += bits
    return code


def mouse_bytes(encoding, kind, button, modifiers, x, y):
    code = mouse_code(kind, button, modifiers)
    if encoding == "sgr":
        return f"\x1b[<{code};{x + 1};{y + 1}{'m' if kind == 'up' else 'M'}"
    if kind == "up":
        code = 3 + (code - (BUTTONS[button] if button else 0))
    if encoding == "x10":
        return "\x1b[M" + chr(code + 32) + chr(x + 33) + chr(y + 33)
    if encoding == "urxvt":
        return f"\x1b[{code + 32};{x + 1};{y + 1}M"
    raise ValueError(encoding)


def mouse_cases():
    cases = []
    positions = [(0, 0), (4, 2), (79, 23)]
    mod_sets = [[], ["shift"], ["alt"], ["ctrl"], ["shift", "alt", "ctrl"]]
    full = [("down", "left"), ("down", "middle"), ("down", "right"), ("up", "left"), ("up", "right"), ("drag", "left"), ("drag", "right"), ("move", None), ("wheel_up", None), ("wheel_down", None)]
    urxvt = [("down", "left"), ("up", "left"), ("wheel_up", None), ("wheel_down", None)]
    for encoding, kinds, sets in (("sgr", full, mod_sets), ("x10", full, mod_sets), ("urxvt", urxvt, [[]])):
        for kind, button in kinds:
            for modifiers in sets:
                for x, y in positions:
                    cases.append({"encoding": encoding, "reporting": "motion", "kind": kind, "button": button, "mods": modifiers, "x": x, "y": y, "bytes": mouse_bytes(encoding, kind, button, modifiers, x, y)})
    cases.append({"encoding": "x10", "reporting": "press", "kind": "down", "button": "left", "mods": [], "x": 223, "y": 0, "bytes": ""})
    cases.append({"encoding": "sgr", "reporting": "press", "kind": "drag", "button": "left", "mods": [], "x": 1, "y": 1, "bytes": ""})
    cases.append({"encoding": "sgr", "reporting": "off", "kind": "down", "button": "left", "mods": [], "x": 1, "y": 1, "bytes": ""})
    cases.append({"encoding": "sgr", "reporting": "click", "kind": "up", "button": "left", "mods": [], "x": 1, "y": 1, "bytes": ""})
    return {
        "provenance": {
            "what": "Pointer events at pane-local cells, the child's reporting level and encoding, and the exact report bytes the child reads.",
            "why": "The embedded terminal dropped every pointer event; a child that asked for mouse tracking (vim, htop, less, the dashboard of another dashboard) got nothing.",
            "oracle": "prompt_toolkit.key_binding.bindings.mouse tables xterm_sgr_mouse_events, typical_mouse_events and urxvt_mouse_events decode each report back to button, event type and modifiers; the decoded event must equal the fixture event.",
            "regenerate": "python r2-tc-oracle.py generate --libs <dir holding prompt_toolkit>",
        },
        "cases": cases,
    }


def verify_mouse(document):
    from prompt_toolkit.key_binding.bindings.mouse import typical_mouse_events, urxvt_mouse_events, xterm_sgr_mouse_events

    failures = []
    kinds = {"MOUSE_DOWN": "down", "MOUSE_UP": "up", "MOUSE_MOVE": None, "SCROLL_UP": "wheel_up", "SCROLL_DOWN": "wheel_down"}
    for case in document["cases"]:
        data = case["bytes"]
        if not data:
            continue
        if case["encoding"] == "sgr":
            match = re.fullmatch(r"\x1b\[<(\d+);(\d+);(\d+)([Mm])", data)
            code, x, y, final = int(match[1]), int(match[2]) - 1, int(match[3]) - 1, match[4]
            button, event_type, modifiers = xterm_sgr_mouse_events[code, final]
            names = sorted({"SHIFT": "shift", "ALT": "alt", "CONTROL": "ctrl"}[m.name] for m in modifiers)
            expected_type = {"down": "MOUSE_DOWN", "up": "MOUSE_UP", "drag": "MOUSE_MOVE", "move": "MOUSE_MOVE", "wheel_up": "SCROLL_UP", "wheel_down": "SCROLL_DOWN"}[case["kind"]]
            expected_button = {"left": "LEFT", "middle": "MIDDLE", "right": "RIGHT", None: "NONE"}[case["button"] if case["kind"] not in ("wheel_up", "wheel_down", "move") else None]
            ok = event_type.name == expected_type and button.name == expected_button and names == sorted(case["mods"]) and (x, y) == (case["x"], case["y"])
        elif case["encoding"] == "x10":
            code, x, y = ord(data[3]), ord(data[4]) - 33, ord(data[5]) - 33
            button, event_type, _ = typical_mouse_events[code & ~(4 | 8 | 16)]
            modifier_bits = code & (4 | 8 | 16)
            expected_bits = sum(b for n, b in (("shift", 4), ("alt", 8), ("ctrl", 16)) if n in case["mods"])
            type_ok = {"down": "MOUSE_DOWN", "up": "MOUSE_UP", "drag": "MOUSE_MOVE", "move": "MOUSE_MOVE", "wheel_up": "SCROLL_UP", "wheel_down": "SCROLL_DOWN"}[case["kind"]] == event_type.name
            button_ok = case["kind"] in ("up", "move", "wheel_up", "wheel_down") or button.name == case["button"].upper()
            ok = type_ok and button_ok and modifier_bits == expected_bits and (x, y) == (case["x"], case["y"])
        else:
            match = re.fullmatch(r"\x1b\[(\d+);(\d+);(\d+)M", data)
            code, x, y = int(match[1]), int(match[2]) - 1, int(match[3]) - 1
            _, event_type, _ = urxvt_mouse_events[code]
            expected_type = {"down": "MOUSE_DOWN", "up": "MOUSE_UP", "wheel_up": "SCROLL_UP", "wheel_down": "SCROLL_DOWN"}[case["kind"]]
            ok = event_type.name == expected_type and (x, y) == (case["x"], case["y"])
        if not ok:
            failures.append(case)
    return failures


def write(path, document):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["generate", "verify"])
    parser.add_argument("--libs", action="append", default=[])
    args = parser.parse_args()
    for lib in args.libs:
        sys.path.insert(0, lib)
    if args.command == "generate":
        write(STREAMS, stream_cases())
        write(KEYS, key_cases())
        document = mouse_cases()
        failed = verify_mouse(document)
        if failed:
            print("mouse oracle disagrees with", len(failed), "generated cases", failed[:3])
            return 1
        write(MOUSE, document)
        print("written", STREAMS, KEYS, MOUSE)
        return 0
    status = 0
    if stream_cases() != read(STREAMS):
        print("streams: pyte disagrees with the committed fixture")
        status = 1
    else:
        print("streams: pyte reproduces", len(read(STREAMS)["cases"]), "cases")
    if key_cases() != read(KEYS):
        print("keys: terminfo disagrees with the committed fixture")
        status = 1
    else:
        print("keys: terminfo reproduces", len(read(KEYS)["cases"]), "cases")
    committed = read(MOUSE)
    failed = verify_mouse(committed)
    if failed or mouse_cases() != committed:
        print("mouse: prompt_toolkit disagrees with", len(failed), "cases", failed[:3])
        status = 1
    else:
        print("mouse: prompt_toolkit decodes", len(committed["cases"]), "cases")
    return status


if __name__ == "__main__":
    sys.exit(main())

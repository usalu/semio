"""📣️ Rewrites the wgpu shell's dead `eprintln!` traces onto its own `Self::debug_log` sink.

`eprintln!` writes to a stderr that does not exist in a `wasm32-unknown-unknown` Worker, so every one
of these traces is a silent no-op on 6118 — the file's own line 3258 already says so for one case and
the rest were never swept. `ShellState::debug_log` is the sink that reaches `console.log` under wasm
and still `eprintln!`s natively, so the trace survives both targets.

Paren-matched rather than regex'd, because several call sites carry nested calls in their format args
(`self.sync_backbone_uri.as_deref().unwrap_or_default()`). The one site INSIDE `debug_log`'s own
non-wasm arm is skipped by name, never by line number, so a concurrent edit cannot shift it.

Usage: python3 🐍️eprintln-to-debug-log.py <file> [--check]
"""

import sys

SKIP_CONTEXT = "fn debug_log"


def convert(text: str) -> tuple[str, int]:
    out = []
    index = 0
    converted = 0
    needle = "eprintln!("
    while True:
        found = text.find(needle, index)
        if found == -1:
            out.append(text[index:])
            break
        guard_start = max(0, found - 400)
        if SKIP_CONTEXT in text[guard_start:found]:
            out.append(text[index : found + len(needle)])
            index = found + len(needle)
            continue
        depth = 0
        cursor = found + len(needle) - 1
        in_string = False
        escaped = False
        while cursor < len(text):
            char = text[cursor]
            if in_string:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    in_string = False
            elif char == '"':
                in_string = True
            elif char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
                if depth == 0:
                    break
            cursor += 1
        if cursor >= len(text):
            raise SystemExit(f"unbalanced eprintln! at offset {found}")
        args = text[found + len(needle) : cursor]
        out.append(text[index:found])
        out.append(f"Self::debug_log(&format!({args}))")
        index = cursor + 1
        converted += 1
    return "".join(out), converted


def main() -> None:
    path = sys.argv[1]
    check = "--check" in sys.argv
    with open(path, encoding="utf-8") as handle:
        original = handle.read()
    rewritten, converted = convert(original)
    print(f"converted {converted} eprintln! call sites")
    if check:
        return
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(rewritten)


if __name__ == "__main__":
    main()

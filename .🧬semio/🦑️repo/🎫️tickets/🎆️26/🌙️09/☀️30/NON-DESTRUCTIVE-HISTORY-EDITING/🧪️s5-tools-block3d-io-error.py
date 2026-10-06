"""🧱️ Ports block 3d's four IO files to the current `IoError` (the peer API dropped the `message`/`diagnostics` fields): every
`IoError { message: <expr>, diagnostics: Vec::new() }` becomes `IoError::from_value_error(ValueError::new(kind, <expr>))` — the
form block 2d and 5d already use — with the DSL parser's own refusal kind kept. Explicit file list, fails closed.

Usage (cwd: repo root): python3 🧪️s5-tools-block3d-io-error.py [--apply]
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
IO = "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
FILES = {
    f"{IO}/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs": 3,
    f"{IO}/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": 4,
    f"{IO}/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs": 1,
    f"{IO}/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": 1,
}
HEAD, TAIL = "IoError { message: ", ", diagnostics: Vec::new() }"
VALUE = "semio_framework_value::ValueError::new"
INVALID = "semio_framework_value::ValueRefusalKind::InvalidValue"


def port(text, expected):
    out, cursor, found = [], 0, 0
    while (start := text.find(HEAD, cursor)) != -1:
        end = text.find(TAIL, start)
        if end == -1 or "\n" in text[start:end]:
            raise SystemExit(f"unterminated site at offset {start}")
        message = text[start + len(HEAD) : end]
        kind = "error.kind" if message == "error.to_string()" else INVALID
        out.append(text[cursor:start] + f"IoError::from_value_error({VALUE}({kind}, {message}))")
        cursor, found = end + len(TAIL), found + 1
    if found != expected:
        raise SystemExit(f"{found} site(s), expected {expected}")
    return "".join(out) + text[cursor:]


def main():
    staged = []
    for relative, expected in FILES.items():
        path = REPO / relative
        if not path.is_file():
            raise SystemExit(f"{relative} is gone")
        staged.append((path, port(path.read_text(encoding="utf-8"), expected)))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'ported' if '--apply' in sys.argv else 'would port'} {sum(FILES.values())} sites in {len(staged)} files")


if __name__ == "__main__":
    main()

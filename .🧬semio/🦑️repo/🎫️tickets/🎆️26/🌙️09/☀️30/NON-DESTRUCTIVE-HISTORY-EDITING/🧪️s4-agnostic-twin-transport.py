#!/usr/bin/env python3
"""🚚️ Binary64Transport readers in every artifact TS twin (S4-AGNOSTIC, coordinator routing from S4-STROKES): an artifact twin reads a float
carrier through `parseBinary64Transport`/`parseBinary32Transport` (word | number, the value schema's `Binary64Transport`) instead of the
strict word parsers — the strict ones stay where a twin re-validates its own in-memory word for output (`parseBinary64(x).bits`) and in
sqlite-snapshot owners. Wrappers that pre-checked a `{bits}` record lose that check. Idempotent, every file re-read right before its write.
Usage: [--apply]."""
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
STRICT = re.compile(r"\bparseBinary(64|32)\(")
SPECIAL = [
    ('function word(value:unknown):Binary64{row(value,["bits"]);return parseBinary64(value);}', "function word(value:unknown):Binary64{return parseBinary64Transport(value);}"),
    ('(v:unknown,at:string):Binary64=>parseBinary64(parseSchemaRecord(v,["bits"],at));', "(v:unknown,_at:string):Binary64=>parseBinary64Transport(v);"),
]
MESSAGES = [
    ('"requires an owned binary64 word"', '"requires a binary64 word or number"'),
    ('"expected canonical IEEE word"', '"expected a canonical IEEE word or number"'),
    ('"value is not an exact unsigned binary64 word"', '"value is neither an exact binary64 word nor a number"'),
    ('"expected an exact binary64 word"', '"expected a binary64 word or number"'),
    ('"expected owned unsigned Binary64 word"', '"expected a Binary64 word or number"'),
]
IMPORT = re.compile(r'import\s*\{([^}]*)\}\s*from\s*(["\'])([^"\']*🔢️ieee754/🟦️\.ts)\2;')


def files() -> list[str]:
    out = subprocess.run(["git", "grep", "-l", "-E", r"parseBinary(64|32)\(", "--", "*.ts", "*.tsx"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    return [path for path in out if "🗿️artifacts/" in path and "🪶️sqlite" not in path and "🧪️tests" not in path]


def close_paren(text: str, open_at: int) -> int:
    depth = 0
    for index in range(open_at, len(text)):
        depth += {"(": 1, ")": -1}.get(text[index], 0)
        if depth == 0:
            return index
    return -1


def rewrite(text: str) -> tuple[str, int, int]:
    for old, new in SPECIAL:
        text = text.replace(old, new)
    out, last, swapped, kept = [], 0, 0, 0
    for match in STRICT.finditer(text):
        end = close_paren(text, match.end() - 1)
        if end >= 0 and text[end + 1 : end + 6] == ".bits":
            kept += 1
            continue
        out.append(text[last : match.start()])
        out.append(f"parseBinary{match.group(1)}Transport(")
        last = match.end()
        swapped += 1
    out.append(text[last:])
    text = "".join(out)
    for old, new in MESSAGES:
        text = "\n".join(line.replace(old, new) if "Transport(" in line else line for line in text.split("\n"))

    def names(match: re.Match[str]) -> str:
        listed = [name.strip() for name in match.group(1).split(",") if name.strip()]
        body = text[: match.start()] + text[match.end() :]
        wanted = [name for name in listed if name.startswith("type ") or re.search(rf"\b{re.escape(name)}\b", body)]
        for width in ("64", "32"):
            transport = f"parseBinary{width}Transport"
            if re.search(rf"\b{transport}\(", body) and transport not in wanted:
                wanted.append(transport)
        return "import {" + ",".join(wanted) + "} from " + match.group(2) + match.group(3) + match.group(2) + ";"

    return IMPORT.sub(names, text, count=1), swapped, kept


def main() -> None:
    apply = "--apply" in sys.argv
    for path in files():
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            before = handle.read()
        after, swapped, kept = rewrite(before)
        if after == before:
            continue
        if not IMPORT.search(after):
            print(f"NO-IMPORT {path}")
            continue
        if apply:
            with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
                if handle.read() != before:
                    print(f"RACE {path}")
                    continue
            with open(f"{ROOT}/{path}", "w", encoding="utf-8") as handle:
                handle.write(after)
        print(f"{'WROTE' if apply else 'WOULD'} {swapped} transport, {kept} output-strict: {path}")


if __name__ == "__main__":
    main()

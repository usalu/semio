"""🧹️ S4-PACKFIX sweep — moves handwritten callers onto the `semio-framework-pack-error` API with the kernel mapping table.

Usage: `python3 🧪️s4-packfix-sweep.py [--write] [--tests] <tree-or-file>...` (paths relative to the repo root). Dry run by default.
Idempotent: only retired `PackError::<variant>` spellings and the S4-TOOLS-B interim `Malformed { what: "pack", offset: 0 }`
form are rewritten. Test directories are skipped unless `--tests` is given.

Mapping (see `📓️s4-packfix-report.md` § Mapping):
- `PackError::Schema(<semio error>.to_string())` on an envelope line → `PackError::from(<error>.into_value_error())`
- `PackError::Schema(<error>.to_string())` → `PackError::from(<error>)` (typed conversion; non-`From` sources are fixed by hand)
- `PackError::Schema(<message>)` → `PackError::from(ValueError::new(<authored kind>, <message>))`
- bare `PackError::Schema` → `|detail| PackError::from(ValueError::new(InvalidValue, detail))`
- `PackError::ValueRefusal|TextRefusal` → `PackError::from`; in a pattern → `PackError::Refusal(PackRefusal::<same>(..))`
- `PackError::Malformed { .. }` → `PackError::Refusal(PackRefusal::Malformed { kind: InvalidValue, .. })`
- `PackError::Truncated|NonCanonical|BadMagic|ContentHashMismatch|UnsupportedCodec|UnknownRequiredFlags|UnsupportedVersion|ChecksumMismatch` → `PackError::Refusal(PackRefusal::<same>)`
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
VALUE = "semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::{kind}, {message})"
PREFIX = r"((?:\$?[A-Za-z_][A-Za-z0-9_]*::)*)"
STRUCTURAL = ("Truncated", "NonCanonical", "BadMagic", "ContentHashMismatch", "UnsupportedCodec", "UnknownRequiredFlags", "UnsupportedVersion", "ChecksumMismatch")
UNSUPPORTED = ("unsupported", "not available", "can only represent", "only exports", "cannot represent")


def closing(text, start):
    pairs = {"(": ")", "{": "}", "[": "]"}
    stack, index = [], start
    while index < len(text):
        char = text[index]
        if char in pairs:
            stack.append(pairs[char])
        elif char in ")}]":
            if not stack or stack.pop() != char:
                raise ValueError(f"unbalanced at {index}")
            if not stack:
                return index
        elif char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        elif char == "'" and re.match(r"'(\\.|[^\\'])'", text[index:index + 4]):
            index += text[index + 1:].index("'") + 1
        index += 1
    raise ValueError("unterminated")


def is_pattern(text, end):
    rest = text[end + 1:end + 200]
    return re.match(r"\s*(\)\s*)*(=>|\|[^|]|if\s)", rest) is not None


def kind_for(message):
    literal = " ".join(re.findall(r'"((?:[^"\\]|\\.)*)"', message)).lower()
    if "address space" in literal:
        return "OwnershipLimit"
    if any(marker in literal for marker in UNSUPPORTED):
        return "UnsupportedOwner"
    return "InvalidValue"


def message_of(argument):
    argument = argument.strip()
    for pattern in (r'^("(?:[^"\\]|\\.)*")\.(?:into|to_string|to_owned)\(\)$', r'^String::from\(("(?:[^"\\]|\\.)*")\)$', r"^([A-Za-z_][A-Za-z0-9_.]*)\.into\(\)$"):
        found = re.match(pattern, argument, re.S)
        if found:
            return found.group(1)
    return argument


def line_of(text, index):
    return text[text.rfind("\n", 0, index) + 1:text.find("\n", index)]


def migrate(text):
    out, at = [], 0
    head = re.compile(PREFIX + r"PackError::(Schema|ValueRefusal|TextRefusal|Malformed|" + "|".join(STRUCTURAL) + r")\b")
    while True:
        found = head.search(text, at)
        if not found:
            out.append(text[at:])
            break
        out.append(text[at:found.start()])
        prefix, variant, after = found.group(1), found.group(2), found.end()
        error, refusal = f"{prefix}PackError", f"{prefix}PackRefusal"
        opener = re.match(r"\s*([({])", text[after:])
        if variant == "Schema":
            if opener and opener.group(1) == "(":
                start = after + opener.start(1)
                end = closing(text, start)
                argument = text[start + 1:end]
                source = re.match(r"^\s*([A-Za-z_][A-Za-z0-9_.]*)\.to_string\(\)\s*$", argument)
                if source and ("from_envelope_id(" in line_of(text, found.start()) or "unwrap_binary" in line_of(text, found.start())):
                    out.append(f"{error}::from({source.group(1)}.into_value_error())")
                elif source:
                    out.append(f"{error}::from({source.group(1)})")
                else:
                    message = message_of(argument)
                    out.append(f"{error}::from({VALUE.format(kind=kind_for(message), message=message)})")
                at = end + 1
            else:
                out.append(f"|detail| {error}::from({VALUE.format(kind='InvalidValue', message='detail')})")
                at = after
        elif variant in ("ValueRefusal", "TextRefusal"):
            if opener and opener.group(1) == "(":
                start = after + opener.start(1)
                end = closing(text, start)
                inner = text[start + 1:end]
                out.append(f"{error}::Refusal({refusal}::{variant}({inner}))" if is_pattern(text, end) else f"{error}::from({inner})")
                at = end + 1
            else:
                out.append(f"{error}::from")
                at = after
        elif variant == "Malformed":
            start = after + opener.start(1)
            end = closing(text, start)
            body = text[start + 1:end]
            pattern = is_pattern(text, end) or body.rstrip().endswith("..")
            if not pattern and "kind" not in body:
                body = f" kind: semio_framework_value::ValueRefusalKind::InvalidValue,{body}"
            out.append(f"{error}::Refusal({refusal}::Malformed {{{body}}})")
            at = end + 1
        else:
            if opener:
                start = after + opener.start(1)
                end = closing(text, start)
                out.append(f"{error}::Refusal({refusal}::{variant}{text[after:end + 1]})")
                at = end + 1
            else:
                out.append(f"{error}::Refusal({refusal}::{variant})")
                at = after
    migrated = "".join(out)
    interim = re.compile(PREFIX + r"PackError::Refusal\(\1PackRefusal::Malformed \{ kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: \"pack\", offset: 0, detail: ")
    out, at = [], 0
    while True:
        found = interim.search(migrated, at)
        if not found:
            out.append(migrated[at:])
            return "".join(out)
        out.append(migrated[at:found.start()])
        brace = migrated.index("Malformed {", found.start()) + len("Malformed ")
        end = closing(migrated, brace)
        detail = migrated[found.end():end].strip()
        source = re.match(r"^([A-Za-z_][A-Za-z0-9_.]*)\.to_string\(\)$", detail)
        if source:
            replacement = f"{found.group(1)}PackError::from({source.group(1)}"
        else:
            message = message_of(detail)
            replacement = f"{found.group(1)}PackError::from({VALUE.format(kind=kind_for(message), message=message)}"
        if not migrated.startswith(")", end + 1):
            raise ValueError("interim form without closing Refusal paren")
        out.append(replacement + ")")
        at = end + 2


def files(arguments, tests):
    for argument in arguments:
        path = ROOT / argument
        candidates = [path] if path.is_file() else path.rglob("🦀️.rs")
        for candidate in candidates:
            parts = candidate.parts
            if any(part in ("dist", "node_modules", "target") for part in parts):
                continue
            if not tests and "🧪️tests" in parts:
                continue
            yield candidate


def main():
    flags = [arg for arg in sys.argv[1:] if arg.startswith("--")]
    trees = [arg for arg in sys.argv[1:] if not arg.startswith("--")]
    if any(flag not in ("--write", "--tests") for flag in flags) or not trees:
        sys.exit("usage: 🧪️s4-packfix-sweep.py [--write] [--tests] <tree-or-file>...")
    write, changed, sites = "--write" in flags, 0, 0
    for path in files(trees, "--tests" in flags):
        text = path.read_text(encoding="utf-8")
        try:
            after = migrate(text)
        except ValueError as error:
            print(f"MANUAL {path.relative_to(ROOT)}: {error}")
            continue
        if after != text:
            changed += 1
            count = sum(1 for a, b in zip(text.split("\n"), after.split("\n")) if a != b)
            sites += count
            print(f"{'migrated' if write else 'pending'} {count:4} {path.relative_to(ROOT)}")
            if write:
                path.write_text(after, encoding="utf-8")
    print(f"{changed} files, {sites} lines {'migrated' if write else 'pending'}")


main()

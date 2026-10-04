#!/usr/bin/env python3
"""🪶️ S4-INFRA: idempotent repo-wide migration of handwritten SQLite snapshot owners onto the typed `ValueError` ABI.

The store trait `ArtifactSqliteSnapshot` (`🏪️store/🦀️.rs`) and every `sqlite_snapshot` helper return
`semio_framework_value::ValueError`; native record closures return `TextError`; subset validators return `IoResult`.
This follows the peer's converted owners (block 5d, workflow, space-history) and extends S4-TOOLS-A's stage:

- `Result<T, String>` → `Result<T, ValueError>` at every depth;
- literal refusals → `invalid(..)` (`InvalidValue`), overflow literals → `WorkLimit`, allocation failures → `AllocationFailed`;
- `validate_sqlite_database_schema(..)` keeps its typed refusal (its `map_err` is dropped);
- since the 07:16 store change native record closures return `ValueError` themselves: `__dsl_{from,to}_record_controlled`
  results pass through unmapped and function-path constructors become plain closures;
- 2-argument `TextError::new(message, span)` → `TextError::new(InvalidValue, message, span)`;
- inside `IoResult` functions every `?` maps through `<io>::IoError::from_value_error`, literal refusals become typed causes;
- `protocol|dsl|store::native_decoding` → `semio_framework_value::native_decoding`.

Files touched by someone else in the last 30 minutes are skipped unless named with `--own=<path suffix>`.
Usage: `python3 🧪️s4-infra-sqlite-abi.py [--apply] [--own=<suffix>…] <file>…` (dry run without `--apply`).
"""
import os
import re
import sys
import time

APPLY = "--apply" in sys.argv
OWN = [arg[6:] for arg in sys.argv[1:] if arg.startswith("--own=")]
FILES = [arg for arg in sys.argv[1:] if not arg.startswith("--")]
QUIET_SECONDS = 30 * 60
KIND = "semio_framework_value::ValueRefusalKind"


def balanced(text, open_index):
    """🧮️ The index just past the bracket closing the one at `open_index` (string literals respected)."""
    pairs = {"(": ")", "[": "]", "{": "}"}
    stack = [pairs[text[open_index]]]
    index = open_index + 1
    while index < len(text) and stack:
        char = text[index]
        if char == '"':
            index += 1
            while index < len(text) and text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        elif char in pairs:
            stack.append(pairs[char])
        elif char == stack[-1]:
            stack.pop()
        index += 1
    return index


def top_level_args(body):
    """🧩️ Splits a call body at its top-level commas."""
    args, depth, start, index = [], 0, 0, 0
    while index < len(body):
        char = body[index]
        if char == '"':
            index += 1
            while index < len(body) and body[index] != '"':
                index += 2 if body[index] == "\\" else 1
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            args.append(body[start:index])
            start = index + 1
        index += 1
    args.append(body[start:])
    return [arg for arg in args if arg.strip()]


def string_results(text):
    """🧾️ `Result<T, String>` → `Result<T, ValueError>` at every depth (generic arguments balanced)."""
    out, at = [], 0
    while True:
        start = text.find("Result<", at)
        if start < 0:
            out.append(text[at:])
            return "".join(out)
        index, depth, last_comma = start + len("Result<"), 1, None
        while index < len(text) and depth:
            char = text[index]
            if char == "<":
                depth += 1
            elif char == ">" and text[index - 1] != "-":
                depth -= 1
            elif char == "," and depth == 1:
                last_comma = index
            index += 1
        close = index - 1
        if last_comma is not None and text[last_comma + 1:close].strip() == "String":
            out.append(text[at:last_comma + 1] + ("" if text[last_comma + 1] != " " else " ") + "ValueError")
            at = close
        else:
            out.append(text[at:start + len("Result<")])
            at = start + len("Result<")


def wrap_calls(text, head, build):
    """🎁️ Rewrites every `head(ARG)` (balanced) through `build(ARG)`; `None` keeps the call."""
    out, at = [], 0
    while True:
        start = text.find(head, at)
        if start < 0:
            out.append(text[at:])
            return "".join(out)
        open_index = start + len(head) - 1
        close = balanced(text, open_index)
        replacement = build(text[open_index + 1:close - 1])
        out.append(text[at:start])
        out.append(text[start:close] if replacement is None else replacement)
        at = close


def literal(arg):
    """🔤️ The refusal prose of a literal/`format!` argument, without its `String` conversion, or `None`."""
    arg = arg.strip()
    for suffix in (".into()", ".to_string()", ".to_owned()"):
        if arg.endswith(suffix):
            arg = arg[: -len(suffix)]
    if arg.startswith("String::from(") and arg.endswith(")"):
        arg = arg[len("String::from("):-1].strip()
    if arg.startswith("(") and arg.endswith(")") and balanced(arg, 0) == len(arg):
        arg = arg[1:-1].strip()
    return arg if arg.startswith('"') or arg.startswith("format!") else None


def refusal(prose):
    """🏷️ The typed refusal for one prose: arithmetic overflow is a work limit, everything else an invalid value."""
    if prose.startswith('"') and "overflow" in prose:
        return f"semio_framework_value::ValueError::new({KIND}::WorkLimit,{prose})"
    if prose.startswith('"') and "allocation failed" in prose:
        return f"semio_framework_value::ValueError::new({KIND}::AllocationFailed,{prose})"
    return f"invalid({prose})"


def io_functions(text):
    """🚪️ Inside every `-> <io>::IoResult<..>` function maps `?` and literal refusals through `IoError::from_value_error`."""
    out, at = [], 0
    pattern = re.compile(r"->\s*((?:[A-Za-z_][A-Za-z0-9_]*::)*)IoResult<")
    while True:
        match = pattern.search(text, at)
        if not match:
            out.append(text[at:])
            return "".join(out)
        brace = text.find("{", match.end())
        semicolon = text.find(";", match.end())
        if brace < 0 or (0 <= semicolon < brace):
            out.append(text[at:match.end()])
            at = match.end()
            continue
        close = balanced(text, brace)
        io = f"{match.group(1)}IoError::from_value_error"
        body = text[brace:close]
        body = wrap_calls(body, "Err(", lambda arg: None if literal(arg) is None else f"Err({io}({refusal(literal(arg))}))")
        body = re.sub(r"Err\(((?:invalid|semio_framework_value::ValueError::new)\((?:[^()]|\([^()]*\))*\))\.into\(\)\)", lambda m: f"Err({io}({m.group(1)}))", body)
        body = re.sub(r"(?<!from_value_error\))\?", f".map_err({io})?", body)
        body = body.replace(f".map_err({io}).map_err({io})", f".map_err({io})")
        out.append(text[at:brace])
        out.append(body)
        at = close


def two_argument_text_errors(text):
    """📍️ `TextError::new(message, span)` → `TextError::new(InvalidValue, message, span)`."""
    def build(arg):
        args = top_level_args(arg)
        return None if len(args) != 2 else f"semio_framework_diagnostic::TextError::new({KIND}::InvalidValue,{arg})"
    return wrap_calls(text, "semio_framework_diagnostic::TextError::new(", build)


def string_error_positioner(text):
    """📍️ A local `error(message: impl Into<String>) -> TextError` used only on refusals becomes the `ValueError` positioner."""
    match = re.search(r"^fn error\((\w+):impl Into<String>\)->TextError\{[^\n]*\}$", text, re.M)
    if not match or re.search(r"(?<![\w.])error\((?:\"|format!)", text):
        return text
    text = text.replace(match.group(0), "fn error(error:ValueError)->TextError{TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}")
    text = re.sub(r"\.map_err\(\|\s*(\w+)\s*\|\s*error\(\1\.to_string\(\)\)\)", ".map_err(error)", text)
    return text.replace(".map_err(positioned)", ".map_err(error)")


def header_index(lines):
    """📌️ The line after the last top-level `use` statement of the file head (multi-line statements completed)."""
    last, index = None, 0
    while index < min(len(lines), 80):
        line = lines[index]
        if line.startswith("use ") or line.startswith("pub use "):
            depth = 0
            while True:
                depth += lines[index].count("{") - lines[index].count("}")
                if depth <= 0 and lines[index].rstrip().endswith(";"):
                    break
                index += 1
            last = index
        index += 1
    if last is None:
        raise SystemExit("no use statement in file head")
    return last + 1


def migrate(text, path=None):
    text = re.sub(r"\b(?:protocol|dsl|store)::native_decoding\b", "semio_framework_value::native_decoding", text)
    text = re.sub(r"\.map_err\(\|\s*(\w+)\s*\|\s*(?:\w+::)*IoError::from\(\1\.into_message\(\)\)\)", "", text)
    text = string_results(text)
    text = re.sub(r"\.map_err\((?:semio_framework_value::)?ValueError::into_message\)", "", text)
    text = re.sub(r"(Ok|Err)::<(\s*[^,<>]+\s*),\s*String>", r"\1::<\2,ValueError>", text)
    text = io_functions(text)
    text = wrap_calls(text, "Err(", lambda arg: None if literal(arg) is None else f"Err({refusal(literal(arg))})")
    text = wrap_calls(text, ".ok_or(", lambda arg: None if literal(arg) is None else f".ok_or_else(||{refusal(literal(arg))})")
    text = wrap_calls(text, ".ok_or_else(", lambda arg: None if not arg.strip().startswith("||") or literal(arg.strip()[2:]) is None else f".ok_or_else(||{refusal(literal(arg.strip()[2:]))})")
    text = re.sub(r'\.ok_or_else\(\|\|\s*("(?:[^"\\]|\\.)*")(?:\.into\(\)|\.to_string\(\)|\.to_owned\(\))\s*\)', lambda m: f".ok_or_else(||{refusal(m.group(1))})", text)
    text = re.sub(r'\.map_err\(\|_\|\s*("(?:[^"\\]|\\.)*")(?:\.into\(\)|\.to_string\(\))?\s*\)', lambda m: f".map_err(|_|{refusal(m.group(1))})", text)
    text = re.sub(r"(SqliteDatabase::from_schema\((?:[^()]|\([^()]*\))*\))\.map_err\(\|\s*(\w+)\s*\|\s*\2\.to_string\(\)\s*\)", r"\1", text)
    text = re.sub(r"(validate_sqlite_database_schema\((?:[^()]|\([^()]*\))*\))\.map_err\(\|\s*(\w+)\s*\|\s*\2\.to_string\(\)\s*\)", r"\1", text)
    text = re.sub(r"\.map_err\(\|\s*(\w+)\s*\|\s*\1\.to_string\(\)\s*\)", r".map_err(|\1|invalid(\1.to_string()))", text)
    text = re.sub(r"([,(]\s*)((?:Self|[A-Z]\w*)::__dsl_from_record_controlled)(\s*[,)])", r"\1|record,native|\2(record,native)\3", text)
    text = two_argument_text_errors(text)
    text = string_error_positioner(text)
    needs = [name for name, used in (("invalid", r"\binvalid\("), ("positioned", r"\(positioned\)|\bpositioned\(")) if re.search(used, text) and f"fn {name}(" not in text]
    if re.search(r"^use super::\*;", text, re.M) and path:
        parent = os.path.join(os.path.dirname(os.path.dirname(path)), os.path.basename(path))
        inherited = open(parent, encoding="utf-8").read() if os.path.exists(parent) else ""
        needs = [name for name in needs if f"fn {name}(" not in inherited]
        if not needs and re.search(r"use semio_framework_value::(?:\{[^}]*\bValueError\b[^}]*\}|ValueError);", inherited):
            return text
    if needs or ("ValueError" in text and not re.search(r"use semio_framework_value::(?:\{[^}]*\bValueError\b[^}]*\}|ValueError);", text)):
        lines = text.split("\n")
        index = header_index(lines)
        header = []
        if not re.search(r"use semio_framework_value::(?:\{[^}]*\bValueError\b[^}]*\}|ValueError);", text) and not re.search(r"\bstruct ValueError\b", text):
            header.append("use semio_framework_value::ValueError;")
        if "invalid" in needs:
            header.append(f"fn invalid(message:impl Into<String>)->ValueError{{ValueError::new({KIND}::InvalidValue,message)}}")
        if "positioned" in needs:
            header.append("fn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}")
        text = "\n".join(lines[:index] + header + lines[index:])
    return text


def main():
    now = time.time()
    changed, skipped, clean = [], [], []
    for path in FILES:
        text = open(path, encoding="utf-8").read()
        after = migrate(text, path)
        if after == text:
            clean.append(path)
            continue
        if now - os.path.getmtime(path) < QUIET_SECONDS and not any(path.endswith(own) for own in OWN):
            skipped.append(path)
            continue
        changed.append(path)
        if APPLY:
            if open(path, encoding="utf-8").read() != text:
                raise SystemExit(f"changed during migration: {path}")
            open(path, "w", encoding="utf-8").write(after)
    for path in skipped:
        print(f"[s4-infra] skipped (touched < 30 min ago) {path}")
    for path in changed:
        print(f"[s4-infra] {'migrated' if APPLY else 'pending'} {path}")
    print(f"[s4-infra] {len(changed)} {'migrated' if APPLY else 'pending'}, {len(skipped)} skipped, {len(clean)} already clean")


if __name__ == "__main__":
    main()

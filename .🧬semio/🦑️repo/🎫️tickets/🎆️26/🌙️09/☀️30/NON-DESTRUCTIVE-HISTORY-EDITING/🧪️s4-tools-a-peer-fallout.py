#!/usr/bin/env python3
"""🧹️ S4-TOOLS-A: idempotent migration of the value/DSL/pack peer fallout in the draw, note, layout, fem, lowpoly and
shooting plugin trees onto the peer's own direction (never back):

- `dsl::json` / `store::json` (gone from the kernel)  →  `semio_framework_pack_json`; the 1-argument `from_json_str` /
  `parse` gain the member policy the peer's migrated trees use (`JsonMemberPolicy::Reject`);
- kernel-private re-exports (`protocol|dsl|store::{ValueError, DslValue, ToValue, FromValue}`)  →  `semio_framework_value::…`;
- `semio_framework_plugin|protocol::{Terminology, Locale}`  →  `semio_framework_ui_locale::…`; grammar API (`::dsl::parse_grammar`, `SemioDialect`, …)  →  `semio_framework_dsl::…`;
- `IoError { message: X, diagnostics: Vec::new() }`  →  `IoError::from_value_error(ValueError::new(InvalidValue, X))`;
- 2-argument `TextError::new(message, span)`  →  `TextError::new(InvalidValue, message, span)`.

Files touched in the last 30 minutes by someone else are skipped (listed). `--apply` writes; the default is a dry run.
"""
import os
import re
import sys
import time

REPO = "/Users/ueli/Documents/semio"
ROOTS = [f"{REPO}/✏️s/🔌️plugins/{name}" for name in ("🖍️draw", "🗒️note", "📏️layout", "🏗️fem", "💠️lowpoly", "🎥️shooting")]
APPLY = "--apply" in sys.argv
OWN = [line.strip() for line in sys.argv[1:] if line.startswith("--own=")]
QUIET_SECONDS = 30 * 60
POLICY = "semio_framework_pack_json::JsonMemberPolicy::Reject"


def balanced(text, open_index):
    """🧮️ The index just past the bracket that closes the one at `open_index` (strings and chars respected)."""
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


def with_policy(text, call):
    """🔐️ Appends the member policy to every 1-argument call of `call` (turbofish allowed) that lacks it."""
    out, at = [], 0
    pattern = re.compile(re.escape(call) + r"(::<[^()]*?>)?\(")
    while True:
        match = pattern.search(text, at)
        if not match:
            out.append(text[at:])
            return "".join(out)
        open_index = match.end() - 1
        close = balanced(text, open_index)
        args = text[open_index + 1:close - 1]
        out.append(text[at:open_index + 1])
        out.append(args if "JsonMemberPolicy" in args else f"{args}, {POLICY}")
        out.append(")")
        at = close


def io_errors(text):
    """🚪️ Rewrites every `IoError { message: X, diagnostics: Vec::new() }` literal onto the typed cause."""
    out, at = [], 0
    pattern = re.compile(r"IoError\s*\{\s*message:\s*")
    while True:
        match = pattern.search(text, at)
        if not match:
            out.append(text[at:])
            return "".join(out)
        brace = text.index("{", match.start())
        close = balanced(text, brace)
        body = text[match.end():close - 1].rstrip().rstrip(",")
        tail = re.search(r",\s*diagnostics:\s*Vec::new\(\)\s*$", body)
        if not tail:
            out.append(text[at:close])
            at = close
            continue
        message = body[:tail.start()].strip()
        out.append(text[at:match.start()])
        out.append(f"IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, {message}))")
        at = close


def top_level_args(args):
    """✂️ Splits a call's argument text at its top-level commas."""
    parts, depth, start, index = [], 0, 0, 0
    while index < len(args):
        char = args[index]
        if char == '"':
            index += 1
            while index < len(args) and args[index] != '"':
                index += 2 if args[index] == "\\" else 1
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            parts.append(args[start:index])
            start = index + 1
        index += 1
    parts.append(args[start:])
    return parts


def text_errors(text):
    """🏷️ The 2-argument `TextError::new(message, span)` gains the refusal kind the 3-argument constructor requires."""
    out, at = [], 0
    for match in re.finditer(r"TextError::new\(", text):
        if match.start() < at:
            continue
        open_index = match.end() - 1
        close = balanced(text, open_index)
        args = text[open_index + 1:close - 1]
        if len(top_level_args(args)) == 2 and "ValueRefusalKind" not in args:
            out.append(text[at:open_index + 1] + "semio_framework_value::ValueRefusalKind::InvalidValue, " + args + ")")
            at = close
    out.append(text[at:])
    return "".join(out)


def migrate(text):
    for prefix in ("dsl::json::", "store::json::"):
        text = with_policy(text, prefix + "from_json_str")
        text = with_policy(text, prefix + "parse")
        text = text.replace(prefix, "semio_framework_pack_json::")
    if re.search(r"^\s*use (dsl|store)::json;", text, re.M):
        text = re.sub(r"^(\s*)use (dsl|store)::json;", r"\1use semio_framework_pack_json as json;", text, flags=re.M)
    if re.search(r"^\s*use semio_framework_pack_json as json;", text, re.M):
        text = with_policy(text, "json::from_json_str")
        text = with_policy(text, "json::parse")
    text = re.sub(r"\buse (dsl|store)::json::", "use semio_framework_pack_json::", text)
    text = re.sub(r"\b(protocol|dsl|store)::ValueError\b", "semio_framework_value::ValueError", text)
    text = re.sub(r"\b(?:protocol|dsl|store)::(DslValue|ToValue|FromValue)\b", r"semio_framework_value::\1", text)
    text = re.sub(r"\b(?:semio_framework_plugin|protocol|dsl|store)::(Terminology|Locale)\b", r"semio_framework_ui_locale::\1", text)
    text = re.sub(r"(?<![\w])(?:::)?dsl::(parse_grammar|parse_protocol|print_grammar|print_protocol|verify_protocol_source|walk_protocol|SemioDialect|GrammarFile|ProtocolFile)\b", r"semio_framework_dsl::\1", text)
    text = io_errors(text)
    text = text_errors(text)
    return text


SQLITE_HEADER = "use semio_framework_value::{ValueError,ValueRefusalKind};\nfn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}\nfn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}\n"


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
            out.append(text[at:last_comma + 1] + "ValueError")
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


def message(arg):
    arg = arg.strip()
    for suffix in (".into()", ".to_string()", ".to_owned()"):
        if arg.endswith(suffix) and (arg.startswith('"') or arg.startswith("format!")):
            arg = arg[:-len(suffix)]
    return arg if arg.startswith('"') or arg.startswith("format!") else None


def migrate_sqlite(text):
    text = string_results(text)
    text = wrap_calls(text, "Err(", lambda arg: None if message(arg) is None else f"Err(invalid({message(arg)}))")
    text = wrap_calls(text, ".ok_or(", lambda arg: None if message(arg) is None else f".ok_or_else(||invalid({message(arg)}))")
    text = re.sub(r'\.map_err\(\|_\|("(?:[^"\\]|\\.)*")(?:\.into\(\)|\.to_string\(\))?\)', r".map_err(|_|invalid(\1))", text)
    text = re.sub(r"\.map_err\(\|(\w+)\|\1\.to_string\(\)\)", r".map_err(|\1|invalid(\1.to_string()))", text)
    text = re.sub(r"(__dsl_(?:from_record|to_record)_controlled\([^()]*\))(?!\.map_err)", r"\1.map_err(positioned)", text)
    if "fn invalid(" not in text:
        lines = text.split("\n")
        index = max(i for i, line in enumerate(lines[:40]) if line.startswith("use ")) + 1
        text = "\n".join(lines[:index]) + "\n" + SQLITE_HEADER.rstrip("\n") + "\n" + "\n".join(lines[index:])
    return text


SQLITE_TARGETS = []  # 🪶️ since 04:05 the sqlite-snapshot ABI files belong to S4-INFRA (coordinator rule 41); the stage stays for reference only.


LEDGER = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s4-tools-a/fallout-ledger.txt"


def main():
    now = time.time()
    changed, skipped = [], []
    mine = set(open(LEDGER, encoding="utf-8").read().split("\n")) if os.path.exists(LEDGER) else set()
    for root in ROOTS:
        for directory, dirs, files in os.walk(root):
            dirs[:] = [d for d in dirs if d not in ("node_modules", "dist", "target")]
            for name in files:
                if name != "🦀️.rs":
                    continue
                path = os.path.join(directory, name)
                text = open(path, encoding="utf-8").read()
                after = migrate(text)
                if any(path.endswith(target) for target in SQLITE_TARGETS):
                    after = migrate_sqlite(after)
                if after == text:
                    continue
                if now - os.path.getmtime(path) < QUIET_SECONDS and path not in mine and not any(path.endswith(own[6:]) for own in OWN):
                    skipped.append(path)
                    continue
                changed.append(path)
                if APPLY:
                    open(path, "w", encoding="utf-8").write(after)
    if APPLY and changed:
        with open(LEDGER, "a", encoding="utf-8") as ledger:
            ledger.write("".join(f"{path}\n" for path in changed))
    for path in skipped:
        print(f"[s4-tools-a] skipped (touched < 30 min ago) {os.path.relpath(path, REPO)}")
    print(f"[s4-tools-a] {len(changed)} files {'migrated' if APPLY else 'pending'}, {len(skipped)} skipped")


if __name__ == "__main__":
    main()

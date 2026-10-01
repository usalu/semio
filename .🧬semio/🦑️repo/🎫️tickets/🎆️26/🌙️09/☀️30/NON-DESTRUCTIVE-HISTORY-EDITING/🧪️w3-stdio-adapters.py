#!/usr/bin/env python3
"""🧾️ W3-STDIO-CASES item 6: moves every stdio Rust caller of a retired per-aggregate bridge onto the shared ones. A case
adapter decodes a row through `law::wire_operation(kind, &params, mutation_from_payload_json, mutation_payload_json)` (the
row must BE the leaf wire, `params_are_wire`) and undoes through `mutation_inverse(&operation, &base)`; a crate-internal
unit test calls `crate::mutation_from_payload_json` / `crate::mutation_inverse`. Bridge names and argument orders are
read from `HEAD`, where the bridges still exist. `🧿️semio` is excluded. Idempotent.
Usage: `python3 🧪️w3-stdio-adapters.py [--dry-run]`."""

import re
import subprocess
import sys

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
DRY = "--dry-run" in sys.argv
LAW = "semio_s_plugin_stdio_test_oracle::law"


def git_lines(*args: str) -> list[str]:
    return subprocess.run(["git", *args], capture_output=True, text=True, check=False).stdout.splitlines()


def bridge_orders() -> tuple[set[str], dict[str, bool]]:
    decoders, inverses = set(), {}
    for line in git_lines("grep", "-hoE", r"pub fn (decode_[a-z0-9_]+_mutation_payload(_json)?|inverse_[a-z0-9_]+_mutation)\(([a-z_]+)", "HEAD", "--", ROOT):
        match = re.match(r"pub fn ([a-z0-9_]+)\(([a-z_]+)", line)
        name, first = match.group(1), match.group(2)
        if name.startswith("decode_"):
            decoders.add(name)
        else:
            inverses[name] = first == "base"
    return decoders, inverses


def split_args(text: str, start: int) -> tuple[list[str], int]:
    depth, args, current, index = 0, [], "", start
    while index < len(text):
        char = text[index]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            if depth == 0:
                args.append(current.strip())
                return args, index
            depth -= 1
        if char == "," and depth == 0:
            args.append(current.strip())
            current = ""
        else:
            current += char
        index += 1
    raise ValueError("unbalanced call")


def params_json(payload: str) -> str:
    for pattern in (r"^&(\w+)\.get\(\"params\"\)\.cloned\(\)\.unwrap_or\(Json::Null\)\.to_string\(\)$", r"^&(\w+)\.get\(\"params\"\)\.map_or_else\(\|\| \"null\"\.to_string\(\), Json::to_string\)$"):
        match = re.match(pattern, payload)
        if match:
            return f'&{match.group(1)}.get("params").cloned().unwrap_or(Json::Null)'
    raise ValueError(f"unknown payload expression {payload}")


def rewrite_calls(text: str, names: set[str], render) -> str:
    pattern = re.compile(r"\b(" + "|".join(sorted(names, key=len, reverse=True)) + r")\(")
    out, cursor = "", 0
    for match in pattern.finditer(text):
        if match.start() < cursor or text[max(0, match.start() - 7):match.start()].endswith("pub fn "):
            continue
        args, close = split_args(text, match.end())
        out += text[cursor:match.start()] + render(match.group(1), args)
        cursor = close + 1
    return out + text[cursor:]


def drop_from_use_lists(text: str, names: set[str]) -> str:
    def braces(match: re.Match) -> str:
        items = [item.strip() for item in match.group(3).split(",") if item.strip()]
        kept = [item for item in items if item not in names]
        if kept == items:
            return match.group(0)
        if not kept:
            return ""
        body = f"{{{', '.join(kept)}}}" if len(kept) > 1 else kept[0]
        return f"{match.group(1)}use {match.group(2)}::{body};\n"
    return re.sub(r"([ \t]*)use ([A-Za-z0-9_:]+)::\{([^{}]*)\};\n", braces, text)


def artifact_crate(text: str) -> str:
    return re.search(r"\b(semio_s_artifact_stdio_[a-z0-9_]+)::", text).group(1)


def add_import(text: str, line: str, near: str) -> str:
    if line.strip() in text:
        return text
    match = re.search(rf"^([ \t]*)use {re.escape(near)}[^\n]*\n", text, re.M)
    return text[:match.end()] + f"{match.group(1)}{line}\n" + text[match.end():]


def convert(path: str, decoders: set[str], inverses: dict[str, bool]) -> None:
    original = open(path, encoding="utf-8").read()
    used = {name for name in decoders | set(inverses) if re.search(rf"\b{name}\(", original)}
    if not used:
        return
    adapter = '#[cfg(feature = "sut")]' in original
    crate = artifact_crate(original) if adapter else "crate"
    prefix = "" if adapter else "crate::"

    def render(name: str, args: list[str]) -> str:
        if name in decoders:
            if adapter:
                return f"wire_operation({args[0]}, {params_json(args[1])}, mutation_from_payload_json, mutation_payload_json)"
            return f"{prefix}mutation_from_payload_json({args[0]}, {args[1]})"
        operation, base = (args[1], args[0]) if inverses[name] else (args[0], args[1])
        return f"{prefix}mutation_inverse({operation}, {base})"

    text = drop_from_use_lists(rewrite_calls(original, used, render), used)
    if adapter:
        anchor = crate
        needed = [name for name in ("mutation_from_payload_json", "mutation_inverse", "mutation_payload_json") if re.search(rf"\b{name}\b", text)]
        if needed:
            text = add_import(text, f"use {crate}::{{{', '.join(needed)}}};" if len(needed) > 1 else f"use {crate}::{needed[0]};", anchor)
        if "wire_operation(" in text:
            text = add_import(text, f"use {LAW}::wire_operation;", anchor)
    if text != original:
        print(f"[w3-stdio] {'would convert' if DRY else 'convert'} {path}")
        if not DRY:
            open(path, "w", encoding="utf-8").write(text)


def main() -> None:
    decoders, inverses = bridge_orders()
    pattern = "|".join(sorted(decoders | set(inverses)))
    for path in git_lines("grep", "-lE", f"({pattern})\\(", "--", ROOT):
        if "🧿️semio" in path or re.search(r"/🧬️mutations/🦀️\.rs$|/⚙️operations/🦀️\.rs$", path):
            continue
        convert(path, decoders, inverses)


if __name__ == "__main__":
    main()

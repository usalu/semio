"""🧯️ Moves one hand-written controlled codec from prose `String` refusals onto typed `ValueError`.

Usage: python3 🧪️s3-graphs-value-error-sweep.py <file> [--keep fn,fn] [--wrap fn,fn]

- every literal refusal becomes `ValueError::new(ValueRefusalKind::<kind>, "...")` with the kind
  derived from its prose (rows/work → WorkLimit, bytes/ownership → OwnershipLimit, …);
- every `Result<T,String>` becomes `Result<T,ValueError>` outside `--keep` functions;
- `--keep` functions keep their `String` trait boundary and are left untouched;
- `--wrap` functions keep their `String` trait boundary but run their body as a typed closure
  mapped through `ValueError::into_message`.
"""
import re
import sys

OWNERSHIP = ("byte", "size", "budget", "ownership", "allocation overflow", "lookup", "frontier overflow")


def kind(message: str) -> str:
    lowered = message.lower()
    if "allocation failed" in lowered:
        return "AllocationFailed"
    if "row" in lowered and ("limit" in lowered or "overflow" in lowered):
        return "WorkLimit"
    if "overflow" in lowered:
        return "OwnershipLimit" if any(word in lowered for word in OWNERSHIP) else "WorkLimit"
    if "limit exceeded" in lowered:
        return "OwnershipLimit" if any(word in lowered for word in OWNERSHIP) else "WorkLimit"
    return "InvalidValue"


def refusal(message: str) -> str:
    return f'ValueError::new(ValueRefusalKind::{kind(message)},"{message}")'


def skip_string(text: str, index: int) -> int:
    index += 1
    while text[index] != '"':
        index += 2 if text[index] == "\\" else 1
    return index + 1


def matching(text: str, start: int, open_char: str, close_char: str) -> int:
    depth, index = 0, start
    while index < len(text):
        char = text[index]
        if char == '"':
            index = skip_string(text, index)
            continue
        if char == open_char:
            depth += 1
        elif char == close_char:
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError(f"unbalanced {open_char} at {start}")


def function_spans(text: str, names: set[str]) -> list[tuple[str, int, int, int]]:
    spans = []
    for match in re.finditer(r"\bfn ([a-z_0-9]+)\b", text):
        if match.group(1) not in names:
            continue
        params = text.index("(", match.end())
        close = matching(text, params, "(", ")")
        body = text.index("{", close)
        end = matching(text, body, "{", "}")
        spans.append((match.group(1), match.start(), body, end))
    return spans


def result_string_types(text: str) -> list[tuple[int, int]]:
    found = []
    for match in re.finditer(r"Result<", text):
        start = match.end() - 1
        depth, index, last_comma = 0, start, None
        while index < len(text):
            char = text[index]
            if char == "<":
                depth += 1
            elif char == ">" and text[index - 1] != "-":
                depth -= 1
                if depth == 0:
                    break
            elif char == "," and depth == 1:
                last_comma = index
            index += 1
        if last_comma is not None and text[last_comma + 1:index].strip() == "String":
            found.append((last_comma + 1, index))
    return found


def convert(segment: str) -> str:
    segment = re.sub(r'\.ok_or\("([^"]*)"\)', lambda m: f".ok_or_else(||{refusal(m.group(1))})", segment)
    segment = re.sub(r'\.ok_or_else\(\|\|"([^"]*)"\.(?:into|to_string)\(\)\)', lambda m: f".ok_or_else(||{refusal(m.group(1))})", segment)
    segment = re.sub(r'Err\("([^"]*)"\.into\(\)\)', lambda m: f"Err({refusal(m.group(1))})", segment)
    segment = re.sub(r'\.map_err\(\|_\|"([^"]*)"(?:\.into\(\))?\)', lambda m: f".map_err(|_|{refusal(m.group(1))})", segment)
    segment = re.sub(r"\.map_err\(\|(e|error)\|\1\.to_string\(\)\)", lambda m: f".map_err(|{m.group(1)}|ValueError::new(ValueRefusalKind::InvalidValue,{m.group(1)}.to_string()))", segment)
    segment = re.sub(r"Ok::<(\(\)|_),String>", r"Ok::<\1,ValueError>", segment)
    for start, end in reversed(result_string_types(segment)):
        segment = segment[:start] + "ValueError" + segment[end:]
    return segment


def main() -> None:
    path, args = sys.argv[1], sys.argv[2:]
    options = {"--keep": set(), "--wrap": set()}
    for flag, value in zip(args[::2], args[1::2]):
        options[flag] = set(filter(None, value.split(",")))
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    spans = sorted(function_spans(text, options["--keep"] | options["--wrap"]), key=lambda span: span[1])
    pieces, cursor = [], 0
    for name, start, body, end in spans:
        pieces.append(convert(text[cursor:start]))
        if name in options["--keep"]:
            pieces.append(text[start:end + 1])
        else:
            signature = text[start:body]
            returned = re.search(r"->\s*Result<(.*),\s*String>\s*$", signature)
            if returned is None:
                raise SystemExit(f"{name}: wrapped function must return Result<_,String>")
            inner = convert(text[body + 1:end])
            pieces.append(f"{signature}{{(||->Result<{returned.group(1)},ValueError>{{{inner}}})().map_err(ValueError::into_message)}}")
        cursor = end + 1
    pieces.append(convert(text[cursor:]))
    result = "".join(pieces)
    if "use semio_framework_value::{ValueError,ValueRefusalKind};" not in result:
        lines = result.split("\n")
        at = next(index for index, line in enumerate(lines) if line.startswith("use "))
        lines.insert(at, "use semio_framework_value::{ValueError,ValueRefusalKind};")
        result = "\n".join(lines)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(result)
    print(f"{path}: {result.count('ValueError::new(')} typed refusals, {len(spans)} boundary functions")


if __name__ == "__main__":
    main()

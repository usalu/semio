#!/usr/bin/env python3
"""🧯️ S4-TEXT (session 4): moves the S4-TEXT trees (writer, vcs, trinity, stdio deflate/md/html/binary) off the retired
`PackError::Schema` / `PackError::{ValueRefusal,TextRefusal,into_value_error}` onto the canonical pack-error API, following the
S4-PACKFIX mapping table (`📓️s4-packfix-report.md` § Schema Decisions): envelope build/unwrap keeps the SemioError kind via
`into_value_error()`, identity mismatches and empty-state/content refusals are `InvalidValue`, UTF-8 errors use
`ValueError::from`, controlled codecs use `PackRefusal::into_value_error`. Every rule asserts its exact per-root count; all files
are staged, then written together (`--check` = dry run). Run from the repo root."""
import re
import sys
from pathlib import Path

P = Path("✏️s/🔌️plugins")
ROOTS = {
    "writer": P / "✒️writer",
    "vcs": P / "🌿️vcs",
    "jack": P / "🔱️trinity/🗿️artifacts/🔌️jack",
    "rewriting": P / "🔱️trinity/🗿️artifacts/♻️rewriting",
    "deflate": P / "🗄️stdio/🗿️artifacts/🗜️deflate",
    "md": P / "🗄️stdio/🗿️artifacts/📝️md",
    "html": P / "🗄️stdio/🗿️artifacts/🌐️html",
    "binary": P / "🗄️stdio/🗿️artifacts/💾️binary",
}
EXPECTED = {
    "envelope": {"writer": 6, "vcs": 5, "jack": 8, "rewriting": 4, "deflate": 2, "md": 2},
    "err": {"writer": 3, "vcs": 4, "jack": 4, "rewriting": 2, "deflate": 1, "md": 1},
    "bare": {"deflate": 3, "md": 1},
    "value-refusal-fn": {"jack": 1},
    "controlled-into": {"rewriting": 1},
    "html-utf8": {"html": 1},
    "html-value-refusal": {"html": 3},
    "html-text-refusal": {"html": 1},
}
VE, VK = "semio_framework_value::ValueError", "semio_framework_value::ValueRefusalKind"
ENVELOPE = re.compile(r"(semio_format::(?:SemioEnvelope::from_envelope_id|unwrap_binary)\((?:[^()]|\([^()]*\))*\))\.map_err\(\|(\w+)\|(\s*)((?:store::)?)PackError::Schema\(\2\.to_string\(\)\)\)")
BARE = re.compile(r"\.map_err\(((?:store::)?)PackError::Schema\)")
ERR = re.compile(r"Err\(((?:store::)?)PackError::Schema\(")
HTML_UTF8_OLD = f"map_err(|e| store::PackError::ValueRefusal({VE}::new({VK}::InvalidValue,e.to_string())))"
HTML_UTF8_NEW = f"map_err(|e| store::PackError::from({VE}::from(e)))"


def balanced_end(text, start):
    depth, index, quoted = 1, start, False
    while depth:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
        index += 1
    return index - 1


def rewrite_err(text):
    out, cursor, count = [], 0, 0
    for match in ERR.finditer(text):
        if match.start() < cursor:
            continue
        close = balanced_end(text, match.end())
        argument = text[match.end():close].strip()
        literal = re.fullmatch(r'("(?:[^"\\]|\\.)*")\.into\(\)', argument)
        argument = literal.group(1) if literal else argument
        out.append(text[cursor:match.start()])
        out.append(f"Err({match.group(1)}PackError::from({VE}::new({VK}::InvalidValue,{argument}))")
        cursor, count = close + 1, count + 1
    out.append(text[cursor:])
    return "".join(out), count


tally = {rule: {} for rule in EXPECTED}
staged = {}
for name, root in ROOTS.items():
    for path in sorted(root.rglob("🦀️.rs")):
        original = path.read_text()
        text = original
        text, found = ENVELOPE.subn(lambda m: f"{m.group(1)}.map_err(|{m.group(2)}|{m.group(3)}{m.group(4)}PackError::from({m.group(2)}.into_value_error()))", text)
        tally["envelope"][name] = tally["envelope"].get(name, 0) + found
        text, found = rewrite_err(text)
        tally["err"][name] = tally["err"].get(name, 0) + found
        text, found = BARE.subn(lambda m: f".map_err(|error|{m.group(1)}PackError::from({VE}::new({VK}::InvalidValue,error)))", text)
        tally["bare"][name] = tally["bare"].get(name, 0) + found
        for rule, old, new in (
            ("value-refusal-fn", ".map_err(PackError::ValueRefusal)", ".map_err(PackError::from)"),
            ("controlled-into", ".map_err(store::PackError::into_value_error)", ".map_err(store::PackRefusal::into_value_error)"),
            ("html-utf8", HTML_UTF8_OLD, HTML_UTF8_NEW),
            ("html-value-refusal", "store::PackError::ValueRefusal(", "store::PackError::from("),
            ("html-text-refusal", ".map_err(store::PackError::TextRefusal)", ".map_err(store::PackError::from)"),
        ):
            found = text.count(old)
            tally[rule][name] = tally[rule].get(name, 0) + found
            text = text.replace(old, new)
        if text != original:
            staged[path] = text
for rule, expected in EXPECTED.items():
    actual = {name: count for name, count in tally[rule].items() if count}
    if actual != expected:
        sys.exit(f"{rule}: expected {expected}, found {actual}")
leftover = [f"{path}: {m}" for path, text in staged.items() for m in re.findall(r"PackError::(?:Schema|ValueRefusal|TextRefusal|into_value_error)", text)]
if leftover:
    sys.exit("leftover retired PackError forms:\n" + "\n".join(leftover))
if "--check" not in sys.argv:
    for path, text in staged.items():
        path.write_text(text)
print(f"pack-error sweep: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}")

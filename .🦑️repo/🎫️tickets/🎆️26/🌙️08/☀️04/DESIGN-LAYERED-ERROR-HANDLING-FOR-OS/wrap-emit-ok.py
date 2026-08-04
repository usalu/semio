#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
SCAN = [ROOT / "✏️s/🔌️plugin", ROOT / "🧰️framework/🛍️product/💻️os"]


def needs_wrap(text: str) -> bool:
    return "Result<Emit<" in text and ", Fault>" in text


def fix_emit_calls(text: str) -> str:
    text = re.sub(r"\breturn Emit::", "return Ok(Emit::", text)
    for meth in ("config", "operations", "effect", "default", "amend", "commit", "event"):
        text = re.sub(rf"(?<!Ok\()Emit::{meth}\(", f"Ok(Emit::{meth}(", text)
    text = re.sub(r"(?<!Ok\()Emit::config\(", "Ok(Emit::config(", text)
    return text


def fix_emit_blocks(text: str) -> str:
    lines = text.splitlines(keepends=True)
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if re.match(r"^(\s+)Emit \{\s*$", line) and (i == 0 or "Ok(Emit" not in lines[i - 1]):
            indent = re.match(r"^(\s+)Emit \{\s*$", line).group(1)
            out.append(f"{indent}Ok(Emit {{\n")
            i += 1
            depth = 1
            while i < len(lines) and depth > 0:
                cur = lines[i]
                depth += cur.count("{") - cur.count("}")
                if depth == 0:
                    if cur.rstrip().endswith("}"):
                        out.append(cur.rstrip()[:-1] + "})\n")
                    else:
                        out.append(cur)
                        out.append(cur.rstrip()[:-1] + "})\n")
                    i += 1
                    break
                out.append(cur)
                i += 1
            continue
        out.append(line)
        i += 1
    return "".join(out)


def wrap_handle_generation_calls(text: str) -> str:
    return re.sub(
        r"=> handle_generation\(",
        "=> Ok(handle_generation(",
        text,
    )


def fix_double_ok(text: str) -> str:
    text = text.replace("Ok(Ok(", "Ok(")
    text = text.replace("return Ok(Ok(", "return Ok(")
    return text


changed = []
for base in SCAN:
    if not base.exists():
        continue
    for path in base.rglob("📦️lib.rs"):
        text = path.read_text()
        if not needs_wrap(text):
            continue
        orig = text
        text = fix_emit_calls(text)
        text = fix_emit_blocks(text)
        text = wrap_handle_generation_calls(text)
        text = fix_double_ok(text)
        if text != orig:
            path.write_text(text)
            changed.append(path)

print(f"wrapped emit in {len(changed)} files")

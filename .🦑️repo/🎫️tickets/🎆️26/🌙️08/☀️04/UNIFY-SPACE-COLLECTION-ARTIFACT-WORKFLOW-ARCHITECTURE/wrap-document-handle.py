#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SIG = re.compile(r"fn handle\((&self, command: &[^)]+\)) -> Emit<([^>]+)>")


def wrap_handle_body(body: str) -> str:
    lines = body.splitlines(keepends=True)
    for idx, line in enumerate(lines):
        if line.startswith("        match "):
            lines[idx] = line.replace("match ", "Ok(match ", 1)
            depth = 0
            for j in range(idx, len(lines)):
                depth += lines[j].count("{") - lines[j].count("}")
                if depth == 0 and j > idx:
                    close = lines[j]
                    if close.strip() == "}":
                        lines[j] = "        )\n" + close
                    break
            break
    return "".join(lines)


def transform(content: str) -> str:
    if "fn handle(&self, command:" not in content or "-> Emit<" not in content:
        return content

    def repl(match: re.Match[str]) -> str:
        return f"fn handle({match.group(1)}) -> Result<Emit<{match.group(2)}>, Fault>"

    content = SIG.sub(repl, content)

    parts = content.split("fn handle(&self, command:")
    if len(parts) < 2:
        return content
    rebuilt = [parts[0]]
    for chunk in parts[1:]:
        fn_chunk = "fn handle(&self, command:" + chunk
        # only transform first handle in chunk until next `    fn `
        m = re.search(r"^(\s+fn handle[^\{]+\{)(.*?)(^\s+fn \w+)", fn_chunk, re.MULTILINE | re.DOTALL)
        if not m:
            rebuilt.append(fn_chunk)
            continue
        head, body, tail = m.group(1), m.group(2), m.group(3)
        if "Ok(match " not in body and "match " in body:
            body = wrap_handle_body(body)
        rebuilt.append(head + body + tail + fn_chunk[m.end() :])
    return "".join(rebuilt)


def main() -> None:
    n = 0
    for path in ROOT.rglob("*.rs"):
        if ".🦑️repo" in str(path):
            continue
        text = path.read_text(encoding="utf-8")
        if "fn handle(&self, command:" not in text:
            continue
        updated = transform(text)
        if updated != text:
            path.write_text(updated, encoding="utf-8")
            n += 1
            print(path)
    print("files", n)


if __name__ == "__main__":
    main()

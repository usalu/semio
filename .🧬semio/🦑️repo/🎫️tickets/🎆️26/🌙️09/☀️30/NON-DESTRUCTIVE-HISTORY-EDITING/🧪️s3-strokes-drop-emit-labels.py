"""🏷️ Strips hand-written `(leaf, "label")` tuple labels from a plugin editor's command→leaf match (design §20.4, G7 gate
`schema mutation-labels`): every `(EXPR, format!("…"))` / `(EXPR, "…".to_string())` / `(EXPR, "…".into())` tuple becomes
`EXPR`, leaving the leaf's own `SemanticMutation::label`. Usage: `python3 🧪️s3-strokes-drop-emit-labels.py <file>`; prints
the count of rewritten tuples (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, S3-STROKES)."""
import re
import sys

LABEL = re.compile(r'\s*,\s*(?:format!\("(?:[^"\\]|\\.)*"(?:\s*,(?:[^()]|\([^()]*\))*)?\)|"(?:[^"\\]|\\.)*"\.(?:to_string|into)\(\))\s*\)')


def top_level_comma(text: str) -> bool:
    depth = 0
    for char in text:
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            return True
    return False


def strip(text: str) -> tuple[str, int]:
    out, count, index = [], 0, 0
    while True:
        match = LABEL.search(text, index)
        if not match:
            out.append(text[index:])
            return "".join(out), count
        depth, start = 0, match.start() - 1
        while start >= 0:
            char = text[start]
            if char == ")" or char == "]" or char == "}":
                depth += 1
            elif char in "([{":
                if depth == 0:
                    break
                depth -= 1
            start -= 1
        inner = text[start + 1:match.start()]
        if start < 0 or text[start] != "(" or top_level_comma(inner) or not re.match(r"\s*[a-z_][a-z_0-9]*\(|\s*Self::", inner) or re.search(r"[A-Za-z_]\s*$", text[:start]):
            out.append(text[index:match.end()])
            index = match.end()
            continue
        out.append(text[index:start])
        out.append(text[start + 1:match.start()].strip())
        index = match.end()
        count += 1


path = sys.argv[1]
source = open(path, encoding="utf-8").read()
result, count = strip(source)
open(path, "w", encoding="utf-8").write(result)
print(count)

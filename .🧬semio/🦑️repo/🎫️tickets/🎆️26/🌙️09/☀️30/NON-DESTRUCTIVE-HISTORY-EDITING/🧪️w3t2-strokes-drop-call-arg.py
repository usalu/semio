"""🧹️ W3-T2-STROKES codemod: drops the trailing argument of every call to one function inside one directory tree —
`python3 <this> <root> <function> <arity>` rewrites `function(a, b, c)` with exactly `arity` top-level arguments to
`function(a, b)`. Brackets, braces, parentheses and string literals are respected; nothing else is touched.
"""

import os
import sys


def split_args(text):
    args, depth, current, quote = [], 0, [], None
    index = 0
    while index < len(text):
        char = text[index]
        if quote:
            current.append(char)
            if char == "\\":
                current.append(text[index + 1])
                index += 2
                continue
            if char == quote:
                quote = None
        elif char == '"':
            quote = char
            current.append(char)
        elif char in "([{":
            depth += 1
            current.append(char)
        elif char in ")]}":
            depth -= 1
            current.append(char)
        elif char == "," and depth == 0:
            args.append("".join(current))
            current = []
        else:
            current.append(char)
        index += 1
    if "".join(current).strip():
        args.append("".join(current))
    return args


def closing(text, start):
    depth, quote, index = 0, None, start
    while index < len(text):
        char = text[index]
        if quote:
            if char == "\\":
                index += 2
                continue
            if char == quote:
                quote = None
        elif char == '"':
            quote = char
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError("unbalanced call")


def rewrite(text, function, arity):
    needle, out, cursor, count = function + "(", [], 0, 0
    while True:
        found = text.find(needle, cursor)
        if found < 0:
            out.append(text[cursor:])
            return "".join(out), count
        if text[max(0, found - 7):found].endswith("pub fn ") or (found > 0 and (text[found - 1].isalnum() or text[found - 1] == "_")):
            out.append(text[cursor:found + len(needle)])
            cursor = found + len(needle)
            continue
        open_index = found + len(function)
        close_index = closing(text, open_index)
        args = split_args(text[open_index + 1:close_index])
        if len(args) == arity:
            kept = ",".join(args[:-1]).rstrip()
            out.append(text[cursor:open_index + 1] + kept + ")")
            count += 1
        else:
            out.append(text[cursor:close_index + 1])
        cursor = close_index + 1


def main():
    root, function, arity = sys.argv[1], sys.argv[2], int(sys.argv[3])
    for directory, _, files in os.walk(root):
        if "node_modules" in directory:
            continue
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(directory, name)
            with open(path, encoding="utf-8") as handle:
                text = handle.read()
            if function + "(" not in text:
                continue
            rewritten, count = rewrite(text, function, arity)
            if count:
                with open(path, "w", encoding="utf-8") as handle:
                    handle.write(rewritten)
                print(count, path)


if __name__ == "__main__":
    main()

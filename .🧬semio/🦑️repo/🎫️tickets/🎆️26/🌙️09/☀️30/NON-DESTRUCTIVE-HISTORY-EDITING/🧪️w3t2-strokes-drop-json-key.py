"""🧹️ W3-T2-STROKES fixture codemod: removes every `"<key>": <scalar>` LINE from pretty-printed JSON files under one
root (fixing the trailing comma of the line before when the removed line closed its object), then proves each file
still parses and no longer carries the key. `python3 <this> <root> <key>`. Formatting of every other line is kept
byte for byte, so committed canonical fixtures stay canonical.
"""

import json
import os
import re
import sys


def main():
    root, key = sys.argv[1], sys.argv[2]
    pattern = re.compile(r'^\s*"' + re.escape(key) + r'": (null|true|false|-?\d+(\.\d+)?|"[^"]*"),?\s*$')
    for directory, _, files in os.walk(root):
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(directory, name)
            with open(path, encoding="utf-8") as handle:
                text = handle.read()
            if '"' + key + '"' not in text:
                continue
            lines = text.split("\n")
            out = []
            for line in lines:
                if pattern.match(line):
                    continue
                if out and re.match(r"^\s*[}\]]", line) and out[-1].rstrip().endswith(","):
                    out[-1] = out[-1].rstrip()[:-1]
                out.append(line)
            rewritten = "\n".join(out)
            value = json.loads(rewritten)
            if ('"' + key + '"') in rewritten:
                raise SystemExit("still carries " + key + ": " + path)
            json.dumps(value)
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(rewritten)
            print("fixed", path)


if __name__ == "__main__":
    main()

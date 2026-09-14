#!/usr/bin/env python3
"""🔧️ Removes the double `{ projection: … }` wrapper from print test adapters.

`subject()` helpers already return `{ projection }` (INTEGRATION-2 item 20), but every call site
still wraps the result once more, so the host compared `{projection:{projection:…}}` with the
oracle's `{projection:…}` and every probe subject reported a difference. Idempotent.
"""
import os
import re
import sys

ROOT = "🧰️framework/🛍️products/📓️print/🧪️tests"
HELPERS = "subject|affineSubject|powerSubject|probe|metrics|order|records"
PATTERN = re.compile(rf"\(\{{\s*projection:\s*(await (?:{HELPERS})\((?:[^()]|\([^()]*\))*\))\s*,?\s*\}}\)", re.S)


def main() -> int:
    os.chdir(sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio")
    changed = 0
    for case in sorted(os.listdir(ROOT)):
        path = os.path.join(ROOT, case, "🟦️.ts")
        if not os.path.isfile(path):
            continue
        with open(path, encoding="utf8") as handle:
            text = handle.read()
        replaced, count = PATTERN.subn(lambda m: "(" + m.group(1) + ")", text)
        if count:
            with open(path, "w", encoding="utf8", newline="\n") as handle:
                handle.write(replaced)
            print(f"{case}: {count}")
            changed += count
    print(f"total {changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

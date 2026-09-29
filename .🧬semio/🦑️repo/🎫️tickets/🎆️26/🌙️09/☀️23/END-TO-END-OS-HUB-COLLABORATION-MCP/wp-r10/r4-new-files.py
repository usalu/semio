#!/usr/bin/env python3
"""🆕️ R10 session 15: the files every T6 round-4 set (L1 `w3-trains.json` T6R4) creates, read from the sets' live dry-run
captures (`s14-l1-logs/T6R4/*-dry.txt`) and their own path constants — no set script runs, nothing is written to the tree.

Prints one `<set>\t<repo path>` per file that does not exist on the live tree and writes the list as the
`--placeholder` arguments `taxonomy-planned.py` takes (`--out <file>`, one path per line).
usage: python3 r4-new-files.py --out <paths.txt>
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.tmp-ticket"
DRY = f"{REPO}/.🧬semio/🌐hub/s14-l1-logs/T6R4"
PATH = re.compile(r"((?:🧰️framework|✏️s|🌎️hub|🧑‍💻dev)/\S+)")


def capture_paths(name, prefix=""):
    text = open(f"{DRY}/{name}", encoding="utf-8").read()
    if prefix:
        return [prefix + match.group(1) for match in re.finditer(r"^dry (\S+) \d+$", text, re.M)]
    return [match.group(1).rstrip(":") for match in PATH.finditer(text)]


def constants(script, names):
    text = open(script, encoding="utf-8").read()
    scope = {"json": json}
    for line in text.splitlines():
        for name in names:
            if line.startswith(f"{name} = "):
                exec(line, scope)
    return scope


def main():
    out = sys.argv[sys.argv.index("--out") + 1]
    store = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/"
    sets = {
        "c13-p4-ephemeral": capture_paths("c13-p4-ephemeral-dry.txt"),
        "lb2-p2b-plugin-calls": capture_paths("lb2-p2b-plugin-calls-dry.txt"),
        "c12-hub-order": [store + "🧫️fixtures/🧭️hub-order/🔣️.json", store + "🧬️schema/🧭️hub-order/🔣️.json"],
        "sh2-p2-space-activity": capture_paths("sh2-p2-space-activity-dry.txt"),
        "u6-e1-paged-docx": list(constants(f"{TICKET}/wp-u6/u6-e1-paged-docx.py", ["NEWS"])["NEWS"]),
        "s18-draw-layer-status": capture_paths("s18-draw-layer-status-dry.txt", "✏️s/🔌️plugins/🖍️draw/"),
        "p9-jack-headless-query": capture_paths("p9-jack-headless-query-dry.txt"),
    }
    created = []
    for name, paths in sets.items():
        fresh = sorted({path for path in paths if not os.path.exists(os.path.join(REPO, path))})
        print(f"[r4-new-files] {name}: {len(set(paths))} files, {len(fresh)} new")
        for path in fresh:
            print(f"{name}\t{path}")
        created.extend(fresh)
    open(out, "w", encoding="utf-8").write("".join(f"{path}\n" for path in created))


main()

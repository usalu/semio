"""🧪️ W2-W-norm-3: reads the `[DEBUG] DUMP <example> <json>` lines the temporary Rust dump tests print, keyed by crate.

`python3 🧪️w2-w-norm-3-dump.py <captured cargo output> <out dir>` writes `<out dir>/<artifact>/<example>.json` (pretty, canonical
float spelling) so the witness and vector generators can read real Rust-encoded example documents.
"""
import json
import os
import re
import sys

SECTION = re.compile(r"Running unittests .*?semio_s_artifact_norm_(en\d+)-[0-9a-f]+\)")
DUMP = re.compile(r"^\[DEBUG\] DUMP (\S+) (.*)$")


def main(source, target):
    artifact = None
    written = 0
    for line in open(source, encoding="utf-8"):
        section = SECTION.search(line)
        if section:
            artifact = section.group(1)
            continue
        dump = DUMP.match(line.rstrip("\n"))
        if dump and artifact:
            os.makedirs(os.path.join(target, artifact), exist_ok=True)
            with open(os.path.join(target, artifact, dump.group(1) + ".json"), "w", encoding="utf-8") as handle:
                handle.write(json.dumps(json.loads(dump.group(2)), ensure_ascii=False, indent=2) + "\n")
            written += 1
    print("wrote", written, "documents")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])

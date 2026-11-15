"""🐍️ Independent oracle of the re-admission of a moved closure, written without sharing a line with the implementation (standard library only): given a root directory and the admitted list it prints the paths that differ from the disk or are gone, in byte order, bounded; given the policy and the number of torn attempts it prints the pauses that were slept and whether the task ends in the refusal. usage: 🐍️.py <root> <rows.json> <schema.json> <torn attempts before success>"""
import hashlib
import json
import os
import sys


def main():
    root, rows_file, schema_file, torn = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
    rows = json.load(open(rows_file, encoding="utf-8"))
    policy = json.load(open(schema_file, encoding="utf-8"))["definitions"]["Policy"]["const"]
    moved = []
    for row in rows:
        path = os.path.join(root, row["path"])
        try:
            digest = hashlib.sha256(open(path, "rb").read()).hexdigest()
        except OSError:
            moved.append(row["path"])
            continue
        if digest != row["sha256"]:
            moved.append(row["path"])
    moved.sort(key=lambda text: text.encode("utf-8"))
    attempts = policy["maximumAttempts"]
    pauses, refused, ran = [], False, 0
    for attempt in range(1, attempts + 1):
        ran += 1
        if attempt > torn:
            break
        if attempt == attempts:
            refused = True
            break
        pauses.append(policy["backoffMs"][attempt - 1])
    print(json.dumps({"moved": moved[: policy["maximumMovedFiles"]], "pauses": pauses, "refused": refused, "ran": ran}, ensure_ascii=False))


main()

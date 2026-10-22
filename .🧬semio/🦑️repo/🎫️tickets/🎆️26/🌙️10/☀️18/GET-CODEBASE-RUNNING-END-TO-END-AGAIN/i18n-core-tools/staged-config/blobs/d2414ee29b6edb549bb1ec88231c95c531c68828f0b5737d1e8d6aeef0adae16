#!/usr/bin/env python3
"""Independent oracle of the local config event log: folds the log texts of the corpus.

usage: oracle.py < corpus.json > result.json
It reads only the event schema and the corpus and shares no code with the TypeScript implementation.
"""
import json
import os
import re
import sys

SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json")
with open(SCHEMA, encoding="utf-8") as handle:
    DEFINITIONS = json.load(handle)["definitions"]

ID = re.compile(DEFINITIONS["Id"]["pattern"])
INSTANT = re.compile(DEFINITIONS["Instant"]["pattern"])
LANGUAGES = DEFINITIONS["LanguageChosen"]["properties"]["language"]["enum"]


def valid(event):
    if not isinstance(event, dict) or event.get("version") != 1 or isinstance(event.get("version"), bool):
        return False
    if not isinstance(event.get("id"), str) or ID.fullmatch(event["id"]) is None:
        return False
    if not isinstance(event.get("at"), str) or INSTANT.fullmatch(event["at"]) is None:
        return False
    if event.get("type") == "language-chosen":
        return set(event) == {"version", "id", "at", "type", "language"} and event["language"] in LANGUAGES
    return event.get("type") == "language-cleared" and set(event) == {"version", "id", "at", "type"}


def fold(text):
    lines = text.split("\n")
    torn = text != "" and not text.endswith("\n")
    language, count = None, 0
    for index, line in enumerate(lines):
        if line.strip() == "":
            continue
        try:
            event = json.loads(line)
        except ValueError:
            if torn and index == len(lines) - 1:
                continue
            return {"refusal": {"line": index + 1}}
        if not valid(event):
            return {"refusal": {"line": index + 1}}
        language = event["language"] if event["type"] == "language-chosen" else None
        count += 1
    return {"events": count, **({"language": language} if language else {})}


def main():
    corpus = json.load(sys.stdin)
    json.dump([{"name": row["name"], "result": fold(row["text"])} for row in corpus["logs"]], sys.stdout, ensure_ascii=False)


if __name__ == "__main__":
    main()

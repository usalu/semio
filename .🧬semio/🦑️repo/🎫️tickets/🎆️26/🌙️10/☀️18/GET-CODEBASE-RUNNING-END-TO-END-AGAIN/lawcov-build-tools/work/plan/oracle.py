"""📋️ Independent Python oracle of the pristine file disposition (standard library only): it decides the disposition of one observation, or the refusal, from the written rules, one decision table row at a time. Requests arrive as JSON on stdin."""
import json, sys


def decide(row):
    path = row["path"]
    if path.startswith("/") or "\\" in path or any(part in ("", ".", "..") for part in path.split("/")):
        return {"error": "Canonical pristine package path required"}
    if path.startswith("node_modules/"):
        return {"value": "nested"}
    if not row["present"]:
        return {"error": "Registry source must replace an existing regular file"}
    if row["shared"]:
        if row["archive"] and row["matches"] and row["links"] >= 2:
            return {"value": "preserve"}
        return {"error": "Exact shared binary registry bytes required"}
    if row["links"] != 1:
        return {"error": "Unknown shared package inode refused"}
    return {"value": "replace" if row["archive"] else "remove"}


def main():
    print(json.dumps([decide(row) for row in json.load(sys.stdin)["rows"]]))


main()

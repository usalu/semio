"""✏️ Structural edit of the graph memo schema for batch `memoedit`: the edit index (suffix, limits), the `edited` text in both languages, the optional `edit` member of a sealed record's relevance and of a sidecar header, and the `$comment` sentence that states the rule.
usage: python3 -I edit_schema.py <root>   (idempotent; the file keeps the json.dumps(indent=2) shape plus a final newline)"""
import json, os, sys
root = sys.argv[1]
path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧬️schema/🔣️.json")
value = json.load(open(path, encoding="utf8"))
policy = value["definitions"]["Policy"]["const"]
ordered = {}
for key, item in policy.items():
    ordered[key] = item
    if key == "stampSuffix": ordered["editSuffix"] = ".rix"
    if key == "maximumStampBytes":
        ordered["maximumEditBytes"] = 134217728
        ordered["maximumEditPaths"] = 256
        ordered["maximumEditRows"] = 4096
policy.clear(); policy.update(ordered)
policy["texts"]["en"]["edited"] = "graph: the memo re-checked the {rows} facts that read the {paths} changed paths with the planner's own checks; they all still hold"
policy["texts"]["de"]["edited"] = "graph: das Memo hat die {rows} Fakten, welche die {paths} geänderten Pfade lesen, mit den Prüfungen des Planers nachgeprüft; alle gelten noch"
edit = {"type": "object", "additionalProperties": False, "required": ["index", "frontier"], "properties": {"index": {"$ref": "#/definitions/Sha256"}, "frontier": {"anyOf": [{"$ref": "#/definitions/Sha256"}, {"type": "null"}]}}}
sealed = json.loads(json.dumps(edit)); sealed["required"] = ["id", "index", "frontier"]; sealed["properties"] = {"id": {"$ref": "#/definitions/Sha256"}, **sealed["properties"]}
sealed["$comment"] = "The edit index of the sealed relevance (`id` = sha256 of the `.rix` file) and the fact index and frontier of the generation it was computed from: `index` names the rows by their position in that fact index, so a lookup fetches exactly the rows it re-checks from the immutable generation store; a retired index makes the memo plan."
value["definitions"]["Record"]["properties"]["relevance"]["properties"]["edit"] = sealed
edit["$comment"] = "Written by the sealing child next to the relevance: `index` and `frontier` are the digests of the fact index and of the frontier of the served generation, `stem.rix` holds the edit index (the rows that name each entry, the rows that read the children of each directory, the directories with something relevant below them, the frontier directories and members)."
value["definitions"]["Sidecar"]["properties"]["edit"] = edit
sentence = " Edit refinement: when the journal names a relevant change, the memo does not give up at once. The edit index of the sealed sidecar maps every entry to the rows that name it and every directory to the rows that read its children; the lookup re-checks exactly those rows with the planner's own checks (an analysis is replayed and compared by its result, so an edit that keeps a file's imports and modules holds), proves a directory that has relevant entries below it to be the same directory (frontier inode), lists a frontier directory whose children changed again with the planner's own walk predicates and compares its subdirectories and members, refuses a touched path that became a symbolic link, a row it cannot check (a derived fact, a closure) and more than `maximumEditPaths` paths or `maximumEditRows` rows; when everything holds it serves and re-seals the stamps of the directories it listed again."
if sentence.strip() not in value["$comment"]: value["$comment"] += sentence
text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
open(path, "w", encoding="utf8").write(text)
print("[DEBUG] schema edited", len(text))

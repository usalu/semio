"""🧮 Structural edit of the graph memo schema for batch `derived`: the derivation child's switch (kept out of the memo key), the bound on the questions one edit may ask again, the `derived` text in both languages, the closed request and answer documents of the derivation child, and the `$comment` sentence that states the rule.
usage: python3 -I edit_schema_derived.py <root>   (idempotent; json.dumps(indent=2) shape plus a final newline)"""
import json, os, sys
root = sys.argv[1]
path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧬️schema/🔣️.json")
value = json.load(open(path, encoding="utf8"))
policy = value["definitions"]["Policy"]["const"]
ordered = {}
for key, item in policy.items():
    ordered[key] = item
    if key == "sidecar": ordered["derive"] = "SEMIO_BUILD_GRAPH_MEMO_DERIVE"
    if key == "maximumEditRows": ordered["maximumDerivedQuestions"] = 64
policy.clear(); policy.update(ordered)
if "SEMIO_BUILD_GRAPH_MEMO_DERIVE" not in policy["environmentDenied"]: policy["environmentDenied"] = sorted(policy["environmentDenied"] + ["SEMIO_BUILD_GRAPH_MEMO_DERIVE"])
policy["texts"]["en"]["derived"] = "graph: {questions} derived facts that read them were asked again by a child process with the planner's own derivation; every answer is unchanged"
policy["texts"]["de"]["derived"] = "graph: {questions} abgeleitete Fakten, die sie lesen, wurden von einem Kindprozess mit der Ableitung des Planers neu befragt; jede Antwort ist unverändert"
names = {"type": "array", "maxItems": 65536, "items": {"type": "string", "minLength": 1, "maxLength": 256}}
value["definitions"]["DeriveRequest"] = {"$comment": "Written by the memo for its derivation child (`Policy.derive` names the file): the fact index of the sealed generation and the pseudo owners whose derived question the child asks again.", "type": "object", "additionalProperties": False, "required": ["version", "index", "suspects"], "properties": {"version": {"const": 1}, "index": {"$ref": "#/definitions/Sha256"}, "suspects": names}}
value["definitions"]["DeriveAnswer"] = {"$comment": "Written by the derivation child next to the request (`.answer.json`): `held` the questions whose answer is the recorded one and whose new support reads no entry the old support did not read, `changed` the questions whose answer moved, `failed` the computations that failed, `uncovered` the held questions whose new support reads a new entry (the memo's relevance does not watch it), `missing` when the fact index is no longer retained.", "type": "object", "additionalProperties": False, "required": ["version", "held", "changed", "failed", "uncovered", "missing"], "properties": {"version": {"const": 1}, "held": names, "changed": names, "failed": {"type": "integer", "minimum": 0}, "uncovered": names, "missing": {"type": "boolean"}}}
sentence = " Derived refinement: a re-checked row that does not hold and that only pseudo owners hold (the support of derived facts), and a derived fact the journal names, make the memo ask those questions again with the planner's own derivation (`openGenerationDeriverV1`, `settleSuspectsV1`) in a child process of the same invocation (`Policy.derive`); it serves only when every answer is the recorded one and every new support reads only entries the old support read; a stale row a project or the environment holds itself, a failed or changed answer, a new entry and more than `maximumDerivedQuestions` questions plan."
if sentence.strip() not in value["$comment"]: value["$comment"] += sentence
text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
open(path, "w", encoding="utf8").write(text)
print("[DEBUG] schema edited", len(text))

# 🗂️ Writes the language-agnostic fixture of the mutation leaf case table contract. usage: make-case-table-fixture.py
import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.join(HERE,"staged/new/fixtures/🗂️mutation-leaf-case-table/🔣️.json")
schema={
  "$schema":"http://json-schema.org/draft-07/schema#",
  "title":"mutation-leaf-case-table-v1",
  "type":"object",
  "additionalProperties":False,
  "required":["cases"],
  "properties":{"cases":{"type":"array","minItems":1,"uniqueItems":True,"items":{"type":"string","pattern":"^[^/\\\\\\s](?:[^/\\\\]*[^/\\\\\\s])?$"}}},
}
def p(i,note,source,valid,cases=None,schema_valid=None,reason=None):
    row={"id":i,"note":note,"source":source,"valid":valid,"schemaValid":valid if schema_valid is None else schema_valid}
    if valid: row["cases"]=cases
    else: row["reason"]=reason
    return row
FLIPS="↔️flips"; UNCH="⚠️unchanged"
fixture={
 "schemaVersion":1,
 "contractId":"mutation-leaf-case-table-v1",
 "semantics":{
  "table":"`<owner>/🧪️tests/🔣️.cases.json` beside a schema-side mutation leaf: one JSON object whose only member lists every fixture bundle directory of that leaf (`🧪️tests/🧫️fixtures/🧬️mutations/<owner>/<case>`).",
  "valid":"Whether the repository's parser accepts the source as a case table.",
  "schemaValid":"Whether the JSON Schema in `schema` accepts the parsed document. The third-party Ajv result must equal it. Folded duplicates (U+FE0F removed, NFC) are beyond JSON Schema, so they are valid there and invalid for the parser.",
  "reason":"A fragment of the parser's refusal.",
  "problems":"How a table disagrees with the bundles of its leaf; names compare with U+FE0F removed and NFC applied."
 },
 "policy":{"fileKindId":"json-case-table","casesField":"cases","extensionChain":".cases.json"},
 "schema":schema,
 "parse":[
  p("two-cases","a plain table keeps its order",json.dumps({"cases":[FLIPS,UNCH]},ensure_ascii=False),True,[FLIPS,UNCH]),
  p("variation-selector-absent","a name without U+FE0F is kept as written",json.dumps({"cases":["\U0001f9f9delete"]},ensure_ascii=False),True,["\U0001f9f9delete"]),
  p("not-json","a truncated document is refused",'{"cases": [',False,reason="is not JSON",schema_valid=None),
  p("array-root","the root must be an object",'["a"]',False,reason="must be a JSON object"),
  p("extra-member","only the cases member is allowed",'{"cases":["a"],"note":"x"}',False,reason="exactly the member"),
  p("missing-member","the cases member is required",'{"list":["a"]}',False,reason="exactly the member"),
  p("empty-array","a table lists at least one case",'{"cases":[]}',False,reason="non-empty array"),
  p("non-string-item","every case is a string",'{"cases":[1]}',False,reason="plain directory names"),
  p("empty-item","a case is not empty",'{"cases":[""]}',False,reason="plain directory names"),
  p("slash-item","a case is one directory name",'{"cases":["a/b"]}',False,reason="plain directory names"),
  p("backslash-item","a case is one directory name",'{"cases":["a\\\\b"]}',False,reason="plain directory names"),
  p("padded-item","a case has no surrounding whitespace",'{"cases":[" a"]}',False,reason="plain directory names"),
  p("literal-duplicate","the same name twice",'{"cases":["a","a"]}',False,reason="twice"),
  p("folded-duplicate","U+FE0F variants are the same directory",json.dumps({"cases":["\U0001f9f9delete","\U0001f9f9️delete"]},ensure_ascii=False),False,reason="twice",schema_valid=True),
  p("normalization-duplicate","NFC and NFD spellings are the same directory",json.dumps({"cases":["é","é"]},ensure_ascii=False),False,reason="twice",schema_valid=True),
 ],
 "problems":[
  {"id":"exact","note":"the table lists exactly the bundles","listed":[FLIPS,UNCH],"bundles":[UNCH,FLIPS],"expected":[]},
  {"id":"fold-equal","note":"U+FE0F spelling differences are not disagreements","listed":["\U0001f9f9delete"],"bundles":["\U0001f9f9️delete"],"expected":[]},
  {"id":"unlisted-bundle","note":"a bundle the table does not list","listed":[FLIPS],"bundles":[FLIPS,UNCH],"expected":["does not list the fixture bundle "+UNCH]},
  {"id":"phantom-case","note":"a listed case without a bundle","listed":[FLIPS,UNCH],"bundles":[FLIPS],"expected":["lists "+UNCH+", which has no fixture bundle"]},
 ],
}
os.makedirs(os.path.dirname(OUT),exist_ok=True)
open(OUT,"w",encoding="utf8").write(json.dumps(fixture,ensure_ascii=False,indent=2)+"\n")
print("wrote",OUT)

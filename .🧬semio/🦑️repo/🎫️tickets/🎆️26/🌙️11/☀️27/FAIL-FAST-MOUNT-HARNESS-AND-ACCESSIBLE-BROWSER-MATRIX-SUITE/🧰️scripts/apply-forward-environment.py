"""🔀 Anchored all-or-nothing patch: checkout-test requests forward allowlisted test-service variables to the worker."""
import json, pathlib, sys
R = pathlib.Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🔬️checkout-test")
edits = {
 R/"🧬️schema/🔣️.json": [
  ('"required": ["mode", "modes", "default", "workerMode"],', '"required": ["mode", "modes", "default", "workerMode", "forward"],'),
  ('"workerMode": {"type": "string"}\n          }', '"workerMode": {"type": "string"},\n            "forward": {"type": "array", "items": {"type": "string", "pattern": "^SEMIO_TEST_SERVICE_[A-Z0-9_]{1,40}$"}}\n          }'),
  ('"stretchBasisPoints": {"type": "integer", "minimum": 10000, "maximum": 640000}\n      }', '"stretchBasisPoints": {"type": "integer", "minimum": 10000, "maximum": 640000},\n        "environment": {"type": "object", "additionalProperties": false, "properties": {"SEMIO_TEST_SERVICE_ORIGIN": {"$ref": "#/definitions/ServiceOrigin"}}}\n      }'),
  ('    "Cut": {', '    "ServiceOrigin": {"type": "string", "maxLength": 280, "pattern": "^https?://[A-Za-z0-9.-]{1,253}(:[0-9]{1,5})?$"},\n    "Cut": {'),
  ('"workerMode": "never"}', '"workerMode": "never", "forward": ["SEMIO_TEST_SERVICE_ORIGIN"]}'),
 ],
 R/"📐️policy/🟦️.ts": [
  ('budgetSeconds:number;stretchBasisPoints?:number}>;', 'budgetSeconds:number;stretchBasisPoints?:number;environment?:Readonly<Record<string,string>>}>;'),
  ("/** 📨️ What the requester ships: a nonce that makes every request its own action (a test result is never taken from a cache), the test file with its content digest, the replica cut it has applied, its language and the seconds the file may run. */",
   "/** 📨️ What the requester ships: a nonce that makes every request its own action (a test result is never taken from a cache), the test file with its content digest, the replica cut it has applied, its language, the seconds the file may run and the allowlisted test-service variables (`policy.environment.forward`) of its environment. */"),
 ],
 R/"🔌️client/🟦️.ts": [
  ("...input.stretchBasisPoints===undefined?{}:{stretchBasisPoints:input.stretchBasisPoints}},text=JSON.stringify(request);",
   "...input.stretchBasisPoints===undefined?{}:{stretchBasisPoints:input.stretchBasisPoints},...checkoutTestForwardedV1(input.environment)},text=JSON.stringify(request);"),
 ],
 R/"🧪️tests/🟦️.ts": [
  ('  expect((await tests({}).begin(request({stretchBasisPoints:5}),signal)).kind).toBe("fail");\n',
   '  expect((await tests({}).begin(request({stretchBasisPoints:5}),signal)).kind).toBe("fail");\n  const served=await tests({}).begin(request({environment:{SEMIO_TEST_SERVICE_ORIGIN:"http://192.168.1.7:6058"}}),signal);\n  expect(served.kind==="run"?served.env.SEMIO_TEST_SERVICE_ORIGIN:undefined).toBe("http://192.168.1.7:6058");\n  expect("SEMIO_TEST_SERVICE_ORIGIN" in ready.env).toBe(false);\n  for(const environment of [{SEMIO_TEST_SERVICE_ORIGIN:"file:///etc"},{SEMIO_TEST_SERVICE_ORIGIN:"http://u:p@h"},{PATH:"/evil"}])expect((await tests({}).begin(request({environment}),signal)).kind).toBe("fail");\n  expect(checkoutTestForwardedV1({SEMIO_TEST_SERVICE_ORIGIN:"http://h:1",PATH:"/bin"})).toEqual({environment:{SEMIO_TEST_SERVICE_ORIGIN:"http://h:1"}});\n  expect(checkoutTestForwardedV1({PATH:"/bin",SEMIO_TEST_SERVICE_ORIGIN:""})).toEqual({});\n'),
 ],
 R/"👷️worker/🟦️.ts": [
  ("...request.stretchBasisPoints===undefined?{}:stretchEnvironmentV1(request.stretchBasisPoints)},",
   "...request.stretchBasisPoints===undefined?{}:stretchEnvironmentV1(request.stretchBasisPoints),...request.environment},"),
 ],
}
appendix = {R/"📐️policy/🟦️.ts": "\n/** 🔀️ The allowlisted test-service variables (`policy.environment.forward`) of an environment, as the optional `environment` field of a request; nothing when none is set.\n * @see ../🧬️schema/🔣️.json */\nexport function checkoutTestForwardedV1(environment:Readonly<Record<string,string|undefined>>):Readonly<{environment?:Readonly<Record<string,string>>}>{\n const forwarded=Object.fromEntries(CHECKOUT_TEST_POLICY_V1.environment.forward.flatMap(name=>{const value=environment[name];return value===undefined||value===\"\"?[]:[[name,value]];}));\n return Object.keys(forwarded).length===0?{}:{environment:forwarded};\n}\n"}
out = {}
for path, pairs in edits.items():
    text = path.read_text(encoding="utf-8")
    for old, new in pairs:
        n = text.count(old)
        if n != 1: sys.exit(f"anchor x{n} in {path}: {old[:80]}")
        text = text.replace(old, new)
    out[path] = text + appendix.get(path, "")
json.loads(out[R/"🧬️schema/🔣️.json"])
for key in (R/"🔌️client/🟦️.ts", R/"🧪️tests/🟦️.ts"):
    lines = out[key].split("\n")
    idx = [i for i,l in enumerate(lines) if l.startswith("import {CHECKOUT_TEST_POLICY_V1")]
    if len(idx) != 1: sys.exit("policy import not unique in " + str(key))
    lines[idx[0]] = lines[idx[0]].replace("{", "{checkoutTestForwardedV1,", 1)
    out[key] = "\n".join(lines)
if "--dry" in sys.argv: print("dry ok"); sys.exit(0)
for path, text in out.items(): path.write_text(text, encoding="utf-8")
print("applied", len(out))

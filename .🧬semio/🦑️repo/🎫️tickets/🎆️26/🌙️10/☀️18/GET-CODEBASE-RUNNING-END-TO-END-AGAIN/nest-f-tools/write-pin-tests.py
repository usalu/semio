import json, os, random
ROOT = "/Users/ueli/Documents/semio"
PIN = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin"
T = PIN + "/🧪️tests"
for d in (T, T + "/🐍️oracle", T + "/🤝️nested"):
    os.makedirs(d, exist_ok=True)

oracle = r'''"""Independent model of the generation pin of a nested build command.

Commands read one JSON document on stdin and print one JSON document:
  encode {"owner","generation","manifest","owners":[...],"covered":[...]|null,"atMs"} -> {"text": canonical pin text}
  read   {"text": any}                                                              -> {"accepted": bool}
  state  {"environment": {...}, "alive": [pid,...]}                                 -> {"state": "none"|"bound"|"foreign"|"refused"}
  open   {"owners": [...], "covered": [...], "needed": [...]|null}                  -> {"open": [...]}
  owner  {"environment": {...}, "pid": n}                                           -> {"owner": n}
Standard library only; names are ordered by code point like the store (Python str order).
"""
import base64
import binascii
import json
import re
import sys

BASE64 = re.compile(r"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$")
HEX = re.compile(r"^[a-f0-9]{64}$")
MAXIMUM_BYTES = 32768
MAXIMUM_OWNERS = 65536
ENVIRONMENT = "SEMIO_BUILD_GENERATION_PIN"
INVOCATION = "SEMIO_BUILD_INVOCATION_PID"
CHAIN = "SEMIO_BUILD_INVOCATION_PIDS"


def is_int(value):
    return isinstance(value, int) and not isinstance(value, bool)


def bitmap(owners, covered):
    covered = set(covered)
    out = bytearray((len(owners) + 7) // 8)
    for at, name in enumerate(owners):
        if name in covered:
            out[at // 8] |= 1 << (at % 8)
    return out


def encode(request):
    owners = request["owners"]
    covered = owners if request["covered"] is None else request["covered"]
    raw = bitmap(owners, covered)
    pin = {
        "version": 1,
        "owner": request["owner"],
        "generation": request["generation"],
        "manifest": request["manifest"],
        "covered": {"atMs": request["atMs"], "count": len(owners), "owners": base64.b64encode(bytes(raw)).decode("ascii")},
    }
    return {"text": json.dumps(pin, separators=(",", ":"), ensure_ascii=False)}


def stamp_accepted(stamp):
    if not isinstance(stamp, dict) or set(stamp) != {"atMs", "count", "owners"}:
        return False
    at, count, owners = stamp["atMs"], stamp["count"], stamp["owners"]
    if not is_int(at) or not is_int(count) or at < 0 or count < 0 or count > MAXIMUM_OWNERS or at > 9007199254740991:
        return False
    if not isinstance(owners, str) or not BASE64.match(owners):
        return False
    try:
        raw = base64.b64decode(owners, validate=True)
    except (binascii.Error, ValueError):
        return False
    if len(raw) != (count + 7) // 8 or base64.b64encode(raw).decode("ascii") != owners:
        return False
    return not (count % 8 and raw[-1] >> (count % 8))


def read(text):
    if not isinstance(text, str) or len(text.encode("utf-8")) > MAXIMUM_BYTES:
        return None
    try:
        value = json.loads(text)
    except ValueError:
        return None
    if not isinstance(value, dict) or set(value) != {"version", "owner", "generation", "manifest", "covered"}:
        return None
    if value["version"] != 1 or not is_int(value["version"]):
        return None
    if not is_int(value["owner"]) or value["owner"] < 1 or value["owner"] > 9007199254740991:
        return None
    if not is_int(value["generation"]) or value["generation"] < 0 or value["generation"] > 9007199254740991:
        return None
    if not isinstance(value["manifest"], str) or not HEX.match(value["manifest"]):
        return None
    if not stamp_accepted(value["covered"]):
        return None
    return value


def number(value):
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def state(request):
    environment, alive = request["environment"], set(request["alive"])
    text = environment.get(ENVIRONMENT)
    if text is None or text == "":
        return {"state": "none"}
    pin = read(text)
    if pin is None:
        return {"state": "refused"}
    owners = [number(environment.get(INVOCATION))] + [number(part) for part in (environment.get(CHAIN) or "").split(",")]
    return {"state": "bound" if pin["owner"] in owners and pin["owner"] in alive else "foreign"}


def open_owners(request):
    covered = set(request["covered"])
    needed = request["owners"] if request["needed"] is None else request["needed"]
    return {"open": sorted(name for name in needed if name not in covered)}


def owner(request):
    value = number(request["environment"].get(INVOCATION))
    return {"owner": value if value is not None and value > 0 else request["pid"]}


def main():
    command = sys.argv[1]
    request = json.load(sys.stdin)
    if command == "encode":
        result = encode(request)
    elif command == "read":
        result = {"accepted": read(request["text"]) is not None}
    elif command == "state":
        result = state(request)
    elif command == "open":
        result = open_owners(request)
    elif command == "owner":
        result = owner(request)
    else:
        raise SystemExit("unknown command")
    print(json.dumps(result, ensure_ascii=False))


main()
'''
open(T + "/🐍️oracle/🐍️.py", "w").write(oracle)

rng = random.Random(1025)
def names(count): return ["o%03d" % i for i in range(count)]
manifest = "0123456789abcdef" * 4
encode = []
for count in (0, 1, 7, 8, 9, 64, 65, 300):
    owners = names(count)
    covered_sets = {"none": [], "all": owners, "odd": owners[1::2], "random": sorted(rng.sample(owners, count // 3))}
    for kind, covered in covered_sets.items():
        encode.append({"id": "%s-%d" % (kind, count), "owner": 4242, "generation": count + 3, "manifest": manifest, "owners": owners, "covered": covered, "atMs": 1790000000000 + count})
    encode.append({"id": "undefined-%d" % count, "owner": 7, "generation": 0, "manifest": manifest, "owners": owners, "covered": None, "atMs": 5})

good = json.dumps({"version": 1, "owner": 9, "generation": 4, "manifest": manifest, "covered": {"atMs": 1, "count": 3, "owners": "BQ=="}}, separators=(",", ":"))
def variant(**changes):
    value = json.loads(good)
    for key, new in changes.items():
        if new is ...: del value[key]
        else: value[key] = new
    return json.dumps(value, separators=(",", ":"))
refusals = [
 {"id": "empty-object", "text": "{}"}, {"id": "array", "text": "[]"}, {"id": "null", "text": "null"}, {"id": "not-json", "text": "{"},
 {"id": "number", "text": "5"}, {"id": "string", "text": "\"x\""},
 {"id": "version-2", "text": variant(version=2)}, {"id": "version-string", "text": variant(version="1")},
 {"id": "owner-zero", "text": variant(owner=0)}, {"id": "owner-float", "text": variant(owner=1.5)}, {"id": "owner-string", "text": variant(owner="9")}, {"id": "owner-bool", "text": variant(owner=True)},
 {"id": "generation-negative", "text": variant(generation=-1)}, {"id": "generation-float", "text": variant(generation=0.5)},
 {"id": "manifest-short", "text": variant(manifest="abc")}, {"id": "manifest-upper", "text": variant(manifest=manifest.upper())}, {"id": "manifest-null", "text": variant(manifest=None)},
 {"id": "missing-covered", "text": variant(covered=...)}, {"id": "missing-owner", "text": variant(owner=...)},
 {"id": "unknown-key", "text": variant(extra=1)},
 {"id": "covered-array", "text": variant(covered=[])},
 {"id": "covered-padding-bits", "text": variant(covered={"atMs": 1, "count": 3, "owners": "/w=="})},
 {"id": "covered-count-mismatch", "text": variant(covered={"atMs": 1, "count": 9, "owners": "BQ=="})},
 {"id": "covered-not-base64", "text": variant(covered={"atMs": 1, "count": 3, "owners": "!!!!"})},
 {"id": "covered-unknown-key", "text": variant(covered={"atMs": 1, "count": 3, "owners": "BQ==", "x": 1})},
 {"id": "covered-negative-time", "text": variant(covered={"atMs": -1, "count": 3, "owners": "BQ=="})},
 {"id": "oversized", "text": variant(manifest=manifest) + " " * 33000},
]
accepted = [{"id": "minimal", "text": good}, {"id": "empty-owner-list", "text": variant(covered={"atMs": 0, "count": 0, "owners": ""})}]

def env(pin_text=None, invocation=None, chain=None):
    value = {}
    if pin_text is not None: value["SEMIO_BUILD_GENERATION_PIN"] = pin_text
    if invocation is not None: value["SEMIO_BUILD_INVOCATION_PID"] = str(invocation)
    if chain is not None: value["SEMIO_BUILD_INVOCATION_PIDS"] = chain
    return value
states = [
 {"id": "no-pin", "environment": env(None, 9), "alive": [9], "expected": "none"},
 {"id": "empty-pin", "environment": env("", 9), "alive": [9], "expected": "none"},
 {"id": "bound-by-invocation", "environment": env(good, 9), "alive": [9], "expected": "bound"},
 {"id": "bound-by-chain", "environment": env(good, 3, "5,9,11"), "alive": [9], "expected": "bound"},
 {"id": "owner-dead", "environment": env(good, 9), "alive": [], "expected": "foreign"},
 {"id": "owner-not-in-invocation", "environment": env(good, 10), "alive": [9, 10], "expected": "foreign"},
 {"id": "daemon-cleared-invocation", "environment": env(good), "alive": [9], "expected": "foreign"},
 {"id": "chain-without-owner", "environment": env(good, 3, "5,11"), "alive": [9], "expected": "foreign"},
 {"id": "chain-garbage", "environment": env(good, 3, "x,,9z"), "alive": [9], "expected": "foreign"},
 {"id": "malformed-pin", "environment": env("{}", 9), "alive": [9], "expected": "refused"},
 {"id": "pin-not-json", "environment": env("nope", 9), "alive": [9], "expected": "refused"},
]
owners9 = names(9)
opens = [
 {"id": "all-covered", "owners": owners9, "covered": owners9, "needed": owners9[:4]},
 {"id": "none-covered", "owners": owners9, "covered": [], "needed": owners9[:3]},
 {"id": "partly", "owners": owners9, "covered": owners9[:5], "needed": owners9[3:8]},
 {"id": "undefined-needed", "owners": owners9, "covered": owners9[:8], "needed": None},
 {"id": "undefined-needed-all", "owners": owners9, "covered": owners9, "needed": None},
 {"id": "unsorted-needed", "owners": owners9, "covered": [], "needed": ["o008", "o002", "o005"]},
]
owners_rows = [
 {"id": "from-invocation", "environment": env(None, 31), "pid": 77},
 {"id": "outermost-itself", "environment": env(None), "pid": 77},
 {"id": "garbage", "environment": env(None, "x"), "pid": 77},
 {"id": "zero", "environment": {"SEMIO_BUILD_INVOCATION_PID": "0"}, "pid": 77},
 {"id": "negative", "environment": {"SEMIO_BUILD_INVOCATION_PID": "-4"}, "pid": 77},
]
fixture = {"version": 1, "encode": encode, "accepted": accepted, "refusals": refusals, "states": states, "opens": opens, "owners": owners_rows}
open(T + "/🔣️.json", "w").write(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n")

fixture_schema = {
 "type": "object", "additionalProperties": False, "required": ["version", "encode", "accepted", "refusals", "states", "opens", "owners"],
 "properties": {"version": {"const": 1}, "encode": {"type": "array", "items": {"type": "object", "required": ["id", "owner", "generation", "manifest", "owners", "covered", "atMs"]}},
  "accepted": {"type": "array", "items": {"type": "object", "required": ["id", "text"]}}, "refusals": {"type": "array", "items": {"type": "object", "required": ["id", "text"]}},
  "states": {"type": "array", "items": {"type": "object", "required": ["id", "environment", "alive", "expected"]}}, "opens": {"type": "array", "items": {"type": "object", "required": ["id", "owners", "covered", "needed"]}},
  "owners": {"type": "array", "items": {"type": "object", "required": ["id", "environment", "pid"]}}},
}
sp = PIN + "/🧬️schema/🔣️.json"
schema = json.load(open(sp))
schema["definitions"]["Fixture"] = fixture_schema
open(sp, "w").write(json.dumps(schema, ensure_ascii=False, indent=1) + "\n")

unit = r'''/** 🧷️ Contract: the generation pin that an Original run hands to its nested commands is a closed document; encoding, reading, the binding to the invocation chain and the owners a pin leaves open equal the independent Python model on closed vectors, every malformed pin is refused by both, and the constants follow the schema and the lease protocol. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{GENERATION_PIN_MESSAGES_V1,GENERATION_PIN_POLICY_V1,encodeGenerationPinV1,generationPinOpenV1,generationPinOwnerV1,generationPinStateV1,readGenerationPinV1,renderGenerationPinMessageV1}from"../🟨️.mjs";
import leases from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🧬️schema/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const model=(command:string,input:unknown)=>{const run=Bun.spawnSync([python!,"-I",oracle,command],{stdin:Buffer.from(JSON.stringify(input))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};
const refused=(body:()=>unknown)=>{try{body();}catch(error){return error as Error&{code?:string;reason?:string};}throw Error("refusal expected");};

test("the fixture, the policy and the messages follow the schema file",()=>{
 expect(new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
 expect(GENERATION_PIN_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof GENERATION_PIN_POLICY_V1);
 expect(JSON.parse(JSON.stringify(GENERATION_PIN_MESSAGES_V1))).toEqual(schema.definitions.Messages.const);
 expect(Object.keys(GENERATION_PIN_MESSAGES_V1.en)).toEqual(Object.keys(GENERATION_PIN_MESSAGES_V1.de));
});

test("the environment names are the ones of the lease protocol that every nested process already carries",()=>{
 const text=JSON.stringify(leases);
 expect(text).toContain(GENERATION_PIN_POLICY_V1.invocationEnvironment);
 expect(text).toContain(GENERATION_PIN_POLICY_V1.chainEnvironment);
});

for(const row of fixture.encode)test("encoded pin equals the Python model and reads back: "+row.id,()=>{
 const text=encodeGenerationPinV1({owner:row.owner,generation:row.generation,manifest:row.manifest,owners:row.owners,covered:row.covered===null?undefined:new Set(row.covered),atMs:row.atMs});
 expect(text).toBe(model("encode",row).text);
 const pin=readGenerationPinV1(text);
 expect(pin.owner).toBe(row.owner);expect(pin.generation).toBe(row.generation);expect(pin.manifest).toBe(row.manifest);expect(pin.covered.count).toBe(row.owners.length);
 const wanted=row.covered===null?row.owners:row.covered;
 expect(generationPinOpenV1({pin,owners:row.owners})).toEqual(row.owners.filter(name=>!wanted.includes(name)));
});

for(const row of fixture.accepted)test("an admitted pin is admitted by both: "+row.id,()=>{
 expect(()=>readGenerationPinV1(row.text)).not.toThrow();expect(model("read",row)).toEqual({accepted:true});
});

for(const row of fixture.refusals)test("a malformed pin is refused by both, in both languages: "+row.id,()=>{
 expect(model("read",row)).toEqual({accepted:false});
 const error=refused(()=>readGenerationPinV1(row.text));
 expect(error.code).toBe("GENERATION_PIN_REFUSED");expect(error.reason).toBe("malformed");
 expect(error.message).toContain("does not plan a graph of its own");expect(error.message).toContain("plant keinen eigenen Graphen");
});

for(const row of fixture.states)test("binding to the invocation chain equals the Python model: "+row.id,()=>{
 const alive=new Set(row.alive),expected=model("state",row).state;
 expect(expected).toBe(row.expected);
 if(expected==="refused"){expect(refused(()=>generationPinStateV1(row.environment,{alive:pid=>alive.has(pid)})).reason).toBe("malformed");return;}
 expect(generationPinStateV1(row.environment,{alive:pid=>alive.has(pid)}).state).toBe(expected);
});

for(const row of fixture.opens)test("the owners a pin leaves open equal the Python model: "+row.id,()=>{
 const pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:1,manifest:"a".repeat(64),owners:row.owners,covered:new Set(row.covered),atMs:1}));
 expect(generationPinOpenV1({pin,owners:row.owners,...(row.needed===null?{}:{needed:new Set(row.needed)})})).toEqual(model("open",row).open);
});

for(const row of fixture.owners)test("the owner invocation of a command equals the Python model: "+row.id,()=>{
 expect(generationPinOwnerV1(row.environment,row.pid)).toBe(model("owner",row).owner);
});

test("an owner list that differs from the manifest of the pin refuses",()=>{
 const pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:1,manifest:"a".repeat(64),owners:["a","b"],covered:new Set(["a"]),atMs:1}));
 expect(refused(()=>generationPinOpenV1({pin,owners:["a","b","c"]})).reason).toBe("malformed");
});

test("the refusal texts are filled in both languages and name the unreadable key",()=>{
 expect(renderGenerationPinMessageV1("uncovered",{generation:4,count:2,names:"x, y"},"en")).toContain("generation 4 does not cover 2");
 expect(renderGenerationPinMessageV1("uncovered",{generation:4,count:2,names:"x, y"},"de")).toContain("Generation 4 2 der benötigten Projekte nicht abdeckt (x, y)");
 expect(()=>renderGenerationPinMessageV1("nope" as never,{},"en")).toThrow("Known generation pin message required");
});

test("the schema file of the pin is read, not copied",()=>{
 expect(readFileSync(join(dirname(fileURLToPath(import.meta.url)),"../🟨️.mjs"),"utf8")).toContain('from"./🧬️schema/🔣️.json"');
});
'''
open(T + "/🟦️.ts", "w").write(unit)
print("tests written")

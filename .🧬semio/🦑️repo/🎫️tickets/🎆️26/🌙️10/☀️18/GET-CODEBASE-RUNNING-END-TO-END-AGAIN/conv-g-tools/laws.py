import ast, json, os, re, subprocess, sys
import jsonschema
HERE = os.path.dirname(os.path.abspath(__file__)); S = HERE + "/staged/files/"; sys.path.insert(0, HERE)
import lib
BUN = sys.argv[1] if len(sys.argv) > 1 else "bun"
FW = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def find(tail):
    hits = [os.path.relpath(os.path.join(b, n), S) for b, _, ns in os.walk(S) for n in ns if (os.path.relpath(os.path.join(b, n), S)).endswith(tail)]
    assert len(hits) == 1, (tail, hits); return hits[0]
def load(tail): return json.load(open(S + find(tail), encoding="utf8"))
fails = []
def law(name, ok, detail=""):
    print(("ok   " if ok else "FAIL ") + name + (" " + detail if detail and not ok else "")); 
    if not ok: fails.append(name)
for b, _, ns in os.walk(S):
    for n in ns:
        if n.endswith(".json"):
            try: json.load(open(os.path.join(b, n), encoding="utf8"))
            except Exception as e: law("json parses " + n, False, str(e))
law("every staged json parses", not fails)
v = lambda schema, doc: [e.message for e in jsonschema.Draft7Validator(schema).iter_errors(doc)]
errs = v(load("🏛️sealed/🧬️schema/🔣️.json"), load("🏛️sealed/🔣️.json")); law("sealed policy satisfies its schema, with the three new bounds", not errs and load("🏛️sealed/🔣️.json")["maximumPathCharacters"] == 256, str(errs[:2]))
errs = v(load("🎚️store-bound/🧬️schema/🔣️.json"), load("🎚️store-bound/🔣️policy.json")); law("store-bound policy satisfies its schema, with probePatienceMs", not errs and load("🎚️store-bound/🔣️policy.json")["probePatienceMs"] == 1000, str(errs[:2]))
errs = v(load("🔭️read-oracle/🧬️schema/🔣️.json"), load("🔭️read-oracle/🔣️.json")); law("read-oracle document satisfies its schema (policy, six new messages)", not errs, str(errs[:2]))
doc = load("🔭️read-oracle/🔣️.json"); law("read-oracle policy keeps the 500 MB and 64 KiB limits", doc["policy"] == {"memoryLimitMegabytes": 500, "tailBytes": 65536})
hostSchema = load("🌡️host-capacity/🧬️schema/🔣️.json"); policyRef = dict(hostSchema, **{"$ref": "#/definitions/Policy"})
for name, doc_ in (("the repository policy", json.load(open(S + find("🏗️build/🔣️policy.json"), encoding="utf8"))["hostCapacity"]), ("the host-capacity fixture", load("🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json")["policy"])):
    errs = v(policyRef, doc_); law(name + " satisfies the host-capacity policy schema with jitterBasisPoints 2500", not errs and doc_["wait"]["jitterBasisPoints"] == 2500, str(errs[:2]))
law("wait jitter 2500 bp equals the old 0.75 + 0.5 * random on both ends", 1 - 2500 / 10000 == 0.75 and 2 * 2500 / 10000 == 0.5)
texts = load("📖️texts/🔣️.json")["messages"]
keys = [k for k in texts if k.startswith("machine.policy.")]; law("ten machine.policy refusals, EN and DE with the same placeholders", len(keys) == 10 and all(sorted(re.findall(r"\{(\w+)\}", texts[k]["en"])) == sorted(re.findall(r"\{(\w+)\}", texts[k]["de"])) and texts[k]["en"].startswith("Closed machine disk budget required") for k in keys))
machine = open(S + find("🎚️machine/🟦️.ts"), encoding="utf8").read(); used = set(re.findall(r'refuse\("(machine\.policy\.\w+)"', machine))
law("every refuse key of the machine policy exists in the texts and none is unused", used == set(keys), str(used ^ set(keys))); law("no throw of Closed machine disk budget remains in code", "throw Error(\"Closed machine" not in machine)
law("changes policy declares startPollMilliseconds 20 and the type names it", load("📜️changes/🧬️schema/🔣️.json")["definitions"]["Policy"]["const"]["startPollMilliseconds"] == 20 and "startPollMilliseconds:number" in open(S + find("📜️changes/🟦️.ts"), encoding="utf8").read())
law("typescript retention policy keeps 2 programs, 2 GiB, 120 s, at most 8", load("🟦️typescript/🧬️schema/🔣️.json")["definitions"]["RetentionPolicy"]["const"] == {"retainedPrograms": 2, "maximumRetainedPrograms": 8, "maximumHeapBytes": 2 * 1024 ** 3, "idleMilliseconds": 120000})
law("authorizer memo policy keeps 4 MiB and 24 lanes", load("🔏️authorizers/🧬️schema/🔣️.json")["$defs"]["MemoPolicy"]["const"] == {"maximumFileBytes": 4 * 1024 ** 2, "concurrency": 24})
unit = load("🗃️unit-cache/🧬️schema/🔣️.json"); flat = json.dumps(unit); law("unit-cache lock and pool both carry pollBackoffBasisPoints 15000", flat.count('"pollBackoffBasisPoints": 15000') == 2)
law("frontier-walk schema keeps 64 / 2000 / .", load("🌿️frontier-walk/🧬️schema/🔣️.json")["definitions"]["Policy"]["const"] == {"concurrency": 64, "racyMilliseconds": 2000, "rootDirectory": "."})
law("backoff 15000 bp equals the old factor 1.5 on every integer poll up to 4096", all(-(-(p * 15000) // 10000) == -(-int(p * 1.5 * 2) // 2) for p in range(1, 4097)))
sealed = open(S + find("🏛️sealed/🟦️.ts"), encoding="utf8").read(); props = set(re.findall(r"readonly (max\w+Characters):number", sealed)); law("sealed policy interface names the three bounds the code reads", props == {"maximumTextCharacters", "maximumPathCharacters", "maximumCommandLineCharacters"})
law("every message key the read-oracle code asks for exists", all(k in doc["messages"] for f in (find("🔭️read-oracle/🟦️.ts"), find("🔭️read-oracle/🚪️cli/🟦️.ts"), find("🔭️read-oracle/🍎️seatbelt/🟦️.ts"), find("🔭️read-oracle/🐧️linux/🟦️.ts")) for k in re.findall(r'readOracleBothV1\("(\w+)"', open(S + f, encoding="utf8").read())))
launcher = ast.parse(open(S + find("🐧️linux/🐍️.py"), encoding="utf8").read()); law("python launcher: every refuse call has an English and a German text", all(len(c.args) == 2 for c in ast.walk(launcher) if isinstance(c, ast.Call) and getattr(c.func, "id", "") == "refuse"))
bad = []
for b, _, ns in os.walk(S):
    for n in ns:
        f = os.path.join(b, n); p = os.path.relpath(f, S); t = open(f, encoding="utf8").read()
        if n.endswith((".ts", ".mjs", ".mts")) and not n.endswith(".d.mts"):
            r = subprocess.run([BUN, "-e", "new Bun.Transpiler({loader:'%s'}).transformSync(await Bun.file(process.argv[1]).text())" % ("ts" if n.endswith((".ts", ".mts")) else "js"), f], capture_output=True, text=True)
            if r.returncode: bad.append(("syntax", p[-60:]))
        if n.endswith((".ts", ".mjs", ".mts", ".rs")) and lib.reuse(p, t): bad.append(("duplicate docstring emoji", p[-60:]))
        if n.endswith((".ts", ".mjs", ".mts", ".rs")) and any(not lib.MARK.match(c) for _, c in lib.openers(p, t)): bad.append(("docstring without emoji", p[-60:]))
law("staged code parses, docstring emojis are unique per file and every docstring starts with one", not bad, str(bad[:3]))
sys.exit(1 if fails else 0)

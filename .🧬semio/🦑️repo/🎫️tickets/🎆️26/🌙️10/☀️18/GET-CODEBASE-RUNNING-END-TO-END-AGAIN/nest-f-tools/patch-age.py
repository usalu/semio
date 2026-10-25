import os, sys, json
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G = B + "/🕸️graph/🕰️generations"
PIN = G + "/🧷️pin"
def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit("anchor count %d for %r" % (text.count(old), old[:80]))
        text = text.replace(old, new)
    tmp = path + ".nest-f.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.rename(tmp, path)

sp = PIN + "/🧬️schema/🔣️.json"
d = json.load(open(sp, encoding="utf-8"))
d["definitions"]["Policy"]["const"]["maximumAgeMilliseconds"] = 14400000
d["description"] += " A pin older than `maximumAgeMilliseconds` (measured from the time the Original began the load that served it) is not bound either: commands that a long-lived task starts hours later plan with the tree of their time, as before."
open(sp, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=1) + "\n")

edit(PIN + "/🟨️.mjs", [
 ('export function generationPinStateV1(environment,{alive=processAlive,self=process.pid}={}){', 'export function generationPinStateV1(environment,{alive=processAlive,self=process.pid,now=Date.now()}={}){'),
 ('pin.owner!==self&&owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign"', 'pin.owner!==self&&now-pin.covered.atMs<=P.maximumAgeMilliseconds&&owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign"'),
 ('alive and not this process itself (the Original', 'alive, not older than `maximumAgeMilliseconds` and not this process itself (the Original'),
])
edit(PIN + "/🟨️.d.mts", [
 ('readonly maximumBytes:number;readonly tools:readonly string[]}', 'readonly maximumBytes:number;readonly maximumAgeMilliseconds:number;readonly tools:readonly string[]}'),
 ('options?:{alive?:(pid:number)=>boolean;self?:number}', 'options?:{alive?:(pid:number)=>boolean;self?:number;now?:number}'),
])
edit(PIN + "/🧪️tests/🐍️oracle/🐍️.py", [
 ('    return {"state": "bound" if pin["owner"] != request.get("self") and pin["owner"] in owners and pin["owner"] in alive else "foreign"}',
  '    young = request["now"] - pin["covered"]["atMs"] <= MAXIMUM_AGE_MILLISECONDS\n    return {"state": "bound" if pin["owner"] != request.get("self") and young and pin["owner"] in owners and pin["owner"] in alive else "foreign"}'),
 ('MAXIMUM_OWNERS = 65536\n', 'MAXIMUM_OWNERS = 65536\nMAXIMUM_AGE_MILLISECONDS = 14400000\n'),
 ('"alive": [pid,...], "self": pid}', '"alive": [pid,...], "self": pid, "now": ms}'),
])
fx = PIN + "/🧪️tests/🔣️.json"
f = json.load(open(fx, encoding="utf-8"))
for row in f["states"]:
    row["now"] = 1000
good = f["states"][2]["environment"]
f["states"] += [
 {"id": "young-enough", "environment": good, "alive": [9], "now": 1 + 14400000, "expected": "bound"},
 {"id": "too-old", "environment": good, "alive": [9], "now": 2 + 14400000, "expected": "foreign"},
 {"id": "clock-before-load", "environment": good, "alive": [9], "now": 0, "expected": "bound"},
]
open(fx, "w", encoding="utf-8").write(json.dumps(f, ensure_ascii=False, indent=1) + "\n")
edit(PIN + "/🧪️tests/🟦️.ts", [
 ('self:(row as {self?:number}).self??-1}).state).toBe(expected);', 'self:(row as {self?:number}).self??-1,now:(row as {now:number}).now}).state).toBe(expected);'),
 ('{alive:pid=>alive.has(pid),self:-1})).reason', '{alive:pid=>alive.has(pid),self:-1,now:(row as {now:number}).now})).reason'),
])
edit(B + "/⌨️command/⚙️runtime/🟦️.ts", [
 ('const generationPin=sealedGenerationPinV1(selected.tool);if(generationPin!==undefined)', 'const generationPin=command==="watch"?undefined:sealedGenerationPinV1(selected.tool);if(generationPin!==undefined)'),
])
edit(PIN + "/🧪️tests/🤝️plan/🟦️.ts", [
 ('.toContain("if(generationPin!==undefined){previous', '.toContain("if(generationPin!==undefined){previous'),
 ('test("the graph module binds only a pin', 'test("a watch re-plans for every change: its children never get a pin",()=>{\n expect(source("../../../../../⌨️command/⚙️runtime/🟦️.ts")).toContain(\'const generationPin=command==="watch"?undefined:sealedGenerationPinV1(selected.tool);\');\n});\n\ntest("the graph module binds only a pin'),
])
print("age patched")

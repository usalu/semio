import os, sys, json
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G = B + "/🕸️graph/🕰️generations"
def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit("anchor count %d for %r" % (text.count(old), old[:80]))
        text = text.replace(old, new)
    tmp = path + ".nest-f.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.rename(tmp, path)

sp = G + "/🧷️pin/🧬️schema/🔣️.json"
d = json.load(open(sp, encoding="utf-8"))
d["definitions"]["Policy"]["const"]["tools"] = ["bazel", "semio"]
d["description"] = d["description"].replace("and every descendant inherits it with the rest of the environment.", "and every descendant inherits it with the rest of the environment; only the build tools whose every task environment is issued by the caller (`tools`) export it, because that issuer drops the pin for every task that is not an orchestrated one.")
open(sp, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=1) + "\n")

edit(G + "/🧷️pin/🟨️.d.mts", [('readonly maximumBytes:number}', 'readonly maximumBytes:number;readonly tools:readonly string[]}')])
edit(B + "/🕸️graph/🟦️.ts", [
 ('import{encodeGenerationPinV1,generationPinOwnerV1,generationPinStateV1,type GenerationPinV1}from', 'import{GENERATION_PIN_POLICY_V1,encodeGenerationPinV1,generationPinOwnerV1,generationPinStateV1,type GenerationPinV1}from'),
 ('/** 📌️ Hands the pin of the newest generation load to the ones that start nested commands. */\nexport function sealedGenerationPinV1():string|undefined{return sealedGenerationPinTextV1;}',
  '/** 📌️ Hands the pin of the newest generation load to the ones that start nested commands; with a `tool` only when that build tool issues every task environment itself (the others keep the graph of their own nested commands). */\nexport function sealedGenerationPinV1(tool?:string):string|undefined{return tool===undefined||GENERATION_PIN_POLICY_V1.tools.includes(tool)?sealedGenerationPinTextV1:undefined;}'),
])
edit(B + "/⌨️command/⚙️runtime/🟦️.ts", [('const generationPin=sealedGenerationPinV1();', 'const generationPin=sealedGenerationPinV1(selected.tool);')])
t = B + "/🕸️graph/🕰️generations/🧷️pin/🧪️tests/🤝️plan/🟦️.ts"
edit(t, [('exported=text.indexOf("const generationPin=sealedGenerationPinV1();")', 'exported=text.indexOf("const generationPin=sealedGenerationPinV1(selected.tool);")')])
print("gate patched")

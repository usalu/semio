import os, sys
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit("anchor count %d for %r" % (text.count(old), old[:80]))
        text = text.replace(old, new)
    tmp = path + ".nest-f.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.rename(tmp, path)

edit(B + "/🕸️graph/🟦️.ts", [
 ('import*as observationV1 from"../👁️observation/🟨️.mjs";\n',
  'import*as observationV1 from"../👁️observation/🟨️.mjs";\nimport{encodeGenerationPinV1,generationPinOwnerV1,generationPinStateV1,type GenerationPinV1}from"./🕰️generations/🧷️pin/🟨️.mjs";\n'),
 ('/** 🔌️ The identity of the one builder custody of the generation store:',
  '''/** 📌️ The pin of the generation the newest graph load of this process served; the descendants that answer to this invocation read exactly that generation. */
let sealedGenerationPinTextV1:string|undefined;
/** 📌️ Hands the pin of the newest generation load to the ones that start nested commands. */
export function sealedGenerationPinV1():string|undefined{return sealedGenerationPinTextV1;}
/** 📌️ Takes over the pin a graph child of this invocation reported (or forgets it with undefined). */
export function recordSealedGenerationPinV1(text:string|undefined):void{sealedGenerationPinTextV1=text;}
/** 🧭️ The pin this command is bound by: the one its Original invocation put in the environment while that process is alive; a pin of anyone else is none. */
function boundGenerationPinV1():GenerationPinV1|undefined{const state=generationPinStateV1(process.env);return state.state==="bound"?state.pin:undefined;}

/** 🔌️ The identity of the one builder custody of the generation store:'''),
 ('    readGraph:(value:unknown)=>readBuildGraphV1(value),\n    clock:',
  '    readGraph:(value:unknown)=>readBuildGraphV1(value),\n    sealed:(record:{generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number})=>{sealedGenerationPinTextV1=encodeGenerationPinV1({owner:generationPinOwnerV1(process.env),...record});},\n    clock:'),
 ('{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))}}) as BuildGraphV1;',
  '{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))},pin:boundGenerationPinV1()}) as BuildGraphV1;'),
 ('{policy:options.graph?.policy??"fresh",closure:options.graph?.closure,lean:options.graph?.lean}) as BuildGraphV1;',
  '{policy:options.graph?.policy??"fresh",closure:options.graph?.closure,lean:options.graph?.lean,pin:boundGenerationPinV1()}) as BuildGraphV1;'),
])
print("graph module patched")

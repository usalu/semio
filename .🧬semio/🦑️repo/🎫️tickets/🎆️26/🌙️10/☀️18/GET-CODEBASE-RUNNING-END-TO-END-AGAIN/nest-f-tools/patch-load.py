import os, sys
G = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit("anchor count %d for %r in %s" % (text.count(old), old[:70], path))
        text = text.replace(old, new)
    tmp = path + ".nest-f.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.rename(tmp, path)

load = G + "/🔌️load/🟨️.mjs"
edit(load, [
 ('import{createGraphPhaseClockV1}from"../../⏱️phases/🟨️.mjs";\n',
  'import{createGraphPhaseClockV1}from"../../⏱️phases/🟨️.mjs";\nimport{decodeVerifiedScopeV1}from"../🔭️coverage/🟨️.mjs";\nimport{readPinnedGenerationV1}from"../🧷️pin/🟨️.mjs";\n'),
 ('  covered:"graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n',
  '  covered:"graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n  pinned:"graph: generation {generation} pinned by the invoking build serves this command ({count} projects), no queue and no check of the live tree",\n'),
 ('  covered:"Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n',
  '  covered:"Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n  pinned:"Graph: die vom aufrufenden Build angeheftete Generation {generation} bedient diesen Befehl ({count} Projekte), ohne Warteschlange und ohne Prüfung des Arbeitsbaums",\n'),
 ('export async function loadGenerationGraphV1(host,{policy="fresh",closure,selection,slice,lean}={}){',
  'export async function loadGenerationGraphV1(host,{policy="fresh",closure,selection,slice,lean,pin}={}){'),
 ('rows=await host.implementationRows(),implementation=implementationDigestV1(rows)',
  'rows=pin===undefined?await host.implementationRows():[],implementation=implementationDigestV1(rows)'),
 ('  const started=Date.now(),result=roots?await engine.refreshClosure({...base,roots,closure:closeFn}):await engine.refresh(base);',
  '  const started=Date.now(),result=pin!==undefined?await readPinnedGenerationV1({engine,pin,identity,roots,closure:closeFn,signal:host.signal}):roots?await engine.refreshClosure({...base,roots,closure:closeFn}):await engine.refresh(base);'),
 ('  else if(result.served==="covered")host.say("covered",{generation});\n',
  '  else if(result.served==="covered")host.say("covered",{generation});\n  else if(result.served==="pinned")host.say("pinned",{generation,count:result.names?result.names.size:Object.keys(result.current.manifest.owners).length});\n'),
 ('  return{graph,result};\n };\n const first=',
  '  if(host.sealed){const manifest=result.current.manifest,owners=Object.keys(manifest.owners).sort(compareCodePointsV1);host.sealed({generation:manifest.generation,manifest:result.current.pointer.manifest,owners,covered:pin!==undefined?decodeVerifiedScopeV1(owners,pin.covered):result.names,atMs:pin!==undefined?pin.covered.atMs:requestedAt});}\n  return{graph,result};\n };\n const first='),
])

dts = G + "/🔌️load/🟨️.d.mts"
edit(dts, [
 ('"current"|"verified"|"waiting"|"covered"|"building"', '"current"|"verified"|"waiting"|"covered"|"pinned"|"building"'),
 (' say(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>):void;\n}',
  ' say(key:keyof typeof GENERATION_MESSAGES_V1["en"],fields:Readonly<Record<string,string|number>>):void;\n /** 📌️ Told once a load served a graph: the sealed generation, its manifest digest, the owners of that manifest in code point order and the owners (every owner when undefined) the load served from it. */\n sealed?(record:{generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number}):void;\n}'),
 ('slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean}',
  'slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean;pin?:import("../🧷️pin/🟨️.mjs").GenerationPinV1}'),
])

schema = G + "/🧬️schema/🔣️.json"
edit(schema, [
 ('     "covered": "graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n',
  '     "covered": "graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n     "pinned": "graph: generation {generation} pinned by the invoking build serves this command ({count} projects), no queue and no check of the live tree",\n'),
 ('     "covered": "Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n',
  '     "covered": "Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n     "pinned": "Graph: die vom aufrufenden Build angeheftete Generation {generation} bedient diesen Befehl ({count} Projekte), ohne Warteschlange und ohne Prüfung des Arbeitsbaums",\n'),
])
print("load patched")

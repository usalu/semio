import os, sys
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

edit(G + "/🧷️pin/🟨️.mjs", [
 ('/** 🧭️ Whether the environment holds a pin this command is bound by: `none` without a pin, `bound` while the owner is one of the invocations the command answers to and alive, `foreign` otherwise (an orphan, a daemon that cleared its invocation, a pin of another tree); a malformed pin refuses. */\nexport function generationPinStateV1(environment,{alive=processAlive}={}){',
  '/** 🧭️ Whether the environment holds a pin this command is bound by: `none` without a pin, `bound` while the owner is one of the invocations the command answers to, alive and not this process itself (the Original reads its own loads fresh; only its descendants are served from its pin), `foreign` otherwise (an orphan, a daemon that cleared its invocation, a pin of another tree); a malformed pin refuses. */\nexport function generationPinStateV1(environment,{alive=processAlive,self=process.pid}={}){'),
 ('owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign"', 'pin.owner!==self&&owners.includes(pin.owner)&&alive(pin.owner)?"bound":"foreign"'),
])
edit(G + "/🧷️pin/🟨️.d.mts", [
 ('options?:{alive?:(pid:number)=>boolean}', 'options?:{alive?:(pid:number)=>boolean;self?:number}'),
])

edit(B + "/🧭️invocation/🕸️graph/🟦️.ts", [
 (' const{open}=await import("node:fs/promises"),{writeJsonStreamV1}=await import("../../🕸️graph/🕰️generations/🌊️json-stream/🟨️.mjs"),handle=await open(file,"wx",0o600);\n try{await writeJsonStreamV1(graph,text=>handle.write(text).then(()=>undefined),{signal});await handle.sync();}finally{await handle.close();}\n}',
  ' const{open,writeFile}=await import("node:fs/promises"),{writeJsonStreamV1}=await import("../../🕸️graph/🕰️generations/🌊️json-stream/🟨️.mjs"),handle=await open(file,"wx",0o600);\n try{await writeJsonStreamV1(graph,text=>handle.write(text).then(()=>undefined),{signal});await handle.sync();}finally{await handle.close();}\n const pin=(await import("../../🕸️graph/🟦️.ts")).sealedGenerationPinV1();if(pin!==undefined)await writeFile(file+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1,pin,{flag:"wx",mode:0o600});\n}'),
 ('/** 🧊️ Names the plan file a graph child writes; the marker is set only in the environment of that child. */\nexport const BUILD_GRAPH_PLAN_OUT_V1="SEMIO_BUILD_GRAPH_PLAN_OUT";\n',
  '/** 🧊️ Names the plan file a graph child writes; the marker is set only in the environment of that child. */\nexport const BUILD_GRAPH_PLAN_OUT_V1="SEMIO_BUILD_GRAPH_PLAN_OUT";\n\n/** 📌️ The sidecar of the plan file that names the generation the child served: the loader that reads the plan hands it to the commands its descendants start. */\nexport const BUILD_GRAPH_PLAN_PIN_SUFFIX_V1=".pin";\n'),
 ('  const graph=readBuildGraphV1(JSON.parse(readFileSync(file,"utf8"))),{renderGenerationMessageV1}',
  '  const graph=readBuildGraphV1(JSON.parse(readFileSync(file,"utf8"))),{recordSealedGenerationPinV1}=await import("../../🕸️graph/🟦️.ts");recordSealedGenerationPinV1(existsSync(file+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1)?readFileSync(file+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1,"utf8"):undefined);\n  const{renderGenerationMessageV1}'),
 ('const{mkdtempSync,rmSync,readFileSync}=await import("node:fs")', 'const{mkdtempSync,rmSync,readFileSync,existsSync}=await import("node:fs")'),
])

edit(B + "/⌨️command/⚙️runtime/🟦️.ts", [
 ('{releaseBuildGraphV1:releaseGraph}=await import("../../🕸️graph/🟦️.ts");', '{releaseBuildGraphV1:releaseGraph,sealedGenerationPinV1}=await import("../../🕸️graph/🟦️.ts");'),
 ('process.env.SEMIO_BUILD_GRAPH_PATH=pinned.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pinned.digest;process.env.SEMIO_BUILD_REUSE_GRAPH="true";\n',
  'process.env.SEMIO_BUILD_GRAPH_PATH=pinned.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pinned.digest;process.env.SEMIO_BUILD_REUSE_GRAPH="true";\n    const generationPin=sealedGenerationPinV1();if(generationPin!==undefined){previous.SEMIO_BUILD_GENERATION_PIN=process.env.SEMIO_BUILD_GENERATION_PIN;process.env.SEMIO_BUILD_GENERATION_PIN=generationPin;}\n'),
])
print("sites patched")

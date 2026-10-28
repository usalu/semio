from make_lib import *

# ---- F: the prefetch books its rebuild only inside a claimed computation (needs rev3-g/upgrade-before-custody for the export)
graph = B + "/🕸️graph/🟦️.ts"
edit(graph, """    async prefetch(request:GenerationPrefetchRequestV1){await markBuildCapacityPhaseV1("rebuild");return prefetchStaleGenerationV1({root,invocation,phaseControl,registry,files:()=>state.files,request,release:()=>this.release(),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language});},""",
"""    async prefetch(request:GenerationPrefetchRequestV1){await upgradeBuildCapacityBeforeRebuildV1();return prefetchStaleGenerationV1({root,invocation,phaseControl,registry,files:()=>state.files,request,release:()=>this.release(),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,book:()=>markBuildCapacityPhaseV1("rebuild")});},""")
edit(graph, 'request:GenerationPrefetchRequestV1;release:()=>Promise<void>;language:"en"|"de"}):Promise<void> {', 'request:GenerationPrefetchRequestV1;release:()=>Promise<void>;language:"en"|"de";book:()=>Promise<unknown>}):Promise<void> {')
edit(graph, "    let begun=false;\n    try{\n      await prepareStaleProjectsV1({", "    let begun=false,booked=false;\n    try{\n      await prepareStaleProjectsV1({")
edit(graph, "          checkScriptInvocation(invocation);\n          const roots=new Set(owners.map(", "          checkScriptInvocation(invocation);\n          if(!booked){booked=true;await input.book();}\n          const roots=new Set(owners.map(")
edit(graph, "/** 🤝️ Prepares the stale projects of a pending refresh together with the commands that wait for the same graph, before the exclusive build: claims a chunk of them,",
 "/** 🤝️ Prepares the stale projects of a pending refresh together with the commands that wait for the same graph, before the exclusive build (the caller has asked for the capacity of the rebuild; the learned size is booked by `book` once a chunk is claimed, not while the run waits for a compute slot): claims a chunk of them,")
newfile(B + "/🕸️graph/🧪️tests/🎟️prefetch-booking/🟦️.ts", "prefetch-booking.ts")

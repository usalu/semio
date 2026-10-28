SESSION = L + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts"
COMMAND = L + "/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts"
REVISION = L + "/🏗️build/📥️inputs/🔏️revision/🟨️.mjs"
PROFILE = 'if(!profile||JSON.stringify(JSON.parse(Buffer.from(profile.base64,"base64").toString("utf8")))!==JSON.stringify(schema))throw staleImplementationV1('
IDENTITY = 'if(identity!==proof.sha256||loaderIdentity!==proof.loaderSha256||proof.loaderSha256!==createHash("sha256").update(buildSourceClosureStreamLoaderV1).digest("hex"))throw staleImplementationV1('
EDITS = [
    (SESSION, "const capturedWorkersV1=new WeakMap<", 'import{implementationPinnedV1}from"' + rel(SESSION, REVISION) + '";\nconst capturedWorkersV1=new WeakMap<'),
    (SESSION, PROFILE, PROFILE.replace("if(!profile||", "if(!implementationPinnedV1()&&(!profile||").replace("!==JSON.stringify(schema))throw", "!==JSON.stringify(schema)))throw")),
    (COMMAND, 'import schema from"../🧬️schema/🔣️.json"with{type:"json"};', 'import{implementationPinnedV1}from"' + rel(COMMAND, REVISION) + '";\nimport schema from"../🧬️schema/🔣️.json"with{type:"json"};'),
    (COMMAND, IDENTITY, "if(!implementationPinnedV1()&&(" + IDENTITY[3:-len(")throw staleImplementationV1(")] + "))throw staleImplementationV1("),
]

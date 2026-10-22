"""🔤️ Round 3: the combined round-6 tree gained two consumers that still named the javascript manager (the resident command's input paths, the cache collector's environment allowlist); both ask the selected manager now, and the descriptor gains `runtime.hostEnvironment`, the ambient variables the tool needs to be found."""
def declare(edit, paths):
    L, K = paths.LIBRARY, paths.MANAGERS
    S = paths.BUILD + "/📜️script.ts"
    CATALOG = "../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts"
    edit(S, "round3-script-manager", "seam3", ";process.exitCode=await runResidentCommandV1({root,args:selected.args.slice(1),", ';const javascriptManagerV1=(await import("%s")).selectPackageManagerV1("javascript");process.exitCode=await runResidentCommandV1({root,args:selected.args.slice(1),' % CATALOG)
    edit(S, "round3-script-lock", "seam3", 'environmentPaths:["package.json","bun.lock","nx.json"', 'environmentPaths:["package.json",javascriptManagerV1.primaryLockName(),"nx.json"')
    edit(S, "round3-script-configuration", "seam3", '"go.work","bunfig.toml"]', '"go.work",...javascriptManagerV1.configurationNames("scope")]')
    G = paths.BUILD + "/🧹️cache-gc/🚀️launch/🟦️.ts"
    edit(G, "round3-launch-manager", "seam3", 'import {spawn} from "node:child_process";\n', 'import {spawn} from "node:child_process";\nimport {createRequire as createManagerRequireV1} from "node:module";\n')
    edit(G, "round3-launch-keys", "seam3", '"SEMIO_LANGUAGE","BUN_INSTALL"]);', '"SEMIO_LANGUAGE",...(createManagerRequireV1(import.meta.url)("../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript").hostEnvironment as readonly string[])]);')
    T = K + "/🟦️.ts"
    edit(T, "round3-interface-descriptor", "seam3", "readonly runtime:Readonly<{hostVersionKey:string|null;probe:", "readonly runtime:Readonly<{hostVersionKey:string|null;hostEnvironment:readonly string[];probe:")
    edit(T, "round3-interface-manager", "seam3", "generationDirectory():string;\n backend(platform", "generationDirectory():string;hostEnvironment():readonly string[];\n backend(platform")
    edit(T, "round3-implementation", "seam3", "generationDirectory:()=>descriptor.generationDirectory,\n  backend:requireBackend,", "generationDirectory:()=>descriptor.generationDirectory,hostEnvironment:()=>descriptor.runtime.hostEnvironment,\n  backend:requireBackend,")
    edit(K + "/🟨️.cjs", "round3-accessor", "seam3", "generationDirectory:descriptor.generationDirectory,", "generationDirectory:descriptor.generationDirectory,hostEnvironment:Object.freeze([...descriptor.runtime.hostEnvironment]),")
    edit(K + "/🧬️schema/🔣️.json", "round3-schema-required", "seam3", '"required": [\n            "hostVersionKey",\n            "probe"\n          ],', '"required": [\n            "hostVersionKey",\n            "hostEnvironment",\n            "probe"\n          ],')
    edit(K + "/🧬️schema/🔣️.json", "round3-schema-property", "seam3", '            "probe": {\n              "type": "object",\n              "additionalProperties": false,\n              "required": [\n                "arguments",', '            "hostEnvironment": {\n              "type": "array",\n              "uniqueItems": true,\n              "items": {\n                "type": "string",\n                "pattern": "^[A-Z][A-Z0-9_]*$"\n              }\n            },\n            "probe": {\n              "type": "object",\n              "additionalProperties": false,\n              "required": [\n                "arguments",')
    for folder, key, values in [("🍞️bun", "bun", ["BUN_INSTALL"]), ("📦️npm", "null", []), ("🦀️cargo", "null", []), ("🐹️go", "null", []), ("🐍️uv", "null", []), ("🔷️dotnet", "null", [])]:
        old = '    "hostVersionKey": %s,\n' % ('"bun"' if key == "bun" else "null")
        listing = '[]' if not values else '[\n      "%s"\n    ]' % '",\n      "'.join(values)
        edit(K + "/" + folder + "/🔣️.json", "round3-descriptor-" + folder, "seam3", old, old + '    "hostEnvironment": %s,\n' % listing)
    edit(K + "/🧪️tests/🟦️.ts", "round3-neutrality-tokens", "seam3", "   if(manager.descriptor.runtime.hostVersionKey)tokens.add(manager.descriptor.runtime.hostVersionKey);\n", "   if(manager.descriptor.runtime.hostVersionKey)tokens.add(manager.descriptor.runtime.hostVersionKey);\n   for(const name of manager.hostEnvironment())tokens.add(name);\n")

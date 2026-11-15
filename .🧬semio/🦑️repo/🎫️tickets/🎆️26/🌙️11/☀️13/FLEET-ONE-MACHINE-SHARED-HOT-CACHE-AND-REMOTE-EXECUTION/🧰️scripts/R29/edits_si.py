LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SI=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
EDITS=[
(SI,'''    ...toolReferencesV1(scripts, "cargo-command"),
    ...[...javascript.files.filter((file) => !["package.json", javascriptManagerV1.primaryLockName].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
''','''    ...toolReferencesV1(scripts, "cargo-command"),
    ...toolReferencesV1(scripts, "native-toolchain"),
''',1),
(SI,'''/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection: the cargo driver by reference to its tool closure (read through the native view: the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out, the native-build modules that assemble the Cargo arguments stay in), the toolchain files, environment and commands by value. */''','''/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection: the cargo driver and the toolchain files by reference to their tool closures (read through the native view: the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out, the native-build modules that assemble the Cargo arguments stay in), environment and commands by value. */''',1),
]
INF=LIB+"🏗️build/🕸️graph/📥️inference/"
EDITS+=[
(INF+"🔖️stamps/🧬️schema/🔣️.json",'"stamp": "semio.graph-inferrer.script-inputs/4"','"stamp": "semio.graph-inferrer.script-inputs/5"',1),
(INF+"🧪️tests/🔖️output-stamps/🔣️.json",'"semio.graph-inferrer.script-inputs/4": {','"semio.graph-inferrer.script-inputs/5": {',1),
]
PCT=SI.rsplit("/",1)[0]+"/🔌️provider-closures/🧪️tests/🟦️.ts"
EDITS+=[
(PCT,'...state.tools["cargo-command"]!])].sort();','...state.tools["cargo-command"]!,...state.tools["native-toolchain"]!])].sort();',1),
(PCT,'toolOf("cargo-command")].sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:1));','toolOf("cargo-command"),toolOf("native-toolchain")].sort((a,b)=>JSON.stringify(a)<JSON.stringify(b)?-1:1));',1),
]
ORC=SI.rsplit("/",1)[0]+"/🔌️provider-closures/🧪️tests/🐍️oracle/🐍️.py"
EDITS+=[(ORC,'| set(tool["resources"]) |','| {r for r in tool["resources"] if not r.startswith("..")} |',1)]

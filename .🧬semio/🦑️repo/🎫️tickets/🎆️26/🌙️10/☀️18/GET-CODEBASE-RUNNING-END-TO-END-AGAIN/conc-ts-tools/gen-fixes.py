#!/usr/bin/env python3
"""Authors staged2/fixes.json from the CURRENT tree: every old text is asserted against the live file, counts are measured, nothing is written outside staged2/."""
import json,os
HERE=os.path.dirname(os.path.abspath(__file__)); R=os.path.abspath(os.path.join(HERE,*['..']*8))
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
W='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts'
fixes=[]
def fix(id,file,ops,why):
    s=open(os.path.join(R,file),encoding='utf8').read()
    for op in ops:
        c=s.count(op['old']); op['count']=c
        assert c>=1,(id,op['old'][:60]); 
        if op.get('expect') is not None: assert c==op.pop('expect'),(id,op['old'][:60],c)
    fixes.append({'id':id,'file':file,'why':why,'ops':[{k:v for k,v in o.items()} for o in ops]})
def op(old,new,expect=None): return {'old':old,'new':new,'expect':expect}
fix('providers-token-interface','🧰️framework/🔨️modules/🏃️process/🔌️providers/🟦️.ts',[op(
 'export type ScriptProviderTokenV1<Provider extends object> = Readonly<{ name: string; methods: readonly (keyof Provider & string)[]; readonly [brand]?: Provider }>;',
 'export interface ScriptProviderTokenV1<Provider extends object> { readonly name: string; readonly methods: readonly (keyof Provider & string)[]; readonly [brand]?: Provider }',1)],
 'TS4023: the alias `Readonly<{...}>` is inlined into every exported provider constant, exposing the unexported `brand`; an interface is referenced by name.')
fix('wasi-ecmascript-private-fields',W,[
 op('constructor(token: symbol, private readonly message: string) { super(token); }','readonly #message: string;\n    constructor(token: symbol, message: string) { super(token); this.#message = message; }',1),
 op('return this.message;','return this.#message;',1),
 op('constructor(token: symbol, parent?: Resource, private readonly deadline?: bigint) {\n      super(token, parent);','readonly #deadline?: bigint;\n    constructor(token: symbol, parent?: Resource, deadline?: bigint) {\n      super(token, parent);\n      this.#deadline = deadline;',1),
 op('this.deadline','this.#deadline',4),
 op('    private permit = 0;\n    private failed = false;\n    private errorCredit = true;\n    constructor(token: symbol, private readonly channel: "stdout" | "stderr") {','    #permit = 0;\n    #failed = false;\n    #errorCredit = true;\n    readonly #channel: "stdout" | "stderr";\n    constructor(token: symbol, channel: "stdout" | "stderr") {',1),
 op('      super(token);\n      errorReservations++;','      super(token);\n      this.#channel = channel;\n      errorReservations++;',1),
 op('this.permit','this.#permit',6),op('this.failed','this.#failed',2),op('this.errorCredit','this.#errorCredit',5),op('this.channel','this.#channel',1),
 op('private writable()','#writable()',1),op('this.writable(','this.#writable(',5)],
 'TS4094: TypeScript `private`/`protected` members cannot appear in the declaration of an exported anonymous class type; ECMAScript private names (`#x`) can and are enforced at run time.')
fix('capacity-tests-spread',L+'⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/🟦️.ts',[op(
 'const sdkCapacityApi=async()=>Object.assign({},await import("../📥️physical/🟦️.ts"),await import("../🧮️formula/🟦️.ts"));',
 'const sdkCapacityApi=async()=>({...await import("../📥️physical/🟦️.ts"),...await import("../🧮️formula/🟦️.ts")});',1)],
 '`Object.assign({}, await import(m))` types as `{}` when `m` is a declaration file (TypeScript 5.9.3); spread keeps the namespace members.')
fix('capacity-derivative-tests-spread',L+'⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts',[op(
 'const sdkCapacityApi=async()=>Object.assign({},await import("../../📥️physical/🟦️.ts"),await import("../../🧮️formula/🟦️.ts"));',
 'const sdkCapacityApi=async()=>({...await import("../../📥️physical/🟦️.ts"),...await import("../../🧮️formula/🟦️.ts")});',1)],
 'same as capacity-tests-spread.')
fix('source-demand-typed-roots',L+'🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts',[
 op('import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";','import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";\ntype SourceDemandRootV1={readonly target:string;readonly runtime:readonly string[];readonly compile:readonly string[]};',1),
 op('row.roots.','(row.roots as readonly SourceDemandRootV1[]).',4)],
 'The JSON `roots` of the traversal cases is a union of three anonymous array types; calling `map`/`flatMap` on it is resolved differently depending on the checker order of the program (monolithic reports TS2769/TS2345 at two sites, any smaller program none). One declared element type makes both agree.')
E='🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/'
fix('plugin-turn-dispatch-types-exported',E+'🔌️PluginRuntime/🧵️turn-dispatch/🟦️.ts',[
 op('interface ThunkTurnPayload {','export interface ThunkTurnPayload {',1),op('interface PluginTurnWaiter {','export interface PluginTurnWaiter {',1),op('interface PluginTurnPayload {','export interface PluginTurnPayload {',1),
 op('type PluginLifecycleWork =','export type PluginLifecycleWork =',1),op('type PluginLifecycleTurnResult =','export type PluginLifecycleTurnResult =',1),op('type PluginLifecycleTurnPayload =','export type PluginLifecycleTurnPayload =',1),
 op('type PendingPluginTurn =','export type PendingPluginTurn =',1),op('type CrossingCensusRowV1 =','export type CrossingCensusRowV1 =',1)],
 'TS4058: exported schedulers, maps and `pluginRuntimeTestDependenciesV1` are typed with these module-private declarations, so no declaration file can name them.')
fix('typedwire-produce-nameable',E+'📃️UiDocumentStore/🧪️tests/🪛️typedwire-helpers/🟦️.tsx',[
 op('  const { produce } = await import("immer");','  const { produce: immerProduce } = await import("immer");\n  const produce = <Base,>(base: Base, recipe: (draft: import("immer").Draft<Base>) => void | Base): Base => immerProduce(base, recipe as never) as Base;',1)],
 'TS4058: immer\'s `IProduce` is not exported from the package entry, so a helper returning `produce` has an unnameable type; a generic wrapper over the exported `Draft` type is nameable and keeps the two-argument form every test uses.')
fix('gis-vitest-config-annotated','✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🎚️config/🧵️gis-inference-bridge/🟦️.ts',[
 op('export default {...base,test:{...base.test,include:[resolve(dirname(fileURLToPath(import.meta.url)),"../../🧵️gis-inference-bridge/🟦️.ts")],includeSource:[]}};','const config: typeof base = {...base,test:{...base.test,include:[resolve(dirname(fileURLToPath(import.meta.url)),"../../🧵️gis-inference-bridge/🟦️.ts")],includeSource:[]}};\nexport default config;',1)],
 'TS2742/TS4082: the spread of the shared vitest configuration inlines vitest-internal option types; the declared type of the shared configuration is nameable.')
fix('cad-ports-line-annotated','✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🔌️ports/🟦️.ts',[
 op('export const { Line, Text } = sceneHostPort.drei;','export const { Text } = sceneHostPort.drei;\nexport const Line: typeof import("@react-three/drei").Line = sceneHostPort.drei.Line;',1)],
 'TS2742: drei `Line` is typed through three-stdlib\'s nested store path; the package-level type is nameable.')
fix('manifest-generated-exports','🧰️framework/🔨️modules/🛂️manifest/package.json',[
 op('    "./generated/ui-axes": "./🤖️generated/🎚️ui-axes/🟦️.ts",\n','    "./generated/manifest": "./🤖️generated/🪪️manifest/🟦️.ts",\n    "./generated/ui-axes": "./🤖️generated/🎚️ui-axes/🟦️.ts",\n    "./generated/ui-contract": "./🤖️generated/📜️ui-contract/🟦️.ts",\n',1)],
 'TS2742 (25 sites in the os renderer hooks): the generated manifest and ui-contract entries are public types of `@semio-tech/framework-manifest` but only the third generated entry had an export, so no bare specifier can name them. Verified in the overlay only together with a manifest that makes the package visible to the renderer (see the report): exports alone do not clear it.')
json.dump(fixes,open(os.path.join(HERE,'staged2','fixes.json'),'w'),ensure_ascii=False,indent=1)
print(len(fixes),'fixes',sum(len(f['ops']) for f in fixes),'ops')

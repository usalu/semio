/** 🛡️ Decides whether a cacheable task is a hermetic action: it carries the neutral declaration or a hermetic class selects it, its command is one the runner interprets, its host inputs are only environment values the class table admits and commands the toolchain covers, and it declares outputs; anything else keeps host state with its reason. A task that names its own declaration is a bare hermetic action with no class. */
export function bazelHermeticDecisionV1(task:BazelTaskV1,cache:{cache:boolean},inputs:readonly unknown[]|(()=>readonly unknown[])):BazelHermeticDecisionV1{
 const semio=(task.definition as {metadata?:{semio?:{hermetic?:unknown;execution?:{kind?:string}}}}).metadata?.semio,execution=task.definition.execution,selected=semio?.hermetic===undefined?hermeticSelectV1({project:task.project,target:task.target,root:task.root}):undefined;
 if(task.definition.continuous)return{hermetic:false,reason:"continuous"};
 if(semio?.execution?.kind==="orchestration")return{hermetic:false,reason:"orchestration"};
 if(selected?.kind==="excepted")return{hermetic:false,reason:"excepted"};
 if(semio?.hermetic===undefined&&selected?.kind!=="class")return{hermetic:false,reason:"undeclared"};
 if(semio?.hermetic!==undefined&&validateJsonSchemaSubset(hermeticSchema,semio.hermetic,hermeticSchema).length)throw new Error(`Closed hermetic declaration required: ${task.id}`);
 const commands=execution.commands??(execution.command===undefined?[]:[execution.command]),text=JSON.stringify([commands,execution.cwd,execution.env,task.definition.outputs]);
 if(execution.kind!=="command"||execution.program!==undefined||execution.ownerArguments!==undefined||execution.readyWhen!==undefined||!commands.length||commands.some(command=>typeof command!=="string")||/\{options\.|\{env\./u.test(text)||Object.values(execution.env??{}).some(value=>typeof value!=="string"))return{hermetic:false,reason:"unsupported-execution"};
 const rows=typeof inputs==="function"?inputs():inputs;
 if(selected?.kind==="class"?"refused" in hermeticInputsV1(rows,{}):rows.some(input=>typeof input==="object"&&input!==null&&("runtime"in input||"env"in input)))return{hermetic:false,reason:"host-inputs"};
 if(!cache.cache||!Array.isArray(task.definition.outputs))return{hermetic:false,reason:"uncached"};
 return selected?.kind==="class"?{hermetic:true,class:selected.class.id}:{hermetic:true};
}

/** 🧭️ What the generator derives for one hermetic action of a class: the declared environment values, the port replacements and the dependency rows with their content-addressed tree. */
export interface BazelHermeticPlanV1{readonly inputEnv:Readonly<Record<string,string>>;readonly providers:Readonly<Record<string,string>>;readonly dependencies:HermeticDependenciesV1}

/** 🧭️ Derives the plan of one hermetic class member from its declared inputs: the environment values read now, the class's port replacements and the dependency selection of the installed generation; `undefined` keeps the task out of the hermetic classes because its host inputs or its installed dependencies cannot be declared. */
export function bazelHermeticPlanV1(options:Readonly<{root:string;task:BazelTaskV1;class:string;values:readonly unknown[];declared:readonly string[];environment:Readonly<Record<string,string|undefined>>;check?:()=>void}>):BazelHermeticPlanV1|undefined{
 const row=HERMETIC_TABLE_V1.classes.find(entry=>entry.id===options.class);if(!row)throw new Error(`Unknown hermetic class: ${options.class}`);
 const host=hermeticInputsV1(options.values,options.environment);if("refused" in host)return undefined;
 const dependencies=hermeticDependenciesV1({root:options.root,directories:[options.task.root==="."?"":options.task.root],names:[...row.packages,...host.packages],packages:row.dependencies==="packages",declared:options.declared,...(options.check?{check:options.check}:{})});
 return dependencies?{inputEnv:host.env,providers:row.providers as Readonly<Record<string,string>>,dependencies}:undefined;
}

/** 📋️ What the generator adds to one hermetic action's manifest beside its task: the declared input files and, for a class member, the class, the declared environment values, the port replacements and the `node_modules` rows. */
export interface BazelHermeticManifestDetailsV1{readonly inputs:readonly string[];readonly class?:string;readonly inputEnv?:Readonly<Record<string,string>>;readonly providers?:Readonly<Record<string,string>>;readonly rows?:readonly HermeticModulesRowV1[]}

/** 🗝️ Builds the closed, content-stable manifest the hermetic runner interprets; it holds no path, environment or implementation identity, only workspace-relative declarations. */
export function bazelHermeticManifestV1(task:BazelTaskV1,args:readonly string[],details:BazelHermeticManifestDetailsV1={inputs:[]}):Record<string,unknown>{
 const execution=task.definition.execution,commands=(execution.commands??(execution.command===undefined?[]:[execution.command])) as string[],manifest={version:1,id:task.id,project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{}),root:task.root,...(execution.cwd===undefined?{}:{cwd:execution.cwd}),args:[...args],...(execution.forwardAllArgs===undefined?{}:{forwardAllArgs:execution.forwardAllArgs}),...(execution.parallel===undefined?{}:{parallel:execution.parallel}),commands,...(execution.env===undefined?{}:{env:{...execution.env}}),outputs:[...(task.definition.outputs??[])],inputs:[...details.inputs],...(details.class===undefined?{}:{class:details.class,entry:HERMETIC_TABLE_V1.runtime.entry,markers:[...HERMETIC_TABLE_V1.runtime.markers],maximumElapsedMilliseconds:HERMETIC_TABLE_V1.runtime.maximumElapsedMilliseconds,providers:{...(details.providers??{})}}),...(details.inputEnv&&Object.keys(details.inputEnv).length?{inputEnv:{...details.inputEnv}}:{}),...(details.rows?.length?{nodeModules:details.rows.map(row=>({path:row.path,workspace:{...row.workspace},tree:{...row.tree},bin:{...row.bin}}))}:{})};
 if(validateJsonSchemaSubset(schema.definitions.Manifest,manifest,schema).length)throw new Error(`Closed hermetic task manifest required: ${task.id}`);
 return manifest;
}

/** 🌳️ Builds the closed manifest of one dependency tree: the installed stores it copies from and the `<store index>/<identity>` entries it holds. */
export function bazelHermeticModulesManifestV1(dependencies:Pick<HermeticDependenciesV1,"stores"|"ids">):{version:1;stores:string[];ids:string[]}{
 const manifest={version:1 as const,stores:[...dependencies.stores],ids:[...dependencies.ids]};
 if(validateJsonSchemaSubset(schema.definitions.DependenciesManifest,manifest,schema).length)throw new Error("Closed hermetic dependency manifest required");
 return manifest;
}

/** 🏷️ Names a dependency tree by the digest of its store identities alone, never of the installed paths, so every checkout that selects the same entries names the tree, and therefore keys the consuming actions, identically. */
export function bazelHermeticModulesLabelV1(dependencies:Pick<HermeticDependenciesV1,"ids">):string{return "m_"+createHash("sha256").update(JSON.stringify(dependencies.ids)).digest("hex").slice(0,16);}


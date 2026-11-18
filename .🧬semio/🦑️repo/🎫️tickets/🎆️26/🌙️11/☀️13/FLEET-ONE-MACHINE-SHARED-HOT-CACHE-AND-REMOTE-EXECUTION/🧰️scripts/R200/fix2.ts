import {plugin} from "bun";
const OLD="  if (files === undefined) groups.set(name, files = new Set((await toolClosureGroupV1(workspaceRoot, scripts, name)).filter(item => typeof item === \"string\")));\n  return files;";
const NEW="  if (files === undefined) { const pending = toolClosureGroupV1(workspaceRoot, scripts, name).then(list => new Set(list.filter(item => typeof item === \"string\"))); groups.set(name, pending); pending.catch(() => { if (groups.get(name) === pending) groups.delete(name); }); files = pending; }\n  return files;";
const OLD2="for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === \"string\" && item.startsWith(\"{workspaceRoot}/\")) files.push(item);";
const NEW2="for (const item of await sharedGroupV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === \"string\" && item.startsWith(\"{workspaceRoot}/\")) files.push(item);";
const HELP=String.raw`
async function sharedGroupV1(recipe, workspaceRoot, scripts) {
  if (recipe.fn !== "tool-closure" && recipe.fn !== "script-provider") return produceClosureRecipeV1(recipe, workspaceRoot, scripts);
  let index = 0;
  const key = closureRecipeKeyV1(recipe), cache = scripts.groupLists ??= new Map(), flights = scripts.groupFlights ??= new Map();
  const run = () => memoizedObservedAsyncComputationV1(cache, key, () => { scripts.groupComputations = (scripts.groupComputations ?? 0) + 1; return produceClosureRecipeV1(recipe, workspaceRoot, scripts); }, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force));
  const flight = flights.get(key);
  if (flight !== undefined) { await flight.catch(() => {}); return run(); }
  const mine = run();
  flights.set(key, mine);
  try { return await mine; } finally { if (flights.get(key) === mine) flights.delete(key); }
}
`;
const CLEAR1="scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); }";
const CLEAR2="scripts.commandDependencies.clear(); scripts.declared.clear();";
plugin({name:"r200-fix",setup(b){b.onLoad({filter:/\.mjs$/},async({path})=>{
 let t=await Bun.file(path).text();
 if(/script-inputs\/[^/]*\.mjs$/.test(path)){
  if(process.env.NOFIX!=="1"){
   for(const a of [OLD,OLD2,CLEAR1,CLEAR2])if(!t.includes(a))throw new Error("anchor missing: "+a.slice(0,50));
   t=t.replace(OLD,NEW).replace(OLD2,NEW2).replace(CLEAR1,"scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.groupLists?.clear(); }").replace(CLEAR2,"scripts.commandDependencies.clear(); scripts.groupLists?.clear(); scripts.declared.clear();")+HELP;
  }
  const A="async function closurePackagesV1(closure, workspaceRoot, graph, checkpoint) {";t=t.replace(A,A+" (globalThis.__n??=[]).push(closure.length);");
  const B="async function toolClosureGroupV1(workspaceRoot, scripts, name) {";t=t.replace(B,B+" (globalThis.__tg??=[]).push(name);");
 }
 return {contents:t,loader:"js"};});}});

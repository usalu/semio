import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const policy=schema.definitions.Policy.const,hash=(value:string)=>createHash("sha256").update(value).digest("hex").slice(0,16);

/** 🕸️ The part of the build graph `affected` needs: project names and the edges `source depends on target`. */
export type BazelAffectedGraphV1=Readonly<{projects:Readonly<Record<string,unknown>>;dependencies:readonly Readonly<{source:string;target:string}>[]}>;

/** 🏷️ Names a project node or a changed-file node of the generated package with an ASCII label. */
export function bazelAffectedLabelV1(kind:"p"|"f",value:string):string{return `${kind}_${hash(value)}`;}

/** 🧱️ Renders the generated package: a node per project that lists its dependencies and the changed files touching it, and a leaf node per changed file. */
export function bazelAffectedBuildV1(graph:BazelAffectedGraphV1,touches:ReadonlyMap<string,readonly string[]>):string{
 if(validateJsonSchemaSubset(schema.definitions.Touches,Object.fromEntries(touches),schema).length)throw Error("Closed affected touches required");
 const dependencies=new Map<string,string[]>(),order=(left:string,right:string)=>left<right?-1:left>right?1:0,quote=(value:string)=>JSON.stringify(value);
 for(const edge of graph.dependencies)if(graph.projects[edge.source]&&graph.projects[edge.target])dependencies.set(edge.source,[...dependencies.get(edge.source)??[],edge.target]);
 const files=[...new Set([...touches.values()].flat())].sort(order),lines:string[]=[];
 for(const name of Object.keys(graph.projects).sort(order)){
  const sources=[...(dependencies.get(name)??[]).map(dependency=>":"+bazelAffectedLabelV1("p",dependency)),...(touches.get(name)??[]).map(file=>":"+bazelAffectedLabelV1("f",file))].sort(order);
  lines.push(`filegroup(name = ${quote(bazelAffectedLabelV1("p",name))}, srcs = [${sources.map(quote).join(", ")}])`);
 }
 for(const file of files)lines.push(`filegroup(name = ${quote(bazelAffectedLabelV1("f",file))})`);
 return lines.join("\n")+"\n";
}

/** 🔎️ The one `rdeps` expression whose answer contains every project reachable from a changed file; it names the generated package's own project nodes as the universe. */
export function bazelAffectedExpressionV1(touches:ReadonlyMap<string,readonly string[]>):string{
 const files=[...new Set([...touches.values()].flat())].sort();
 if(!files.length)throw Error("Changed files required");
 return `rdeps(//:all, set(${files.map(file=>"//:"+bazelAffectedLabelV1("f",file)).join(" ")}))`;
}

/** 📥️ Reads the query output back into sorted project names, ignoring the changed-file nodes. */
export function bazelAffectedProjectsFromOutputV1(graph:BazelAffectedGraphV1,output:string):string[]{
 const lines=output.split("\n").map(line=>line.trim()).filter(Boolean);
 if(validateJsonSchemaSubset(schema.definitions.Answer,lines,schema).length)throw Error("Closed affected answer required");
 const names=new Map(Object.keys(graph.projects).map(name=>["//:"+bazelAffectedLabelV1("p",name),name]));
 return lines.flatMap(line=>names.has(line)?[names.get(line)!]:[]).sort();
}

/** 📦️ The fixed files and arguments of the generated package. */
export function bazelAffectedWorkspaceV1():Readonly<{module:string;options:readonly string[];directory:string}>{return Object.freeze({module:`module(name = "${policy.module}", version = "0.0.0")\n`,options:policy.options,directory:policy.workspace});}

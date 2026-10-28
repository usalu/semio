/** 🔬️ Retain owned central header-command output for independent STEP/header comparison. */
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const ticket=import.meta.dir,root=resolve(ticket,"../../../../../../.."),base=resolve(root,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any");
const {ifc4x3MvdHeaderMutationAsync}=await import(pathToFileURL(resolve(base,"🧬️schema/👁️mvd/header.ts")).href);
const {applyIfcMutationAsync}=await import(pathToFileURL(resolve(base,"🧬️schema/🧬️mutations/🟦️.ts")).href);
const {writeIfc4x3Step}=await import(pathToFileURL(resolve(base,"🚪️io/step.ts")).href);
const cases=await Bun.file(resolve(base,"🧬️schema/👁️mvd/header-cases.json")).json();
const string=(value:string)=>({kind:"string",value}),aggregate=(values:string[])=>({kind:"aggregate",value:values.map(string)}),facts=[];
for(const row of cases.cases){const snapshot={schema:"stdio.ifc",header:{fileDescription:[aggregate(row.before),string("2;1")],fileName:[string("mvd.ifc"),string("2026-10-21T12:00:00"),aggregate(["Author"]),aggregate(["Organisation"]),string("Writer"),string("Application"),string("Authorisation")],fileSchema:[aggregate(["IFC4X3_ADD2"])]},entities:[]};const mutation=await ifc4x3MvdHeaderMutationAsync(snapshot.header.fileDescription,{view:row.view}),next=await applyIfcMutationAsync(snapshot,mutation);facts.push({name:row.name,description:next.header.fileDescription[0].value.map((item:{value:string})=>item.value),implementationLevel:next.header.fileDescription[1].value,step:await writeIfc4x3Step(next)});}
await Bun.write(resolve(ticket,"🗑️generated/mvd-header-facts.json"),JSON.stringify(facts,null,2));
console.log(JSON.stringify({cases:facts.length}));

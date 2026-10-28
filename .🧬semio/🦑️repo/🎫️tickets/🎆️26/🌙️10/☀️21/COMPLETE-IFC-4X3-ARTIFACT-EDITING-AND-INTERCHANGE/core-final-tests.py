from pathlib import Path
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any')
p=R/'🧬️schema/🔺️diff/🟦️.ts';s=p.read_text(encoding='utf-8').replace(' const ids=new Set<bigint>();for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!;',' const valueLimit=limit(options.maxValues,10_000_000);for(const lane of[snapshot.header.fileDescription,snapshot.header.fileName,snapshot.header.fileSchema])if(lane.length>valueLimit)fail("valueLimit","header roots exceed value admission");\n const ids=new Set<bigint>();for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!;if(entity.args.length>valueLimit||entity.complex.length>valueLimit)fail("valueLimit","entity roots exceed value admission");')
s=s.replace('for(let j=0;j<entity.complex.length;j++){if(!(yield* identifierSteps','for(let j=0;j<entity.complex.length;j++){if(entity.complex[j]!.args.length>valueLimit)fail("valueLimit","complex roots exceed value admission");if(!(yield* identifierSteps');p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🧪️tests/🧬️editing/🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

import {absorbIfcDiffAsync,composeIfcDiffAsync} from "../../🧬️schema/🔺️diff/🟦️.ts";
test("asynchronous sparse composition retains independent patch output and can cancel sorting",async()=>{
 const before=graph(),first=ifcMutationDiff(before,{mutation:"setEntityArg",id:1n,index:0,value:{kind:"string",value:"Edited"}}),second={entities:{modified:[{id:1n,diff:{args:{added:[{index:1,value:integer(8)}]}}}]}};
 const combined=await composeIfcDiffAsync(before,first,second);expect(applyIfcDiff(before,combined)).toEqual(applyIfcDiff(applyIfcDiff(before,first),second));expect(wire(applyIfcDiff(before,await absorbIfcDiffAsync({},first)))).toEqual(applyPatch(wire(before),fixture.jsonPatch as Parameters<typeof applyPatch>[1]).newDocument);
 const controller=new AbortController();await expect(absorbIfcDiffAsync(first,second,{yieldEvery:1,signal:controller.signal,onProgress:p=>{if(p.phase==="applyEntities")controller.abort();}})).rejects.toThrow(/cancelled/);
});
test("late cancellation leaves all history lanes and immutable internal payloads unchanged",async()=>{
 const before=graph(),history=await IfcEditHistory.create(before),mutation={mutation:"setEntityName" as const,id:1n,name:"IFCWALL"};let enteredCopy=false;const controller=new AbortController();await expect(history.executeAsync(mutation,0,{yieldEvery:1,signal:controller.signal,onProgress:p=>{if(p.phase==="copyOwned"&&p.completed>=20){enteredCopy=true;controller.abort();}}})).rejects.toThrow(/cancelled/);expect(enteredCopy).toBe(true);expect(history.revision).toBe(0);expect(history.events).toEqual([]);expect(history.snapshot).toEqual(before);expect(await history.undoAsync(0)).toBeUndefined();expect(await history.redoAsync(0)).toBeUndefined();
 const event=(await history.executeAsync(mutation,0))!;event.diff.entities!.modified![0]!.diff.name="IFCSLAB";expect(history.events[0]!.diff.entities!.modified![0]!.diff.name).toBe("IFCWALL");await history.undoAsync(1);expect(history.snapshot).toEqual(before);
});
test("tiny budgets reject header roots and proposed values before wide scalar reads",async()=>{
 let reads=0;const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==="string"&&/^\\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}}),before=graph();before.entities=[];before.header.fileName=children;
 expect(()=>applyIfcDiff(before,{}, {maxValues:3})).toThrow(/valueLimit/);expect(reads).toBe(0);before.header.fileName=[];before.entities=[{id:1n,name:"IFCTEST",args:[],complex:[]}];await expect(ifcMutationDiffAsync(before,{mutation:"insertEntityArg",id:1n,index:0,value:{kind:"aggregate",value:children}},{maxValues:3})).rejects.toThrow(/valueLimit/);expect(reads).toBe(0);
});
''';p.write_text(s,encoding='utf-8',newline='\n')

from pathlib import Path
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any')
p=R/'🧬️schema/🔺️diff/🟦️.ts';s=p.read_text(encoding='utf-8').replace('}const owned=yield* copyIfcOwnedSteps(result,options);yield* admitIfcDiffSteps(owned,options);return owned;','}yield* admitIfcDiffSteps(result,options);return result;');s=s.replace('/** 🔺️ Bounded, owned IFC graph transactions shared by commands, history and replay. */','/** 🔺️ Bounded, owned IFC graph transactions. Capture may observe concurrent caller changes; only captured owned data is admitted. Keep inputs unchanged until completion when exact start-state capture is required. */');p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🧬️schema/🧬️mutations/🟦️.ts';s=p.read_text(encoding='utf-8').replace('diff=yield* mutationPlan(snapshot,mutation,options);return yield* copyIfcOwnedSteps(diff,options);','diff=yield* mutationPlan(snapshot,mutation,options);return diff;').replace(' }return yield* copyIfcOwnedSteps(result,options);',' }return result;');p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🧪️tests/🧬️editing/🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

test("captured sparse diffs own their payloads without rereading caller inputs",async()=>{
 const before=graph(),value=integer(9),first={entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value}]}}}]}};const absorbed=await absorbIfcDiffAsync(first,{});absorbed.entities!.modified![0]!.diff.args!.modified![0]!.value={kind:"integer",value:10n};expect(first.entities.modified[0]!.diff.args.modified[0]!.value).toEqual(integer(9));
 const planned=await ifcMutationDiffAsync(before,{mutation:"setEntityArg",id:1n,index:0,value});planned.entities!.modified![0]!.diff.args!.modified![0]!.value={kind:"integer",value:11n};expect(value).toEqual(integer(9));
});
''';p.write_text(s,encoding='utf-8',newline='\n')

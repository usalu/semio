from pathlib import Path
import json
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any')
p=R/'🧪️tests/🧬️editing/🔣️.json';d=json.loads(p.read_text(encoding='utf-8'));d['omittedComplex']={'id':'1','name':'IFCTEST','args':[{'kind':'integer','value':'7'}]};p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
p=R/'🧪️tests/🧬️editing/🥒️.feature';s=p.read_text(encoding='utf-8');s+='''

  Scenario: Omitted complex constituents mean the canonical empty collection
    Given an IFC entity whose optional complex field is omitted
    When the entity is captured edited and restored by its inverse
    Then its complex collection is empty
    And its transport validates against the native snapshot schema
    And independent JSON Patch agrees with the canonical graph
''';p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🧪️tests/🧬️editing/🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

test("omitted complex fields admit native transport and restore canonical empty collections",async()=>{
 const before=graph();before.entities=[{id:BigInt(fixture.omittedComplex.id),name:fixture.omittedComplex.name,args:fixture.omittedComplex.args.map(value=>({kind:"integer",value:BigInt(value.value)}))}];const{default:Ajv}=await import("ajv/dist/2020.js"),validate=new Ajv({strict:false}).compile(await Bun.file(new URL("../../🧬️schema/🔣️.json",import.meta.url)).json());expect(validate(wire(before))).toBe(true);
 const canonical=applyIfcDiff(before,{}),mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:integer(8)},diff=ifcMutationDiff(before,mutation),after=applyIfcMutation(before,mutation);expect(canonical.entities[0]!.complex).toEqual([]);expect(after.entities[0]!.complex).toEqual([]);expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(canonical);
 const oracle=applyPatch(wire(before),[{op:"add",path:"/entities/0/complex",value:[]},{op:"replace",path:"/entities/0/args/0/value",value:"8"}]).newDocument;expect(wire(after)).toEqual(oracle);const history=await IfcEditHistory.create(before);await history.executeAsync(mutation,0);await history.undoAsync(1);expect(history.snapshot).toEqual(canonical);
});
''';p.write_text(s,encoding='utf-8',newline='\n')

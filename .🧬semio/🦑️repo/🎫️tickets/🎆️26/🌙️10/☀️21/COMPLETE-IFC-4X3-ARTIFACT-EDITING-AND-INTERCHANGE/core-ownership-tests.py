from pathlib import Path
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/🧬️editing')
p=R/'🥒️.feature';s=p.read_text(encoding='utf-8');s+='''

  Scenario: Caller mutation during capture cannot publish an invalid owned graph
    Given a valid snapshot and a mutable proposed integer value
    When a progress callback replaces that integer with a string during owned capture
    Then the operation either refuses or returns a structurally valid owned graph
    And history never publishes an invalid snapshot or consumes undo state
  Scenario: A recorded command is the captured command that produced its diff
    Given a command whose caller-owned value changes during cooperative capture
    When a valid event is published
    Then its recorded command and replay diff reproduce its owned snapshot
  Scenario: Base-free absorption admits native coordinate and identity domains
    Given a diff with NaN, negative or duplicate coordinates or overlapping targets
    When sparse diffs are absorbed
    Then admission rejects the malformed diff before algebra coalesces its targets
''';p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

test("caller mutation during copy cannot escape final owned structural admission",async()=>{
 for(const asynchronous of[false,true]){const before=graph();let changed=false;const options={yieldEvery:1,onProgress:(p:any)=>{if(p.phase==="copyOwned"&&!changed){changed=true;(before.entities[0]!.args[0] as any).kind="integer";(before.entities[0]!.args[0] as any).value="invalid";}}};if(asynchronous)await expect(applyIfcDiffAsync(before,{},options)).rejects.toThrow(/invalidValue/);else expect(()=>applyIfcDiff(before,{},options)).toThrow(/invalidValue/);expect(changed).toBe(true);}
});
test("history refuses capture corruption and records its captured command consistently",async()=>{
 const before=graph(),history=await IfcEditHistory.create(before),value:any=integer(2);let changed=false;await expect(history.executeAsync({mutation:"setEntityArg",id:1n,index:0,value},0,{yieldEvery:1,onProgress:p=>{if(p.phase==="copyOwned"&&!changed){changed=true;value.value="invalid";}}})).rejects.toThrow(/invalidValue/);expect(history.revision).toBe(0);expect(history.events).toEqual([]);expect(history.snapshot).toEqual(before);expect(await history.undoAsync(0)).toBeUndefined();
 const valid:any=integer(3);let mutated=false;const event=(await history.executeAsync({mutation:"setEntityArg",id:1n,index:0,value:valid},0,{yieldEvery:1,onProgress:p=>{if(p.phase==="copyOwned"&&!mutated){mutated=true;valid.value=4n;}}}))!;expect(applyIfcMutation(before,event.mutation!)).toEqual(history.snapshot);expect(applyIfcDiff(before,event.diff)).toEqual(history.snapshot);expect(applyIfcDiff(history.snapshot,event.inverse)).toEqual(before);
});
test("base-free absorption refuses malformed coordinate identity and overlap domains",async()=>{
 for(const diff of[
  {entities:{modified:[{id:1n,diff:{args:{removed:[NaN]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{removed:[-1]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{added:[{index:0,value:integer(1)},{index:0,value:integer(2)}]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{removed:[0],modified:[{index:0,value:integer(2)}]}}}]}},
  {entities:{modified:[{id:0n,diff:{name:"IFCWALL"}}]}},
  {entities:{modified:[{id:1n,diff:{name:"invalid name"}}]}},
  {entities:{removed:[{id:1n,index:0},{id:1n,index:0}]}},
  {entities:{removed:[{id:1n,index:0}],modified:[{id:1n,diff:{name:"IFCWALL"}}]}},
 ])await expect(absorbIfcDiffAsync(diff as any,{})).rejects.toThrow();
});
''';p.write_text(s,encoding='utf-8',newline='\n')

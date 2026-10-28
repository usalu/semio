from pathlib import Path
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any')
p=R/'🧪️tests/🧬️editing/🥒️.feature';s=p.read_text(encoding='utf-8');s+='''

  Scenario: A tiny value budget refuses a wide aggregate before visiting every child
    Given an aggregate with 100000 children and a maximum of 3 values
    When a diff or mutation is admitted
    Then at most 3 child values have been read
    And the original snapshot and all history state are unchanged
  Scenario: Public mutable results own every returned graph value
    Given an IFC graph and an unchanged diff
    When the returned graph is mutated
    Then the original graph is unchanged
  Scenario: Concurrent asynchronous commands publish one revision atomically
    Given two commands targeting the same original revision
    When both commands finish cooperative preparation
    Then exactly one event is published
    And the other command refuses its stale revision
  Scenario: Cancellation during undo redo or snapshot reading preserves history
    Given an event history with undo and redo entries
    When a cooperative operation is cancelled
    Then snapshot revision events undo and redo entries are unchanged
''';p.write_text(s,encoding='utf-8',newline='\n')
p=R/'🧪️tests/🧬️editing/🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

test("tiny value admission never expands a wide frontier before refusing",()=>{
 let reads=0;const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==="string"&&/^\\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}}),before=graph();before.entities[0]!.args=[{kind:"aggregate",value:children}];
 expect(()=>applyIfcDiff(before,{}, {maxValues:3})).toThrow(/valueLimit/);expect(reads).toBeLessThanOrEqual(3);
 const replacement={kind:"aggregate" as const,value:children};reads=0;expect(()=>applyIfcMutation(graph(),{mutation:"setEntityArg",id:1n,index:0,value:replacement},{maxValues:3})).toThrow();expect(reads).toBeLessThanOrEqual(3);
});
test("direct graph results own every mutable branch",()=>{
 const before=graph(),saved=structuredClone(before),after=applyIfcDiff(before,{});after.entities[0]!.name="IFCWALL";after.entities[0]!.args[0]={kind:"string",value:"Mutated"};after.entities[0]!.complex[0]!.args.push(integer(5));after.header.fileDescription.push(integer(6));expect(before).toEqual(saved);
});
''';p.write_text(s,encoding='utf-8',newline='\n')

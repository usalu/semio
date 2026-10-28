from pathlib import Path
path=Path(__file__).parent.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🧪️tests/session.test.ts"
source=path.read_text(encoding="utf8")
addition="\ntest('retired cursor cleanup is inert during a later observer',()=>{let retired:SolidSteps<SolidMesh>|undefined;const session=create({tick:()=>{if(retired){expect(retired.next().done).toBe(true);expect(retired.return(undefined as unknown as SolidMesh).done).toBe(true);let caught=false;try{retired.throw('retired-only');}catch(error){caught=true;expect(error).toBe('retired-only');}expect(caught).toBe(true);}}});const first=session.booleanSteps('union',box(cases.cases[0]!.host),box(cases.cases[0]!.host));drain(first);retired=first;expect(volume(drain(session.booleanSteps('intersection',box(cases.cases[0]!.host),box(cases.cases[0]!.host))))).toBeCloseTo(64,8);session.close();});\n"
assert "retired cursor cleanup is inert during a later observer"not in source
path.write_text(source+addition,encoding="utf8")


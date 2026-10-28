from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts')
s=p.read_text(encoding='utf-8')
def change(old,new):
 global s
 assert old in s,old[:100]
 s=s.replace(old,new)
change('interface Polygon{vertices:SolidPoint[];plane:Plane;}', 'interface Polygon{vertices:SolidPoint[];plane:Plane;}\nconst plane=(control:Control|undefined,normal:SolidPoint,w:number):Plane=>control?.arena?control.arena.born(1,()=>({normal,w})):({normal,w});')
change('plane:{normal:row(control,()=>mul(p.plane.normal,-1)),w:-p.plane.w}', 'plane:plane(control,row(control,()=>mul(p.plane.normal,-1)),-p.plane.w)')
change('node.plane={normal:row(this.control,()=>mul(node.plane!.normal,-1)),w:-node.plane.w}', 'node.plane=plane(this.control,row(this.control,()=>mul(node.plane!.normal,-1)),-node.plane.w)')
change('function* admitSteps', 'function* validationSteps(mesh:SolidMesh,control:Control):SolidSteps{const parent=control.arena,arena=new SolidArena(control.workspace);control.arena=arena;try{yield* solidMeshSteps(mesh,control);}finally{control.arena=parent;arena.close();}}\nfunction* admitSteps')
change('yield* solidMeshSteps(yield* normalizedSteps(mesh,origin,extent,control),control);', 'const parent=control.arena,arena=new SolidArena(control.workspace);control.arena=arena;try{yield* solidMeshSteps(yield* normalizedSteps(mesh,origin,extent,control),control);}finally{control.arena=parent;arena.close();}')
change('yield* solidMeshSteps(mesh,control);return mesh;', 'yield* validationSteps(mesh,control);return mesh;')
p.write_text(s,encoding='utf-8')

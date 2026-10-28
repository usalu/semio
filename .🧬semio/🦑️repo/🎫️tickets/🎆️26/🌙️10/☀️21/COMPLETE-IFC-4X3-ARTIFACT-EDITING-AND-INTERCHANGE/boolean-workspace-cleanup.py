from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts')
s=p.read_text(encoding='utf-8')
def change(old,new):
 global s
 assert old in s,old[:120]
 s=s.replace(old,new)
change('const token={},vertices:SolidPoint[]=[],triangles:SolidTriangle[]=[];let points=0,faces=0,closed=false;', 'const token={};let vertices:SolidPoint[]|undefined=[],triangles:SolidTriangle[]|undefined=[],points=0,faces=0,closed=false;')
change('index(at,vertexCount,vertices)', 'index(at,vertexCount,vertices!)')
change('index(at,triangleCount,triangles)', 'index(at,triangleCount,triangles!)')
change('vertices[at]=[x,y,z]', 'vertices![at]=[x,y,z]')
change('triangles[at]=[a,b,c]', 'triangles![at]=[a,b,c]')
change('this.#handle({vertices,triangles},lease);closed=true;', 'this.#handle({vertices:vertices!,triangles:triangles!},lease);vertices=undefined;triangles=undefined;closed=true;')
change('vertices.length=0;triangles.length=0;', 'vertices=undefined;triangles=undefined;')
change('let arena:SolidArena|undefined;const control=this.#control,check=()=>{for(const input of inputs)this.#mesh(input);},source=function*():SolidSteps<SolidMesh>{check();arena=new SolidArena(control.workspace);control.arena=arena;return yield* steps;},cleanup=()=>{control.arena=undefined;arena?.close();arena=undefined;};', 'let arena:SolidArena|undefined,pending:SolidValueReservation|undefined;const control=this.#control,check=()=>{for(const input of inputs)this.#mesh(input);},source=function*():SolidSteps<SolidMesh>{arena=new SolidArena(control.workspace,!owned);control.arena=arena;const mesh=yield* steps;if(owned)pending=yield* arena.takeSteps([mesh.vertices,mesh.triangles]);return mesh;},cleanup=()=>{if(pending){control.workspace.release(pending);pending=undefined;}if(arena){if(control.arena===arena)control.arena=undefined;arena.close();arena=undefined;}};')
change('const lease=arena!.take([...mesh.vertices,...mesh.triangles]);let handle:SolidOwnedMesh;try{handle=this.#handle(mesh,lease);}catch(error){control.workspace.release(lease);throw error;}', 'const handle=this.#handle(mesh,pending!);pending=undefined;')
change('return handle;},cleanup);', 'return handle;},cleanup,check);')
change('cleanup:()=>void=()=>{}):SolidSteps<R>', 'cleanup:()=>void=()=>{},enter:()=>void=()=>{}):SolidSteps<R>')
change('this.#active=token;try{const result=cursor.next();', 'this.#active=token;try{enter();const result=cursor.next();')
change('throw:{value:(error:unknown)=>{this.#control.outside();', 'throw:{value:(error:unknown)=>{if(retired)throw error;this.#control.outside();')
change('function* admitSteps(mesh:SolidMesh,control:Control):SolidSteps{yield;', 'function* admitSteps(mesh:SolidMesh,control:Control):SolidSteps{yield;for(const point of mesh.vertices){yield;if(point.length!==3)fail("nonfinite-vertex");control.arena?.borrow(point,3);}for(const ids of mesh.triangles){yield;if(ids.length!==3)fail("invalid-triangle-index");control.arena?.borrow(ids,3);}')
p.write_text(s,encoding='utf-8')

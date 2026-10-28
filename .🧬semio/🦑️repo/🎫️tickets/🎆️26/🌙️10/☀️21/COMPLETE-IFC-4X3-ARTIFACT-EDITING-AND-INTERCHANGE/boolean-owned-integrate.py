from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts')
s=p.read_text(encoding='utf-8')
start=s.index('export class SolidBooleanEvaluation{')
end=s.index('\n/** 🧊️ Capture',start)
s=s[:start]+'''export interface SolidOwnedMesh{readonly vertexCount:number;readonly triangleCount:number;}
/** 🏗️ Closed source construction consumes one original prepaid backing reservation. */
export interface SolidMeshBuilder{point(index:number,x:number,y:number,z:number):void;triangle(index:number,a:number,b:number,c:number):void;seal():SolidOwnedMesh;close():void;}
interface OwnedMesh{mesh:SolidMesh;lease:SolidValueReservation;}
export class SolidBooleanEvaluation{
 readonly #control:Control;readonly #owned=new Map<SolidOwnedMesh,OwnedMesh>();#active?:object;#builder?:object;#closed=false;
 constructor(options:SolidBooleanOptions={}){this.#control=new Control(options);}
 get work():number{return this.#control.work;}
 get usedValues():number{return this.#control.workspace.used;}
 get peakValues():number{return this.#control.workspace.peak;}
 #prepare(builder=false):void{this.#control.outside();this.#control.ensure();if(this.#closed)this.#control.refuse("session-closed");if(this.#active||!builder&&this.#builder)this.#control.refuse("session-active");}
 #idle():void{this.#control.outside();if(this.#active||this.#builder)this.#control.refuse("session-active");}
 #handle(mesh:SolidMesh,lease:SolidValueReservation):SolidOwnedMesh{const handle=Object.freeze({vertexCount:mesh.vertices.length,triangleCount:mesh.triangles.length});this.#owned.set(handle,{mesh,lease});return handle;}
 #mesh(handle:SolidOwnedMesh):OwnedMesh{return this.#owned.get(handle)??this.#control.refuse("owned-mesh");}
 checkpoint():void{this.#prepare(true);this.#control.tick();}
 meshBuilder(vertexCount:number,triangleCount:number):SolidMeshBuilder{
  this.#prepare();if(!Number.isSafeInteger(vertexCount)||vertexCount<0||!Number.isSafeInteger(triangleCount)||triangleCount<0||!Number.isSafeInteger(3*(vertexCount+triangleCount)))this.#control.refuse('owned-mesh');
  let lease:SolidValueReservation;try{lease=this.#control.workspace.reserve(3*(vertexCount+triangleCount));}catch(error){return this.#control.poison(error);}const token={},vertices:SolidPoint[]=[],triangles:SolidTriangle[]=[];let points=0,faces=0,closed=false;this.#builder=token;
  const ready=()=>{this.#prepare(true);if(closed||this.#builder!==token)this.#control.refuse('owned-mesh');},index=(value:number,count:number,rows:unknown[])=>{if(!Number.isSafeInteger(value)||value<0||value>=count||Object.hasOwn(rows,value))this.#control.refuse('owned-mesh');};
  return Object.freeze({
   point:(at:number,x:number,y:number,z:number)=>{ready();index(at,vertexCount,vertices);if(!Number.isFinite(x)||!Number.isFinite(y)||!Number.isFinite(z))this.#control.refuse('owned-mesh');this.#control.tick();vertices[at]=[x,y,z];points++;},
   triangle:(at:number,a:number,b:number,c:number)=>{ready();index(at,triangleCount,triangles);if(![a,b,c].every(value=>Number.isSafeInteger(value)&&value>=0&&value<vertexCount))this.#control.refuse('owned-mesh');this.#control.tick();triangles[at]=[a,b,c];faces++;},
   seal:()=>{ready();if(points!==vertexCount||faces!==triangleCount)this.#control.refuse('owned-mesh');this.#control.tick();const handle=this.#handle({vertices,triangles},lease);closed=true;this.#builder=undefined;return handle;},
   close:()=>{this.#control.outside();if(closed)return;closed=true;if(this.#builder===token)this.#builder=undefined;this.#control.workspace.release(lease);vertices.length=0;triangles.length=0;}
  });
 }
 releaseMesh(handle:SolidOwnedMesh):void{this.#idle();const record=this.#mesh(handle);this.#owned.delete(handle);this.#control.workspace.release(record.lease);}
 publishMesh(handle:SolidOwnedMesh):SolidMesh{this.#prepare();const record=this.#mesh(handle);this.#owned.delete(handle);this.#control.workspace.release(record.lease);return record.mesh;}
 booleanSteps(operation:SolidBooleanOperation,left:SolidMesh,right:SolidMesh):SolidSteps<SolidMesh>{this.#prepare();this.#operation(operation);return this.#evaluate(booleanWithControlSteps(operation,left,right,this.#control),[],false);}
 halfSpaceSteps(left:SolidMesh,plane:SolidPlane):SolidSteps<SolidMesh>{this.#prepare();return this.#evaluate(halfSpaceWithControlSteps(left,plane,this.#control),[],false);}
 booleanOwnedSteps(operation:SolidBooleanOperation,left:SolidOwnedMesh,right:SolidOwnedMesh):SolidSteps<SolidOwnedMesh>{this.#prepare();this.#operation(operation);const a=this.#mesh(left),b=this.#mesh(right);return this.#evaluate(booleanWithControlSteps(operation,a.mesh,b.mesh,this.#control),left===right?[left]:[left,right],true);}
 halfSpaceOwnedSteps(left:SolidOwnedMesh,plane:SolidPlane):SolidSteps<SolidOwnedMesh>{this.#prepare();const source=this.#mesh(left);return this.#evaluate(halfSpaceWithControlSteps(source.mesh,plane,this.#control),[left],true);}
 #operation(operation:SolidBooleanOperation):void{if(!["difference","union","intersection"].includes(operation))this.#control.refuse("unsupported-operation");}
 #evaluate(steps:SolidSteps<SolidMesh>,inputs:SolidOwnedMesh[],owned:false):SolidSteps<SolidMesh>;
 #evaluate(steps:SolidSteps<SolidMesh>,inputs:SolidOwnedMesh[],owned:true):SolidSteps<SolidOwnedMesh>;
 #evaluate(steps:SolidSteps<SolidMesh>,inputs:SolidOwnedMesh[],owned:boolean):SolidSteps<SolidMesh|SolidOwnedMesh>{
  let arena:SolidArena|undefined;const control=this.#control,source=function*():SolidSteps<SolidMesh>{arena=new SolidArena(control.workspace);control.arena=arena;return yield* steps;},cleanup=()=>{control.arena=undefined;arena?.close();arena=undefined;};
  return this.#steps(source(),mesh=>{if(!owned)return mesh;for(const input of inputs)this.#mesh(input);const lease=arena!.take([...mesh.vertices,...mesh.triangles]);let handle:SolidOwnedMesh;try{handle=this.#handle(mesh,lease);}catch(error){control.workspace.release(lease);throw error;}for(const input of inputs){const record=this.#mesh(input);this.#owned.delete(input);control.workspace.release(record.lease);}return handle;},cleanup);
 }
 close():void{this.#idle();this.#closed=true;this.#owned.clear();this.#control.workspace.close();}
 #steps<T,R=T>(steps:SolidSteps<T>,complete:(value:T)=>R=(value=>value as unknown as R),cleanup:()=>void=()=>{}):SolidSteps<R>{const cursor=controlledSteps(steps,this.#control),token={},wrapper=Object.create(Object.getPrototypeOf(cursor)) as SolidSteps<R>;let retired=false;const retire=()=>{retired=true;if(this.#active===token)this.#active=undefined;cleanup();};Object.defineProperties(wrapper,{
  next:{value:()=>{if(retired)return {done:true,value:undefined};this.#control.outside();this.#control.ensure();if(this.#closed)this.#control.refuse("session-closed");if(this.#builder||this.#active&&this.#active!==token)this.#control.refuse("session-active");this.#active=token;try{const result=cursor.next();if(!result.done)return result;const value=complete(result.value);retire();return {done:true,value};}catch(error){retire();return this.#control.poison(error);}}},
  return:{value:(value:R)=>{if(retired)return {done:true,value};this.#control.outside();try{cursor.return(undefined as T);retire();return {done:true,value};}catch(error){retire();return this.#control.poison(error);}}},
  throw:{value:(error:unknown)=>{this.#control.outside();this.#control.record(error);try{if(!retired)cursor.throw(error);}catch(failure){this.#control.record(failure);}finally{retire();}return this.#control.poison(error);}}
 });return Object.preventExtensions(wrapper);}
}
''' + s[end:]
p.write_text(s,encoding='utf-8')

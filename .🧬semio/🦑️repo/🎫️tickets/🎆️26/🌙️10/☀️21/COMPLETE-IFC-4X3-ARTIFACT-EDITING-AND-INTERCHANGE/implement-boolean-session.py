from pathlib import Path
ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts"
source=path.read_text(encoding="utf8")
a=source.index("class Control{");b=source.index("interface Plane",a)
source=source[:a]+'''class Control{
 #work=0;#failed=false;#failure:unknown;#observing=false;readonly #budget:number;readonly #faces:number;readonly #depth:number;readonly #observer?:()=>void;readonly #progress?:(work:number,budget:number)=>boolean;
 constructor(options:SolidBooleanOptions){const {maxWork,maxPolygons,maxDepth,tick,progress}=options;this.#budget=maxWork===undefined?2_000_000:maxWork;this.#faces=maxPolygons===undefined?200_000:maxPolygons;this.#depth=maxDepth===undefined?256:maxDepth;this.#observer=tick;this.#progress=progress;if([this.#budget,this.#faces,this.#depth].some(v=>!Number.isSafeInteger(v)||v<1))fail("invalid-limits");if(tick!==undefined&&typeof tick!=="function"||progress!==undefined&&typeof progress!=="function")fail("invalid-observer");}
 get work():number{return this.#work;}
 ensure():void{if(this.#failed)throw this.#failure;}
 record(error:unknown):void{if(!this.#failed){this.#failed=true;this.#failure=error;}}
 poison(error:unknown):never{this.record(error);throw this.#failure;}
 refuse(reason:string):never{return this.poison(Error("boolean-"+reason));}
 outside():void{if(this.#observing)this.refuse("session-reentrant");}
 tick(cost=1):void{this.ensure();if(cost>this.#budget-this.#work)this.refuse("work-limit");this.#work+=cost;const observer=this.#observer,progress=this.#progress;this.#observing=true;try{observer?.();if(progress?.(this.#work,this.#budget)===false)this.refuse("cancelled");}catch(error){this.poison(error);}finally{this.#observing=false;}this.ensure();}
 check(depth:number,count=0):void{this.ensure();if(depth>this.#depth)this.refuse("depth-limit");if(count>this.#faces)this.refuse("polygon-limit");}
}
'''+source[b:]
old='for(;;){const step=steps.next();if(step.done){control.tick(0);return step.value;}control.tick();yield;}'
new='for(;;){control.ensure();const step=steps.next();if(step.done){control.tick(0);return step.value;}control.tick();yield;}'
assert source.count(old)==1
source=source.replace(old,new)
anchor='export type SolidBooleanOperation = "difference" | "union" | "intersection";'
assert source.count(anchor)==1
session='''
/** 🧬🧊 One captured cumulative evaluation lends work to sequential source and Boolean stages. */
export class SolidBooleanEvaluation{
 readonly #control:Control;#active?:object;#closed=false;
 constructor(options:SolidBooleanOptions={}){this.#control=new Control(options);}
 get work():number{return this.#control.work;}
 #prepare():void{this.#control.outside();this.#control.ensure();if(this.#closed)this.#control.refuse("session-closed");if(this.#active)this.#control.refuse("session-active");}
 checkpoint():void{this.#prepare();this.#control.tick();}
 booleanSteps(operation:SolidBooleanOperation,left:SolidMesh,right:SolidMesh):SolidSteps<SolidMesh>{this.#prepare();if(!["difference","union","intersection"].includes(operation))this.#control.refuse("unsupported-operation");return this.#steps(booleanWithControlSteps(operation,left,right,this.#control));}
 halfSpaceSteps(left:SolidMesh,plane:SolidPlane):SolidSteps<SolidMesh>{this.#prepare();return this.#steps(halfSpaceWithControlSteps(left,plane,this.#control));}
 close():void{this.#control.outside();if(this.#active)this.#control.refuse("session-active");this.#closed=true;}
 #steps<T>(steps:SolidSteps<T>):SolidSteps<T>{const cursor=controlledSteps(steps,this.#control),token={},wrapper=Object.create(Object.getPrototypeOf(cursor)) as SolidSteps<T>;let retired=false;const retire=()=>{retired=true;if(this.#active===token)this.#active=undefined;};Object.defineProperties(wrapper,{
  next:{value:(...args:[]|[void])=>{if(retired)return cursor.next(...args);this.#control.outside();this.#control.ensure();if(this.#closed)this.#control.refuse("session-closed");if(this.#active&&this.#active!==token)this.#control.refuse("session-active");this.#active=token;try{const result=cursor.next(...args);if(result.done)retire();return result;}catch(error){retire();return this.#control.poison(error);}}},
  return:{value:(value:T)=>{this.#control.outside();try{const result=cursor.return(value);if(result.done)retire();return result;}catch(error){retire();return this.#control.poison(error);}}},
  throw:{value:(error:unknown)=>{this.#control.outside();if(retired)return cursor.throw(error);this.#control.record(error);try{cursor.throw(error);}catch(failure){this.#control.record(failure);}finally{retire();}return this.#control.poison(error);}}
 });return Object.preventExtensions(wrapper);}
}
'''
source=source.replace(anchor,anchor+session,1)
old='const control=new Control(options);return controlledSteps(booleanWithControlSteps(operation,left,right,control),control);'
new='return new SolidBooleanEvaluation(options).booleanSteps(operation,left,right);'
assert source.count(old)==1
source=source.replace(old,new)
old='const control=new Control(options);return controlledSteps(halfSpaceWithControlSteps(left,plane,control),control);'
new='return new SolidBooleanEvaluation(options).halfSpaceSteps(left,plane);'
assert source.count(old)==1
source=source.replace(old,new)
path.write_text(source,encoding="utf8")
test=path.parent/"🧪️tests/session.test.ts"
source=test.read_text(encoding="utf8")
a=source.index("interface Session{");b=source.index("interface Row",a)
source=source[:a]+'''type Session=core.SolidBooleanEvaluation;
const create=(options:SolidBooleanOptions={}):Session=>new core.SolidBooleanEvaluation(options);
'''+source[b:]
source=source.replace('expect(next.value).toBeUndefined();','if(next.value!==undefined)throw Error("Private Boolean turns must not publish partial values");')
test.write_text(source,encoding="utf8")
print("[DEBUG] one cumulative private Boolean session, protected cursor entry, latching failures and fixed source checkpoints")


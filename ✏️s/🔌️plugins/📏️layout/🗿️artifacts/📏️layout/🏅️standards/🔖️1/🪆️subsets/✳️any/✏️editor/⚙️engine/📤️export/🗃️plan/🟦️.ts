/** 🗃️ Original output rows retain payload custody through actual issued-loan admission and close. */
import {RetainedWorkBuffer,RetainedWorkLedger,RetainedWorkRetirement,RetainedWorkTurnCounter,assertRetainedWorkTurn,type RetainedWorkTurn,type RetainedWorkRetirementCursor,retireRetainedWorkBuffer} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
/** 🎟️ Each append reports its actual defining work; a query or refused grant creates no CPU. */
export class ExportPlanList<T>{
 #rows:RetainedWorkBuffer<T>;#fault:T|undefined;#hasFault=false;#closing=false;#counter=new RetainedWorkTurnCounter();#last=0;#view:Iterable<T>;
 constructor(bytes:number,private readonly closePayload:(value:T,work:RetainedWorkLedger,turn:RetainedWorkTurn)=>boolean){this.#rows=new RetainedWorkBuffer<T>(bytes);const owner=this;this.#view=Object.freeze({*[Symbol.iterator](){if(owner.#closing)throw Error("Layout plan borrow expired");for(let at=0;at<owner.#rows.length;at++){if(owner.#closing)throw Error("Layout plan borrow expired");yield owner.#rows.get(at)!;}}});}
 get length(){return this.#rows.length;}get(index:number){if(this.#closing)throw Error("Layout plan borrow expired");return this.#rows.get(index);}values():Iterable<T>{if(this.#closing)throw Error("Layout plan borrow expired");return this.#view;}
 nextAppendDemand(){const demand=this.#rows.nextSlotDemand();return{...demand,depth:demand.depth+1};}lastTurnWork(){return this.#last;}
 reserveOneIn(work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{assertRetainedWorkTurn(turn);if(this.#closing||this.#hasFault)throw Error("Layout plan closed or faulted");return work.slot(this.#rows,turn);}
 pushIn(value:T,work:RetainedWorkLedger,turn:RetainedWorkTurn):void{assertRetainedWorkTurn(turn);if(this.#closing||this.#hasFault)throw Error("Layout plan closed or faulted");const length=this.#rows.length;try{work.push(this.#rows,value,turn);}catch(error){if(this.#rows.length===length){this.#fault=value;this.#hasFault=true;}throw error;}}
 append(work:RetainedWorkLedger,produce:(work:RetainedWorkLedger,turn:RetainedWorkTurn)=>T|undefined):boolean{
  this.#last=0;const turn=this.#counter.beginTurn(),entered=work.enter(),failures:unknown[]=[];let result=false;
  try{if(entered!==null&&this.reserveOneIn(work,turn)){const value=produce(work,turn);if(value!==undefined){this.pushIn(value,work,turn);result=true;}}}catch(error){failures.push(error);}
  finally{this.#last=this.#counter.endTurn(turn);try{if(entered!==null)for(let at=0;at<this.#last;at++)work.noteSource(entered);}catch(error){failures.push(error);}finally{if(entered!==null)work.exit();}}
  if(failures.length)throw failures.length===1?failures[0]:new AggregateError(failures,"Actual plan payload and accounting failures");return result;
 }
 intoRetirement():RetainedWorkRetirementCursor{
  if(this.#closing)throw Error("Layout plan ownership transferred");this.#closing=true;
  return new RetainedWorkRetirement((work,turn)=>{
   if(this.#hasFault){const before=turn.actualWork();if(!this.closePayload(this.#fault!,work,turn)||turn.actualWork()!==before)return false;this.#fault=undefined;this.#hasFault=false;turn.commitActualWork();return false;}
   if(this.#rows.length){const before=turn.actualWork();if(!this.closePayload(this.#rows.at(-1)!,work,turn)||turn.actualWork()!==before)return false;if(!work.pop(this.#rows,turn))return false;return false;}
   return retireRetainedWorkBuffer(this.#rows,work,turn);
  });
 }
 terminalIsEmpty(){return this.#closing&&!this.#hasFault&&this.#rows.length===0&&this.#rows.capacity===0;}
}

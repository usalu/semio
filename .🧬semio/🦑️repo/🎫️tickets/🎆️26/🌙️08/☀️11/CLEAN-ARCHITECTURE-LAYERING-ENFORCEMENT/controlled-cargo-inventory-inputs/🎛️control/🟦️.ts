export type CargoProgress = Readonly<{stage:string;path:string;completed:number;ownedBytes:number}>;
export interface CargoControl { readonly signal:AbortSignal; readonly maximumUnits:number; readonly maximumOwnedBytes:number; readonly maximumDepth:number; onProgress(progress:CargoProgress):void; yieldContinuation():Promise<void>; }
export interface CargoAccounting { completed:number;ownedBytes:number; }
export interface CargoRetirementOperation extends CargoControl {readonly accounting:CargoAccounting;}
const yieldMarks=new WeakMap<CargoAccounting,number>();

/** 🎛️ Accounts before ownership and yields at bounded work frontiers. */
export class CargoController {
 constructor(readonly operation:CargoControl,readonly accounting:CargoAccounting){if(!yieldMarks.has(accounting))yieldMarks.set(accounting,accounting.completed);for(const value of [operation.maximumUnits,operation.maximumOwnedBytes,operation.maximumDepth])if(!Number.isSafeInteger(value)||value<0)throw Error("Cargo control limit is invalid");}
 check():void {if(this.operation.signal.aborted)throw Error("Cargo operation cancelled");}
 async step(stage:string,path:string,units=1,bytes=0):Promise<void>{this.check();if(!Number.isSafeInteger(units)||units<0||!Number.isSafeInteger(bytes)||bytes<0)throw Error("Cargo accounting requires finite nonnegative admissions");if(units>this.operation.maximumUnits-this.accounting.completed||bytes>this.operation.maximumOwnedBytes-this.accounting.ownedBytes)throw Error("Cargo operation budget refused");this.accounting.completed+=units;this.accounting.ownedBytes+=bytes;if(this.accounting.completed-yieldMarks.get(this.accounting)!>=128||units===0){this.operation.onProgress({stage,path,completed:this.accounting.completed,ownedBytes:this.accounting.ownedBytes});this.check();await this.operation.yieldContinuation();this.check();yieldMarks.set(this.accounting,this.accounting.completed);}}
 async finish(stage:string,path:string):Promise<void>{await this.step(stage,path,0);}
}

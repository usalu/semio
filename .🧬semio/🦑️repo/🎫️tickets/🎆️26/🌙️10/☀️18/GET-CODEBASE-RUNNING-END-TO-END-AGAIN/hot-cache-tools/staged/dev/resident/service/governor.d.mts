export class BusyErrorV1 extends Error{readonly code:"busy";readonly inFlight:number;readonly queued:number}
export function createGovernorV1(options:{maximumInFlight:number;maximumQueued:number}):{enter(lane:string,signal?:AbortSignal,onPosition?:(position:number)=>void):Promise<()=>void>;readonly inFlight:number;readonly queued:number};

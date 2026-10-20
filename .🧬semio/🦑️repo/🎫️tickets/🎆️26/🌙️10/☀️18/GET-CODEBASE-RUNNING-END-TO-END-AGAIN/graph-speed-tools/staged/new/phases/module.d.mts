/** ⏱️ One finished graph phase. */
export interface GraphPhaseRowV1{readonly phase:string;readonly milliseconds:number;readonly done?:number;readonly total?:number;readonly [detail:string]:unknown}
export interface GraphPhaseClockOptionsV1{publish?:(resource:string)=>void|Promise<void>;write?:(row:GraphPhaseRowV1)=>void;now?:()=>number;minimumMilliseconds?:number}
export interface GraphPhaseClockV1{begin(phase:string,total:number):void|Promise<void>;advance(phase:string,done:number):void|Promise<void>;end(phase:string,details?:Record<string,unknown>):Promise<GraphPhaseRowV1>;rows():GraphPhaseRowV1[]}
export function createGraphPhaseClockV1(options?:GraphPhaseClockOptionsV1):GraphPhaseClockV1;
export function writeGraphPhaseRowV1(row:GraphPhaseRowV1):void;
export function writeGraphPhaseSummaryV1(rows:readonly GraphPhaseRowV1[]):void;

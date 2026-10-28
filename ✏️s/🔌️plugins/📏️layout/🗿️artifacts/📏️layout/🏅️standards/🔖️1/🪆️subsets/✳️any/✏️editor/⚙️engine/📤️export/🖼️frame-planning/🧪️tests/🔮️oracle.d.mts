export interface FrameChecks{frames:{runs:number;rows:number;blocked:number;cancelled:number;refusals:number};metadata:{runs:number;queries:number;blocked:number;closed:number;refusals:number;collision:(string|number)[]}}
export function run(root:string):FrameChecks;
export function repositoryRoot():string;
export function expectedFrames(doc:unknown):unknown[];

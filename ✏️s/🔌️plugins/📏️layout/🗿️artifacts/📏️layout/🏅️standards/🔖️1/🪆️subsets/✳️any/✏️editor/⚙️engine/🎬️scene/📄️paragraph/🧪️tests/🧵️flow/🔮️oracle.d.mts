export interface LayoutFlowChecks{labels:{runs:number;cancelled:number;refusals:number};borrowedFaceRuns:number;flow:{cases:number;runs:number;cancelled:number;refusals:number};addressed:{runs:number;requests:number;analyses:number;refusals:number}}
export function run(root:string):Promise<LayoutFlowChecks>;
export function repositoryRoot():string;

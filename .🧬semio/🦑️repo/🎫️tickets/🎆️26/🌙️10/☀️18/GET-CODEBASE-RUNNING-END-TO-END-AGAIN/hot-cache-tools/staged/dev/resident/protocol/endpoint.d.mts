export interface ResidentEndpointsV1{id:string;directory:string;endpoint:string;key:string;descriptor:string;state:string;lock:string}
export function worktreeIdV1(root:string):string;
export function endpointsV1(root:string,environment?:Readonly<Record<string,string|undefined>>,platform?:string,user?:string):ResidentEndpointsV1;

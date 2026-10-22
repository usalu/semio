export class ProtocolErrorV1 extends Error{readonly code:string;constructor(code:string,message:string)}
export function encodeFrameV1(frame:unknown):string;
export function createFrameDecoderV1(options:{maximumFrameBytes:number;onFrame:(frame:unknown)=>void;onError:(error:ProtocolErrorV1)=>void}):{push(chunk:string|Uint8Array):void;end():void};
export function compileSchemaV1(document:{definitions:Record<string,unknown>},name?:string):(value:unknown)=>string[];

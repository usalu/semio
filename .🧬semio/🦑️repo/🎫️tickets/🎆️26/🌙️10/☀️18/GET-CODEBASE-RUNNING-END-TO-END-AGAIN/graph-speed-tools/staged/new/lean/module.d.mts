/** 🪶️ A validated-by-name inventory leaf: its sha256 equals its file name and its identity stamp was taken while it was read. */
export interface InventoryRecordV1{readonly sha256:string;readonly byteLength:number;readonly value:unknown;readonly stamp:string}
export function readInventoryRecordLeanV1(directory:string,key:string,maximumBytes:number):InventoryRecordV1;
export function rememberInventoryRecordV1(root:string,directory:string,key:string,stamp:string):void;
export function inventoryRecordStillHoldsV1(root:string,directory:string,key:string):boolean;
export function forgetInventoryRecordsV1():void;

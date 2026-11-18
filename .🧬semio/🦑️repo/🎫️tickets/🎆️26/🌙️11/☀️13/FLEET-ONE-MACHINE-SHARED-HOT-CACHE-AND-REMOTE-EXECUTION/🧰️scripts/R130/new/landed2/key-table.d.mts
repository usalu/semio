/** 🔑️ A table from texts to unsigned numbers with a fixed number of bytes per entry. */
export interface KeyTableV1{get(text:string):number|undefined;set(text:string,value:number):void;intern(text:string,value:number):number;getDigest(digest:Uint8Array):number|undefined;setDigest(digest:Uint8Array,value:number):void;size():number;bytes():number;clear():void}
/** 🔑️ Creates a table that starts with room for `capacity` entries. */
export function createKeyTableV1(capacity?:number):KeyTableV1;

/** 🧾️ The declared limits of deferred optional accounting and of worker grants. */
export const OPTIONAL_LEAVES_POLICY_V1: Readonly<{flushBytes:number;flushLeaves:number;flushMilliseconds:number;finalFlushAttempts:number;flushRetryMilliseconds:number;grantBytes:number;maximumLeafBytes:number}>;
/** 🏷️ Whether a name relative to the inference namespace is an optional cache leaf. */
export function isOptionalLeafNameV1(name:unknown):boolean;
/** 🧮️ A leaf written by a worker process: its name relative to the inference namespace and its retained bytes. */
export type OptionalLeafV1=Readonly<{name:string;bytes:number}>;
/** 🧮️ Accounts optional leaves against the quota and the durable ledger in batches. */
export function createOptionalAccountingV1(options:Readonly<{ledger:Readonly<{publish(plan:Readonly<{peakBytes:number;retainedDeltaBytes:number}>,write:()=>void):boolean}>;quota:Readonly<{reserve(plan:Readonly<{peakBytes:number;retainedDeltaBytes:number}>):Readonly<{commit():void;release():void}>|undefined}>;policy?:Readonly<Record<string,number>>;clock?:()=>number;wait?:(milliseconds:number)=>void}>):Readonly<{publish(plan:Readonly<{peakBytes:number;retainedDeltaBytes:number}>,write:()=>void,deferred?:boolean,held?:Readonly<{commit():void;release():void}>):boolean;adopt(items:readonly OptionalLeafV1[],remove:(name:string)=>void):Readonly<{accepted:string[];refused:string[]}>;grant(bytes:number):(()=>void)|undefined;flush(final?:boolean):boolean;facts():Readonly<Record<string,number>>}>;
/** 🛰️ The writer scope of one graph worker job. */
export function createWorkerWriterV1(options:Readonly<{roots:Readonly<Record<string,string>>;grantBytes:number}>):Readonly<{presence:Set<string>;directory(namespace:string):string;publish(path:string,plan:Readonly<{peakBytes:number;retainedDeltaBytes:number}>,write:()=>void|false):boolean;defer(manifest:string):void;report():Readonly<{leaves:OptionalLeafV1[];declined:string[];receipts:string[]}>}>;

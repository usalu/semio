/** 📦️ Own neutral boundary for fresh Source sealing and authenticated direct PAX transfer. */
export interface CapturePlan {
 version:1;
 kind:"fresh-source-capture-plan";
 captureId:string;
 workspaceRoot:string;
 inputs:Readonly<{core:string;host:string;entry:string;schema:string;contract:string}>;
 critical:readonly {path:string;sha256:string|null}[];
}
export interface CaptureControl {
 signal:AbortSignal;
 remainingMilliseconds:()=>number;
 progress:(stage:string,facts:Readonly<Record<string,string|number|boolean>>)=>Promise<void>;
}
export interface InventorySnapshot {
 paths:readonly {path:string;owner:string}[];
 criticalPaths:readonly string[];
 absent:readonly string[];
 control:Readonly<{tracked:string;untracked:string;providers:string;membership:string;registry:string}>;
 observedAt:string;
}
export interface StableSource {
 bytes:Uint8Array;
 identity:string;
 executable:boolean;
 observedAt:string;
}
export interface SourceObservationPort {
 inventory:(control:CaptureControl)=>Promise<InventorySnapshot>;
 readStable:(path:string,maximumBytes:number,control:CaptureControl)=>Promise<StableSource>;
 observeAbsent:(path:string,control:CaptureControl)=>Promise<boolean>;
 observeIdentity:(path:string,control:CaptureControl)=>Promise<string|null>;
}
export interface SealedSourceRow {
 path:string;
 owner:string;
 bytes:number;
 sha256:string;
 identity:string;
 executable:boolean;
 epoch:number;
}
export interface SourceManifest {
 version:1;
 kind:"fresh-source-manifest";
 captureId:string;
 planSha256:string;
 atomicHostSnapshot:false;
 model:"stable-file-critical-control-and-owner-epochs";
 controlBefore:InventorySnapshot["control"];
 controlAfter:InventorySnapshot["control"];
 criticalPaths:readonly string[];
 rows:readonly SealedSourceRow[];
 absent:readonly {path:string;epoch:number;observedAt:string}[];
 epochs:readonly {epoch:number;owner:string;openedAt:string;sealedAt:string;rowsSha256:string;files:number;bytes:number}[];
 divergence:Readonly<{added:readonly string[];missing:readonly string[];changedIdentity:readonly string[];controlChanged:boolean}>;
 bytes:number;
}
export interface SourceSeal {
 manifest:SourceManifest;
 manifestSha256:string;
}

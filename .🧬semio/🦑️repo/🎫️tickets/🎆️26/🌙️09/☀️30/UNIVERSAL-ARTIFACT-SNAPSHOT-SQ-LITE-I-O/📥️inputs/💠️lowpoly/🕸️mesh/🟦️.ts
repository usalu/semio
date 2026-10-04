/** 🕸️ Complete managed halfedge topology and rich surface channels remain typed owned fields. */
import type{Binary32}from"../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
export interface LowpolyMeshState{vertices:LowpolyMeshVertex[];halfedges:LowpolyMeshHalfedge[];faces:LowpolyMeshFace[];uvSeams:number[];attributes:LowpolyMeshAttribute[];materials:LowpolyMeshMaterial[];textures:LowpolyMeshTexture[]}
export interface LowpolyMeshVertex{position:[Binary32,Binary32,Binary32];normal:[Binary32,Binary32,Binary32]|null;halfedge:number|null}
export interface LowpolyMeshHalfedge{vertex:number;twin:number|null;next:number;face:number|null;uv:[Binary32,Binary32]}
export interface LowpolyMeshFace{halfedge:number;smooth:boolean;flipped:boolean}
export interface LowpolyMeshAttribute{name:string;domain:"vertex"|"corner"|"face"|"edge";semantic:"normal"|"uv"|"color"|"material"|"custom";interpolation:"linear"|"nearest"|"constant";values:IntrinsicValue[];indices:number[]|null}
export interface LowpolyMeshMaterial{name:string;value:IntrinsicValue}
export interface LowpolyMeshTexture{name:string;mime:string;bytes:Uint8Array}

/** 🧱️ The declared constants of the chunked construction. */
export interface IndexChunkPolicyV1{readonly version:1;readonly chunkCutBits:number;readonly bulkUpsertRows:number;readonly scratchSlackRows:number;readonly scratchInitialEntries:number}
export const INDEX_CHUNK_POLICY_V1:IndexChunkPolicyV1;
/** 🧩️ Interned runs of row numbers and the owners that hold each. */
export interface ChunkInternerV1{readonly chunks:Uint32Array[];readonly holders:number[][];cuts(number:number):boolean;add(numbers:Uint32Array,from:number,to:number,owner:number):void}
/** 🏗️ Creates an empty interner of runs. */
export function createChunkInternerV1():ChunkInternerV1;
/** ✂️ Cuts one owner's distinct row numbers into runs and interns them. */
export function addOwnerRunsV1(interner:ChunkInternerV1,numbers:Uint32Array,count:number,owner:number):void;
/** 🗃️ The classes of an index over `rowCount` rows from interned runs. */
export function classesOfChunksV1(rowCount:number,interner:ChunkInternerV1):{r:number[];o:number[]}[];
/** 📥️ Pushes the classes of a bulk upsert onto `classes`. */
export function bulkUpsertClassesV1(input:{upserts:ReadonlyMap<string,readonly unknown[]>;ownerIds:ReadonlyMap<string,number>;idOfRow:(row:never)=>number;rowCount:number;classes:{r:number[];o:number[]}[]}):void;

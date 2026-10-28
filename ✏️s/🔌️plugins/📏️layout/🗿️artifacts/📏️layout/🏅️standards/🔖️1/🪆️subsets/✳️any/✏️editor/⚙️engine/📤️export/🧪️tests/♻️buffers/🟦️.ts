import {strictEqual} from 'node:assert/strict';
import fixture from './🔣️.json' with {type:'json'};
export interface PhysicalBufferWitness{name:string;release:number}
/** 🧪️ The shared native fixture quotes full byte capacity independently of the initialized prefix. */
export function checkPhysicalBufferWitnesses(oracle:(fixture:unknown)=>PhysicalBufferWitness[]):number{const rows=oracle(fixture);strictEqual(rows.length,fixture.cases.length);for(let index=0;index<rows.length;index++){strictEqual(rows[index]!.name,fixture.cases[index]!.name);strictEqual(rows[index]!.release,fixture.cases[index]!.expectedRelease);}return rows.length;}

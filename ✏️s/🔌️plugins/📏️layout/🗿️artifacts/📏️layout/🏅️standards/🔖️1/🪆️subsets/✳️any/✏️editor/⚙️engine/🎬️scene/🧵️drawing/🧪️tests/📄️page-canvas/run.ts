/** 🧪️ Durable Page Drawing source, identity and real SQLite ownership verification. */
import * as placement from './oracle.mjs';
import * as sqlite from './sqlite-oracle.mjs';
import {testPageDrawing} from './🟦️.ts';
import {testPageDrawingSqlite} from './sqlite.ts';
export async function verifyPageDrawing(){return{...testPageDrawing(placement),...await testPageDrawingSqlite(sqlite)};}

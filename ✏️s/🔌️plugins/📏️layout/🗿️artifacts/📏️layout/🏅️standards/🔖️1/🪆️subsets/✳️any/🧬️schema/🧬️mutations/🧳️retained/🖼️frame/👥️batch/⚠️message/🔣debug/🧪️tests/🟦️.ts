import{rustStringDebugScalar}from'../🟦️.ts';
import fixture from'./🔣️.json';
export interface DebugOracle{body(value:string):string}
/** 🧪️ Shared scalar bodies compare with an independent Unicode table oracle. */
export function checkDebugScalars(oracle:DebugOracle){for(const row of fixture.cases){const actual=rustStringDebugScalar(row.input);if(actual!==row.body||actual!==oracle.body(row.input)||'\"'+actual+'\"'!==row.quoted)throw Error('canonical Rust String Debug scalar '+row.name)}for(const value of['','ab','\ud800']){let refused=false;try{rustStringDebugScalar(value)}catch{refused=true}if(!refused)throw Error('unbounded or invalid scalar accepted')}return{cases:fixture.cases.length,invalidScalars:3}}

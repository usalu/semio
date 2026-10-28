/** 🔮️ Test-only Lodash comparison of untouched opaque encoded source implement the owned cold plan oracle. */
import{createRequire}from'node:module';
const require=createRequire(import.meta.url),lodash=require('lodash');
export const sourcePlanOracle={equal:(a,b)=>lodash.isEqual(a,b)};

import { emitBlock5d, CASES } from "./block5d.ts";
const r = emitBlock5d();
console.log(`cases=${CASES.length} files=${r.written.length}`);

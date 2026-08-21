import { emitBlock2d, CASES } from "./block2d.ts";
const w = emitBlock2d();
console.log(`cases=${CASES.length} files=${w.length}`);

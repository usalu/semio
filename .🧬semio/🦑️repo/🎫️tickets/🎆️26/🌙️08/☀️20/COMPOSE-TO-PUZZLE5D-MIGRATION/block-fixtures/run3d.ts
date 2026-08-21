import { emitBlock3d, CASES } from "./block3d.ts";
const w = emitBlock3d();
console.log(`cases=${CASES.length} files=${w.length}`);

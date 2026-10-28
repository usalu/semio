"""🕳️ Mount the shared continuation behind the original ring interface after verifying its source."""
import hashlib,sys
from pathlib import Path
target=Path(sys.argv[1])/'📐️geometry/🕳️rings/🟦️.ts'
expected='bf481de6caf29a374a34393166db1db391242458881fdff47930232964ae1d7a'
if hashlib.sha256(target.read_bytes()).hexdigest()!=expected:raise RuntimeError('Ring source changed; reread before mounting the shared core')
target.write_text("""export type RingPoint=[number,number,number];
export type RingTriangle=[number,number,number];
import {triangulateRingsSteps} from './work.ts';
export {triangulateRingsSteps};
export type {RingSteps} from './work.ts';

/** 🕳️ Synchronous callers drain the shared evaluator with their original work callback. */
export function triangulateRings(vertices:readonly RingPoint[],input:readonly(readonly number[])[],tick:()=>void):RingTriangle[]{
 const steps=triangulateRingsSteps(vertices,input);
 try{for(;;){const step=steps.next();if(step.done)return step.value;tick();}}finally{steps.return([]);}
}
""",encoding='utf-8')
print('[DEBUG] Mounted the one shared ring algorithm behind both execution modes')

/** 🪜 Compare broad-phase containment of independently closed translated parts to the unpartitioned thermal mesh. */
import {plugin} from 'bun'
import {readFileSync} from 'node:fs'
function insert(source,before,after){
 if(!source.includes(before))throw Error('Containment implementation has changed')
 return source.replace(before,after)
}
export function accelerate(source){
 source=insert(source,'const SKEW: Vec',`const DISJOINT_PARTS = new WeakMap<World, readonly World[]>();

/** 🪜 Closed translated parts retain their own bounds and local triangle index. */
function disjointParts(solid: World): readonly World[] {
  const source = solid.geometrySource ?? solid;
  const known = DISJOINT_PARTS.get(source);
  if (known) return known;
  const { coordinatesPerCopy, indices } = source.separateCopies!;
  const parts: World[] = [];
  for (let start = 0; start < source.positions.length; start += coordinatesPerCopy)
    parts.push({ id: source.id, positions: source.positions.subarray(start, start + coordinatesPerCopy), indices });
  DISJOINT_PARTS.set(source, parts);
  return parts;
}

const SKEW: Vec`)
 source=insert(source,'    if (skip(solid, k)) continue;\n','')
 return insert(source,' || p[1] > max[1] || !isClosed(solid)) continue;',` || p[1] < min[1] || p[1] > max[1] || skip(solid, k) || !isClosed(solid)) continue;
    if (solid.separateCopies) {
      if (insideAny(disjointParts(solid), p)) return true;
      continue;
    }`)
}
if(!import.meta.main&&!process.env.STAFFAGE_TIMING)plugin({name:'independent-disjoint-containment',setup(b){
 b.onLoad({filter:/🧮️measures\/🟦️\.ts$/},a=>({loader:'ts',contents:accelerate(readFileSync(a.path,'utf8'))}))
}})

import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
const ext = (s: any) => { const g = tessellateSolid({ ...s, node: "n" } as never, { chordM: 0.002, minSegments: 12, maxSegments: 64 }); const mn=[1e9,1e9,1e9], mx=[-1e9,-1e9,-1e9]; for (let i=0;i<g.positions.length;i+=3) for(let k=0;k<3;k++){mn[k]=Math.min(mn[k],g.positions[i+k]);mx[k]=Math.max(mx[k],g.positions[i+k]);}
 // signed volume
 let vol=0; for(let t=0;t<g.indices.length;t+=3){const a=g.indices[t]*3,b=g.indices[t+1]*3,c=g.indices[t+2]*3; const P=g.positions; vol+= (P[a]*(P[b+1]*P[c+2]-P[b+2]*P[c+1]) - P[a+1]*(P[b]*P[c+2]-P[b+2]*P[c]) + P[a+2]*(P[b]*P[c+1]-P[b+1]*P[c]))/6;}
 return {mn:mn.map(x=>+x.toFixed(3)),mx:mx.map(x=>+x.toFixed(3)),vol:+vol.toFixed(5)}; };
const QU=JSON.parse(process.argv[2]);
console.log("QU path u, tilted", ext({ id:"a", material:"m", kind:"loft", sections:[{at:[0,0,0],rx:0.1,ry:0.02,rotation:QU},{at:[0.25,0.01,0],rx:0.1,ry:0.02,rotation:QU},{at:[0.5,0.0,0],rx:0.1,ry:0.02,rotation:QU}], capStart:true, capEnd:true }));
console.log("noRot tilted", ext({ id:"a", material:"m", kind:"loft", sections:[{at:[0,0,0],rx:0.1,ry:0.02},{at:[0.25,0.01,0],rx:0.1,ry:0.02},{at:[0.5,0.0,0],rx:0.1,ry:0.02}], capStart:true, capEnd:true }));

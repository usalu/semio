import {readFileSync,writeFileSync} from 'node:fs';
import {Vector3,REVISION} from 'three';
const root=import.meta.dir;
const input=JSON.parse(readFileSync(root+'/🗑️generated/faceted-brep-vectors/void-shell-evidence.json','utf8'));
const rows=input.shells.map(shell=>{
 const edges=new Map(); let volume=0; const points=[];
 for(const face of shell.faces){
  for(let i=0;i<face.length;i++){const a=JSON.stringify(face[i]),b=JSON.stringify(face[(i+1)%face.length]);const key=[a,b].sort().join('|');const entry=edges.get(key)||[];entry.push([a,b]);edges.set(key,entry);points.push(face[i]);}
  const a=new Vector3(...face[0]);for(let i=1;i+1<face.length;i++)volume+=a.dot(new Vector3(...face[i]).cross(new Vector3(...face[i+1])))/6;
 }
 return {id:shell.id,signedVolume:volume,closedOpposingEdges:[...edges.values()].every(e=>e.length===2&&e[0][0]===e[1][1]&&e[0][1]===e[1][0]),bounds:[0,1].map(side=>[0,1,2].map(k=>side?Math.max(...points.map(p=>p[k])):Math.min(...points.map(p=>p[k]))))};
});
const contained=[0,1,2].every(k=>rows[0].bounds[0][k]<rows[1].bounds[0][k]&&rows[1].bounds[1][k]<rows[0].bounds[1][k]);
const output={library:'Three '+REVISION,rows,strictlyContained:contained,signedVolume:rows.reduce((v,r)=>v+r.signedVolume,0),scope:'Source polygon oriented-surface arithmetic; not a successful compound IfcOpenShell mesh'};
writeFileSync(root+'/🗑️generated/faceted-brep-vectors/void-shell-certificate.json',JSON.stringify(output,null,2));console.log(output);

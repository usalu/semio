import { openCtx, drawn, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
const cell = Number(flag(a, "--cell", "0.01"));
const inside = (r: any[], x: number, y: number) => { let w = false; for (let i = 0, j = r.length - 1; i < r.length; j = i++) { if ((r[i].y > y) !== (r[j].y > y) && x < ((r[j].x - r[i].x) * (y - r[i].y)) / (r[j].y - r[i].y) + r[i].x) w = !w; } return w; };
const S: Record<string, any[]> = {};
const B: Record<string, number[]> = {};
for (const v of ["elevation", "front", "plan"] as const) { S[v] = drawn(c, v); let x0=1e9,x1=-1e9,y0=1e9,y1=-1e9; for (const s of S[v]!) for (const r of s.rings) for (const p of r) { x0=Math.min(x0,p.x);x1=Math.max(x1,p.x);y0=Math.min(y0,p.y);y1=Math.max(y1,p.y);} B[v]=[x0,x1,y0,y1]; }
const mask: Record<string, Uint8Array> = {}; const dims: Record<string, [number, number]> = {};
for (const v of ["elevation", "front", "plan"] as const) { const [x0,x1,y0,y1]=B[v]!; const nx=Math.ceil((x1-x0)/cell), ny=Math.ceil((y1-y0)/cell); dims[v]=[nx,ny]; const m=new Uint8Array(nx*ny); for(let j=0;j<ny;j++)for(let i=0;i<nx;i++){const x=x0+(i+.5)*cell,y=y0+(j+.5)*cell; if(S[v]!.some((s)=>s.rings.some((r:any)=>inside(r,x,y)))) m[j*nx+i]=1;} mask[v]=m; }
const get = (v: string, x: number, y: number) => { const [x0,,y0]=B[v]!; const [nx,ny]=dims[v]!; const i=Math.floor((x-x0)/cell), j=Math.floor((y-y0)/cell); return i>=0&&j>=0&&i<nx&&j<ny?mask[v]![j*nx+i]!:0; };
const proj: Record<string, Set<number>> = { elevation: new Set(), front: new Set(), plan: new Set() };
const [ex0,ex1]=B.elevation!, [fx0,fx1]=B.front!;
const X0=B.elevation![0]!, X1=B.elevation![1]!, Z0=B.front![0]!, Z1=B.front![1]!, Y0=0, Y1=Math.max(B.elevation![3]!,B.front![3]!);
let vox=0;
for (let x=X0+cell/2;x<X1;x+=cell) for(let z=Z0+cell/2;z<Z1;z+=cell){ if(!get("plan",x,z)) continue; for(let y=B.elevation![2]!+cell/2;y<Y1;y+=cell){ if(get("elevation",x,y)&&get("front",z,y)){vox++; proj.elevation.add(Math.floor((x-B.elevation![0]!)/cell)+1e5*Math.floor((y-B.elevation![2]!)/cell)); proj.front.add(Math.floor((z-B.front![0]!)/cell)+1e5*Math.floor((y-B.front![2]!)/cell)); proj.plan.add(Math.floor((x-B.plan![0]!)/cell)+1e5*Math.floor((z-B.plan![2]!)/cell));}}}
for (const v of ["elevation","front","plan"]) { let d=0; for(const q of mask[v]!) d+=q; console.log(v, "drawn cells", d, "hull cells", proj[v]!.size, "IoU(hull,drawn)=", (proj[v]!.size/d).toFixed(3)); }
console.log("voxels", vox);

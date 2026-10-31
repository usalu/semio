/** ⚡ Author original photographed kitchen appliance envelopes and visible pipe portions. */
const vDot = (a,b)=>a.reduce((s,v,i)=>s+v*b[i],0);
const vCross = (a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const vUnit = v=>{var l=Math.hypot(...v);return v.map(n=>n/l);};
const closedMesh = (vertices,faces,role,material='MAT-METAL')=>{var volume=0;for(var face of faces)for(var i=1;i<face.length-1;i++)volume+=vDot(vertices[face[0]],vCross(vertices[face[i]],vertices[face[i+1]]))/6;if(volume<0)faces=faces.map(f=>[...f].reverse());return{mesh:{vertices,faces},role,material};};
const loftCellMesh = (lo,hi,role,material='MAT-METAL')=>closedMesh([...lo,...hi],[[3,2,1,0],[4,5,6,7],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7]],role,material);
const sourceTubePath = (points,ro,ri,perp,segments=48)=>{var dirs=points.slice(1).map((b,i)=>vUnit(b.map((v,k)=>v-points[i][k]))),parts=[];for(var i=0;i<dirs.length;i++){var d=dirs[i],e=perp,f=vCross(d,e),planes=[i?dirs[i-1].map((v,k)=>v+d[k]):null,i<dirs.length-1?dirs[i+1].map((v,k)=>v+d[k]):null],pt=(radius,angle,end)=>{var p=points[i+end],r=p.map((v,k)=>v+radius*(e[k]*Math.cos(angle)+f[k]*Math.sin(angle))),n=planes[end];if(n){var shift=vDot(n,r.map((v,k)=>v-p[k]))/vDot(n,d);r=r.map((v,k)=>v-shift*d[k]);}return r;};for(var j=0;j<segments;j++){var a=2*Math.PI*j/segments,b=2*Math.PI*(j+1)/segments,ring=end=>[pt(ro,a,end),pt(ro,b,end),pt(ri,b,end),pt(ri,a,end)];parts.push(loftCellMesh(ring(0),ring(1),'pipe_wall_'+i));}}return parts;};
const ringParts = (outer,inner,z0,z1,role,material,upperOuter=null,upperInner=null)=>outer.map((a,i)=>{var j=(i+1)%outer.length;return{outline:[a,outer[j],inner[j],inner[i]],z0,z1,role,material,...(upperOuter?{top_outline:[upperOuter[i],upperOuter[j],upperInner[j],upperInner[i]]}:{})};});
const rectPart = (x0,x1,y0,y1,z0,z1,role,material='MAT-METAL')=>({outline:[[x0,y0],[x1,y0],[x1,y1],[x0,y1]],z0,z1,material,role});
const circle=(x,y,r,n=48,rotation=Math.PI/n)=>Array.from({length:n},(_,i)=>[x+r*Math.cos(2*Math.PI*i/n+rotation),y+r*Math.sin(2*Math.PI*i/n+rotation)]);
const hollowBox=(x0,x1,y0,y1,z0,z1,axis,role)=>{
 const t=.002;
 if(axis==='Z')return [[x0,x0+t,y0,y1,z0,z1],[x1-t,x1,y0,y1,z0,z1],[x0+t,x1-t,y0,y0+t,z0,z1],[x0+t,x1-t,y1-t,y1,z0,z1]].map(b=>rectPart(...b,role));
 if(axis==='Y')return [[x0,x0+t,y0,y1,z0,z1],[x1-t,x1,y0,y1,z0,z1],[x0+t,x1-t,y0,y1,z0,z0+t],[x0+t,x1-t,y0,y1,z1-t,z1]].map(b=>rectPart(...b,role));
 return [[x0,x1,y0,y0+t,z0,z1],[x0,x1,y1-t,y1,z0,z1],[x0,x1,y0+t,y1-t,z0,z0+t],[x0,x1,y0+t,y1-t,z1-t,z1]].map(b=>rectPart(...b,role));
};
export function buildOriginalKitchenAppliances(){
 const metal='MAT-METAL',dark='MAT-KITCHEN-APPLIANCE-DARK',outer=circle(.44,3.65,.21),inner=circle(.44,3.65,.208),neckOuter=circle(.44,3.65,.188),neckInner=circle(.44,3.65,.186),faceX=.44+.21*Math.cos(Math.PI/48),faceY=3.65+.21*Math.cos(Math.PI/48);
 const cylinder=[{outline:outer,z0:.72,z1:.722,material:metal,role:'inferred_body_base'},...ringParts(outer,inner,.722,1.218,'inferred_hollow_cylinder',metal),...ringParts(outer,neckInner,1.218,1.22,'inferred_lid_shoulder',metal),...ringParts(neckOuter,neckInner,1.22,1.238,'inferred_stepped_lid',metal),{outline:neckOuter,z0:1.238,z1:1.24,material:metal,role:'inferred_lid_cap'},...sourceTubePath([[faceX,3.65,1.08],[.72,3.65,1.08],[.72,3.65,.96],[faceX,3.65,.96]],.006,.004,[0,1,0],16).map(p=>({...p,material:dark,role:'inferred_hollow_side_handle'})),{section:{plane:'XZ',profile:circle(.44,.84,.018,32,0),sweep:[faceY,faceY+.006]},material:dark,role:'inferred_visible_control'}];
 const plate=[rectPart(.2,.7,4.075,4.625,.72,.722,'inferred_body_base'),rectPart(.2,.202,4.075,4.625,.722,.795,'inferred_hollow_body'),rectPart(.698,.7,4.075,4.625,.722,.795,'inferred_hollow_body'),rectPart(.202,.698,4.075,4.077,.722,.795,'inferred_hollow_body'),rectPart(.202,.698,4.623,4.625,.722,.795,'inferred_hollow_body'),rectPart(.2,.7,4.075,4.625,.795,.797,'photographed_dark_top',dark),...ringParts(circle(.45,4.35,.09),circle(.45,4.35,.07),.797,.8,'single_visible_ring',dark)];
 const stand=[...hollowBox(.16,.18,3.25,4.85,.70,.72,'Y','inferred_top_frame'),...hollowBox(.76,.78,3.25,4.85,.70,.72,'Y','inferred_top_frame')];
 for(const y of [3.25,4.83,3.55,3.75,4.15,4.53])stand.push(...hollowBox(.18,.76,y,y+.02,.70,.72,'X','inferred_top_frame'));
 for(const x of [.16,.76])for(const y of [3.25,4.83])stand.push(...hollowBox(x,x+.02,y,y+.02,0,.70,'Z','inferred_leg_shell'));
 const rows=[{tag:'KITCHEN-APPLIANCE-CYLINDER',class:'IfcBuildingElementProxy',predefined_type:'USERDEFINED',parts:cylinder,material:metal},{tag:'KITCHEN-APPLIANCE-PLATE',class:'IfcBuildingElementProxy',predefined_type:'USERDEFINED',parts:plate,material:metal},{tag:'KITCHEN-APPLIANCE-STAND',class:'IfcFurnishingElement',parts:stand,material:metal}];
 for(let i=0;i<3;i++){
  const z=.2+i*.07,n=48,angle=Math.PI/n,perp=[Math.cos(angle),0,Math.sin(angle)],parts=sourceTubePath([[.115,1.7,z],[.115,4.85,z]],.0125,.0105,perp,n).map(p=>({...p,role:'inferred_hollow_visible_pipe',material:metal})),face=i===0?.08:.07,outerFace=.115-.0125*Math.cos(angle);
  for(const y of [2.8,4.75])parts.push(rectPart(face,outerFace,y-.0125,y+.0125,z-.0005,z+.0005,'inferred_local_mount'));
  rows.push({tag:'PIPE-KITCHEN-VISIBLE-'+(i+1),class:'IfcPipeSegment',predefined_type:'RIGIDSEGMENT',parts,material:metal});
 }
 return rows;
}

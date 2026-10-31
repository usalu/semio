/** 🛝️ Author explicit ramp finish meshes while preserving finished walklines and sole floor arrival ownership. */
export function buildRampFinish(model, parent, depth, wearing, direction, wearingMaterial, beddingMaterial) {
  const ramp=model.elements.find(e=>e.tag===parent), level=key=>typeof key==='number'?key:model.storeys.find(s=>s.id===key).elevation.value;
  const base=level(ramp.geometry.from_level),end=level(ramp.geometry.to_level),parts=[];
  for(const flight of model.elements.filter(e=>e.class==='IfcRampFlight'&&e.geometry.ramp===parent)){
    const g=flight.geometry,[a,b]=g.path,run=Math.hypot(b[0]-a[0],b[1]-a[1]),n=[-(b[1]-a[1])/run,(b[0]-a[0])/run],zs=g.portion.map(p=>(end-base)*p),slope=(zs[1]-zs[0])/run,factor=direction==='normal'?Math.sqrt(1+slope*slope):1;
    const [lo,hi]=g.walking_offsets.map(v=>v.value),top=[[a[0]+n[0]*hi,a[1]+n[1]*hi,zs[0]],[a[0]+n[0]*lo,a[1]+n[1]*lo,zs[0]],[b[0]+n[0]*lo,b[1]+n[1]*lo,zs[1]],[b[0]+n[0]*hi,b[1]+n[1]*hi,zs[1]]];
    for(const [lo,hi,role,material]of [[depth,wearing,'bedding',beddingMaterial],[wearing,0,'wearing',wearingMaterial]])parts.push({mesh:{vertices:top.map(p=>[p[0],p[1],p[2]-lo*factor]).concat(top.map(p=>[p[0],p[1],p[2]-hi*factor])),faces:[[3,2,1,0],[4,5,6,7],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7]]},material,role});
  }
  for(const landing of ramp.geometry.landings.filter(l=>l.ownership==='ramp_component'))for(const[lo,hi,role,material]of [[depth,wearing,'bedding',beddingMaterial],[wearing,0,'wearing',wearingMaterial]])parts.push({outline:landing.walking_outline,z0:(end-base)*landing.portion-lo,z1:(end-base)*landing.portion-hi,material,role});
  return parts;
}
                                    
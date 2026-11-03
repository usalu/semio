const fs=require("fs");
const path="C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json";
const j=JSON.parse(fs.readFileSync(path,"utf8"));
const depth=j.dimensions.depthM;
for (const k of ["50","100","200"]) {
  const r=j.representations[k];
  for (const s of r.geometry.front) {
    if (s.id==="arm-l") s.x=0.18;
    if (s.id==="arm-r") s.x=k==="200"?-0.28:-0.27;
  }
  for (const s of r.geometry.plan) {
    if (s.id==="arm-l") s.y0=s.y1=0.225;
    if (s.id==="arm-r") s.y0=s.y1=-0.225;
  }
}
const b=j.symbols.find((x)=>x.id==="figure-block");
Object.assign(b.geometry.front[0],{x:-depth/2,w:depth});
b.geometry.plan[0].ry=depth/2;
b.description={en:`The bounding volume of a standing person at real size: ${j.dimensions.widthM} m ahead, ${depth} m across, ${j.dimensions.heightM} m tall.`,de:`Das umhüllende Volumen einer stehenden Person in echter Größe: ${String(j.dimensions.widthM).replace(".",",")} m in Gehrichtung, ${String(depth).replace(".",",")} m quer, ${String(j.dimensions.heightM).replace(".",",")} m hoch.`};
fs.writeFileSync(path,JSON.stringify(j,null,2)+"\n");

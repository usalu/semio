const fs=require("fs");
const path="C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json";
const j=JSON.parse(fs.readFileSync(path,"utf8"));
for (const k of ["50","100","200"]) {
  const r=j.representations[k];
  for (const s of r.geometry.front) {
    if (s.id==="arm-l") s.x=Number((0.23-s.w).toFixed(4));
    if (s.id==="arm-r") s.x=-0.23;
  }
  for (const s of r.geometry.plan) {
    if (s.id==="arm-l") { s.y0=s.y1=Number((0.23-s.r0).toFixed(4)); }
    if (s.id==="arm-r") { s.y0=s.y1=-Number((0.23-s.r0).toFixed(4)); }
  }
}
const block=(kind)=>{ const base={ id:"figure-block", bone:"pelvis", role:"body", z:1, filled:true, tone:0.3 }; return base; };
if (!j.symbols.some((s)=>s.id==="figure-block")) {
  j.symbols.push({
    id:"figure-block",
    title:{en:"Figure block",de:"Figurenblock"},
    description:{en:"The bounding volume of a standing person at real size: 0.37 m ahead, 0.46 m across, 1.73 m tall.",de:"Das umhüllende Volumen einer stehenden Person in echter Größe: 0,37 m in Gehrichtung, 0,46 m quer, 1,73 m hoch."},
    semantics:"true-extent",
    units:"model-m",
    denominators:[500],
    pens:{ body:0.18, detail:null, texture:null, glazing:null, ground:0.25 },
    hatches:[],
    geometry:{
      elevation:[{ ...block(), kind:"rect", x:-0.96, y:-0.185, w:1.73, h:0.37 }],
      front:[{ ...block(), kind:"rect", x:-0.23, y:-0.96, w:0.46, h:1.73 }],
      plan:[{ ...block(), kind:"ellipse", cx:0, cy:0, rx:0.185, ry:0.23 }],
    },
    dimensionalNote:{en:"True extent: the block has the real size of the person's bounding volume; it shows no limbs.",de:"Wahre Ausdehnung: Der Block hat die echte Größe des umhüllenden Volumens der Person; er zeigt keine Gliedmaßen."},
  });
  const u=j.representations["500"];
  if (!u.alternatives.some((a)=>a.kind==="symbol"&&a.symbol==="figure-block")) u.alternatives.push({kind:"symbol",symbol:"figure-block"});
}
fs.writeFileSync(path,JSON.stringify(j,null,2)+"\n");

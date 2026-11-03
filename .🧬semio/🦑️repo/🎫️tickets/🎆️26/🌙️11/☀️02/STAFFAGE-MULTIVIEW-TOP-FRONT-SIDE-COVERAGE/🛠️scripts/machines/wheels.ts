import { catalog, extentOf, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const ids = ["city-car","vintage-car-1929","delivery-van","city-bus","concrete-mixer-truck","wheel-loader","forklift","tram","motor-scooter","city-bicycle","cargo-bike","electric-kick-scooter","excavator","scissor-lift"];
const re = /^(tyre|tire|wheel|track-belt|ws-)/;
for (const id of ids) { const a = catalog().find(x=>x.id===id)!;
  const rows: string[] = [];
  for (const s of authoredScales(a)) {
    const st:any = evaluate(a,{kind:"pose",pose:a.presentation.defaultPose},{kind:"scale",scale:{denominator:s}} as any);
    const el = st.shapes.elevation.filter((x:any)=>re.test(x.shape.id) && x.shape.role!=="ground").map((x:any)=>{const e=extentOf([x])!; return (e.minX+e.maxX)/2;});
    const pl = st.shapes.plan.filter((x:any)=>re.test(x.shape.id)).map((x:any)=>{const e=extentOf([x])!; return [(e.minX+e.maxX)/2,(e.minY+e.maxY)/2];});
    const fr = st.shapes.front.filter((x:any)=>re.test(x.shape.id)).map((x:any)=>{const e=extentOf([x])!; return (e.minX+e.maxX)/2;});
    const xs=[...new Set(pl.map((p:number[])=>p[0].toFixed(2)))].map(Number).sort((p,q)=>p-q); const zs=[...new Set(pl.map((p:number[])=>p[1].toFixed(2)))].map(Number).sort((p,q)=>p-q);
    const fz=[...new Set(fr.map((p:number)=>p.toFixed(2)))].map(Number).sort((p,q)=>p-q);
    rows.push(`1:${s} elev wheel x ${[...new Set(el.map((n:number)=>n.toFixed(2)))].join(",")} | plan axle x ${xs.join(",")} z ${zs.join(",")} | front z ${fz.join(",")}`);
  }
  console.log("##", id); console.log(rows.join("\n"));
}

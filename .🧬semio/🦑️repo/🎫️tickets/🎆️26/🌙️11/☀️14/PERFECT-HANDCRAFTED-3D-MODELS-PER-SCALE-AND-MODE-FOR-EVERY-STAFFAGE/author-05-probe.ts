import { Author } from "./author-05-lib.ts";
const [slug, sc] = process.argv.slice(2);
const a = new Author(slug!, Number(sc));
for (const v of ["elevation","front","plan"] as const) {
  console.log("==",v);
  for (const e of (a as any).rest[v]) { const s=e.shape; const xs=e.rings.flat().map((p:any)=>p.x), ys=e.rings.flat().map((p:any)=>p.y); console.log(s.id,s.bone,s.kind,s.role,s.region??s.regions,s.material??'',s.tone, 'x',Math.min(...xs).toFixed(3),Math.max(...xs).toFixed(3),'y',Math.min(...ys).toFixed(3),Math.max(...ys).toFixed(3)); }
}

/** 🌡️ Author 47: miscanthus thermal patches after kit-thermal: 1:200 culm strokes (slender capsules the kit skips) as references; 1:500 culms as a buried stem solid. */
import { openCtx, saveLists, flag, type Json } from "./kit-lib.ts";
const c = openCtx("vegetation/miscanthus-grass", Number(flag(process.argv.slice(2), "--scale")));
const th: Json[] = c.vol.thermal.filter((t: Json) => !(c.scale === 200 && t.region === "culms" && t.id === "th-tuft-x"));
if (c.scale === 200) for (const b of c.vol.body.filter((s: Json) => /^culm-\da-\d+$/.test(s.id))) th.push({ id: `th-${b.id}`, of: b.id, region: "culms" });
if (c.scale === 500) {
  const keep = th.filter((t: Json) => t.region !== "culms");
  keep.splice(keep.length - 1, 0, { id: "th-culm-stems", kind: "ellipsoid", bone: "root-plate", material: "miscanthus-culm", role: "body", tone: 0.4, radii: [0.45, 0.0413, 0.0413], transform: { translation: [0.5, 0, 0] }, region: "culms" });
  th.splice(0, th.length, ...keep);
}
saveLists(c, { thermal: th });

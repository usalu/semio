import { load, save } from "./lib.ts";
import { shift } from "./lateral.ts";
const d = load("walking-commuter");
for (const k of ["50", "100", "200"]) for (const s of d.asset.representations[k].geometry.front) if (/^bag-(body|strap|flap)$/.test(s.id)) shift(s, "front", -0.021);
save("walking-commuter", d);

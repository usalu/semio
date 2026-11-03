import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const t = readFileSync(file, "utf8");
const start = t.indexOf("/** 🧲️ A world-space force at a point");
const end = t.indexOf("export type WorldForce");
if (start < 0 || end < start) throw new Error("anchor");
const doc = "/** 🧲️ A world-space force at the point `(at.x, at.y, z)`: components `fxN` ahead, `fyN` up, `fzN` left (all in world axes), and a couple `(mxNm, myNm, mzNm)` that acts in addition to the force and is independent of any reference point (a free moment applied at that point). The sign of the couple is positive about axis k when it turns i towards j for the cyclic order (i, j, k): `mxNm` turns y towards z, `myNm` turns z towards x, `mzNm` turns x towards y. `mzNm` is the sagittal moment about the lateral axis: it turns x (ahead) towards y (up), counter-clockwise in the side elevation. Because the world frame is left-handed, a moment arrow follows the left-hand rule; converted to a right-handed frame `(x, y, -z)` the couple is `(-mx, -my, mz)`. The moment of the force itself about a point `p` is `r × F` with `r = (at.x - p.x, at.y - p.y, z - p.z)`. */\n";
writeFileSync(file, t.slice(0, start) + doc + t.slice(end));

import { load, save } from "./lib.ts";
const d = load("excavator"); const a = d.asset;
a.rig.bones = a.rig.bones.filter((b: any) => b.id !== "slew-ring");
const up = a.rig.bones.find((b: any) => b.id === "upper");
up.parent = "undercarriage"; up.x = 0; up.y = 1; up.angleDeg = 0;
up.title = { en: "Upper structure with cab (slew ring fixed in elevation)", de: "Oberwagen mit Kabine (Drehkranz in der Ansicht fest)" };
for (const c of a.clips) for (const k of c.keys) if (k.bones) delete k.bones["slew-ring"];
save("excavator", d);

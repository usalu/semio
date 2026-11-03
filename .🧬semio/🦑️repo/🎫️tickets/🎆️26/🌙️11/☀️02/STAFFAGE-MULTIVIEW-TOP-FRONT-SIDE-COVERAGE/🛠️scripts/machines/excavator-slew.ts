import { load, save } from "./lib.ts";
const d = load("excavator");
const a = d.asset;
if (!a.rig.bones.some((b: any) => b.id === "slew-ring")) {
  const i = a.rig.bones.findIndex((b: any) => b.id === "upper");
  a.rig.bones.splice(i, 0, { id: "slew-ring", parent: "undercarriage", title: { en: "Slew ring (vertical axis)", de: "Drehkranz (senkrechte Achse)" }, x: 0, y: 1, angleDeg: 90, lengthM: 0.1, joint: { type: "revolute", minDeg: 0, maxDeg: 0 } });
  const up = a.rig.bones[i + 1];
  up.parent = "slew-ring"; up.x = 0; up.y = 0; up.angleDeg = -90;
  up.title = { en: "Upper structure with cab (turns on the slew ring)", de: "Oberwagen mit Kabine (dreht auf dem Drehkranz)" };
}
const clip = a.clips.find((c: any) => c.id === "dump-and-stow");
const twist = [0, 12, 35, 62, 82, 90, 90, 90, 90, 90, 78, 55, 30, 12, 3, 0];
clip.keys.slice(0, 16).forEach((k: any, n: number) => { k.bones["slew-ring"] = { twistDeg: twist[n] }; });
save("excavator", d);

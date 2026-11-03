import { load, save } from "./lib.ts";
import { shift } from "./lateral.ts";
const d = load("standing-adult"); const a = d.asset;
const go = (k: string, view: any, id: string, delta: number) => { const s = a.representations[k].geometry[view].find((x: any) => x.id === id); console.log(k, view, id, shift(s, view, delta)); };
for (const k of ["50", "100", "200"]) { go(k, "plan", "forearm-l", 0.011); go(k, "plan", "forearm-r", -0.011); }
for (const id of ["sil-arm", "sil-forearm", "sil-hand"]) { go("500", "front", id + "-l", 0.010); go("500", "front", id + "-r", -0.010); }
go("500", "plan", "sil-forearm-l", 0.011); go("500", "plan", "sil-forearm-r", -0.011);
save("standing-adult", d);

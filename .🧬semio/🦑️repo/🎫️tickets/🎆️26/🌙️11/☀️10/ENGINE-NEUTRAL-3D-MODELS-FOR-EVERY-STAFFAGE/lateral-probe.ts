import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = loadAsset(catalogEntries().find((x) => x.slug === "sleeping-person")!.path);
const rep = a.representations["50"];
const shape = rep.geometry.front.find((s: any) => s.id === "foot-l");
const mini = { ...a, representations: { ...a.representations, "50": { ...rep, geometry: { elevation: [], front: [shape], plan: [] } } } };
const t = performance.now();
const st = evaluate(mini, { kind: "pose", pose: "supine" }, { kind: "scale", scale: { denominator: 50 } } as never) as any;
console.log(performance.now() - t, JSON.stringify(st.shapes.front[0].rings).slice(0, 300), JSON.stringify(st.shapes.front[0].shape.id));

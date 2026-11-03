import { load, save } from "../humans/lib.ts";
const doc = load("wheelchair-user"); const a = doc.asset;
const h = a.representations["50"].geometry.plan.find((s: any) => s.id === "frame-push-handle-r");
h.w = 0.13;
const f = a.representations["500"].geometry.front.find((s: any) => s.id === "sil-footplate");
f.h = 0.07;
save("wheelchair-user", doc);

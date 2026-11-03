import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🪑️objects", "charcoal-grill");
let handle: any;
patchView(f, 500, "plan", (shapes) => { handle = shapes.find((s) => s.id === "handle"); return shapes; });
patchView(f, 500, "front", (shapes) => (shapes.some((s) => s.id === "handle") ? shapes : [...shapes, handle]));

import { SemioKitClient } from "/home/user/semio/semio/client/lib/react/index.ts";
const payload = JSON.stringify({
  id: "kit-import-test", name: "Import Test",
  families: [{ id: "fam1", name: "Tower", ports: [ { id: "p1", name: "bottom", compatiblePorts: [{ id: "p2" }] }, { id: "p2", name: "top", compatiblePorts: [{ id: "p1" }] } ] }],
  types: [ { id: "t1", name: "A", connectors: [{ id: "c1", name: "c1", port: { id: "p1" } }] }, { id: "t2", name: "B", connectors: [{ id: "c2", name: "c2", port: { id: "p2" } }] } ],
  designs: [],
});
const c = await SemioKitClient.open({ kind: "bytes", data: new TextEncoder().encode(payload) });
console.log("[DEBUG]", JSON.stringify(c.getSnapshot(), null, 1).slice(0, 2500));
const r = await c.execute((kit) => kit.createDesign("Layout A"));
console.log("[DEBUG] create", r, c.getSnapshot().designs?.map((d) => d.name));
await c.dispose();

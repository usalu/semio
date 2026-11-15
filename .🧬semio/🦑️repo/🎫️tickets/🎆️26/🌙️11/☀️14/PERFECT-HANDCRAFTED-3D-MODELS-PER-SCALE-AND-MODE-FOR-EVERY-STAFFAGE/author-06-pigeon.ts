import { run } from "./author-06-run.ts";
import { notesFor } from "./author-06-notes.ts";
await run("pigeon", { extraBody: (c) => {
  if (c.scale !== 50) return;
  const l = c.lat("head", "head")!;
  const z = l[0] * 0.78;
  const w = c.world("head", [0.024, 0.007, 0]);
  c.ell("eye", "head", [w[0], w[1], z], [0.006, 0.006, 0.0045], 0, { regions: ["eye"], role: "body", tone: 0.1 });
  c.ell("eye-ring", "head", [w[0], w[1], z * 0.97], [0.0085, 0.0085, 0.004], 0, { regions: ["eye"], role: "body", tone: 0.7 });
}, emit: { "breath-nostril": "cere" }, notes: notesFor("pigeon"), struct: () => ({ spine: ["trunk", "pelvis"] }) });

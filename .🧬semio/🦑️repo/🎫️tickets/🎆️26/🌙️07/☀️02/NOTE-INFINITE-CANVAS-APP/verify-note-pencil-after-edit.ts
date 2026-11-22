/** 🎬 Seeds a real Note text edit before reusing the isolated Pencil and eraser actions. */
import { dirname } from "node:path";

Bun.plugin({
  name: "note-pencil-after-edit",
  setup(build) {
    build.onLoad({ filter: /note-acceptance\.mjs$/ }, async ({ path }) => {
      const original = await Bun.file(path).text();
      const marker = "if (exercisePencil) {";
      const seed = `
if (pencilOnly) {
  const baseline = await submitAction("addBlock", {kind: "text", x: 320, y: 180});
  const editingProved = await settle(state => state.blocks === baseline.before + 1);
  await note("pencil-baseline-edit", { before: baseline.before, after: editingProved.blocks, submitted: baseline.submitted }, baseline.from);
  assert.equal(baseline.submitted, "ok", "the baseline edit must use the normal Canvas Action");
  assert.equal(editingProved.blocks, baseline.before + 1, "the baseline Text block must be published");
  const actionToggle = page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first();
  if (editingProved.actionRows.length) await actionToggle.click();
}
`;
      if (original.split(marker).length !== 2) throw Error("The Note Pencil acceptance entry is absent");
      return { contents: original.replace(marker, seed + marker), loader: "js", resolveDir: dirname(path) };
    });
  },
});

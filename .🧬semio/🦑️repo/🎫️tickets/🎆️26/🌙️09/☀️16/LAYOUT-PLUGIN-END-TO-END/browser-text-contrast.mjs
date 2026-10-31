/** 🔆️ Measures one real retained text control from its renderer-owned linear color projection. */
import assert from "node:assert/strict";
export function assertRenderedTextContrast(structure, frame, witness, minimumRatio = 4.5) {
  assert.equal(structure.windowId, witness.window, "contrast renderer owner");
  assert.equal(frame.windowId, witness.window, "contrast painted frame owner");
  assert.ok(frame.glyphCount > 0, "contrast owner actually paints text glyphs");
  const nodes = structure.nodes.filter(node => node.path.split("/").at(-1)?.split("#").slice(1).join("#") === witness.key);
  assert.equal(nodes.length, 1, "same declared node in semantic and rendered projection");
  const node = nodes[0];
  assert.ok(["button", "input"].includes(witness.kind), "known retained paint-color mapping");
  assert.equal(node.kind, witness.kind, "same actual control kind");
  assert.equal(node.text, witness.label, "projected text matches the actual accessible control");
  assert.ok(node.text.trim().length > 0 && node.fontSize > 0 && node.visible && !node.state.disabled, "measured text is visible and enabled");
  assert.ok(Array.isArray(node.rect) && node.rect.length === 4 && node.rect.every(Number.isFinite) && node.rect[2] > 0 && node.rect[3] > 0, "measured node has actual layout bounds");
  for (const color of [node.color, node.bg]) assert.ok(Array.isArray(color) && color.length === 4 && color.every(value => Number.isFinite(value) && value >= 0 && value <= 1) && color[3] === 1, "opaque measured linear foreground and background required");
  const luminance = color => 0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2];
  const a = luminance(node.color), b = luminance(node.bg), ratio = (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
  assert.ok(ratio + 1e-12 >= minimumRatio, "actual rendered normal text contrast meets " + minimumRatio + ":1 (observed " + ratio + ")");
  return { valid: true, measurement: "retained-render-color-projection", owner: witness.window, key: witness.key, text: node.text, rect: node.rect, linearForeground: node.color, linearBackground: node.bg, ratio, minimumRatio, glyphCount: frame.glyphCount };
}

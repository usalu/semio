// ⚖️ c4p30: evaluates the four tool-job coverage laws that read the trinity jack editor and the draw editor sources, with the source text of the unsplit snapshots (orig), of the live root files only (file) and of the live module trees (tree). usage: bun laws-probe.ts
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { policyReadFileSafe, policyReadModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { policyReadOsStoreSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { toolJobDrawingEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";
import { toolJobDrawingGestureOperationOwnerExact } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/⚙️fixed-operation/🟦️.ts";
import { toolJobProductionSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const orig = (name: string): string => readFileSync(join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p30-tools/orig", name), "utf8");
const T = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts";
const D = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing";
const E = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const JACK_EDITOR = `${T}/🔌️jack/${E}/🦀️.rs`;
const DRAW_EDITOR = `${D}/${E}/🦀️.rs`;
const POINTER = `${D}/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`;
const WINDOW = `${D}/${E}/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs`;
const store = policyReadOsStoreSource(root);
const plugin = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
const jackEnvelopeCodec = policyReadFileSafe(root, `${T}/🔌️jack/${E}/🚪️io/💾️binary/🧬️mutations/🦀️.rs`);
const jackWasm = policyReadFileSafe(root, `${T}/🔌️jack/${E}/🌉️wasm/🦀️component.rs`);
const trinityRewrite = policyReadFileSafe(root, `${T}/♻️rewriting/${E}/🌍️world/🦀️.rs`);
const drawingEnvelopeCodec = policyReadFileSafe(root, `${D}/${E}/🧬️schema/🧰️owned/🦀️.rs`);
const drawingConfig = policyReadFileSafe(root, `${D}/${E}/🎚️config/🦀️.rs`);
const drawingConfigProto = policyReadFileSafe(root, `${D}/${E}/🎚️config/🧬️schema/🛰️.proto`);
const window = policyReadFileSafe(root, WINDOW);
const modes: Record<string, { jack: string; draw: string[] }> = {
  orig: { jack: orig("jack-editor.rs.txt"), draw: [orig("draw-editor.rs.txt"), orig("draw-pointerdown.rs.txt"), window] },
  file: { jack: policyReadFileSafe(root, JACK_EDITOR), draw: [policyReadFileSafe(root, DRAW_EDITOR), policyReadFileSafe(root, POINTER), window] },
  tree: { jack: policyReadModuleTreeSource(root, JACK_EDITOR), draw: [policyReadModuleTreeSource(root, DRAW_EDITOR), policyReadModuleTreeSource(root, POINTER), window] },
};
for (const [mode, m] of Object.entries(modes)) {
  const drawingEditor = m.draw.map(toolJobProductionSource).join("\n");
  const res = {
    mode,
    jackEditorBytes: m.jack.length,
    drawingEditorBytes: drawingEditor.length,
    gestureOperationOwner: toolJobDrawingGestureOperationOwnerExact(drawingEditor, drawingConfig, drawingConfigProto, plugin),
    jackEnvelope: toolJobJackEnvelopeCallerRetainedExact(store, jackEnvelopeCodec, m.jack, jackWasm, plugin),
    trinityRewrite: toolJobTrinityRewriteEnvelopeCallerRetainedExact(store, jackEnvelopeCodec, m.jack, trinityRewrite, plugin),
    drawingEnvelope: toolJobDrawingEnvelopeCallerRetainedExact(store, drawingEnvelopeCodec, drawingEditor, plugin, m.draw[0] ?? ""),
  };
  console.log(JSON.stringify(res));
}

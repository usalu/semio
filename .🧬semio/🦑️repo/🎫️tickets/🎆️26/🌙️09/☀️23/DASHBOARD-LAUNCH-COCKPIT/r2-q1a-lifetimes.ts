import { patch } from "./r2-q1a-edit.ts";

patch("⌨️tui/🖥️chrome/🦀️.rs", [["stack: &WindowLayoutStackNode, slot: &StackSlot) {", "stack: &WindowLayoutStackNode, slot: &StackSlot<'_>) {"]]);
patch("🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs", [
  ["is_bottom: bool, frame: &Frame, paint_text", "is_bottom: bool, frame: &Frame<'_>, paint_text"],
  ["active: bool, is_bottom: bool, frame: &Frame) {", "active: bool, is_bottom: bool, frame: &Frame<'_>) {"],
  ["layout: &WindowChipLayout, frame: &Frame) {", "layout: &WindowChipLayout, frame: &Frame<'_>) {"],
]);
console.log("lifetimes");

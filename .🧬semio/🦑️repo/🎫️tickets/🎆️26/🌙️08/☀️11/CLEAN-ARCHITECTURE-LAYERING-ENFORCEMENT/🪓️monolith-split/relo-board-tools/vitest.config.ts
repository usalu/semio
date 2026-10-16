/** 🧪️ Slice D vitest scope: the board host suites, the scene lane suite and the pointer-cancellation source contract. */
import { resolve } from "node:path";
import { mergeConfig } from "vitest/config";
import base from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const E = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const include = [
  resolve(E, "🧱️elements/🖥️Board2dHost/🧪️tests/**/🟦️.{ts,tsx}"),
  resolve(E, "🧱️elements/🖥️Board2dHost/⏯️tool-run-trace/🧪️tests/**/🟦️.{ts,tsx}"),
  resolve(E, "🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx"),
  resolve(E, "🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts"),
];
export default mergeConfig(base as never, { test: { include }, server: { fs: { strict: false } } } as never);

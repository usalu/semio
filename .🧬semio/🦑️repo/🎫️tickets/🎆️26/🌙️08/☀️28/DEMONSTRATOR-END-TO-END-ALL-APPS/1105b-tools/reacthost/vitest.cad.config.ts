import { defineConfig, mergeConfig } from "vitest/config";
import base from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

export default mergeConfig(base, defineConfig({ test: { name: "cad-renderer", include: [], includeSource: ["/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx"] } }), true);

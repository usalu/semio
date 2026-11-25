import { renderCargoProfileConfigurationV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";

const profile = renderCargoProfileConfigurationV1({
  version: 1,
  mode: "prototype",
  target: "wasm32-unknown-unknown",
  producer: "browser-module",
  operation: "build",
}, "🌎️hub/Cargo.toml", "repository");

process.stdout.write(profile.selectors.join("\n") + "\n");

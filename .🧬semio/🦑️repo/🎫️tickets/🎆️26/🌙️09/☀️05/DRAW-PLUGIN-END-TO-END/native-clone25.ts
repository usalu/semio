import { renderCargoProfileConfigurationV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";
const configuration = renderCargoProfileConfigurationV1({ version: 1, mode: "prototype", target: "wasm32-wasip2", producer: "wasm-component", operation: "build", composition: "semio-hub-draw-component" }, "🌎️hub/Cargo.toml", "repository");
const args = ["build", "--manifest-path", "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml", "-p", "semio-hub-draw-component", "--lib", "--target", "wasm32-wasip2", "--profile", "prototype", "--locked", "--offline", "--color", "never", "--message-format", "short", ...configuration.selectors.flatMap(value => ["--config", value])];
console.error("[DEBUG] Canonical profile", JSON.stringify(configuration));
const process = Bun.spawn(["repo", "tools", "exec", "cargo", "--", ...args], { stdout: "inherit", stderr: "inherit", env: Bun.env });
await process.exited.then(code => globalThis.process.exit(code));

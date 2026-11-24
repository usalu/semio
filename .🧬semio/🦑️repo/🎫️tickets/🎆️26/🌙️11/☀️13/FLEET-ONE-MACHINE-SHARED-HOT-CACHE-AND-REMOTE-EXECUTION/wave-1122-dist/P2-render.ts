import {renderCargoProfileConfigurationV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";
const r=renderCargoProfileConfigurationV1({version:1,mode:"release",target:"wasm32-wasip2",producer:"wasm-component",operation:"build"},"🌎️hub/Cargo.toml","repository");
console.log(JSON.stringify({role:r.plan.role,selectors:r.selectors}));

import { readdirSync, statSync } from "fs";
import { join } from "path";

const world = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx";
let text = await Bun.file(world).text();
if (!/\bSuspense\b/.test(text.match(/from "react";/) ? text.slice(0, text.indexOf('from "react";') + 20) : "")) {
  // check named import
}
const reactImport = text.match(/import React, \{([^}]*)\} from "react";/);
if (!reactImport) {
  console.error("react import shape unexpected");
  process.exit(1);
}
if (/\bSuspense\b/.test(reactImport[1])) {
  console.log("World3dHost already imports Suspense");
} else {
  const old = reactImport[0];
  // insert Suspense after useState alphabetically-ish: after useRef, before useSyncExternalStore
  const next = old.replace("useState, useSyncExternalStore", "useState, Suspense, useSyncExternalStore");
  if (next === old) {
    // fallback append
    const next2 = old.replace("} from \"react\";", ", Suspense } from \"react\";").replace(", ,", ",");
    text = text.replace(old, next2);
  } else {
    text = text.replace(old, next);
  }
  await Bun.write(world, text);
  console.log("added Suspense to World3dHost");
}

// WasmSessionLoader
const wasm = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/WasmSessionLoader/🟦️component.tsx";
const wasmText = await Bun.file(wasm).text();
console.log("--- WasmSessionLoader head ---");
console.log(wasmText.split("\n").slice(0, 100).join("\n"));
console.log("--- around line 60-90 ---");
console.log(wasmText.split("\n").slice(55, 95).join("\n"));

import { readFileSync } from "fs";

const paths = [
  "/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/package.json",
  "/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/flow_core.js",
  "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies",
];
const nmPkg = JSON.parse(readFileSync("/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/package.json","utf8"));
console.log("nm pkg", nmPkg);
const js = readFileSync("/Users/ueli/Documents/semio/node_modules/@semio-tech/flow-core/flow_core.js","utf8");
console.log("js bytes", js.length);
// find export default / __wbg_init
for (const pat of [/export default [^\n]+/, /export \{[^}]*as default[^}]*\}/, /function __wbg_init/, /async function __wbg_init/, /export async function init/, /export \{[^}]*__wbg_init[^}]*\}/]) {
  const m = js.match(pat);
  if (m) console.log("match", pat, "=>", m[0].slice(0,200));
}
console.log("tail:\n", js.slice(-800));
console.log("head:\n", js.slice(0,400));

// compare workspace pkg
const ws = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies";
const wsJs = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies";
const real = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies";
const realJs = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/
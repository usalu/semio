const fs = require("node:fs");
const original = fs.readFileSync;
const marker = "(declare the built wasm package";
fs.readFileSync = function (path, ...rest) {
  const out = original.call(this, path, ...rest);
  const text = typeof out === "string" ? out : Buffer.isBuffer(out) && out.length < 200000 ? out.toString("utf8") : "";
  if (!text.includes(marker) && !text.includes('": ".//')) return out;
  let fixed = text.replace(/": "\.\/\/+/gu, '": "./');
  if (fixed.includes(marker)) {
    const json = JSON.parse(fixed);
    for (const key of Object.keys(json.exports ?? {})) if (!key.startsWith(".")) delete json.exports[key];
    fixed = JSON.stringify(json);
  }
  return typeof out === "string" ? fixed : Buffer.from(fixed);
};

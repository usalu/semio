const fs = require("node:fs");
const orig = fs.readFileSync;
const fix = (v) => typeof v === "string" ? v.replace(/^\.\/\/+/u, "./") : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, fix(x)])) : v;
fs.readFileSync = function (target, ...rest) {
  const data = orig.call(this, target, ...rest);
  const text = Buffer.isBuffer(data) ? data.toString("utf8") : typeof data === "string" ? data : null;
  if (text === null || !text.includes('"exports"') || text.length > 2e5 || text[0] !== "{") return data;
  let json;
  try { json = JSON.parse(text); } catch { return data; }
  if (!json || typeof json.name !== "string" || !json.exports || typeof json.exports !== "object") return data;
  const entries = Object.entries(json.exports).filter(([k]) => k.startsWith("."));
  if (entries.length === Object.keys(json.exports).length && !text.includes('".//')) return data;
  json.exports = fix(Object.fromEntries(entries));
  const next = JSON.stringify(json, null, 2) + "\n";
  return Buffer.isBuffer(data) ? Buffer.from(next) : next;
};

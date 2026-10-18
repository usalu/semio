import { readFileSync } from "node:fs";
const bytes = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/📋️data/🟨️.mjs");
const blob = "Bun" in globalThis;
const target = blob ? URL.createObjectURL(new Blob([bytes], { type: "text/javascript" })) : "data:text/javascript;base64," + bytes.toString("base64");
try { const m = await import(target); console.log("loaded", Object.keys(m).slice(0,5)); } catch (e) { console.log("ERR", String(e).slice(0, 400)); }

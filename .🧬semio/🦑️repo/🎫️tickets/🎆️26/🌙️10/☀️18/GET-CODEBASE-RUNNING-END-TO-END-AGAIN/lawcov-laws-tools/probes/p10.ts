import * as mod from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/📋️allowlists/🟦️.ts";
const out: Record<string, unknown> = {};
for (const [name, value] of Object.entries(mod)) out[name] = value instanceof Set ? { kind: "set", values: [...value].sort() } : Array.isArray(value) ? { kind: "array", values: value } : { kind: "string", values: [value] };
console.log(JSON.stringify(out));

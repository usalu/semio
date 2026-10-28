// [gram3-g] counting stand-in for "node:crypto" (createHash counted by call site); not part of any batch.
import * as real from "crypto";
export const counters = (globalThis.__g3 ??= {sites: new Map(), add(kind, site, n = 1, bytes = 0) { const key = kind + " " + site; const row = this.sites.get(key) ?? {n: 0, bytes: 0}; row.n += n; row.bytes += bytes; this.sites.set(key, row); }});
export const siteOf = (skip = 3) => { const lines = new Error().stack.split("\n"); for (let i = skip; i < lines.length; i++) { const l = lines[i]; if (!l.includes("count/shim.mjs") && !l.includes("count/preload")) return l.trim().replace(/^at /, "").replace(/^.*🏗️build\//, "").slice(0, 150); } return "?"; };
export const createHash = (...a) => { counters.add("createHash", siteOf()); return real.createHash(...a); };
export const hash = (...a) => { counters.add("hash", siteOf()); return real.hash(...a); };
export * from "crypto";
export default {...real, createHash, hash};

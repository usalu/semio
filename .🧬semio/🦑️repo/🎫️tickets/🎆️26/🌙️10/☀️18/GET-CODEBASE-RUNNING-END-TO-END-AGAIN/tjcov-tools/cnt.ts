import { policyAllRustFiles } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🏛️foundation/🟦️.ts";
import { listSourceFamily } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { relative, sep } from "node:path";
const R = "/Users/ueli/Documents/semio";
const a = policyAllRustFiles(R).filter((f) => f.includes("✏️s/🔌️plugins/") && !f.includes("/🧪️tests/"));
const b = new Set(listSourceFamily(R + "/✏️s/🔌️plugins").map((x) => relative(R, x).split(sep).join("/")).filter((f) => f.endsWith(".rs") && !f.includes("/🧪️tests/")));
const miss = a.filter((f) => !b.has(f));
console.log("policy", a.length, "family", b.size, "missing-from-family", miss.length);
const by = new Map<string, number>(); for (const f of miss) { const k = f.split("/").slice(0, 4).join("/"); by.set(k, (by.get(k) ?? 0) + 1); }
console.log([...by].slice(0, 30));
console.log(miss.slice(0, 5));

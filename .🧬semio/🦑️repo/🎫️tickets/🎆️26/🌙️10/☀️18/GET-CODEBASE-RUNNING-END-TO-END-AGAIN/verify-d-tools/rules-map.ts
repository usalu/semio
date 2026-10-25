import { loadDependencyDirectionPolicy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { dependencyDirectionStrictRules } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts";
const snap = loadDependencyDirectionPolicy("/Users/ueli/Documents/semio");
const strict = dependencyDirectionStrictRules(snap);
const pol = (snap.policy as any).forbidden as any[];
console.log("POLICY", pol.length); for (const r of pol) console.log(" P", r.name, r.severity, JSON.stringify(r.from).slice(0,90), "->", JSON.stringify(r.to).slice(0,120));
console.log("STRICT", strict.length); for (const r of strict) console.log(" S", r.name, r.severity, JSON.stringify(r.from).slice(0,90), "->", JSON.stringify(r.to).slice(0,120));
console.log(JSON.stringify((snap.policy as any).options).slice(0,600));

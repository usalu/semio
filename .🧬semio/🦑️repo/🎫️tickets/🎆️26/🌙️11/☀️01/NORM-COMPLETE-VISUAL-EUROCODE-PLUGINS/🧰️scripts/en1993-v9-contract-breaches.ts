import { repoWideBreaches } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧰️repository-scan/🟦️.ts";
const all = (repoWideBreaches as any[]).map((b) => typeof b === "string" ? b : JSON.stringify(b));
console.log("total", all.length);
for (const b of all) if (/en1993|rainflow/i.test(b)) console.log(b.slice(0, 600));

import { loadOracleRegistry } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const registry: any = loadOracleRegistry("/home/deck/semio");
for (const c of registry.contributions) {
  if (!String(c.owner ?? c.manifestPath).includes("en1993")) continue;
  console.log(c.manifestPath, "problems:", JSON.stringify(c.problems, null, 1));
  console.log("oracles", c.oracles.map((o: any) => o.id).join(","));
}

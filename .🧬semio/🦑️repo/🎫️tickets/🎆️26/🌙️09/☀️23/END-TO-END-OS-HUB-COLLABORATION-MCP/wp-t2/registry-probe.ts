import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const r = loadOracleRegistry(process.cwd());
const what = process.argv[2];
if (what === "tolerances") for (const t of r.toleranceProfiles) console.log(JSON.stringify(t));
if (what === "fixtures") for (const f of r.fixtureManifests.filter((f: any) => JSON.stringify(f).includes(process.argv[3] ?? ""))) console.log(JSON.stringify(f));
if (what === "catalogs") for (const c of r.mutationCatalogs) console.log(c.id, JSON.stringify(c).slice(0, 300));

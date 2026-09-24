// 🧫️ Lists the live mutation-without-fixture rows (the test-platform's own rule) grouped by manifest.
import { loadOracleRegistry, mutationFixtureBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const rows = mutationFixtureBreaches(loadOracleRegistry("/Users/ueli/Documents/semio"));
const by = new Map<string, string[]>();
for (const r of rows) { const m = r.summary.match(/^Mutation (\S+) of (\S+) /)!; by.set(`${m[2]} ${r.scope}`, [...(by.get(`${m[2]} ${r.scope}`) ?? []), m[1]!]); }
console.log(rows.length);
for (const [k, v] of by) console.log(v.length, k, v.join(","));

/** 🔒️ Moves the nine carrier-engine manifests from the serde_json baseline entry onto json-rust, with oracle ids derived from the live registry. */
import { readFileSync, writeFileSync } from "node:fs";
import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio", path = `${root}/🔒️dependencies.json`;
const raw = readFileSync(path, "utf8"), baseline = JSON.parse(raw);
const engines = (users: string[]): string[] => users.filter((user) => user.includes("/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.toml") && /^json = "0\.12"$/m.test(readFileSync(`${root}/${user}`, "utf8")));
const serde = baseline.entries.find((entry: any) => entry.ecosystem === "rust" && entry.name === "serde_json"), json = baseline.entries.find((entry: any) => entry.ecosystem === "rust" && entry.name === "json");
const moved = engines(serde.users);
serde.users = serde.users.filter((user: string) => !moved.includes(user));
delete serde.oracleIds;
json.users = [...new Set([...json.users, ...moved])].sort();
json.oracleIds = [...new Set(loadOracleRegistry(root).oracles.filter((oracle: any) => oracle.ecosystem === "rust" && oracle.package === "json").map((oracle: any) => oracle.id))].sort();
writeFileSync(path, `${JSON.stringify(baseline, null, 2)}\n`);
console.log(`moved ${moved.length} engine manifest(s); json oracleIds: ${json.oracleIds.join(", ")}`);

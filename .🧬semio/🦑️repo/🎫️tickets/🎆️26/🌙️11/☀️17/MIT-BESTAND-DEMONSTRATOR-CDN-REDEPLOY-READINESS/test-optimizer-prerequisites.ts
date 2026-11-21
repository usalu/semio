import {test, expect} from "bun:test";
import {resolve} from "node:path";
import {Database} from "bun:sqlite";
import fixture from "./🔣️optimizer-prerequisites.json";
const directory = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const {componentDriverStagesV1} = await import(resolve(directory, "🏗️build/🧩️driver-stages/🟦️.ts"));
test("the release optimizer admits only a source-verified linked component", async () => {
  const component = {pass:{id:fixture.pass,profile:"release"},package:{manifest:fixture.manifest},wasm:"unused",staged:"unused"};
  const stages = componentDriverStagesV1(process.cwd(), {}, {}, "browser", "verify", () => {})(component);
  const optimizer = stages.find(row => row.name === "optimize");
  const db = new Database(":memory:");
  try {
    const expected = db.query("SELECT value FROM json_each(?) ORDER BY value").all(JSON.stringify(fixture.requires)).map(row => row.value);
    expect([...optimizer.after].sort()).toEqual(expected);
    for (const independent of fixture.independent) expect(optimizer.after).not.toContain(independent);
    await expect(optimizer.run({provenance:async () => { throw Error(fixture.rejection); }})).rejects.toThrow(fixture.rejection);
  } finally { db.close(); }
});

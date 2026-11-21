import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(process.cwd(), "🧰️framework");
const fixture = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));

test("actor preparation requires a completed import-free constructor", () => {
  const { cases, storageCases } = fixture("🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🔌️owned-semio-boundary/🌱️preparation/🧪️tests/🧫️fixtures/🔣️.json");
  for (const row of cases) {
    const module = new WebAssembly.Module(Buffer.from(row.component, "hex"));
    const imports: string[] = [];
    const instance = new WebAssembly.Instance(module, { env: { seed() { imports.push("env::seed"); throw Error("sandboxed"); } } });
    let ready = false;
    if (row.fuel && row.continue) {
      try { ready = (instance.exports.semio_owned_prepare_actor_v1 as () => number)() === 1; } catch (error) {
        expect((error as Error).message).toBe("sandboxed");
      }
    }
    expect(ready).toBe(row.ready);
    expect(imports).toEqual(row.hostImport ? [row.hostImport] : []);
  }
  for (const row of storageCases) {
    const run = (source: string) => {
      const module = new WebAssembly.Module(Buffer.from(source, "hex"));
      return (new WebAssembly.Instance(module).exports.semio_owned_prepare_actor_v1 as () => number)();
    };
    expect(run(row.component)).toBe(0);
    expect(run(row.oracle)).toBe(1);
  }
});

test("factory keys follow stable controller and tool coordinates", () => {
  const data = fixture("🔨️modules/🎯️action-bus/🧪️tests/🔬️unit/🧫️ordered-registry.json");
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE factories(controller TEXT, tool TEXT, PRIMARY KEY (controller, tool))");
    for (const [controller, tool] of data.registration) db.query("INSERT INTO factories VALUES (?, ?)").run(controller, tool);
    expect(db.query("SELECT controller, tool FROM factories ORDER BY controller COLLATE BINARY, tool COLLATE BINARY").values()).toEqual(data.factoryOrder);
    expect(db.query("SELECT controller, tool FROM factories WHERE controller = ? AND tool = ?").values(...data.alias.target)).toEqual([data.alias.target]);
    expect(db.query("SELECT controller, tool FROM factories WHERE controller = ? AND tool = ?").values(...data.alias.key)).toEqual([]);
  } finally { db.close(); }
});

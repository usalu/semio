import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

test("selected member factory graph prices every original Arc and preborn ticket once", () => {
  type Node = { kind: string; children: Node[] };
  const fold = (node: Node): number => 1 + node.children.reduce((sum, child) => sum + fold(child), 0);
  for (const row of fixture.cases) {
    const nodes = JSON.parse(Buffer.from(JSON.stringify(row.preparation), "utf8").toString("utf8")) as Node;
    expect(row.baseFactories.length + fold(nodes)).toBe(row.factoryCount);
  }
  let root = import.meta.dir;
  while (!existsSync(join(root, "nx.json"))) { const parent = dirname(root); if (parent === root) throw new Error("canonical workspace root absent"); root = parent; }
  const store = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  expect(store.includes("document_store_owners_constructor_birth_bytes")).toBe(true);
});

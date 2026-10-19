import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { frameworkSourcePath } from "@semio-tech/framework/test/source-text";
import { findWorkspaceRoot } from "@semio-tech/framework/process/routing";

test("selected member factory graph prices every original Arc and preborn ticket once", () => {
  type Node = { kind: string; children: Node[] };
  const fold = (node: Node): number => 1 + node.children.reduce((sum, child) => sum + fold(child), 0);
  for (const row of fixture.cases) {
    const nodes = JSON.parse(Buffer.from(JSON.stringify(row.preparation), "utf8").toString("utf8")) as Node;
    expect(row.baseFactories.length + fold(nodes)).toBe(row.factoryCount);
  }
  findWorkspaceRoot(import.meta.dir);
  const store = readFileSync(frameworkSourcePath("🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  expect(store.includes("document_store_owners_constructor_birth_bytes")).toBe(true);
});

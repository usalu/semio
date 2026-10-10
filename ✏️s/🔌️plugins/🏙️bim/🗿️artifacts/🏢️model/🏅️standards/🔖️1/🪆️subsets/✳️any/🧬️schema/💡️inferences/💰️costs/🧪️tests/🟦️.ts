import {expect, test} from "bun:test";
import Ajv from "ajv";
import Decimal from "decimal.js";
import contract from "../../../📸️snapshot/💰️costs/🔣️.json";
import fixture from "../../../../🧫️fixtures/💡️inferences/💰️costs/🔣️.json";
import {inferCosts} from "../🟦️.ts";
import type {CostItem, TypeCostLink} from "../../../📸️snapshot/💰️costs/🟦️.ts";

test("BIM authored cost records satisfy the schema and reject invalid prices", () => {
  const validator = new Ajv().addSchema(contract);
  const item = validator.getSchema(contract.$id + "#/$defs/CostItem")!;
  const link = validator.getSchema(contract.$id + "#/$defs/TypeCostLink")!;
  for (const value of Object.values(fixture.items)) expect(item(value)).toBe(true);
  for (const value of Object.values(fixture.links)) expect(link(value)).toBe(true);
  expect(item({...fixture.items.finish, unit_cost: -1})).toBe(false);
  expect(item({...fixture.items.finish, currency: "  "})).toBe(false);
  expect(link({...fixture.links["wall-finish"], factor: -1})).toBe(false);
});

test("BIM costs agree with independent decimal arithmetic and currency totals", () => {
  const costs = inferCosts(fixture.items as Record<string, CostItem>, fixture.links, fixture.elements);
  expect(costs.project).toEqual(fixture.expected.project);
  expect(costs.storeys).toEqual(fixture.expected.storeys);
  expect(costs.types).toEqual(fixture.expected.types);
  expect(costs.items).toEqual(fixture.expected.items);
  for (const row of costs.lines) {
    expect(row.total).toBe(new Decimal(row.quantity).mul(row.unit_cost).mul(row.factor).toDecimalPlaces(row.precision, Decimal.ROUND_HALF_UP).toNumber());
  }
  expect(costs.diagnostics).toEqual([]);
  console.log("[DEBUG] BIM costs matched decimal.js for all lines and independent authored fixture totals.");
});

test("BIM quantity edits and inverse replay update costs without mutating authored records", () => {
  const items = fixture.items as Record<string, CostItem>;
  const links = fixture.links as Record<string, TypeCostLink>;
  const before = JSON.stringify({items, links});
  const original = inferCosts(items, links, fixture.elements);
  const doubled = {...fixture.elements, "wall-a": {...fixture.elements["wall-a"], net_side_area: 16, net_volume: 4}};
  expect(inferCosts(items, links, doubled).elements["wall-a"]).toEqual({EUR: 620});
  expect(inferCosts(items, links, fixture.elements)).toEqual(original);
  expect(JSON.stringify({items, links})).toBe(before);
});

test("BIM costing reports missing references and numeric overflow without incomplete totals", () => {
  const bad = {
    ...fixture.links,
    missing: {type_id: "wall-type", item: "absent", factor: 1},
    overflow: {type_id: "wall-type", item: "structure", factor: Number.MAX_VALUE}
  };
  const result = inferCosts(fixture.items as Record<string, CostItem>, bad, fixture.elements);
  expect(result.diagnostics.some(row => row.link === "missing" && row.code === "cost.item-missing")).toBe(true);
  expect(result.diagnostics.some(row => row.link === "overflow" && row.code === "cost.overflow")).toBe(true);
  expect(result.project).toEqual(fixture.expected.project);
  expect(result.lines.every(row => Number.isFinite(row.total))).toBe(true);
});

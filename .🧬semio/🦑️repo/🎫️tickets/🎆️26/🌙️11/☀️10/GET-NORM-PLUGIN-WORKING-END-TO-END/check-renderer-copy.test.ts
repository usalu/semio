import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import stock from "../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🧵️renderer-work/🔣️stock.json";
import schema from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧬️schema/🔣️.json";
import {
  KernelOriginalBootstrapStock,
  originalBootstrapStockFromWire,
} from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🟦️.ts";

test("the renderer copy grant funds four original actor admissions", () => {
  expect(new Ajv({ strict: true }).compile(schema)(stock)).toBe(true);
  const renderer = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stock));
  const database = new Database(":memory:");
  try {
    const originalActors = database.query(
      "SELECT ? / ? AS count, ? - 4 * ? AS remaining",
    ).get(stock.stock.grant.maximumCopyBytes, stock.operation.grant.maximumCopyBytes,
      stock.stock.grant.maximumCopyBytes, stock.operation.grant.maximumCopyBytes);
    expect(originalActors).toEqual({ count: 4, remaining: 0 });
    const actors = Array.from({ length: 4 }, () => renderer.admit());
    expect(new Set(actors.map(actor => actor.operation)).size).toBe(4);
    expect(actors.every(actor => actor.grant.maximumCopyBytes === 2 ** 31)).toBe(true);
    expect(renderer.retainedRemaining.maximumCopyBytes).toBe(0);
  } finally {
    database.close();
  }
});

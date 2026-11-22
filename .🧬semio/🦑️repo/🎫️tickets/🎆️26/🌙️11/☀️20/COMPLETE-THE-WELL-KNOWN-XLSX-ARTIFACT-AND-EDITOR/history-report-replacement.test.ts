import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";

const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️replacement/🔣️.json", "utf8"));

test("replacement Source publishes and returns its original payload in separate turns", () => {
  using database = new Database(":memory:");
  const wire = database.query("SELECT json(json_extract(?1, '$')) AS value").get(fixture.wire.slice(2)) as { value: string };
  expect(JSON.parse(wire.value)).toEqual(fixture.value);

  const publication = database.query(`
    WITH turns(turn, destinationInitialized, donorOwnsPayload) AS (
      VALUES (0, 0, 1)
      UNION ALL
      SELECT turn + 1, 1, NOT destinationInitialized FROM turns WHERE donorOwnsPayload
    )
    SELECT turn, destinationInitialized, destinationInitialized AND NOT donorOwnsPayload AS borrowable
    FROM turns WHERE turn > 0 ORDER BY turn
  `);
  expect(publication.all().map((row) => {
    const observed = row as { turn: number; destinationInitialized: number; borrowable: number };
    return { turn: observed.turn, destinationInitialized: Boolean(observed.destinationInitialized), borrowable: Boolean(observed.borrowable) };
  })).toEqual(fixture.sourcePublication);

  const handback = database.query(`
    WITH turns(turn, donorOwnsPayload, receiverOwnsPayload, ownerHeader, rowOwnsPayload, binding) AS (
      VALUES (0, 1, 0, 1, 0, 1)
      UNION ALL
      SELECT turn + 1, 0, ownerHeader, donorOwnsPayload,
        rowOwnsPayload OR (receiverOwnsPayload AND NOT ownerHeader), NOT rowOwnsPayload
      FROM turns WHERE binding
    )
    SELECT min(turn) FILTER (WHERE rowOwnsPayload) AS payloadTurn, max(turn) AS terminalTurn
    FROM turns
  `).get() as { payloadTurn: number; terminalTurn: number };
  expect(handback.terminalTurn - handback.payloadTurn).toBe(fixture.closeTurnsAfterPayloadReturn);
});

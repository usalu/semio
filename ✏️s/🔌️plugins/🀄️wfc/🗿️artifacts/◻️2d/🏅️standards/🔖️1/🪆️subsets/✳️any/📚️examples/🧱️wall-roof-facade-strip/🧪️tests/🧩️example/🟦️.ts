// 🧪️ Example `wall-roof-facade-strip` — the laws every bundled problem keeps, asserted against the TypeScript twin.

import { describe, expect, it } from "vitest";

import { orderedIndex, wfc2dRelations } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { applyWfc2dMutation, wfc2dInverse, type Wfc2dMutation } from "../../../../🧬️schema/🧬️mutations/🟦️.ts";
import { document, ID } from "../../🟦️.ts";

describe("wall-roof-facade-strip", () => {
  it("is a non-empty problem with a stable id", () => {
    const board = document();
    expect(ID).toBe("wall-roof-facade-strip");
    expect(board.schema).toBe("s.wfc.wfc2d");
    expect(board.slots.length).toBeGreaterThan(0);
    expect(board.tiles.length).toBeGreaterThan(1);
    expect(board.rules.length).toBeGreaterThan(0);
  });

  it("keeps every collection in canonical ascending id order", () => {
    const board = document();
    for (const rows of [board.slots, board.edges, board.tiles, board.rules]) {
      const ids = rows.map((row) => row.id);
      expect(ids).toEqual([...ids].sort());
    }
  });

  it("names only slots and tiles it declares", () => {
    const board = document();
    const slots = new Set(board.slots.map((slot) => slot.id));
    const tiles = new Set(board.tiles.map((tile) => tile.id));
    const relations = new Set(wfc2dRelations(board));
    for (const edge of board.edges) {
      expect(slots.has(edge.fromSlotId)).toBe(true);
      expect(slots.has(edge.toSlotId)).toBe(true);
    }
    for (const rule of board.rules) {
      expect(tiles.has(rule.tileAId)).toBe(true);
      expect(tiles.has(rule.tileBId)).toBe(true);
      if (rule.relation !== undefined) expect(relations.has(rule.relation)).toBe(true);
    }
    for (const slot of board.slots) {
      if (slot.pinnedTileId !== undefined) expect(tiles.has(slot.pinnedTileId)).toBe(true);
    }
  });

  it("survives a pin and its inverse, positions included", () => {
    const board = document();
    const target = board.slots[0]!;
    const tile = board.tiles[0]!;
    const mutation: Wfc2dMutation = { PinSlot: { id: target.id, tileId: tile.id } };
    let next = applyWfc2dMutation(mutation, board);
    expect(next).not.toEqual(board);
    for (const step of wfc2dInverse(mutation, board)) next = applyWfc2dMutation(step, next);
    expect(next).toEqual(board);
  });

  it("inserts a new slot at its canonical position", () => {
    const board = document();
    expect(orderedIndex(board.slots, "aaa-first", (slot) => slot.id)).toBe(0);
    expect(orderedIndex(board.slots, "zzz-last", (slot) => slot.id)).toBe(board.slots.length);
  });
});

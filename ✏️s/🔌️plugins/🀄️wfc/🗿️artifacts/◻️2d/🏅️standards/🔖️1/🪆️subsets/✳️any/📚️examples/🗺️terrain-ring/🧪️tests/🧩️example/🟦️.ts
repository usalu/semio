// 🧪️ Example `terrain-ring` — the raster laws, asserted against the TypeScript twin.

import { describe, expect, it } from "vitest";

import { orderedIndex } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { applyWfc2dMutation, wfc2dInverse, type Wfc2dMutation } from "../../../../🧬️schema/🧬️mutations/🟦️.ts";
import { document, ID, TILE_PIXELS } from "../../🟦️.ts";

describe("terrain-ring", () => {
  it("is a non-empty raster problem with a stable id", () => {
    const board = document();
    expect(ID).toBe("terrain-ring");
    expect(board.schema).toBe("s.wfc.wfc2d");
    expect(board.slots).toHaveLength(6);
    expect(board.tiles).toHaveLength(4);
  });

  it("carries a palette-indexed bitmap on every tile", () => {
    for (const tile of document().tiles) {
      expect(tile.media).not.toBe("Empty");
      expect(typeof tile.media === "object" && "Bitmap" in tile.media).toBe(true);
      if (typeof tile.media === "object" && "Bitmap" in tile.media) {
        expect(tile.media.Bitmap.width).toBe(TILE_PIXELS);
        expect(tile.media.Bitmap.height).toBe(TILE_PIXELS);
        expect(tile.media.Bitmap.palette).toHaveLength(2);
        expect(atob(tile.media.Bitmap.pixels)).toHaveLength(TILE_PIXELS * TILE_PIXELS);
      }
    }
  });

  it("keeps every collection in canonical ascending id order", () => {
    const board = document();
    for (const rows of [board.slots, board.edges, board.tiles, board.rules]) {
      const ids = rows.map((row) => row.id);
      expect(ids).toEqual([...ids].sort());
    }
  });

  it("survives a raster media swap and its inverse", () => {
    const board = document();
    const target = board.tiles[0]!;
    const mutation: Wfc2dMutation = { ChangeTileMedia: { tileId: target.id, media: "Empty" } };
    let next = applyWfc2dMutation(mutation, board);
    expect(next).not.toEqual(board);
    for (const step of wfc2dInverse(mutation, board)) next = applyWfc2dMutation(step, next);
    expect(next).toEqual(board);
  });

  it("inserts a new tile at its canonical position", () => {
    const board = document();
    expect(orderedIndex(board.tiles, "aaa", (tile) => tile.id)).toBe(0);
    expect(orderedIndex(board.tiles, "zzz", (tile) => tile.id)).toBe(board.tiles.length);
  });
});

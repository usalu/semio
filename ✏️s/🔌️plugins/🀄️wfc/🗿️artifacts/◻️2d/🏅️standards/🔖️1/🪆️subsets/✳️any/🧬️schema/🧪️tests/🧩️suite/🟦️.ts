// 🧪️ Cross-language fixture oracle for `s.wfc.wfc2d`: every committed quintet under this subset's
// `🧫️fixtures/🧬️mutations/` replayed through the TypeScript twin, which must produce the SAME sparse
// diff, the SAME diagnostics and the SAME after-snapshot as Rust did when it committed them.

import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { applyWfc2dDiff, type Wfc2dDiff } from "../../🔺️diff/🟦️.ts";
import type { Wfc2dSnapshot } from "../../📸️snapshot/🟦️.ts";
import { applyWfc2dMutation, wfc2dDiff, wfc2dInverse, WFC_2D_MUTATION_KINDS, type Wfc2dMutation } from "../../🧬️mutations/🟦️.ts";

const here = fileURLToPath(new URL(".", import.meta.url));
const fixtures = join(here, "..", "..", "..", "🧫️fixtures", "🧬️mutations");

const read = <T,>(...parts: readonly string[]): T => JSON.parse(readFileSync(join(fixtures, ...parts), "utf8")) as T;
const directories = (...parts: readonly string[]): readonly string[] =>
  readdirSync(join(fixtures, ...parts), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort();

/** 🏷️ A fixture directory is `<emoji><kebab kind>`; the kind is what the schema declares. */
const kindOf = (directory: string): string => directory.replace(/^[^a-z0-9]+/u, "");

describe("wfc2d mutation vectors", () => {
  const kindDirectories = directories();

  it("ships one fixture directory per declared kind", () => {
    expect(kindDirectories.map(kindOf).sort()).toEqual([...WFC_2D_MUTATION_KINDS].sort());
  });

  for (const kindDirectory of kindDirectories) {
    const kind = kindOf(kindDirectory);
    for (const scenario of directories(kindDirectory)) {
      const base = [kindDirectory, scenario] as const;
      const before = read<Wfc2dSnapshot>(...base, "📸️snapshot", "⬅️before", "🔣️.json");
      const after = read<Wfc2dSnapshot>(...base, "📸️snapshot", "➡️after", "🔣️.json");
      const mutation = read<Wfc2dMutation>(...base, "🦠️mutation", "🔣️.json");
      const committed = read<Wfc2dDiff>(...base, "🔺️diff", "🔣️.json");
      const outcome = read<{ status: string; messages?: readonly { level: string; code: string }[] }>(...base, "🎯️outcome", "🔣️.json");

      describe(`${kind}/${scenario}`, () => {
        it("produces the committed diff", () => {
          expect(wfc2dDiff(mutation, before).diff).toEqual(committed);
        });

        it("raises the declared diagnostics", () => {
          expect(wfc2dDiff(mutation, before).messages).toEqual(outcome.messages ?? []);
        });

        it("carries before to the committed after", () => {
          expect(applyWfc2dMutation(mutation, before)).toEqual(after);
          expect(applyWfc2dDiff(committed, before)).toEqual(after);
        });

        it("inverts back to before, positions included", () => {
          let restored = applyWfc2dMutation(mutation, before);
          for (const step of wfc2dInverse(mutation, before)) restored = applyWfc2dMutation(step, restored);
          expect(restored).toEqual(before);
        });

        it("keeps every collection in canonical ascending id order", () => {
          for (const rows of [after.slots, after.edges, after.tiles, after.rules]) {
            const ids = rows.map((row) => row.id);
            expect(ids).toEqual([...ids].sort());
          }
        });
      });
    }
  }
});

/** ✅️ Probe: the validation module of the pets product alone (with the schema twin it imports), the only part of the barrel the pet-cast test calls. */
import { expect, it } from "vitest";
import { assembleMenagerie } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✅️validation/🟦️.ts";

it("loads the validation module", () => expect(assembleMenagerie).toBeTypeOf("function"));

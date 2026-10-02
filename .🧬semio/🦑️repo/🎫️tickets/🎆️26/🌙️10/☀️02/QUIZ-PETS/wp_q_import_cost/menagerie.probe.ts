/** 🎪️ Probe: the architecture menagerie module with its twenty-one static JSON imports (it imports the barrel as well). */
import { expect, it } from "vitest";
import { ARCHITECTURE_MENAGERIE } from "../../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts";

it("loads the menagerie", () => expect(ARCHITECTURE_MENAGERIE.species).toHaveLength(20));

/** 📦️ Probe: the whole package barrel of the pets product, as the pet-cast test imports it. */
import { assembleMenagerie } from "@semio-tech/pets";
import { expect, it } from "vitest";

it("loads the barrel", () => expect(assembleMenagerie).toBeTypeOf("function"));

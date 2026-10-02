/** 🫙️ Probe: nothing but Vitest itself, the floor every other probe is read against. */
import { expect, it } from "vitest";

it("loads nothing", () => expect(1).toBe(1));

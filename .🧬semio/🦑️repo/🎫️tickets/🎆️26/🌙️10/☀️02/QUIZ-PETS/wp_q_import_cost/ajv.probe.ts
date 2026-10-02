/** ⚖️ Probe: ajv alone. */
import Ajv from "ajv";
import { expect, it } from "vitest";

it("loads ajv", () => expect(new Ajv({ strict: false })).toBeDefined());

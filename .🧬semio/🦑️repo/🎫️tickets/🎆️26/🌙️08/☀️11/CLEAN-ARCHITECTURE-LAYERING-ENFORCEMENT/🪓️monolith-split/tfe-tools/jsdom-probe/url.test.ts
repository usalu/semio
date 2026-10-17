/** 🔎 Probe: what import.meta.url is under vitest jsdom. */
import { it } from "vitest";
it("prints", () => { throw new Error("[DEBUG] import.meta.url = " + import.meta.url.slice(0, 80) + " | URL=" + URL.name + " " + String(URL).slice(0, 40) + " | doc=" + typeof document + " | meta.dirname=" + String(import.meta.dirname).slice(0, 40)); });

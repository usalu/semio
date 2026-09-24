#!/usr/bin/env bun
/** GJ3 — language-agnostic oracle for inference-pool-pump-law.json (third-party: JSON.parse). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = join(here, "links/inference-pool-pump-law.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as {
  reactorTurn: { maxPumps: number; wallMs: number };
  inferenceCrossing: { minPumps: number; wallMs: null };
  missingClockMustNotSkipPump: boolean;
};
if (!(fixture.inferenceCrossing.minPumps > fixture.reactorTurn.maxPumps)) {
  throw new Error(`minPumps ${fixture.inferenceCrossing.minPumps} must exceed turn maxPumps ${fixture.reactorTurn.maxPumps}`);
}
if (fixture.reactorTurn.wallMs !== 2) throw new Error("reactorTurn.wallMs must be 2");
if (fixture.inferenceCrossing.wallMs !== null) throw new Error("inferenceCrossing.wallMs must be null");
if (fixture.missingClockMustNotSkipPump !== true) throw new Error("missingClockMustNotSkipPump must be true");
console.log("PASS gj3-pump-law", fixture.id ?? "inference-pool-pump-until-settled");

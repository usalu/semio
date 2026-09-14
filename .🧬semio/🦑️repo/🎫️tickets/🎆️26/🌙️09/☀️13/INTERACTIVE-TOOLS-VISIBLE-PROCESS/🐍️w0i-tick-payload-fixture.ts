/** 📨️ W0-I: adds `ToolRunTick.payload` (field 9, bytes) to the tool-run schema of record and a `payload tick` case to
 * `🎞️ticks.json` whose hex comes from the TS encoder; the Rust fixture law then checks byte equality independently.
 * Idempotent. Run: `bun T/🐍️w0i-tick-payload-fixture.ts` from the repo root. */
import { readFileSync, writeFileSync } from "node:fs";
import { encodeToolRunTick, toolRunBytesToHex, toolRunTickFromJson } from "../../../../../../../🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts";

const root = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run";
const schemaPath = `${root}/🧬️schema/🔣️.json`;
const ticksPath = `${root}/🧫️fixtures/🎞️ticks.json`;

const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
schema.$defs.ToolRunTick.properties.payload = { description: "Opaque plugin bytes the run's windows read back through the run view; the newest tick carrying one wins.", $ref: "#/$defs/Hex" };
writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);

const fixture = JSON.parse(readFileSync(ticksPath, "utf8"));
const base = fixture.ticks[0].tick;
const tick = { identity: base.identity, sequence: 13, steps: [], trace: [], appendOps: [], appendEntities: [], payload: "7b227469657273223a5b312c322c335d7d" };
const hex = toolRunBytesToHex(encodeToolRunTick(toolRunTickFromJson(tick)));
fixture.ticks = fixture.ticks.filter((row: { readonly name: string }) => row.name !== "payload tick").concat([{ name: "payload tick", tick, hex }]);
writeFileSync(ticksPath, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`[w0i] payload tick hex ${hex}`);

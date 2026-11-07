import { strict as assert } from "node:assert";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { ticketOutput } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🎫️output/🟦️.ts";

const cases = JSON.parse(readFileSync("🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎭️e2e-output/🔣️.json", "utf8")).physicalContainment;
const generated = resolve(process.env.SEMIO_TICKET_DIR!, "🗑️generated");
const scratch = mkdtempSync(join(generated, "containment-"));
try {
  for (const row of cases) {
    const root = join(scratch, row.link);
    const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/a");
    const outside = join(root, "outside");
    mkdirSync(ticket, { recursive: true });
    mkdirSync(outside, { recursive: true });
    writeFileSync(join(ticket, "🎫️ticket.json"), "{}");
    if (row.link !== "none") {
      const generated = join(ticket, "🗑️generated");
      if (row.link === "nx") mkdirSync(generated);
      symlinkSync(outside, row.link === "nx" ? join(generated, "nx") : generated, process.platform === "win32" ? "junction" : "dir");
    }
    let accepted = true;
    try { ticketOutput(root, [], { SEMIO_TICKET_DIR: ticket }); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, row.name);
    assert.deepEqual(readdirSync(outside), [], row.name);
  }
  console.log(`${cases.length} physical containment cases passed`);
} finally { rmSync(scratch, { recursive: true, force: true }); }

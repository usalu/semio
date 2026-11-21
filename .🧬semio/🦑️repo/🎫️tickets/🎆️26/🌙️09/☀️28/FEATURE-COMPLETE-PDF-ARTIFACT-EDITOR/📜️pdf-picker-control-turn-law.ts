/** 📥 Preserves picker cancellation while a control waits for a funded turn. */
import { mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const generated = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const tests = root + "🧪️tests/🔬️plugin-runtime-runtime-close-budget/";
const law = tests + "🟦️.ts";
const fixture = tests + "🧫️fixtures/🎞️media-control/🔣️.json";
const ladder = root + "🪜️maintenance-ladder/🦀️.rs";
const before = readFileSync(law, "utf8");
const oldGate = "self.ingest.live_document_load().is_some()";
const currentGate = "self.ingest.document_blocks_pending_effects()";
if (before.split(oldGate).length !== 3 || before.includes("unpaid original media control")) throw Error("Mounted source law changed");
const body = before.replaceAll(oldGate, currentGate) + `
test("unpaid original media control keeps the mounted format worker parked", async () => {
  const fixture = await Bun.file(new URL("🧫️fixtures/🎞️media-control/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    const format = db.query("SELECT NOT (? OR ?) AS runnable");
    for (const row of fixture) expect(format.get(Number(row.pagePending), Number(row.controlPending)))
      .toEqual({ runnable: Number(row.formatRunnable) });
  } finally { db.close(); }
  const turn = readFileSync(new URL("../../⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs", import.meta.url), "utf8");
  const ingress = turn.indexOf("::drive_raw_media(runtime, events");
  const format = turn.indexOf("::drive_raw_media_format(runtime");
  expect(ingress).toBeGreaterThanOrEqual(0);
  expect(format).toBeGreaterThan(ingress);
  expect(turn.slice(ingress, format)).toContain("matches!(events.as_ref(), Some(Event::OriginalMediaControl(_)))");
  expect(turn.slice(ingress, format)).toContain("command_page.is_none() && !pending_media_control");
});
`;
const oldLadder = readFileSync(ladder, "utf8");
if (oldLadder.split(oldGate).length + oldLadder.split(currentGate).length !== 5) throw Error("Document maintenance ladder changed");
const rows = [
    { pagePending: false, controlPending: false, formatRunnable: true },
    { pagePending: true, controlPending: false, formatRunnable: false },
    { pagePending: false, controlPending: true, formatRunnable: false },
    { pagePending: true, controlPending: true, formatRunnable: false },
];
mkdirSync(dirname(fixture), { recursive: true });
writeFileSync(fixture, JSON.stringify(rows, null, 2) + "\n", { flag: "wx" });
for (const [index, [file, preimage, postimage]] of [[law, before, body], [ladder, oldLadder, oldLadder.replaceAll(oldGate, currentGate)]].entries()) {
    if (postimage === preimage) continue;
    const temporary = generated + "pdf-picker-control-law-" + index;
    writeFileSync(temporary, postimage);
    if (readFileSync(file, "utf8") !== preimage) throw Error("Shared picker law changed: " + file);
    renameSync(temporary, file);
}

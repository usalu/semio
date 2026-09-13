/** ⛔️ The RED half of the new law, kept runnable.
 *
 * Replays `🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json` against the ledger rule that shipped
 * before this lane — "a checkout that outlives 240 blocked apply opportunities is stale" — and reports
 * every row that rule now breaks. It must report the live-owner row; if it ever reports nothing, the
 * fixture has stopped pinning the defect.
 *
 * Usage: cd <ticket> && bun 🐍️checkout-credits-counterproof.mjs
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const fixturePath = join(import.meta.dir, "../../../../../../..", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));

/** 🕰️ The retired rule, verbatim: an age in blocked opportunities IS the verdict. */
const CREDITS = 240;

const replay = (row) => {
  const ready = [];
  let inFlight = 0;
  let site = null;
  let age = 0;
  let stale = false;
  let notified = false;
  const notices = [];
  let available = true;
  const breaks = [];
  const room = (key, limit) => {
    if (ready.length + inFlight < limit) return true;
    if (key === null) return false;
    const index = ready.findIndex((queued) => queued.key === key);
    if (index < 0) return false;
    ready.splice(index, 1);
    return true;
  };
  for (const [index, step] of row.steps.entries()) {
    const repeat = typeof step.repeat === "number" ? step.repeat : 1;
    for (let iteration = 0; iteration < repeat; iteration += 1) {
      if (step.op === "enqueue") {
        if (room(step.key ?? null, row.capacity - 1)) ready.push({ key: step.key ?? null, revision: step.revision, requiresInteraction: step.requiresInteraction });
      } else if (step.op === "reserveInteraction") {
        if (ready.length + inFlight !== row.capacity) inFlight += 1;
      } else if (step.op === "finish") {
        inFlight -= 1;
        ready.unshift({ key: step.key ?? null, revision: step.revision, requiresInteraction: step.requiresInteraction });
      } else if (step.op === "checkOut") {
        if (site === null) { site = step.site; age = 0; stale = false; notified = false; available = false; }
      } else if (step.op === "checkIn") {
        site = null; age = 0; stale = false; notified = false; available = true;
      } else if (step.op === "apply") {
        const headRequires = ready.length > 0 && ready[0].requiresInteraction;
        let admission = "admitted";
        if (!(available || !headRequires)) {
          age += 1;
          admission = age <= CREDITS ? "deferred" : "stale";
          if (admission === "stale") stale = true;
        }
        const at = available ? (ready.length > 0 ? 0 : null) : ready.findIndex((completion) => !completion.requiresInteraction);
        const applied = at === null || at < 0 ? null : ready.splice(at, 1)[0].revision;
        if (stale && !notified) { notified = true; notices.push({ site: site ?? "unknown", opportunities: age }); }
        const last = iteration + 1 === repeat;
        if (last && admission !== step.admission) breaks.push(`step ${index} (${step.op}): the retired rule says "${admission}", the law says "${step.admission}"`);
        if (last && (applied ?? null) !== (step.appliedRevision ?? null)) breaks.push(`step ${index}: applied ${applied} vs ${step.appliedRevision}`);
      }
    }
  }
  const expected = JSON.stringify(row.expect.abandonedNotices);
  if (JSON.stringify(notices) !== expected) breaks.push(`notices ${JSON.stringify(notices)} vs ${expected}`);
  return breaks;
};

let broken = 0;
for (const row of fixture.rows) {
  const breaks = replay(row);
  if (breaks.length === 0) continue;
  broken += 1;
  console.log(`RED ${row.id}`);
  for (const line of breaks.slice(0, 3)) console.log(`    ${line}`);
}
console.log(`rows=${fixture.rows.length} broken-by-the-retired-credit-rule=${broken}`);
if (broken === 0) {
  console.log("FAIL: the fixture no longer pins the defect — the retired credit rule passes it");
  process.exit(1);
}

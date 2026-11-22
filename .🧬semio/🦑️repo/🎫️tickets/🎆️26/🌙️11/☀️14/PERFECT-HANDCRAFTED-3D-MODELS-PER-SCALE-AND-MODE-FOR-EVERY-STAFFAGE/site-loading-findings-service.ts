/** 🧵️ Smoke run of the findings service natively (bun, and node with `--experimental-transform-types`): progress, the kept result, cancellation of a running check and the time the asking thread stays free.
 * Usage: `bun site-loading-findings-service.ts <asset.json> <other asset.json>` */
import { readFileSync } from "node:fs";
import { StaffageFindings } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🌐️vite/🩺️findings/🟦️.ts";
import { staffageRevision } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🌐️vite/🟦️.ts";

const [first, second] = process.argv.slice(2).map((file) => readFileSync(file!, "utf8"));
const service = new StaffageFindings({ threads: 1 });
let ticks = 0;
const beat = setInterval(() => (ticks += 1), 10);
const start = performance.now();
const progress: string[] = [];
const ticket = service.ask(staffageRevision(first!), first!, (done, total, label) => progress.push(`${done}/${total} ${label.en}`));
const queued = service.ask(staffageRevision(second!), second!);
queued.cancel();
const issues = await ticket.findings;
const elapsed = performance.now() - start;
const again = performance.now();
await service.ask(staffageRevision(first!), first!).findings;
const running = service.ask(staffageRevision(second!), second!);
await new Promise((resolve) => setTimeout(resolve, 150));
running.cancel();
const cancelled = await running.findings.then(() => "finished", (error: Error) => error.name);
const queuedOutcome = await queued.findings.then(() => "finished", (error: Error) => error.name);
clearInterval(beat);
console.log(JSON.stringify({ runtime: "bun" in process.versions ? "bun" : "node", findings: issues.length, checkMs: Math.round(elapsed), timerTicksWhileChecking: ticks, progress, keptAnswerMs: Math.round(performance.now() - again - 150), cancelledWhileQueued: queuedOutcome, cancelledWhileRunning: cancelled, pendingAfter: service.pending.length }));
await service.close();

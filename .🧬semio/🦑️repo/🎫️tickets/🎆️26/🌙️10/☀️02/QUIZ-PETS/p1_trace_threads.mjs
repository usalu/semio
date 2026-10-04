/** 🧵️ Ticket tool of work package P1: per thread of a kept trace, the busy time of its top-level events between the sweep's marks and its heaviest event names — to see where a frame goes besides the renderer's main thread.
 *
 * Usage: node ".../p1_trace_threads.mjs" <trace.json>
 */
import { readFileSync } from "node:fs";

const trace = JSON.parse(readFileSync(process.argv[2], "utf8"));
const events = trace.traceEvents ?? trace;
const names = new Map();
const processes = new Map();
for (const event of events) {
  if (event.ph === "M" && event.name === "thread_name") names.set(`${event.pid}:${event.tid}`, event.args.name);
  if (event.ph === "M" && event.name === "process_name") processes.set(event.pid, event.args.name);
}
const start = events.find((event) => event.name === "p1-sweep-start");
const end = events.find((event) => event.name === "p1-sweep-end");
const from = start.ts;
const to = end.ts;
const threads = new Map();
for (const event of events) {
  if (event.ph !== "X" || event.ts > to || event.ts + (event.dur ?? 0) < from) continue;
  const key = `${event.pid}:${event.tid}`;
  const list = threads.get(key) ?? [];
  list.push(event);
  threads.set(key, list);
}
const rows = [];
for (const [key, list] of threads) {
  list.sort((a, b) => a.ts - b.ts || b.dur - a.dur);
  let busy = 0;
  let reach = -Infinity;
  const totals = {};
  for (const event of list) {
    const begin = Math.max(event.ts, from);
    const finish = Math.min(event.ts + event.dur, to);
    totals[event.name] = (totals[event.name] ?? 0) + Math.max(0, finish - begin);
    if (begin >= reach) {
      busy += Math.max(0, finish - begin);
      reach = finish;
    } else if (finish > reach) {
      busy += finish - reach;
      reach = finish;
    }
  }
  const top = Object.entries(totals)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 6)
    .map(([name, us]) => `${name} ${Math.round(us / 1000)}`)
    .join(", ");
  rows.push({ thread: `${processes.get(Number(key.split(":")[0])) ?? "?"} ${key} ${names.get(key) ?? "?"}`, busyMs: Math.round(busy / 1000), top });
}
rows.sort((a, b) => b.busyMs - a.busyMs);
process.stdout.write(`window ${Math.round((to - from) / 1000)} ms\n`);
for (const row of rows.slice(0, 25)) process.stdout.write(`${row.busyMs} ms  ${row.thread}  [${row.top}]\n`);
const instants = {};
for (const event of events) if ((event.ph === "I" || event.ph === "i" || event.ph === "n") && event.ts >= from && event.ts <= to) instants[`${event.name} ${names.get(`${event.pid}:${event.tid}`) ?? "?"}`] = (instants[`${event.name} ${names.get(`${event.pid}:${event.tid}`) ?? "?"}`] ?? 0) + 1;
process.stdout.write(`instants: ${JSON.stringify(Object.entries(instants).sort((a, b) => b[1] - a[1]).slice(0, 20))}\n`);

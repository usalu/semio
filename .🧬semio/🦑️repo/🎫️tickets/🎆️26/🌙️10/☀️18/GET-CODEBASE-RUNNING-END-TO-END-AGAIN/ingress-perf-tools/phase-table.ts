const text = await Bun.stdin.text();
const rows: Array<{ phase: string; point: string; elapsed: number }> = [];
for (const line of text.split("\n")) {
  const at = line.indexOf("Original Cargo owned timing ");
  if (at < 0) continue;
  try { const value = JSON.parse(line.slice(at + 28)); if (value.kind === "phase") rows.push({ phase: value.phase, point: value.point, elapsed: value.elapsedMilliseconds ?? value.elapsed }); } catch {}
}
const begin = new Map<string, number>();
const table: string[] = [];
let last = 0;
for (const row of rows) {
  if (row.point === "begin") begin.set(row.phase, row.elapsed);
  else if (begin.has(row.phase)) { table.push(`${row.phase.padEnd(10)} ${String(Math.round(row.elapsed - begin.get(row.phase)!)).padStart(8)} ms`); begin.delete(row.phase); }
  last = Math.max(last, row.elapsed);
}
console.log(table.join("\n") + `\ntotal      ${String(Math.round(last)).padStart(8)} ms`);

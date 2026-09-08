import { scanTestLayout } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { writeFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
if (!root) throw new Error('SEMIO_LAYOUT_REPO_ROOT is required');
const ticket = dirname(dirname(import.meta.dir)), lane = join(ticket, '🗑️generated', 'root-layout');
mkdirSync(lane, { recursive: true });
const controller = new AbortController();
process.on('SIGINT', () => controller.abort());
process.on('SIGTERM', () => controller.abort());
let completed = 0, sourceCount = 0, phase = '';
const findings = await scanTestLayout(root, { signal: controller.signal, concurrency: 4, progress: progress => {
  if (progress.phase === "inspect") sourceCount = progress.total;
  if (phase !== progress.phase || progress.completed - completed >= 1000 || progress.completed === progress.total) {
    phase = progress.phase; completed = progress.completed;
    console.log(`[DEBUG] layout ${phase}: ${completed}/${progress.total}`);
  }
} });
const counts = Object.fromEntries([...new Set(findings.map(row => row.code))].map(code => [code, findings.filter(row => row.code === code).length]));
writeFileSync(join(lane, 'layout-current.json'), JSON.stringify(findings, null, 2) + '\n');
writeFileSync(join(ticket, '📓️test-layout-current-snapshot-2026-09-08.md'), '# Current Layout Snapshot\n\nScan completed at ' + new Date().toISOString() + '. This is a live snapshot while other executors may still be moving sources.\n\nAuthored sources inspected: ' + sourceCount + '.\n\n' + JSON.stringify(counts) + '\n\n```json\n' + JSON.stringify(findings, null, 2) + '\n```\n');
console.log(JSON.stringify({ sourceCount, total: findings.length, counts }));
process.exitCode = findings.length ? 1 : 0;

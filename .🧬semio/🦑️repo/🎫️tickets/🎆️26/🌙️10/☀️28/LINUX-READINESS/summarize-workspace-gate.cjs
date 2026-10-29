const fs = require("node:fs");
const path = require("node:path");
const ticket = __dirname;
const input = path.join(ticket, "🗑️generated", "workspace-isolated-contract.log");
const text = fs.readFileSync(input, "utf8").replace(/\u001b\[[0-?]*[ -/]*[@-~]/gu, "");
const files = [];
let active;
for (const line of text.split("\n")) {
  if (line.includes("🧪️tests") && /\.ts:$/u.test(line.trim()) && !/^\s+at /u.test(line)) {
    active = { file: line.trim().replace(/:$/u, ""), passes: [], failures: [], skips: [] };
    files.push(active);
  }
  if (!active) continue;
  if (/^\(pass\)/u.test(line)) active.passes.push(line);
  if (/^\(fail\)/u.test(line)) active.failures.push(line);
  if (/^\(skip\)/u.test(line)) active.skips.push(line);
}
const result = { files, failedFiles: files.filter(file => file.failures.length), tail: text.slice(-3000) };
fs.writeFileSync(path.join(ticket, "🗑️generated", "workspace-isolated-summary.json"), JSON.stringify(result, null, 2) + "\n");
process.stdout.write(JSON.stringify({ fileCount: files.length, failed: result.failedFiles.map(file => ({ file: file.file, failures: file.failures })), tail: result.tail }) + "\n");

const fs = require("node:fs");
const path = require("node:path");
const child = require("node:child_process");
const log = path.join(__dirname, "🗑️generated", "native-process-monitor.jsonl");
let last;
function query(command) {
  const result = child.spawnSync(command, { encoding: "utf8", shell: true, maxBuffer: 16000000, timeout: 30000 });
  if (result.status !== 0) throw new Error(result.stderr || result.error?.message || "Query failed");
  return JSON.parse(result.stdout.slice(result.stdout.indexOf("{")));
}
function inspect() {
  try {
    const leases = query("repo leases status --lang=en --json");
    const holders = leases.holders.filter(row => row.pid === 29656);
    const standing = holders.find(row => row.standing)?.standing;
    let physical;
    if (!holders.length) {
      const doctor = query("repo doctor --only=processes,leases --facts --lang=en --json");
      physical = doctor.processes?.filter(row => row.pid === 29656 || row.pid === 34296).map(row => ({ pid: row.pid, parent: row.parent }));
    }
    const event = { atMs: leases.atMs, standing, resources: holders.map(row => row.resource), physical, cycles: leases.cycles, waitFor: leases.waitFor };
    fs.appendFileSync(log, JSON.stringify(event) + "\n");
    const key = JSON.stringify({ phase: standing?.phase, resources: event.resources, physical, cycles: event.cycles, waitFor: event.waitFor });
    if (key !== last) { console.log(JSON.stringify(event)); last = key; }
    if (!holders.length && physical && !physical.length) { console.log("Native process absent; no terminal success inferred"); process.exit(0); }
  } catch (error) { console.log("[DEBUG] " + error.message); }
}
inspect();
setInterval(inspect, 60000);

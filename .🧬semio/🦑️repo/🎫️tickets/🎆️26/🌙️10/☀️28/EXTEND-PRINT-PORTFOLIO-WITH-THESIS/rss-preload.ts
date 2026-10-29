import { afterAll } from "bun:test";
import { readFileSync } from "node:fs";

let reported = false;
const report = () => {
  if (reported) return;
  reported = true;
  const usage = process.resourceUsage();
  let parent: Record<string, string> | null = null;
  if (process.platform === "linux") {
    try {
      parent = Object.fromEntries(readFileSync("/proc/" + process.ppid + "/status", "utf8").split("\n").filter(line => /^(Name|VmHWM|VmRSS):/.test(line)).map(line => { const at = line.indexOf(":"); return [line.slice(0, at), line.slice(at + 1).trim()]; }));
    } catch {}
  }
  process.stderr.write("[DEBUG] issuer-rss " + JSON.stringify({ pid: process.pid, parentPid: process.ppid, platform: process.platform, runtime: process.versions.bun ? "bun " + process.versions.bun : process.version, maxRSSKiB: usage.maxRSS, maxRSSMiB: usage.maxRSS / 1024, rssMiBAtExit: process.memoryUsage().rss / 1048576, parent }) + "\n");
};
afterAll(report);
process.once("exit", report);

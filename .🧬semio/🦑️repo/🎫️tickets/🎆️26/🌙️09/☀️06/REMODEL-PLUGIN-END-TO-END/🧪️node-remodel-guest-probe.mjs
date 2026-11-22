import { writeFile } from "node:fs/promises";

const staging = new URL("./🗑️generated/browser-materialization/", import.meta.url);
const deadline = setTimeout(() => { throw Error("Node guest probe timed out"); }, 45_000);
try {
  const { run } = await import(new URL("guest-probe.js", staging));
  const result = await run();
  await writeFile(new URL("node-isolated-guest.json", staging), JSON.stringify({ node: process.version, ...result }, null, 2) + "\n");
  if (!result.ok) process.exitCode = 1;
} finally {
  clearTimeout(deadline);
}

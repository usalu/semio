/** 🧪️ Checks the prescribed auxiliary descriptor using each runtime's subprocess interface. */
import { spawn } from "node:child_process";

const child = spawn("/bin/bash", ["-c", "IFS= read -r line <&3 && [[ $line == probe ]]"], {
  detached: true,
  stdio: ["ignore", "inherit", "inherit", "pipe"],
});
const channel = child.stdio[3];
channel.on("error", error => console.error("[DEBUG] pipe error", error));
channel.end("probe\n", "utf8", error => {
  console.error("[DEBUG] write completed", error ?? null);
  if (error) process.exitCode = 1;
});
const deadline = setTimeout(() => child.kill(), 2000);
child.on("close", (status, signal) => {
  clearTimeout(deadline);
  console.error("[DEBUG] reader settled", status, signal);
  if (status !== 0) process.exitCode = 1;
});

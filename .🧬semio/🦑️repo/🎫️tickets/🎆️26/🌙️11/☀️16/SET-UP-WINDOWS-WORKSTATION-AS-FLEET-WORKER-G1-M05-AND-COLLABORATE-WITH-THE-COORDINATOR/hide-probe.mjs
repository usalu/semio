/** 🧪 Probe: does `windowsHide` keep a console child of a console-less (detached) Bun process from opening a terminal window? Roles: (none) starts a detached middle; `middle <variant>` starts one leaf; `leaf` sleeps. Variants: plain, hide, bun-plain, bun-hide. */
import { spawn } from "node:child_process";
import { appendFileSync } from "node:fs";
const [role, variant, log] = process.argv.slice(2), self = process.argv[1];
if (role === "leaf") { setTimeout(() => {}, 8000); }
else if (role === "middle") {
  const args = [self, "leaf"];
  let pid;
  if (variant === "plain") pid = spawn(process.execPath, args, { stdio: ["pipe", "pipe", "inherit"] }).pid;
  if (variant === "hide") pid = spawn(process.execPath, args, { stdio: ["pipe", "pipe", "inherit"], windowsHide: true }).pid;
  if (variant === "ipc-hide") pid = spawn(process.execPath, args, { stdio: ["ignore", "inherit", "inherit", "ipc"], serialization: "json", windowsHide: true }).pid;
  if (variant === "bun-plain") pid = Bun.spawn([process.execPath, ...args], { stdin: "ignore", stdout: "pipe", stderr: "inherit" }).pid;
  if (variant === "bun-hide") pid = Bun.spawn([process.execPath, ...args], { stdin: "ignore", stdout: "pipe", stderr: "inherit", windowsHide: true }).pid;
  appendFileSync(log, `${variant} leaf ${pid}\n`);
  setTimeout(() => process.exit(0), 9000);
} else {
  spawn(process.execPath, [self, "middle", variant, log], { detached: true, windowsHide: true, stdio: "ignore" }).unref();
}

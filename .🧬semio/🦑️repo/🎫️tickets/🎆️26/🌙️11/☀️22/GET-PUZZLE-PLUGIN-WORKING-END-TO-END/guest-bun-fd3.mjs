/** 🧪️ Compares Bun's wrappers around the prescribed inherited auxiliary descriptor. */
import { spawn } from "node:child_process";
import { closeSync, fstatSync, readFileSync, write, writeSync } from "node:fs";
import { createHash } from "node:crypto";
import { Socket } from "node:net";

const mode = process.argv[2];
const bytes = Buffer.from("provisional \u0301🧩".repeat(Number(process.argv[3] ?? 1)));
const expected = createHash("sha256").update(bytes).digest("hex");
const args = ["-c", '[[ $(sha256sum <&3) == "$1  -" ]]', "reader", expected];
const nodeChild = mode.startsWith("node-") ? spawn("/bin/bash", args, { stdio: ["ignore", "inherit", "inherit", "pipe"], detached: true }) : null;
const bunChild = nodeChild === null ? Bun.spawn(["/bin/bash", ...args], { stdio: ["ignore", "inherit", "inherit", mode.includes("socket") ? "socket-fd" : "pipe"], detached: true }) : null;
const terminal = nodeChild ? new Promise(resolve => nodeChild.on("close", (code, signal) => resolve([code, signal]))) : bunChild.exited;
const transport = nodeChild?.stdio[3];
const fd = nodeChild ? transport?._handle?.fd : bunChild.stdio[3];
console.error("[DEBUG] fd / transport", fd, transport?.constructor?.name, typeof fd === "number" ? fstatSync(fd).isSocket() : undefined, readFileSync(`/proc/self/fdinfo/${fd}`, "utf8"));
const socket = /-(?:raw|file|sink|fs)$/.test(mode) ? null : transport ?? new Socket({ fd, readable: false, writable: true });
const timer = setTimeout(() => {
  console.error("[DEBUG] cancelled reader");
  socket?.destroy();
  (nodeChild ?? bunChild).kill();
}, 3000);
try {
  if (socket) {
    socket.on("error", error => console.error("[DEBUG] write error event", error));
    if (mode.endsWith("-write")) {
      await new Promise((complete, fail) => socket.write(bytes, error => error ? fail(error) : complete()));
      socket.destroy();
    } else {
      await new Promise((complete, fail) => socket.end(bytes, error => error ? fail(error) : complete()));
      socket.destroy();
    }
  } else if (mode.endsWith("-file")) {
    const length = await Bun.write(Bun.file(fd), bytes);
    console.error("[DEBUG] bytes written", length);
    closeSync(fd);
  } else if (mode.endsWith("-sink")) {
    const sink = Bun.file(fd).writer();
    sink.write(bytes);
    console.error("[DEBUG] sink end", await sink.end());
  } else if (mode.endsWith("-fs")) {
    for (let offset = 0; offset < bytes.length; ) {
      try {
        offset += await new Promise((resolve, reject) => write(fd, bytes, offset, bytes.length - offset, null, (error, count) => error ? reject(error) : resolve(count)));
      } catch (error) {
        if (error.code !== "EAGAIN") throw error;
        await new Promise(resolve => setTimeout(resolve, 1));
      }
    }
    closeSync(fd);
  } else {
    for (let at = 0; at < bytes.length;) at += writeSync(fd, bytes.subarray(at));
    closeSync(fd);
  }
} catch (error) {
  console.error("[DEBUG] write callback refused", error);
  socket?.destroy();
  (nodeChild ?? bunChild).kill();
}
const status = await terminal;
clearTimeout(timer);
console.error("[DEBUG] reader status", status);
if ((Array.isArray(status) ? status[0] : status) !== 0) process.exitCode = 1;

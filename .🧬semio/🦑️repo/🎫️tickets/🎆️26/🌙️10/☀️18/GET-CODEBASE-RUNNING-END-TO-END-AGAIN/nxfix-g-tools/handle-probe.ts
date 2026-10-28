/** 🔬 Diagnostic preload (`BUN_OPTIONS=--preload <this>`, env `HANDLE_PROBE_DIR`): records the creation stack of every ref'd timer, child process, socket/server and fs watcher of a process until it is cleared, unref'd, closed or exited, and writes the live set to `<dir>/handles-<pid>.json` every 10 s (unref'd interval) and on SIGUSR2. Bun 1.4 has no getActiveResourcesInfo, so the creation sites are the only evidence of what keeps the loop alive. Not part of the product; deleted with the ticket. */
import cp from "node:child_process";
import fs from "node:fs";
import net from "node:net";
import path from "node:path";

const dir = process.env.HANDLE_PROBE_DIR;
if (dir) {
  fs.mkdirSync(dir, { recursive: true });
  type Row = { kind: string; stack: string; at: number; ref: boolean };
  const live = new Map<number, Row>(), owners = new WeakMap<object, number>();
  let counter = 0;
  const stack = (): string => (new Error().stack ?? "").split("\n").slice(3, 11).map(line => line.trim().replace(/^at /u, "")).join(" | ");
  const track = (owner: object, kind: string): number => { const id = ++counter; live.set(id, { kind, stack: stack(), at: Date.now(), ref: true }); owners.set(owner, id); return id; };
  const drop = (owner: object): void => { const id = owners.get(owner); if (id !== undefined) live.delete(id); };
  const unrefOf = (owner: any): void => { const original = owner.unref?.bind(owner); if (original) owner.unref = (...args: unknown[]) => { const id = owners.get(owner); if (id !== undefined) live.get(id)!.ref = false; return original(...args); }; };
  const timers = (name: "setTimeout" | "setInterval", clear: "clearTimeout" | "clearInterval"): void => {
    const original = (globalThis as any)[name], cleared = (globalThis as any)[clear];
    (globalThis as any)[name] = (callback: (...a: unknown[]) => void, ms?: number, ...rest: unknown[]) => {
      let handle: any;
      const wrapped = name === "setTimeout" ? (...a: unknown[]) => { drop(handle); return callback(...a); } : callback;
      handle = original(wrapped, ms, ...rest);
      if (handle && typeof handle === "object") { track(handle, `${name} ${ms}`); unrefOf(handle); }
      return handle;
    };
    (globalThis as any)[clear] = (handle: any) => { if (handle && typeof handle === "object") drop(handle); return cleared(handle); };
  };
  timers("setTimeout", "clearTimeout"); timers("setInterval", "clearInterval");
  const spawn = cp.spawn.bind(cp);
  (cp as any).spawn = (command: string, ...rest: any[]) => {
    const child = spawn(command, ...rest); track(child, `spawn ${command} ${(Array.isArray(rest[0]) ? rest[0] : []).slice(0, 3).join(" ").slice(0, 100)}`); unrefOf(child);
    child.once("close", () => drop(child)); child.once("error", () => drop(child)); return child;
  };
  const connect = net.connect.bind(net);
  (net as any).connect = (...args: any[]) => { const socket = connect(...args); track(socket, "net.connect " + JSON.stringify(args[0]).slice(0, 100)); unrefOf(socket); socket.once("close", () => drop(socket)); return socket; };
  const createServer = net.createServer.bind(net);
  (net as any).createServer = (...args: any[]) => { const server = createServer(...args); track(server, "net.createServer"); unrefOf(server); server.once("close", () => drop(server)); return server; };
  const watch = fs.watch.bind(fs);
  (fs as any).watch = (...args: any[]) => { const watcher = (watch as any)(...args); track(watcher, "fs.watch " + String(args[0]).slice(-80)); unrefOf(watcher); watcher.once("close", () => drop(watcher)); return watcher; };
  const dump = (): void => {
    const now = Date.now(), rows = [...live.values()].filter(row => row.ref).map(row => ({ kind: row.kind, ageMs: now - row.at, stack: row.stack }));
    try { fs.writeFileSync(path.join(dir, `handles-${process.pid}.json`), JSON.stringify({ pid: process.pid, argv: process.argv.slice(0, 6).map(value => value.slice(-80)), exitCode: process.exitCode ?? null, at: now, live: rows }, null, 1)); } catch {}
  };
  const timer = setInterval(dump, 10000); (timer as any).unref();
  process.on("SIGUSR2", dump);
  process.on("exit", dump);
}

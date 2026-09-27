/** 🔌️ C10 link proxy: a TCP relay `listenPort → 127.0.0.1:upstreamPort` whose link a test can cut, to prove the shell's
 * short-connection-shortage behaviour on a REAL transport loss (Playwright's `setOffline` leaves an open WebSocket
 * silently black-holed instead). Control (HTTP on `controlPort`):
 *   POST /cut?mode=close&ms=15000      every relayed connection is destroyed and new ones are refused for `ms`
 *   POST /cut?mode=stall&ms=15000      connections stay open but every byte waits for `ms` (a TCP stall: delayed, never lost)
 *   GET  /status                       live connection count and the current mode
 * Usage: bun c10-link-proxy.ts <listenPort> <upstreamPort> <controlPort> */
import { connect, createServer, type Socket } from "node:net";

const [listenPort, upstreamPort, controlPort] = process.argv.slice(2).map(Number) as [number, number, number];
type Pair = { readonly client: Socket; readonly upstream: Socket };
const pairs = new Set<Pair>();
let mode: "open" | "close" | "stall" = "open";
let until = 0;

const log = (line: string) => console.log(`${new Date().toISOString()} ${line}`);

createServer((client) => {
  if (mode === "close" && Date.now() < until) {
    client.destroy();
    return;
  }
  const upstream = connect(upstreamPort, "127.0.0.1");
  const pair = { client, upstream };
  pairs.add(pair);
  client.pipe(upstream);
  upstream.pipe(client);
  if (mode === "stall" && Date.now() < until) {
    client.pause();
    upstream.pause();
  }
  const drop = () => {
    pairs.delete(pair);
    client.destroy();
    upstream.destroy();
  };
  client.on("error", drop).on("close", drop);
  upstream.on("error", drop).on("close", drop);
}).listen(listenPort, "127.0.0.1", () => log(`LISTEN ${listenPort} -> ${upstreamPort} control ${controlPort} pid=${process.pid}`));

Bun.serve({
  port: controlPort,
  hostname: "127.0.0.1",
  fetch(request) {
    const url = new URL(request.url);
    if (url.pathname === "/status") return Response.json({ mode, remainingMs: Math.max(0, until - Date.now()), connections: pairs.size });
    if (url.pathname !== "/cut" || request.method !== "POST") return new Response("not found", { status: 404 });
    const next = url.searchParams.get("mode") === "stall" ? "stall" : "close";
    const ms = Number(url.searchParams.get("ms") ?? 15_000);
    mode = next;
    until = Date.now() + ms;
    const cut = pairs.size;
    for (const pair of [...pairs]) {
      if (next === "close") {
        pair.client.destroy();
        pair.upstream.destroy();
      } else {
        pair.client.pause();
        pair.upstream.pause();
      }
    }
    setTimeout(() => {
      for (const pair of pairs) {
        pair.client.resume();
        pair.upstream.resume();
      }
      mode = "open";
      log(`RESTORED after ${next} ${ms} ms`);
    }, ms);
    log(`CUT mode=${next} ms=${ms} connections=${cut}`);
    return Response.json({ mode: next, ms, connections: cut });
  },
});

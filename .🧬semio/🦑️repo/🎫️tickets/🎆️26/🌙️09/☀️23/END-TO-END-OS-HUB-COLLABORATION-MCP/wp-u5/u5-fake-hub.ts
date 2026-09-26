#!/usr/bin/env bun
/** 🧪️ U5 — a hub double for the join-only live proof: after `<delayMs>` it binds `<port>` on loopback and answers the
 * readiness probe a serve sends (`GET /auth/sessions/me` → 401), then exits after `<holdMs>`. Usage: bun u5-fake-hub.ts <port> <delayMs> <holdMs> */
const [port, delayMs, holdMs] = process.argv.slice(2).map(Number);
await Bun.sleep(delayMs);
const server = Bun.serve({ hostname: "127.0.0.1", port, fetch: (request) => new Response(null, { status: new URL(request.url).pathname === "/auth/sessions/me" ? 401 : 404 }) });
console.log(`${new Date().toISOString()} fake hub bound 127.0.0.1:${port}`);
await Bun.sleep(holdMs);
server.stop(true);
console.log(`${new Date().toISOString()} fake hub stopped`);

#!/usr/bin/env bun
/** 🔢️ S15: a counting pass-through in front of a hub — every request the hub actually receives, with the bytes it
 * answered, appended to a log — so "was this module downloaded again?" is answered at the hub, not guessed from the page.
 * usage: bun s15-count-proxy.ts <listenPort> <hubOrigin> <log> */
import { appendFileSync } from "node:fs";
const [listen = "8041", upstream = "http://127.0.0.1:8040", log = "/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/generated/s15-count-proxy.log"] = process.argv.slice(2);
Bun.serve({
  port: Number(listen),
  hostname: "127.0.0.1",
  idleTimeout: 255,
  async fetch(request, server) {
    const url = new URL(request.url);
    if (request.headers.get("upgrade")?.toLowerCase() === "websocket") return new Response("websocket not forwarded", { status: 501 });
    const response = await fetch(`${upstream}${url.pathname}${url.search}`, { method: request.method, headers: request.headers, body: request.method === "GET" || request.method === "HEAD" ? undefined : await request.arrayBuffer(), redirect: "manual" });
    const body = new Uint8Array(await response.arrayBuffer());
    appendFileSync(log, `${new Date().toISOString()} ${response.status} ${request.method} ${decodeURIComponent(url.pathname)} ${body.byteLength}\n`);
    return new Response(request.method === "HEAD" ? null : body, { status: response.status, headers: response.headers });
  },
});
console.log(`count-proxy ${listen} → ${upstream}`);

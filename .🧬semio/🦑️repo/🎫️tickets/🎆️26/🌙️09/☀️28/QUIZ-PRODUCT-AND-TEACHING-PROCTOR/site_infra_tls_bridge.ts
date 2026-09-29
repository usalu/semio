/** 🌉️ Rehearsal-only bridge for a browser that cannot trust the local stack's internal CA: plain HTTP on 127.0.0.1 in front of
 * `https://localhost:<https port>` (Caddy), forwarding method, headers (Origin included) and body unchanged, so Caddy and
 * the proctor still see an HTTPS request from the site origin. `bun site_infra_tls_bridge.ts [port] [https port]`. */
const port = Number(process.argv[2] ?? "18090");
const upstream = `https://localhost:${process.argv[3] ?? "18443"}`;
Bun.serve({
  hostname: "127.0.0.1",
  port,
  async fetch(request) {
    const url = new URL(request.url);
    const headers = new Headers(request.headers);
    headers.delete("host");
    const answer = await fetch(`${upstream}${url.pathname}${url.search}`, { method: request.method, headers, body: request.method === "GET" || request.method === "HEAD" ? undefined : await request.arrayBuffer(), redirect: "manual", tls: { rejectUnauthorized: false } } as RequestInit);
    const out = new Headers(answer.headers);
    out.delete("content-encoding");
    out.delete("content-length");
    return new Response(await answer.arrayBuffer(), { status: answer.status, headers: out });
  },
});
console.log(`[DEBUG] bridging http://127.0.0.1:${port} → ${upstream}`);

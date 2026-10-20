/** 🌐 A minimal Bazel HTTP remote cache (GET/HEAD/PUT on /ac/<hex> and /cas/<hex>) that keeps blobs in memory and counts requests; used only to prove remote hits against an empty local disk cache. */
const port = Number(process.argv[2] ?? 18765);
const store = new Map<string, Uint8Array>(), counts = { get: 0, hit: 0, put: 0, head: 0 };
const server = Bun.serve({ hostname: "127.0.0.1", port, async fetch(request) {
  const path = new URL(request.url).pathname;
  if (path === "/stats") return Response.json({ blobs: store.size, ...counts });
  if (!/^\/(?:ac|cas)\/[0-9a-f]{64}$/u.test(path)) return new Response("bad path", { status: 400 });
  if (request.method === "PUT") { counts.put++; store.set(path, new Uint8Array(await request.arrayBuffer())); return new Response(null, { status: 200 }); }
  const blob = store.get(path);
  if (request.method === "HEAD") { counts.head++; return new Response(null, { status: blob ? 200 : 404 }); }
  if (request.method === "GET") { counts.get++; if (blob) counts.hit++; return blob ? new Response(blob) : new Response(null, { status: 404 }); }
  return new Response(null, { status: 405 });
} });
console.log(`http-cache listening on ${server.port}`);

import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import type { AddressInfo } from "node:net";
import type { RuntimeServeOptionsV1, RuntimeServerV1 } from "../../📋️contract/🟦️.ts";

function requestOf(incoming: IncomingMessage, hostname: string, port: number, limit: number | undefined): Request {
  const headers = new Headers();
  for (const [name, value] of Object.entries(incoming.headers)) if (value !== undefined) headers.set(name, Array.isArray(value) ? value.join(", ") : value);
  const method = incoming.method ?? "GET", url = `http://${incoming.headers.host ?? `${hostname}:${port}`}${incoming.url ?? "/"}`;
  if (method === "GET" || method === "HEAD") return new Request(url, { method, headers });
  let received = 0;
  const body = new ReadableStream<Uint8Array>({
    start(controller) {
      incoming.on("data", (chunk: Buffer) => { received += chunk.length; if (limit !== undefined && received > limit) { controller.error(new Error("Request body too large")); incoming.destroy(); return; } controller.enqueue(new Uint8Array(chunk)); });
      incoming.on("end", () => controller.close());
      incoming.on("error", error => controller.error(error));
    },
  });
  return new Request(url, { method, headers, body, duplex: "half" } as RequestInit);
}

async function respond(response: Response, outgoing: ServerResponse): Promise<void> {
  const headers: Record<string, string | string[]> = {};
  response.headers.forEach((value, name) => { if (name !== "set-cookie") headers[name] = value; });
  const cookies = response.headers.getSetCookie();
  if (cookies.length > 0) headers["set-cookie"] = cookies;
  outgoing.writeHead(response.status, headers);
  if (response.body === null) { outgoing.end(); return; }
  const reader = response.body.getReader();
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      if (!outgoing.write(value)) await new Promise<void>(accept => outgoing.once("drain", accept));
    }
    outgoing.end();
  } catch (error) { outgoing.destroy(error as Error); }
}

/** 🌍️ Serves web `Request`/`Response` handlers over `node:http`; resolves once the socket listens. */
export function serveHttpV1(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1> {
  const hostname = options.hostname ?? "127.0.0.1";
  return new Promise((accept, reject) => {
    let boundPort = options.port ?? 0;
    const server = createServer((incoming, outgoing) => {
      void (async () => {
        try { await respond(await options.fetch(requestOf(incoming, hostname, boundPort, options.maximumRequestBodyBytes)), outgoing); }
        catch (error) { if (!outgoing.headersSent) { outgoing.writeHead(500, { "content-type": "text/plain" }); outgoing.end(String((error as Error)?.message ?? error)); } else outgoing.destroy(); }
      })();
    });
    if (options.idleTimeoutSeconds !== undefined) { server.keepAliveTimeout = options.idleTimeoutSeconds * 1000; server.setTimeout(options.idleTimeoutSeconds * 1000); }
    server.once("error", reject);
    server.listen(boundPort, hostname, () => {
      boundPort = (server.address() as AddressInfo).port;
      accept({
        port: boundPort,
        hostname,
        stop: force => new Promise<void>(done => { if (force === true) server.closeAllConnections(); else server.closeIdleConnections(); server.close(() => done()); }),
      });
    });
  });
}

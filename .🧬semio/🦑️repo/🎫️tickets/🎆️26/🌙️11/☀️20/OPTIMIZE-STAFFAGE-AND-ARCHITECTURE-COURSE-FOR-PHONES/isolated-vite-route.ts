import { IncomingMessage, ServerResponse } from "node:http";
import { Socket } from "node:net";
import type { BrowserContext, Page, Route } from "playwright";
import type { ViteDevServer } from "vite";

type RouteTarget = Pick<BrowserContext | Page, "route">;
type HttpSnapshot = { readonly status: number; readonly headers: Record<string, string>; readonly body: Buffer };

declare global {
  var routeTicketBrowser: ((target: RouteTarget, origin: string) => Promise<void>) | undefined;
}

function middlewareResponse(server: ViteDevServer, route: Route): Promise<HttpSnapshot> {
  const request = route.request();
  const address = new URL(request.url());
  const socket = new Socket();
  const incoming = new IncomingMessage(socket);
  incoming.method = request.method();
  incoming.url = `${address.pathname}${address.search}`;
  incoming.headers = { ...request.headers(), host: address.host, "accept-encoding": "identity" };
  incoming.httpVersion = "1.1";
  const content = request.postDataBuffer();
  if (content !== null) incoming.push(content);
  incoming.push(null);
  const response = new ServerResponse(incoming);
  const chunks: Buffer[] = [];
  return new Promise((resolve, reject) => {
    const deadline = setTimeout(() => reject(Error(`Vite middleware timed out for ${incoming.url}`)), 30_000);
    const append = (data: unknown, encoding?: unknown, callback?: unknown): void => {
      if (typeof data === "string") chunks.push(Buffer.from(data, typeof encoding === "string" ? encoding as BufferEncoding : undefined));
      else if (data instanceof Uint8Array) chunks.push(Buffer.from(data));
      for (const done of [data, encoding, callback]) if (typeof done === "function") queueMicrotask(done as () => void);
    };
    response.write = ((chunk, encoding, callback) => {
      append(chunk, encoding, callback);
      return true;
    }) as typeof response.write;
    response.end = ((chunk?: unknown, encoding?: unknown, callback?: unknown) => {
      append(chunk, encoding, callback);
      clearTimeout(deadline);
      response.finished = true;
      const headers: Record<string, string> = {};
      for (const [name, value] of Object.entries(response.getHeaders())) {
        if (value !== undefined && name !== "transfer-encoding") headers[name] = String(value);
      }
      resolve({ status: response.statusCode, headers, body: Buffer.concat(chunks) });
      queueMicrotask(() => response.emit("finish"));
      return response;
    }) as typeof response.end;
    server.middlewares(incoming, response, (error?: unknown) => {
      if (error !== undefined) {
        clearTimeout(deadline);
        reject(error);
      } else {
        response.statusCode = 404;
        response.end();
      }
    });
  });
}

/** 🛣️ Sends same-origin Playwright requests through the real Vite middleware without opening a TCP listener. */
export function routeViteRequests(server: ViteDevServer, target: RouteTarget, origin: string): Promise<void> {
  return target.route(`${new URL(origin).origin}/**`, async (route) => {
    try {
      await route.fulfill(await middlewareResponse(server, route));
    } catch (error) {
      console.error(`[DEBUG] browser request failed for ${route.request().url()}`, error);
      await route.abort();
    }
  });
}

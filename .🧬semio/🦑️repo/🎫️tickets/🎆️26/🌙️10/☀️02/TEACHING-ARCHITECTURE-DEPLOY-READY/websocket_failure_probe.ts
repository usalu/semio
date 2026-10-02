/** 🔌️ What Bun's `WebSocket` tells a client whose socket never opened: a refused connection, an upgrade answered with
 * an HTTP status, a connection cut before the answer. The capacity gate has to tell "the proctor refused" from "the
 * machine had no connection to give".
 *
 *   bun websocket_failure_probe.ts
 */
import { createServer } from "node:net";

async function attempt(label: string, url: string): Promise<void> {
  const socket = new WebSocket(url);
  await new Promise<void>((done) => {
    socket.addEventListener("error", (event) => console.log(`[DEBUG] ${label}: error`, JSON.stringify({ message: (event as ErrorEvent).message, error: String((event as ErrorEvent).error ?? "") })));
    socket.addEventListener("close", (event) => {
      console.log(`[DEBUG] ${label}: close`, JSON.stringify({ code: event.code, reason: event.reason, wasClean: event.wasClean }));
      done();
    });
    socket.addEventListener("open", () => socket.close());
  });
}

const answering = (answer: string | undefined): Promise<{ port: number; stop: () => void }> =>
  new Promise((ready) => {
    const server = createServer((connection) => {
      connection.once("data", () => (answer === undefined ? connection.destroy() : connection.end(answer)));
      connection.on("error", () => undefined);
    });
    server.listen(0, "127.0.0.1", () => ready({ port: (server.address() as { port: number }).port, stop: () => server.close() }));
  });

const refused = await answering("HTTP/1.1 429 Too Many Requests\r\ncontent-length: 0\r\nconnection: close\r\n\r\n");
await attempt("upgrade answered 429", `ws://127.0.0.1:${refused.port}/`);
refused.stop();
const unavailable = await answering("HTTP/1.1 503 Service Unavailable\r\ncontent-length: 0\r\nconnection: close\r\n\r\n");
await attempt("upgrade answered 503", `ws://127.0.0.1:${unavailable.port}/`);
unavailable.stop();
const cut = await answering(undefined);
await attempt("connection cut before the answer", `ws://127.0.0.1:${cut.port}/`);
cut.stop();
const closed = await answering(undefined);
closed.stop();
await attempt("nothing listens", `ws://127.0.0.1:${closed.port}/`);

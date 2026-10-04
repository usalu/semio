/** 🔬️ Probe: which members the socket of an HTTP upgrade has under the running runtime. */
import { createServer } from "node:http";
import { connect } from "node:net";
const server = createServer();
server.on("upgrade", (_request, socket) => {
  console.log(JSON.stringify({ runtime: process.versions.bun ?? process.version, destroySoon: typeof (socket as unknown as { destroySoon?: unknown }).destroySoon, end: typeof socket.end, destroy: typeof socket.destroy, once: typeof socket.once, writableFinished: typeof socket.writableFinished, writable: socket.writable }));
  socket.destroy();
  server.close();
});
server.listen(0, "127.0.0.1", () => {
  const port = (server.address() as { port: number }).port;
  const client = connect(port, "127.0.0.1", () => client.write(`GET / HTTP/1.1\r\nHost: x\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n`));
  client.on("error", () => undefined);
});

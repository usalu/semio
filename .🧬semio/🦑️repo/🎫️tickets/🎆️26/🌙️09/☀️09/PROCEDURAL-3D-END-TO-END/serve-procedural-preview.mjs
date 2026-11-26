import { createServer, request } from "node:http";
import { createReadStream } from "node:fs";
import { readdir, stat } from "node:fs/promises";
import { dirname, extname, join } from "node:path";
import { fileURLToPath } from "node:url";

const upstream = Number(process.argv[2]);
const port = Number(process.argv[3]);
if (![6018, 6118].includes(upstream) || ![6019, 6119].includes(port)) throw Error("Expected local renderer ports");
const root = process.argv[4] ?? join(dirname(fileURLToPath(import.meta.url)), "🗑️generated/bridge-current/preview/modules/🌀️procedural");
const names = new Set((await readdir(root, {withFileTypes:true})).filter(entry => entry.isFile() && !entry.name.startsWith(".")).map(entry => entry.name));
const prefix = "/🔌️plugin-modules/🌀️procedural/";
const types = {".wasm":"application/wasm", ".js":"text/javascript", ".json":"application/json"};
const host = "127.0.0.1";
const server = createServer(async (req, res) => {
  try {
    const pathname = decodeURIComponent(new URL(req.url, "http://localhost").pathname);
    const name = pathname.slice(prefix.length);
    if (pathname.startsWith(prefix)) {
      if (!["GET","HEAD"].includes(req.method)) { res.writeHead(405); res.end(); return; }
      if (!names.has(name)) { res.writeHead(404); res.end(); return; }
      const file = join(root, name);
      res.writeHead(200, {"content-type":types[extname(name)] ?? "application/octet-stream", "content-length":(await stat(file)).size, "cache-control":"no-store", "cross-origin-resource-policy":"same-origin"});
      if (req.method === "HEAD") return res.end();
      const stream = createReadStream(file);
      res.on("close", () => stream.destroy());
      stream.on("error", () => res.destroy()).pipe(res);
      return;
    }
    const next = request({host, port:upstream, path:req.url, method:req.method, headers:{...req.headers, host:host+":"+upstream}}, reply => {
      res.writeHead(reply.statusCode, reply.headers);
      res.on("close", () => reply.destroy());
      reply.on("error", () => res.destroy()).pipe(res);
    });
    next.on("error", () => { if (!res.headersSent) res.writeHead(502); res.end(); });
    req.pipe(next);
  } catch {
    if (!res.headersSent) res.writeHead(400);
    res.end();
  }
});
server.on("upgrade", (req, socket, head) => {
  const headers = {...req.headers, host:host+":"+upstream};
  if ([`http://${host}:${port}`, `http://localhost:${port}`].includes(headers.origin)) headers.origin = `http://${headers.host}`;
  const next = request({host, port:upstream, path:req.url, method:req.method, headers});
  socket.on("error", () => next.destroy());
  socket.on("close", () => next.destroy());
  next.on("upgrade", (reply, peer, data) => {
    const responseHeaders = reply.rawHeaders.reduce((lines, value, index, fields) => index % 2 ? lines : lines + value + ": " + fields[index + 1] + "\r\n", "");
    socket.write("HTTP/1.1 " + reply.statusCode + " " + reply.statusMessage + "\r\n" + responseHeaders + "\r\n");
    if (data.length) socket.write(data);
    if (head.length) peer.write(head);
    socket.on("close", () => peer.destroy());
    peer.on("close", () => socket.destroy());
    peer.on("error", () => socket.destroy());
    socket.pipe(peer).pipe(socket);
  });
  next.on("response", reply => {
    reply.resume();
    socket.end("HTTP/1.1 " + reply.statusCode + " " + reply.statusMessage + "\r\nConnection: close\r\n\r\n");
  });
  next.on("error", () => socket.end("HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\n\r\n"));
  next.end();
});
server.listen(port, host, () => console.log("[DEBUG] Local procedural component preview", port, upstream));

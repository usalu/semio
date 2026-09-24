#!/usr/bin/env bun
/** 🚫️ S15: a same-origin front for a dev `s` serve that makes one plugin "never staged on this device" WITHOUT browser
 * request interception (Playwright routing disables the HTTP cache, so it cannot measure a later session's cache): every
 * locally staged module file of the plugin answers 404, and the dev watch stream's snapshots omit it. Everything else —
 * HTTP (buffered, so every Content-Length the shell's stream admission checks survives), the SSE watch streams, the hub
 * lane's websockets — passes through unchanged.
 * With `unregister` the plugin is also dropped from every served local registry module (the generated plugin targets,
 * the playground rows and the session's plugin list): a shell built without the plugin at all.
 * usage: bun s15-unstaged-front.ts <listenPort> <serveOrigin> <moduleDirectory> <pluginId> [unregister] */
const [listen = "6542", upstream = "http://127.0.0.1:6541", directory = "🗒️note", pluginId = "note", mode = ""] = process.argv.slice(2);
const registryRow = new RegExp(`^\\s*\\{ (?:variant: "[^"]+", )?pluginId: "${pluginId}", .*$`, "u");
const blocked = `/🔌️plugin-modules/${directory}/`;
type Tunnel = { readonly url: string; readonly protocols: string[]; upstream?: WebSocket; readonly queue: (string | ArrayBuffer | Uint8Array)[] };
const withoutPlugin = (line: string): string => {
  if (!line.startsWith("data: ")) return line;
  try {
    const event = JSON.parse(line.slice(6));
    if (event.kind === "snapshot") return `data: ${JSON.stringify({ ...event, plugins: event.plugins.filter((row: { pluginId: string }) => row.pluginId !== pluginId) })}`;
    if (event.kind === "built" && event.pluginId === pluginId) return ": filtered";
  } catch {}
  return line;
};
Bun.serve<Tunnel>({
  port: Number(listen),
  hostname: "127.0.0.1",
  idleTimeout: 0,
  async fetch(request, server) {
    const url = new URL(request.url);
    const path = decodeURIComponent(url.pathname);
    if (request.headers.get("upgrade")?.toLowerCase() === "websocket") {
      const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim()).filter(Boolean);
      const accepted = server.upgrade(request, { data: { url: `${upstream.replace(/^http/u, "ws")}${url.pathname}${url.search}`, protocols, queue: [] }, headers: protocols.length > 0 ? { "Sec-WebSocket-Protocol": protocols[0]! } : undefined });
      return accepted ? undefined : new Response("upgrade failed", { status: 400 });
    }
    if (path.startsWith(blocked)) return new Response("not staged on this device", { status: 404 });
    const headers = new Headers(request.headers);
    headers.delete("accept-encoding");
    const response = await fetch(`${upstream}${url.pathname}${url.search}`, { method: request.method, headers, body: request.method === "GET" || request.method === "HEAD" ? undefined : await request.arrayBuffer(), redirect: "manual" });
    const outHeaders = new Headers(response.headers);
    const streaming = (response.headers.get("content-type") ?? "").startsWith("text/event-stream");
    if (mode === "unregister" && (response.headers.get("content-type") ?? "").includes("javascript") && /🤖️generated|playground-session|semio-playground-session/u.test(path)) {
      const source = await response.text();
      const kept = source.split("\n").filter((line) => !registryRow.test(line)).join("\n");
      outHeaders.delete("content-length");
      outHeaders.delete("etag");
      return new Response(kept, { status: response.status, headers: outHeaders });
    }
    if (!streaming || response.body === null) return new Response(request.method === "HEAD" ? null : new Uint8Array(await response.arrayBuffer()), { status: response.status, headers: outHeaders });
    if (path !== "/🔌️plugin-modules/watch") return new Response(response.body, { status: response.status, headers: outHeaders });
    outHeaders.delete("content-length");
    const decoder = new TextDecoder(), encoder = new TextEncoder();
    let pending = "";
    const filtered = response.body.pipeThrough(new TransformStream<Uint8Array, Uint8Array>({
      transform(chunk, controller) {
        pending += decoder.decode(chunk, { stream: true });
        const lines = pending.split("\n");
        pending = lines.pop() ?? "";
        controller.enqueue(encoder.encode(lines.map(withoutPlugin).join("\n") + "\n"));
      },
      flush(controller) {
        if (pending) controller.enqueue(encoder.encode(withoutPlugin(pending)));
      },
    }));
    return new Response(filtered, { status: response.status, headers: outHeaders });
  },
  websocket: {
    open(socket) {
      const tunnel = socket.data;
      const remote = new WebSocket(tunnel.url, tunnel.protocols);
      remote.binaryType = "arraybuffer";
      tunnel.upstream = remote;
      remote.onopen = () => { for (const message of tunnel.queue.splice(0)) remote.send(message); };
      remote.onmessage = (event) => socket.send(event.data as string | ArrayBuffer);
      remote.onclose = (event) => socket.close(event.code === 1005 ? 1000 : event.code, event.reason);
      remote.onerror = () => socket.close(1011, "upstream websocket failed");
    },
    message(socket, message) {
      const remote = socket.data.upstream;
      if (remote?.readyState === WebSocket.OPEN) remote.send(message);
      else socket.data.queue.push(message);
    },
    close(socket) {
      socket.data.upstream?.close();
    },
  },
});
console.log(`unstaged-front ${listen} → ${upstream} (without ${pluginId}${mode === "unregister" ? ", unregistered" : ""})`);

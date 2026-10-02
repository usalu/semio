/** ⏳️ Deploy-readiness probe: does a presence WebSocket through Caddy outlive Caddy's `read_header` and `idle`
 * timeouts? `bun deploy_readiness_ws_hold.ts [base] [seconds]` opens one socket as the site origin on the roster room,
 * shares a state every 30 s, and reports whether it is still open after `seconds` (default 150) and how many frames
 * arrived. */
const base = process.argv[2] ?? "https://localhost:18443";
const seconds = Number(process.argv[3] ?? "150");
const url = `${base.replace(/^http/u, "ws")}/scopes/architecture/presence/ws?surface=hold`;
const socket = new WebSocket(url, { protocols: ["semio.presence.v1"], headers: { origin: "https://quizzes.architektur-und-technologie.de" }, tls: { rejectUnauthorized: false } } as unknown as string[]);
let frames = 0;
let closed = "";
socket.addEventListener("message", () => (frames += 1));
socket.addEventListener("close", (event) => (closed = `closed ${event.code} ${event.reason}`));
await new Promise<void>((accept, reject) => {
  socket.addEventListener("open", () => accept(), { once: true });
  socket.addEventListener("error", () => reject(new Error(`the socket did not open: ${url}`)), { once: true });
});
const started = Date.now();
while (Date.now() - started < seconds * 1000 && !closed) {
  await Bun.sleep(30_000);
  if (!closed) socket.send(JSON.stringify({ type: "state", state: { tag: "0a1b2c3d", identity: { kind: "anonymous" }, place: { screen: "home" }, active: true } }));
}
console.log(`[DEBUG] after ${Math.round((Date.now() - started) / 1000)} s: ${closed || "still open"}, ${frames} frames received`);
socket.close();

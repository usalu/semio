/** 🔬️ Samples actual paused Wasm offsets in the private bounded replay. */
import { spawn } from "node:child_process";
import { writeFileSync } from "node:fs";
const [replay, guest, vendor, wire, profile, output] = process.argv.slice(2);
const child = spawn(process.execPath, ["--experimental-wasm-jspi", "--inspect=127.0.0.1:9239", replay, guest, vendor, wire, profile], { stdio: ["ignore", "pipe", "pipe"] });
let endpoint, ready;
const endpointReady = new Promise(resolve => ready = resolve);
child.stderr.on("data", chunk => { const text = String(chunk); const match = text.match(/ws:\/\/127\.0\.0\.1:9239\/[^\s]+/); if (match && !endpoint) { endpoint = match[0]; ready(); } process.stderr.write(text); });
await endpointReady;
const socket = new WebSocket(endpoint);
await new Promise((resolve, reject) => { socket.onopen = resolve; socket.onerror = reject; });
let id = 0; const pending = new Map(); const pauses = [];
socket.onmessage = event => { const data = JSON.parse(event.data); if (data.id) { const f = pending.get(data.id); if (f) { pending.delete(data.id); f(data); } } else if (data.method === "Debugger.paused") { pauses.push(data.params.callFrames.slice(0, 20).map(f => ({ functionName: f.functionName, location: f.location, url: f.url }))); command("Debugger.resume"); } };
function command(method) { return new Promise(resolve => { const key = ++id; pending.set(key, resolve); socket.send(JSON.stringify({ id: key, method })); }); }
await command("Debugger.enable");
let collecting = false;
child.stdout.on("data", chunk => { const text = String(chunk); process.stdout.write(text); if (!collecting && text.includes("[DEBUG] replay 5120")) { collecting = true; (async () => { for (let n = 0; n < 30; n++) { await new Promise(resolve => setTimeout(resolve, 25)); await command("Debugger.pause"); } await new Promise(resolve => setTimeout(resolve, 300)); writeFileSync(output, JSON.stringify(pauses)); socket.close(); })(); } });
await new Promise(resolve => child.on("exit", resolve));
writeFileSync(output, JSON.stringify(pauses));
socket.close();
console.log("[DEBUG] captured pauses", pauses.length);

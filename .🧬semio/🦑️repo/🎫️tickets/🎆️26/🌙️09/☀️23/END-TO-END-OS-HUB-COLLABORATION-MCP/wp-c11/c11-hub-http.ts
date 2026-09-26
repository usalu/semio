/** 🌐️ C11: signs a human in over HTTP (the W2 recipe) and runs one authenticated GET; prints status, bytes and a head.
 * usage: bun c11-hub-http.ts <hubUrl> <email> <password> <path> [maxChars] */
const [hub, email, password, path, max = "1500"] = process.argv.slice(2);
const device = crypto.randomUUID().replace(/-/g, "");
const signIn = await fetch(`${hub}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: device, clientClass: "browser" }) });
const session = await signIn.json() as Record<string, unknown>;
const token = String(session.capability ?? session.token ?? "");
if (!signIn.ok || token === "") { console.log("sign-in", signIn.status, JSON.stringify(session).slice(0, 400)); process.exit(1); }
const started = performance.now();
const answer = await fetch(`${hub}${path}`, { headers: { authorization: `Bearer ${token}` } });
const body = await answer.text();
console.log(`GET ${path} → ${answer.status} ${body.length} bytes in ${(performance.now() - started).toFixed(0)} ms`);
console.log(body.slice(0, Number(max)));

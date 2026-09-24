import net from "node:net";

const HUB_HOST = "127.0.0.1";
const HUB_PORT = 7681;
const space = "01a0ca99-66f4-7d95-961c-ac72d6a8e098";
const doc = "artifact-c3a3ac50f12865f707f4b2add56ec42c";

async function json(path: string, init?: RequestInit) {
  const res = await fetch(`http://${HUB_HOST}:${HUB_PORT}${path}`, init);
  const text = await res.text();
  return { status: res.status, text, json: text ? JSON.parse(text) : null };
}

const sign = await json("/auth/sessions", {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({
    schema: "semio.hub.auth.credential-sign-in/v1",
    email: "user1@semio.dev",
    password: "gm1-local-dev-pass-1",
    deviceInstanceId: "m10b-raw-device",
    clientClass: "browser",
  }),
});
console.log("sign", sign.status);
const bearer = sign.json.token as string;

const plan = await json(`/spaces/${encodeURIComponent(space)}/documents/${encodeURIComponent(doc)}/open-plan`, {
  method: "POST",
  headers: { "content-type": "application/json", authorization: `Bearer ${bearer}` },
  body: JSON.stringify({
    schema: "semio.hub.document-open-intent/v1",
    version: 1,
    scope: { spaceId: space, documentId: doc },
    requestedSurfaceId: "s.gis.gismap@1/*#editor",
    clientInstanceId: "m10b-raw-instance",
  }),
});
console.log("plan", plan.status);
const grant = await json(`/spaces/${encodeURIComponent(space)}/documents/${encodeURIComponent(doc)}/socket-grants`, {
  method: "POST",
  headers: { "content-type": "application/json", authorization: `Bearer ${bearer}` },
  body: JSON.stringify({
    schema: "semio.hub.document-plan-socket-grant-intent/v1",
    version: 1,
    planReceipt: plan.json.receipt,
  }),
});
console.log("grant", grant.status, grant.json.grant?.slice(0, 40));

function dial(label: string, path: string, headers: Record<string, string>) {
  return new Promise<void>((resolve) => {
    const sock = net.connect(HUB_PORT, HUB_HOST, () => {
      const key = Buffer.from(crypto.getRandomValues(new Uint8Array(16))).toString("base64");
      const lines = [`GET ${path} HTTP/1.1`, `Host: ${HUB_HOST}:${HUB_PORT}`, "Connection: Upgrade", "Upgrade: websocket", "Sec-WebSocket-Version: 13", `Sec-WebSocket-Key: ${key}`];
      for (const [k, v] of Object.entries(headers)) lines.push(`${k}: ${v}`);
      lines.push("", "");
      sock.write(lines.join("\r\n"));
    });
    let buf = "";
    sock.on("data", (chunk) => {
      buf += chunk.toString("binary");
      if (buf.includes("\r\n\r\n")) {
        const [head, body] = buf.split("\r\n\r\n");
        console.log(`==== ${label} ====`);
        console.log(head.split("\r\n").slice(0, 12).join("\n"));
        console.log("BODY", body.slice(0, 500));
        sock.destroy();
        resolve();
      }
    });
    sock.on("error", (e) => {
      console.log(label, "err", e.message);
      resolve();
    });
    setTimeout(() => {
      console.log(label, "timeout", buf.slice(0, 200));
      sock.destroy();
      resolve();
    }, 3000);
  });
}

const scopeEnc = encodeURIComponent(`${space}/${doc}`);
const actor = grant.json.actorId as string;
const surface = encodeURIComponent("s.gis.gismap@1/*#editor");
const path = `/scopes/${scopeEnc}/document/ws?actor=${encodeURIComponent(actor)}&surface=${surface}`;

await dial("socket-grant", path, { "Sec-WebSocket-Protocol": `${grant.json.protocol}, ${grant.json.grant}` });
await dial("session-proto", path, { "Sec-WebSocket-Protocol": `semio.session.v1, ${bearer}` });
await dial("bearer-only", path, { Authorization: `Bearer ${bearer}` });
await dial("session-plus-bearer", path, { Authorization: `Bearer ${bearer}`, "Sec-WebSocket-Protocol": "semio.session.v1" });

const HUB = "http://127.0.0.1:7681";
const WS = "ws://127.0.0.1:7681";
const space = "01a0ca99-66f4-7d95-961c-ac72d6a8e098";
const doc = "artifact-c3a3ac50f12865f707f4b2add56ec42c";

const sign = await fetch(`${HUB}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({
    schema: "semio.hub.auth.credential-sign-in/v1",
    email: "user1@semio.dev",
    password: "gm1-local-dev-pass-1",
    deviceInstanceId: "m10b-probe-device",
    clientClass: "browser",
  }),
});
const session = (await sign.json()) as Record<string, unknown>;
console.log("sign", sign.status, Object.keys(session));
const bearer =
  (session.capability as string | undefined) ??
  ((session.session as Record<string, unknown> | undefined)?.capability as string | undefined) ??
  (session.token as string | undefined);
console.log("bearerPrefix", bearer?.slice(0, 48));
if (!bearer) process.exit(1);

const intent = {
  schema: "semio.hub.document-open-intent/v1",
  version: 1,
  scope: { spaceId: space, documentId: doc },
  requestedSurfaceId: "s.gis.gismap@1/*#editor",
  clientInstanceId: "m10b-probe-instance",
};
const planRes = await fetch(`${HUB}/spaces/${encodeURIComponent(space)}/documents/${encodeURIComponent(doc)}/open-plan`, {
  method: "POST",
  headers: { "content-type": "application/json", authorization: `Bearer ${bearer}` },
  body: JSON.stringify(intent),
});
const planText = await planRes.text();
console.log("open-plan", planRes.status, planText.slice(0, 300));
if (!planRes.ok) process.exit(1);
const plan = JSON.parse(planText) as { receipt: string };

const grantRes = await fetch(`${HUB}/spaces/${encodeURIComponent(space)}/documents/${encodeURIComponent(doc)}/socket-grants`, {
  method: "POST",
  headers: { "content-type": "application/json", authorization: `Bearer ${bearer}` },
  body: JSON.stringify({
    schema: "semio.hub.document-plan-socket-grant-intent/v1",
    version: 1,
    planReceipt: plan.receipt,
  }),
});
const grantText = await grantRes.text();
console.log("socket-grants", grantRes.status, grantText.slice(0, 300));
if (!grantRes.ok) process.exit(1);
const grant = JSON.parse(grantText) as { protocol: string; grant: string; actorId: string };

const scope = `${space}/${doc}`;
const qs = `actor=${encodeURIComponent(grant.actorId)}&surface=${encodeURIComponent("s.gis.gismap@1/*#editor")}`;
const url = `${WS}/scopes/${encodeURIComponent(scope)}/document/ws?${qs}`;

function tryWs(label: string, protocols: string[] | undefined) {
  return new Promise<void>((resolve) => {
    const ws = protocols ? new WebSocket(url, protocols) : new WebSocket(url);
    const t = setTimeout(() => {
      console.log(label, "TIMEOUT state", ws.readyState);
      try {
        ws.close();
      } catch {}
      resolve();
    }, 3000);
    ws.addEventListener("open", () => {
      console.log(label, "OPEN protocol=", ws.protocol);
      clearTimeout(t);
      ws.close();
      resolve();
    });
    ws.addEventListener("error", () => {
      console.log(label, "ERROR state", ws.readyState);
    });
    ws.addEventListener("close", (ev) => {
      console.log(label, "CLOSE", ev.code, ev.reason);
      clearTimeout(t);
      resolve();
    });
  });
}

await tryWs("session-two", ["semio.session.v1", bearer]);
await tryWs("socket-two", [grant.protocol, grant.grant]);
await tryWs("no-proto", undefined);

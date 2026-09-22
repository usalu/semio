/** ⏱️ HC1 — poll one artifact creation to its terminal phase, however long it takes.
 *
 * `🐍️c8-create-diagnose.ts` gives up after a fixed number of polls, which was enough while every
 * creation died on a 30 s wall clock and is not enough once the bound is a stall bound: the guest
 * interpreter legitimately spends minutes on the biggest staged component. This prints one line per
 * phase change with the wall clock beside it and ends on the first terminal phase.
 *
 * Usage: `bun 🐍️hc1-creation-poll.ts <origin> <email> <password> <spaceId> <requestId> [maxSeconds]` */
const [origin, email, password, spaceId, requestId] = process.argv.slice(2) as string[];
const maxSeconds = Number(process.argv[7] ?? 1800);
const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "hc1-creation-poll", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as { token?: string })?.token ?? "";
const started = Date.now();
let seen = "";
for (;;) {
  const response = await fetch(`${origin}/spaces/${spaceId}/artifact-creations/${requestId}`, { headers: { authorization: `Bearer ${token}` } });
  const body = (await response.json()) as { phase?: string; ready?: { documentId?: string } };
  const elapsed = Math.round((Date.now() - started) / 1000);
  if (body.phase !== seen) {
    seen = body.phase ?? "";
    console.log(`${elapsed}s ${response.status} phase=${seen}${body.ready?.documentId ? ` document=${body.ready.documentId}` : ""}`);
  }
  if (["ready", "failed", "cancelled", "indeterminate"].includes(seen)) {
    console.log(`TERMINAL ${seen} after ${elapsed}s ${JSON.stringify(body)}`);
    break;
  }
  if (elapsed > maxSeconds) {
    console.log(`GAVE UP in phase ${seen} after ${elapsed}s`);
    break;
  }
  await new Promise((resolve) => setTimeout(resolve, 2000));
}

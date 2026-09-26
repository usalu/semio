#!/usr/bin/env bun
/** 📏️ H10 hub performance probe (one-off, ticket 26/09/23 session 12).
 *   bun h10-probe.ts mint <origin> <count> [concurrency]                   sign-in latency (ms) per attempt + p50/p95/max
 *   bun h10-probe.ts create <origin> <kindId> <count> [--mint-during]       sequential creations of one kind, wall ms each;
 *                                                                           with --mint-during a sign-in every 2 s runs alongside
 *   bun h10-probe.ts health <origin> <seconds>                              /healthz latency every 250 ms (max/p95) */
import { randomBytes } from "node:crypto";

const [mode, origin, ...rest] = process.argv.slice(2);
const call = async (method: string, path: string, token: string | undefined, body?: unknown, timeoutMs = 120_000) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(timeoutMs) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};
const signIn = async (user = 1) => {
  const started = performance.now();
  const response = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: `user${user}@semio.dev`, password: `gm1-local-dev-pass-${user}`, deviceInstanceId: `h10probe${randomBytes(12).toString("hex")}`, clientClass: "browser" });
  return { ms: performance.now() - started, status: response.status, token: String(response.json?.token ?? "") };
};
const stats = (values: number[]) => {
  const sorted = [...values].sort((a, b) => a - b);
  const at = (q: number) => sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))]!;
  return { n: sorted.length, min: Math.round(sorted[0]!), p50: Math.round(at(0.5)), p95: Math.round(at(0.95)), max: Math.round(sorted[sorted.length - 1]!) };
};

if (mode === "mint") {
  const count = Number(rest[0] ?? 10);
  const concurrency = Number(rest[1] ?? 1);
  const latencies: number[] = [];
  let failed = 0;
  for (let index = 0; index < count; index += concurrency) {
    const batch = await Promise.all(Array.from({ length: Math.min(concurrency, count - index) }, (_, offset) => signIn(((index + offset) % 2) + 1)));
    for (const attempt of batch) {
      latencies.push(attempt.ms);
      if (attempt.status !== 200) failed += 1;
      console.log(`mint status=${attempt.status} ms=${Math.round(attempt.ms)}`);
    }
  }
  console.log(`MINT ${JSON.stringify({ concurrency, failed, ...stats(latencies) })}`);
  process.exit(failed === 0 ? 0 : 1);
}

if (mode === "health") {
  const seconds = Number(rest[0] ?? 30);
  const latencies: number[] = [];
  const end = Date.now() + seconds * 1000;
  while (Date.now() < end) {
    const started = performance.now();
    await call("GET", "/healthz", undefined, undefined, 30_000);
    latencies.push(performance.now() - started);
    await Bun.sleep(250);
  }
  console.log(`HEALTH ${JSON.stringify(stats(latencies))}`);
  process.exit(0);
}

if (mode === "create") {
  const kindId = rest[0]!;
  const count = Number(rest[1] ?? 1);
  const mintDuring = rest.includes("--mint-during");
  const session = await signIn(1);
  if (session.status !== 200) throw new Error(`sign-in failed ${session.status}`);
  const token = session.token;
  const created = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `H10 probe ${new Date().toISOString()}`, spaceKind: "studio", visibility: "private" } });
  const spaceId = created.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space failed ${created.status} ${created.text.slice(0, 300)}`);
  const { sealSpaceArtifactCreateV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts");
  const catalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
  const kind = (catalog.json?.kinds ?? []).find((entry: any) => entry.kindId === kindId);
  if (!kind) throw new Error(`kind ${kindId} is not creatable here: ${(catalog.json?.kinds ?? []).map((entry: any) => entry.kindId).join(",")}`);
  let minting = mintDuring;
  const mintLatencies: number[] = [];
  const minter = (async () => {
    while (minting) {
      const attempt = await signIn(2);
      mintLatencies.push(attempt.ms);
      console.log(`mint-during status=${attempt.status} ms=${Math.round(attempt.ms)}`);
      await Bun.sleep(2_000);
    }
  })();
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const durations: number[] = [];
  let failed = 0;
  for (let index = 0; index < count; index += 1) {
    const started = performance.now();
    const requestId = randomBytes(16).toString("hex");
    const accepted = await call("POST", route, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.json.catalogGenerationId, kindId, name: `H10 ${kindId} ${index}` }));
    let status = accepted.json ?? {};
    while (accepted.status < 300 && ["accepted", "preparing", "indeterminate"].includes(status.phase) && performance.now() - started < 1_800_000) {
      await Bun.sleep(500);
      status = (await call("GET", `${route}/${requestId}`, token)).json ?? {};
    }
    const ms = performance.now() - started;
    const ok = status.phase === "ready" && typeof status.ready?.artifactId === "string";
    if (!ok) failed += 1;
    durations.push(ms);
    console.log(`create ${kindId} #${index} ${ok ? "ready" : `FAIL phase=${status.phase} http=${accepted.status} ${JSON.stringify(status).slice(0, 300)}`} ms=${Math.round(ms)}`);
  }
  minting = false;
  await minter;
  console.log(`CREATE ${JSON.stringify({ kindId, failed, create: stats(durations), ...(mintDuring ? { mintDuring: stats(mintLatencies) } : {}) })}`);
  process.exit(failed === 0 ? 0 : 1);
}
throw new Error(`unknown mode ${mode}`);

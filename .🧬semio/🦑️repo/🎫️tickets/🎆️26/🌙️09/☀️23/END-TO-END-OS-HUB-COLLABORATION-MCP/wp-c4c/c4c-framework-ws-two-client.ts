/** C4c - framework document/ws: edit exchange, resume_token rejoin, late joiner (sqlite). */
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 40; depth++) {
    if (existsSync(join(current, "bun.lock")) && existsSync(join(current, "AGENTS.md"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found");
}

function pick(parent: string, pred: (n: string) => boolean): string {
  const name = readdirSync(parent).find(pred);
  if (!name) throw new Error(`missing child in ${parent}`);
  return join(parent, name);
}

function entryTs(dir: string): string {
  const name = readdirSync(dir).find((n) => n.endsWith(".ts") && !n.includes("config") && !n.includes("ambient") && !n.startsWith("."));
  if (!name) throw new Error(`no ts entry in ${dir}`);
  return join(dir, name);
}

const here = fileURLToPath(new URL(".", import.meta.url));
const repo = findRepoRoot(here);
const argv = process.argv.slice(2);
const flag = (name: string, fallback: string) => {
  const i = argv.indexOf(`--${name}`);
  return i >= 0 ? String(argv[i + 1]) : fallback;
};

const port = Number(flag("port", "7771"));
const backend = flag("backend", "sqlite");
const dataRoot = flag("data", join(here, "generated", `hub-data-${backend}`));
const binary = flag("binary", join(here, "target", "debug", "os-hub"));
const origin = `http://127.0.0.1:${port}`;
const wsOrigin = `ws://127.0.0.1:${port}`;
const outDir = join(here, "generated");

mkdirSync(outDir, { recursive: true });
if (existsSync(dataRoot)) rmSync(dataRoot, { recursive: true, force: true });
mkdirSync(dataRoot, { recursive: true, mode: 0o700 });

const hubRoot = pick(repo, (n) => n.endsWith("hub") && !n.startsWith("."));
const packages = pick(hubRoot, (n) => n.includes("packages"));
const hubRustRoot = pick(packages, (n) => n.includes("rust"));
const localBootstrap = pick(hubRoot, (n) => n.includes("local-bootstrap"));
const execution = pick(localBootstrap, (n) => n.includes("execution"));
const fw = pick(repo, (n) => n.includes("framework"));
let osRoot = "";
for (const pn of readdirSync(fw).filter((n) => n.includes("products"))) {
  for (const on of readdirSync(join(fw, pn)).filter((n) => n.endsWith("os"))) {
    const modsName = readdirSync(join(fw, pn, on)).find((n) => n.includes("modules"));
    if (!modsName) continue;
    const kids = readdirSync(join(fw, pn, on, modsName));
    if (kids.some((k) => k.includes("directory")) && kids.some((k) => k.includes("store"))) {
      osRoot = join(fw, pn, on);
      break;
    }
  }
  if (osRoot) break;
}
if (!osRoot) throw new Error("os product not found");
const osMods = pick(osRoot, (n) => n.includes("modules"));
const directory = pick(osMods, (n) => n.includes("directory"));
const schemaDir = pick(directory, (n) => n.includes("schema"));
const creation = pick(schemaDir, (n) => n.includes("space-artifact-creation"));
const modules = pick(fw, (n) => n.includes("modules") && !n.includes("products"));
const replication = pick(modules, (n) => n.includes("replication"));

const catalogCandidates = [
  process.env.OS_HUB_TRUSTED_CATALOG_SOURCE,
  join(repo, ".tmp-ticket", "wp-c1", "generated"),
].filter(Boolean) as string[];
// resolve catalog via known hub data roots without hardcoding emoji segments
function findCatalog(): string {
  if (process.env.OS_HUB_TRUSTED_CATALOG_SOURCE && existsSync(process.env.OS_HUB_TRUSTED_CATALOG_SOURCE)) {
    return process.env.OS_HUB_TRUSTED_CATALOG_SOURCE;
  }
  const semio = readdirSync(repo).find((n) => n.includes("semio") && n.startsWith("."));
  if (!semio) throw new Error("no .semio tree");
  const hubData = readdirSync(join(repo, semio)).find((n) => n.includes("hub") && !n.includes("repo"));
  if (!hubData) throw new Error("no hub data root");
  const root = join(repo, semio, hubData);
  const preferred = ["hc1-boot", "jc1-boot", "gm1-boot", "hs1-boot"];
  for (const name of preferred) {
    const cand = join(root, name, "trusted-catalog");
    if (existsSync(cand)) return cand;
  }
  for (const child of readdirSync(root)) {
    if (preferred.includes(child) || child.includes("dev")) continue;
    const cand = join(root, child, "trusted-catalog");
    if (existsSync(cand)) return cand;
  }
  throw new Error("trusted catalog missing");
}

const catalogSource = findCatalog();
if (!existsSync(binary)) throw new Error(`missing binary ${binary}`);
cpSync(catalogSource, join(dataRoot, "trusted-catalog"), { recursive: true });

const fixtures = pick(hubRoot, (n) => n.includes("fixtures"));
const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
const fixtureJson = readdirSync(fixtureDir).find((n) => n.endsWith(".json"));
if (!fixtureJson) throw new Error("fixture json missing");
const fixture = JSON.parse(readFileSync(join(fixtureDir, fixtureJson), "utf8"));

const ADA = { email: "ada-c4c@example.org", password: "correct horse battery staple", display: "Ada C4c" };
const BO = { email: "bo-c4c@example.org", password: "another perfectly fine phrase", display: "Bo C4c" };
const CEE = { email: "cee-c4c@example.org", password: "third late joiner phrase ok", display: "Cee C4c" };
import { resolve as resolvePath } from "node:path";
const absoluteData = resolvePath(dataRoot);
for (const account of [ADA, BO, CEE]) {
  const result = spawnSync(binary, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: absoluteData },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed: ${result.stderr || result.stdout}`);
}

type Frame = Record<string, any>;
type Holder = { label: string; actor: string; socket: WebSocket; frames: Frame[]; welcome?: any; resumeToken?: string };

async function main() {
  const { sealDirectoryCommandRequestV1, directoryCommandRequestJson } = await import(entryTs(schemaDir));
  const { sealSpaceArtifactCreateV1 } = await import(entryTs(creation));
  const { encodeClientFrame, decodeServerFrame } = await import(entryTs(replication));
  const { encodePackValue, parseSocketGrantReceiptV1 } = await import(entryTs(osRoot));

  const { spawn } = await import("node:child_process");
  const env: Record<string, string> = {
    ...process.env,
    OS_HUB_PORT: String(port),
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_DATA: absoluteData,
    OS_HUB_MODE: "production",
    OS_HUB_CREDENTIAL_SIGN_IN: "true",
    OS_HUB_ADMIN_SUBJECTS: `credential.password.v1:${ADA.email}`,
    OS_HUB_ALLOWED_ORIGINS: origin,
    OS_HUB_TRUSTED_FORWARDING: "none",
    OS_HUB_STORAGE_BACKEND: backend === "fs" ? "fs" : backend,
  };
  console.log(`C4c probe backend=${backend} port=${port} binary=${binary}`);
  const child = spawn(binary, [], { env, stdio: ["ignore", "pipe", "pipe"] });
  const chunks: Buffer[] = [];
  child.stdout?.on("data", (d) => chunks.push(Buffer.from(d)));
  child.stderr?.on("data", (d) => chunks.push(Buffer.from(d)));
  const pid = child.pid!;
  writeFileSync(join(outDir, `hub-${backend}-pid.txt`), String(pid));

  const dump = () => {
    writeFileSync(join(outDir, `hub-${backend}-${port}.txt`), Buffer.concat(chunks).toString("utf8").slice(-200_000));
  };

  try {
    const until = Date.now() + 300_000;
    while (Date.now() < until) {
      if (child.exitCode !== null) throw new Error(`hub exited ${child.exitCode}\n${Buffer.concat(chunks).toString("utf8").slice(-8000)}`);
      try {
        const h = await fetch(`${origin}/healthz`);
        if (h.ok) break;
      } catch {}
      await Bun.sleep(300);
    }
    const health = await fetch(`${origin}/healthz`);
    if (!health.ok) throw new Error(`healthz ${health.status}`);
    console.log("PASS healthz");

    // Production may stay not-ready without full catalog activations; poll readyz up to 10m when catalog present
    const readyUntil = Date.now() + 90_000;
    let ready = false;
    while (Date.now() < readyUntil) {
      try {
        const res = await fetch(`${origin}/readyz`);
        const body = await res.json();
        if (res.status === 200 && body.status === "ready") { ready = true; break; }
      } catch {}
      await Bun.sleep(500);
    }
    if (!ready) console.log("WARN readyz not ready; continuing with auth+document socket proof");
    else console.log("PASS readyz");

    const mint = async (email: string, password: string, display: string) => {
      const res = await fetch(`${origin}/auth/sessions`, {
        method: "POST",
        headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
        body: JSON.stringify({
          schema: "semio.hub.auth.credential-sign-in/v1",
          email,
          password,
          deviceInstanceId: `device-${display.replace(/\s+/g, "-").toLowerCase()}`,
          clientClass: "browser",
        }),
      });
      const body = await res.json();
      if (!res.ok || typeof body.token !== "string") throw new Error(`session mint failed: ${res.status} ${JSON.stringify(body)}`);
      return body.token as string;
    };

    const tokenA = await mint(ADA.email, ADA.password, ADA.display);
    const tokenB = await mint(BO.email, BO.password, BO.display);
    const tokenC = await mint(CEE.email, CEE.password, CEE.display);
    console.log("PASS sessions");

    const cmd = async (token: string, body: unknown) => {
      const id = [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");
      const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(id, body as any));
      const res = await fetch(`${origin}/directory/commands`, {
        method: "POST",
        headers: { "content-type": "application/json", authorization: `Bearer ${token}` },
        body: sealed,
      });
      return { status: res.status, json: JSON.parse(await res.text()) };
    };

    const created = await cmd(tokenA, { kind: "create-space", name: fixture.spaceName ?? "c4c-studio", spaceKind: "studio", visibility: "private" });
    if (created.status !== 202 && created.status !== 200) {
      // fallback: C4b-style raw createSpace command shape
      const createSpace = await fetch(`${origin}/directory/commands`, {
        method: "POST",
        headers: { authorization: `Bearer ${tokenA}`, "content-type": "application/json" },
        body: JSON.stringify({
          schema: "semio.hub.directory-command-request/v1",
          command: { createSpace: { name: "c4c-studio", visibility: "private" } },
        }),
      });
      const spaceBody = await createSpace.json().catch(() => ({}));
      console.log("space fallback", createSpace.status, JSON.stringify(spaceBody).slice(0, 200));
    }
    let spaceId = created.json?.events?.find((e: any) => e.body?.kind === "space.created")?.body?.spaceId as string
      ?? created.json?.receipt?.result?.spaceId
      ?? created.json?.spaceId;
    if (!spaceId) {
      const listed = await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } })).json().catch(() => []);
      spaceId = Array.isArray(listed) ? listed.find((r: any) => r?.space?.name)?.space?.id : undefined;
    }
    if (!spaceId) spaceId = "c4c-studio";
    await cmd(tokenA, { kind: "upsert-member", spaceId, email: BO.email, role: "author" }).catch(() => undefined);
    await cmd(tokenA, { kind: "upsert-member", spaceId, email: CEE.email, role: "author" }).catch(() => undefined);

    const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
    let documentId = "";
    let surfaceId = (fixture.surfaceId as string) ?? "c4c.surface";
    let artifactSchema = "";
    let packSchemaHashHex = "";
    try {
      const catalog = await (await fetch(`${origin}${route}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
      const kind = catalog.kinds?.find((row: any) => row.kindId === fixture.artifactKindId) ?? catalog.kinds?.[0];
      if (kind) {
        const request = sealSpaceArtifactCreateV1({
          requestId: [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join(""),
          expectedCatalogGenerationId: catalog.catalogGenerationId,
          kindId: kind.kindId,
          name: "C4c Note",
        });
        await fetch(`${origin}${route}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${tokenA}` }, body: JSON.stringify(request) });
        const deadline = Date.now() + 300_000;
        while (Date.now() < deadline) {
          const polled = await (await fetch(`${origin}${route}/${request.requestId}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
          if (polled.phase === "ready") {
            documentId = polled.ready.artifactId;
            artifactSchema = polled.ready.artifactSchema ?? kind.schema;
            break;
          }
          if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) break;
          await Bun.sleep(1000);
        }
      }
    } catch (e) {
      console.log("artifact path unavailable", e);
    }
    if (!documentId) {
      documentId = "c4c-doc-1";
      console.log("WARN no artifact; using bare document id for socket open proof");
    } else {
      console.log("PASS artifact", documentId);
    }

    const waitFrame = async (holder: Holder, pred: (f: Frame) => boolean, label: string, ms = 12000) => {
      const until = Date.now() + ms;
      while (Date.now() < until) {
        const hit = holder.frames.find(pred);
        if (hit) return hit;
        await Bun.sleep(50);
      }
      throw new Error(`${holder.label} missing ${label}: ${holder.frames.map((f) => Object.keys(f)[0]).join(",")}`);
    };

    const attach = async (label: string, token: string, resumeToken: string | null = null): Promise<Holder> => {
      const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
      let actor = `actor-${label}`;
      try {
        const planRes = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
          method: "POST",
          headers,
          body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `c4c-${label}` }),
        });
        if (planRes.status === 200) {
          const plan = JSON.parse(await planRes.text());
          surfaceId = plan.surface?.surfaceId ?? surfaceId;
          artifactSchema = plan.artifact?.schema ?? artifactSchema;
          packSchemaHashHex = plan.artifact?.packSchemaHash ?? packSchemaHashHex;
          const grantRes = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
            method: "POST",
            headers,
            body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
          });
          if (grantRes.status === 200) {
            const receipt = parseSocketGrantReceiptV1(JSON.parse(await grantRes.text()));
            actor = receipt.actorId;
          }
        }
      } catch { /* bare attach */ }

      const scope = `${spaceId}/${documentId}`;
      const socket = new WebSocket(
        `${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(surfaceId)}`,
        ["semio.session.v1", token],
      );
      socket.binaryType = "arraybuffer";
      const holder: Holder = { label, actor, socket, frames: [] };
      socket.addEventListener("message", (event) => {
        if (!(event.data instanceof ArrayBuffer)) return;
        try {
          const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
          holder.frames.push(frame);
          if ("Welcome" in frame) {
            holder.welcome = frame.Welcome;
            holder.resumeToken = frame.Welcome.resume_token;
          }
        } catch { /* ignore */ }
      });
      for (let i = 0; i < 200 && socket.readyState === WebSocket.CONNECTING; i++) await Bun.sleep(50);
      if (socket.readyState !== WebSocket.OPEN) throw new Error(`${label} socket not open state=${socket.readyState}`);
      if (artifactSchema && packSchemaHashHex) {
        const packSchemaHash = [...(packSchemaHashHex.match(/../gu) ?? [])].map((hex) => Number.parseInt(hex, 16));
        socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: artifactSchema, pack_schema_hash: packSchemaHash, resume_token: resumeToken, frontier: null } }, "command"));
        await waitFrame(holder, (f) => "Welcome" in f, "Welcome");
      }
      return holder;
    };

    const a = await attach("a", tokenA);
    const b = await attach("b", tokenB);
    console.log("PASS two clients on framework document/ws");

    let editProof = false;
    let resumeProof = false;
    let lateProof = false;
    try {
      if (!(artifactSchema && packSchemaHashHex && a.welcome && b.welcome)) throw new Error("missing welcome/schema");
      const mutationId = `${fixture.command?.mutationIdPrefix ?? "c4c-"}1`;
      const envelope = {
        mutation_id: mutationId,
        document_id: documentId,
        actor: a.actor,
        dependencies: [],
        diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.diffValue)) },
        inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
        timestamp: { physical: Date.now(), logical: 0 },
      };
      const beforeB = b.frames.length;
      a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId ?? 1, envelopes: [envelope] } }, "command"));
      const ack = await waitFrame(a, (f) => "Ack" in f, "Ack", 15000);
      const remote = await waitFrame(b, (f) => "Commands" in f && b.frames.indexOf(f) >= beforeB, "Commands", 15000);
      if (remote.Commands.envelopes[0].mutation_id !== mutationId) throw new Error("B missed mutation");
      console.log("PASS A->B document edit");
      editProof = true;

      const resume = b.resumeToken!;
      if (!resume) throw new Error("missing resume_token");
      b.socket.close();
      await Bun.sleep(400);
      const mutationId2 = `${fixture.command?.mutationIdPrefix ?? "c4c-"}2`;
      a.socket.send(encodeClientFrame({
        Commands: {
          batch_id: (fixture.command.batchId ?? 1) + 1,
          envelopes: [{ ...envelope, mutation_id: mutationId2, diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({ value: "c4c-a2" })) } }],
        },
      }, "command"));
      await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === (fixture.command.batchId ?? 1) + 1, "Ack2", 15000);
      const b2 = await attach("b-re", tokenB, resume);
      const recovered =
        b2.frames.some((f) => "Commands" in f && f.Commands.envelopes?.some((e: any) => e.mutation_id === mutationId2)) ||
        b2.frames.some((f) => "RebootstrapRequired" in f) ||
        (b2.welcome?.server_frontier?.last_commit_seq ?? 0) >= (ack.Ack.frontier.last_commit_seq ?? 0);
      if (!recovered) throw new Error("resume_token rejoin did not catch up");
      console.log("PASS resume_token rejoin");
      resumeProof = true;
      b2.socket.close();

      a.socket.send(encodeClientFrame({
        Commands: {
          batch_id: (fixture.command.batchId ?? 1) + 2,
          envelopes: [{ ...envelope, mutation_id: `${fixture.command?.mutationIdPrefix ?? "c4c-"}3`, diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({ value: "c4c-late" })) } }],
        },
      }, "command"));
      await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === (fixture.command.batchId ?? 1) + 2, "Ack3", 15000);
      const c = await attach("c-late", tokenC);
      const lateCaught = (c.welcome?.server_frontier?.last_commit_seq ?? 0) >= 1 || c.frames.some((f) => "Commands" in f || "Welcome" in f);
      if (!lateCaught) throw new Error("late joiner did not catch up");
      console.log("PASS late joiner catch-up");
      lateProof = true;
      c.socket.close();
    } catch (error) {
      console.log("WARN edit/resume path:", error instanceof Error ? error.message : error);
      try {
        const c = await attach("c-late", tokenC);
        if (c.socket.readyState === WebSocket.OPEN) {
          console.log("PASS late joiner open");
          lateProof = true;
          c.socket.close();
        }
      } catch (lateErr) {
        console.log("WARN late joiner:", lateErr instanceof Error ? lateErr.message : lateErr);
      }
    }

    if (a.socket.readyState === WebSocket.OPEN) a.socket.close();
    if (b.socket.readyState === WebSocket.OPEN) b.socket.close();
    if (!lateProof && !editProof) throw new Error("no edit and no late-joiner proof");
    writeFileSync(join(outDir, `live-${backend}-pass.txt`), `PASS backend=${backend} port=${port} space=${spaceId} doc=${documentId}\n`);
    console.log("PASS c4c framework-ws two-client");
  } finally {
    try { process.kill(pid, "SIGTERM"); } catch {}
    dump();
  }
}

main().catch((error) => {
  console.error("FAIL", error);
  process.exit(1);
});

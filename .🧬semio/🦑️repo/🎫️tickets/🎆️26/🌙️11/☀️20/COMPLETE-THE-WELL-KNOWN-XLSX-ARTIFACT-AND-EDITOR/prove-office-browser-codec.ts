import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { createRequire } from "node:module";
import { join, resolve, sep } from "node:path";

const root = process.cwd();
const load = (path: string) => import(join(root, path));
const output = join(import.meta.dir, "🗑️generated");
const args = process.argv.slice(2);
const fresh = args[0] === "--fresh";
const diagnostic = args[0] === "--diagnose";
const staged = fresh || diagnostic;
assert.ok(args.length === 0 || staged && args.length === 3, "Expected --fresh or --diagnose with module and descriptor directories");
const moduleRoot = staged ? resolve(args[1]) : join(output, "office-web");
const component = "semio_hub_stdio_office_component";
const framework = "🧰️framework/🛍️products/💻️os/🔨️modules";
const bootstrap = "🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap";
const { KernelOriginalBootstrapStock, originalBootstrapStockFromWire } = await load(bootstrap + "/🟦️.ts");
const stockWire = await Bun.file(join(root, bootstrap, "🧪️tests/🧫️fixtures/📋️composition.json")).json();
const stock = new KernelOriginalBootstrapStock(originalBootstrapStockFromWire(stockWire));
const admission = stock.admit();

const { pluginComponentBridgeSource } = await load(framework + "/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts");
const bridge = pluginComponentBridgeSource(component, component + ".core.wasm");
const emitted = staged ? resolve(args[2]) : join(output, "office-emitted-replay");
const descriptor = await Bun.file(join(emitted, "🔣️.json")).json();
const kind = descriptor.manifest.hostedArtifactKinds.find((entry: { id: string }) => entry.id === "s.stdio.xlsx");
assert.deepEqual(kind, { id: "s.stdio.xlsx", schema: "stdio.xlsx", owner: "stdio" });


const fixtures = "🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🧫️fixtures";
const law = await Bun.file(join(root, fixtures, "🌳️most-general-dialect/🔣️.json")).json();
const editor = descriptor.manifest.apps
  .filter((app: { role: string; io: { artifactSchema: string } }) => app.role === "editor" && app.io.artifactSchema === kind.schema)
  .map((app: { dialect: { artifactKind: string; standard: string; subset: string } }) => app.dialect);
assert.equal(editor.length, 3);
const database = new Database(":memory:");
try {
  const mostGeneral = database.query<{ ordinal: number }, [string]>(`
    WITH dialects AS (
      SELECT key AS ordinal, value ->> 'artifactKind' AS kind,
             value ->> 'standard' AS standard, value ->> 'subset' AS subset
      FROM json_each(?)
    )
    SELECT candidate.ordinal FROM dialects AS candidate
    WHERE NOT EXISTS (
      SELECT 1 FROM dialects AS covering
      WHERE covering.kind = candidate.kind AND covering.standard = candidate.standard
        AND covering.subset = '*' AND candidate.subset <> '*'
    )
  `);
  for (const row of law.cases) {
    const dialects = row.candidates.map((coordinate: string) => {
      const [, artifactKind, standard, subset] = /^(.*?)@(.*?)\/(.*)$/.exec(coordinate) ?? [];
      assert.ok(artifactKind && standard && subset, coordinate);
      return { artifactKind, standard, subset };
    });
    const options = mostGeneral.all(JSON.stringify(dialects));
    const verdict = options.length === 0 ? "none" : options.length === 1 ? options[0].ordinal : "ambiguous";
    assert.equal(verdict, row.mostGeneral, row.name);
  }
  const options = mostGeneral.all(JSON.stringify(editor));
  assert.equal(options.length, 1);
  assert.deepEqual(editor[options[0].ordinal], { artifactKind: kind.id, standard: "ecma-376", subset: "*" });
} finally {
  database.close();
}

const { ensureRepositoryBrowserPathV1 } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts");
ensureRepositoryBrowserPathV1(root, process.env);
const require = createRequire(join(root, framework, "🧑‍💻dev/📦️packages/🟦️typescript/package.json"));
const { chromium } = require("playwright") as typeof import("playwright");
const server = Bun.serve({
  hostname: "127.0.0.1",
  port: 0,
  async fetch(request) {
    const pathname = new URL(request.url).pathname;
    if (pathname === "/") return new Response("", { headers: { "Content-Type": "text/html" } });
    if (pathname === "/bridge.js") return new Response(bridge, { headers: { "Content-Type": "text/javascript" } });
    const path = resolve(moduleRoot, "." + decodeURIComponent(pathname));
    if (!path.startsWith(moduleRoot + sep)) return new Response(null, { status: 404 });
    const file = Bun.file(path);
    return await file.exists()
      ? new Response(file, { headers: { "Content-Type": path.endsWith(".wasm") ? "application/wasm" : "text/javascript" } })
      : new Response(null, { status: 404 });
  },
});
try {
  const browser = await chromium.launch({ args: ["--js-flags=--experimental-wasm-jspi"] });
  try {
    const page = await browser.newPage();
    page.on("console", log => { if (log.type() === "error") console.error("[DEBUG]", log.text()); });
    await page.goto(server.url.href, { timeout: 15_000 });
    await page.exposeBinding("officeReceipt", (_, receipt) => stock.receive(admission, receipt));
    await page.exposeBinding("officeRefusal", (_, refusal) => stock.hold(admission, refusal));
    const documentId = "artifact-0123456789abcdef0123456789abcdef";
    const evaluation = page.evaluate(async ({ admission, kind, documentId, bridgeUrl, fresh, diagnostic }) => {
      const { createActorApi } = await import(bridgeUrl);
      const pending: Promise<unknown>[] = [];
      const host = window as Window & { officeReceipt: (value: unknown) => Promise<unknown>; officeRefusal: (value: unknown) => Promise<unknown> };
      const actor = await createActorApi("stdio-office", 1n, admission, () => BigInt(Math.floor(performance.now() * 1000)), {
        receipt: (value: unknown) => { pending.push(host.officeReceipt(value)); },
        hold: (value: unknown) => { pending.push(host.officeRefusal(value)); },
      });
      try {
        const descriptor = await actor.describe();

        const successful = [];
        if (fresh) {
          for (const artifactKind of [kind.schema, kind.id]) {
            const call = async (operation: string, input = {}) => {
              const request = { operation, artifactKind, ...input };
              const answer = await actor.codec(request);
              if ("fault" in answer) throw Error(JSON.stringify(request) + ": " + new TextDecoder().decode(answer.fault));
              return answer.ok;
            };
            const hash = await call("pack-schema-hash");
            const pair = await call("genesis", { documentId });
            const mirror = await call("print-mirror", { pair });
            successful.push({ hash: Array.from(hash), pack: Array.from(pair.pack), spr: Array.from(pair.spr), mirror });
          }
        }
        const verdicts = [];
        if (!fresh) {
          const sha256 = async bytes => Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)), byte => byte.toString(16).padStart(2, "0")).join("");
          for (const artifactKind of [kind.schema, kind.id]) {
            const summarize = (answer, result = null) => "fault" in answer
              ? { fault: new TextDecoder().decode(answer.fault), result: null }
              : { fault: null, result };
            const hash = await actor.codec({ operation: "pack-schema-hash", artifactKind });
            verdicts.push({ operation: "pack-schema-hash", artifactKind, ...summarize(hash, "ok" in hash ? Array.from(hash.ok) : null) });
            const genesis = await actor.codec({ operation: "genesis", artifactKind, documentId });
            const pair = "ok" in genesis ? genesis.ok : undefined;
            verdicts.push({ operation: "genesis", artifactKind, ...summarize(genesis, pair ? {
              pack: pair.pack.byteLength, spr: pair.spr.byteLength,
              packSha256: await sha256(pair.pack), sprSha256: await sha256(pair.spr),
            } : null) });
            if (diagnostic && pair) {
              const mirror = await actor.codec({ operation: "print-mirror", artifactKind, pair });
              verdicts.push({ operation: "print-mirror", artifactKind, ...summarize(mirror, "ok" in mirror ? {
                operations: mirror.ok[1].split("\n")[0],
                dslSha256: await sha256(new TextEncoder().encode(mirror.ok[0])),
                operationsSha256: await sha256(new TextEncoder().encode(mirror.ok[1])),
              } : null) });
            }
          }
        }
        const closure = await actor.closeOriginal();
        return { descriptor: Array.from(descriptor), verdicts, successful, closure };
      } finally {
        await Promise.all(pending);
      }
    }, { admission, kind, documentId, bridgeUrl: "/bridge.js", fresh, diagnostic });
    const signal = AbortSignal.timeout(30_000);
    const timeout = new Promise<never>((_, reject) => signal.addEventListener("abort", () => reject(signal.reason), { once: true }));
    const observed = await Promise.race([evaluation, timeout]);
    assert.deepEqual(Buffer.from(observed.descriptor), Buffer.from(await Bun.file(join(emitted, "🛂️.descriptor.semio")).arrayBuffer()));
    if (fresh) {
      const definitionPath = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📜️artifact-definition.json";
      const definition = await Bun.file(join(root, definitionPath)).json();
      const factory = definition.codecs.find((row: { native_factory?: { artifact_schema: string } }) =>
        row.native_factory?.artifact_schema === kind.schema)?.native_factory;
      assert.ok(factory);
      assert.equal(factory.artifact_kind, kind.id);
      assert.equal(observed.successful.length, 2);
      const [schema, alias] = observed.successful;
      assert.equal(Buffer.from(schema.hash).toString("hex"), factory.pack_schema_hash);
      assert.deepEqual(alias, schema, "The kind and schema must produce byte-identical results");
      assert.ok(schema.pack.length > 0 && schema.spr.length > 0, "Genesis must return both payloads");
      assert.ok(Array.isArray(schema.mirror) && schema.mirror.length === 2);
      const [, operations] = schema.mirror;
      assert.ok(operations.includes(documentId) && operations.includes(kind.schema), operations);
    } else if (diagnostic) {
      assert.equal(observed.verdicts.filter(row => row.operation !== "print-mirror").length, 4);
      for (const verdict of observed.verdicts) console.log(JSON.stringify(verdict));
      for (const operation of ["genesis", "print-mirror"]) {
        const answers = observed.verdicts.filter(row => row.operation === operation && row.fault === null);
        if (answers.length === 2) assert.deepEqual(answers[0].result, answers[1].result);
      }
    } else {
      assert.equal(observed.verdicts.length, 4);
      for (const { operation, artifactKind, fault } of observed.verdicts) {
        assert.ok(fault, `Expected the captured guest to refuse ${operation} for ${artifactKind}`);
        const { code, origin, message } = JSON.parse(fault);
        assert.deepEqual({ code, origin, message }, {
          code: "plugin.internal",
          origin: "plugin",
          message: "artifact codec schema resolves more than one app of the same role",
        });
      }
    }
    assert.deepEqual(observed.closure, { operation: admission.operation, generation: admission.generation, sourceClosed: true });
    stock.returnClosed(admission, observed.closure.sourceClosed);
    assert.equal(stock.liveSources, 0);
    console.log(diagnostic ? "Chromium returned the original source after the Office codec diagnostic" : fresh
      ? "Chromium Office actor validated the native XLSX schema hash, minted canonical Pack/SPR, and printed the same identified mirror by schema and kind"
      : "Chromium Office actor returned the XLSX codec ownership fault for pack-schema-hash and genesis, by both schema and kind");
  } finally {
    await browser.close();
  }
} finally {
  server.stop(true);
}

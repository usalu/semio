import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { createRequire } from "node:module";
import { createServer, type Server } from "node:http";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, extname, join, normalize, relative, resolve, sep } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🧫️cases.json" with { type: "json" };
import { installPublishedPageFetch, publishedPageUrl, relocatePublishedRequestUrl, runtimeBasePath } from "../../../../🧰️framework/🔨️modules/🖼️assets/🌐️published-page/🟦️.ts";
import { shardWorkerUrl } from "../../../../🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import { demonstratorFooterItems } from "../../🪧️brand.ts";
import { packageAssetUrl } from "../../../../🧰️framework/🔨️modules/🎠️kernel/📦️package-assets/🟦️.ts";

const { URL: OracleURL }: { URL: typeof URL } = createRequire(import.meta.url)("whatwg-url");
type Scope = { document?: { baseURI: string }; fetch?: unknown; location?: { origin: string } };
const scope = globalThis as unknown as Scope;
const withDocument = <T>(baseURI: string, run: () => T): T => {
  const previous = scope.document;
  scope.document = { baseURI };
  try { return run(); } finally { if (previous === undefined) delete scope.document; else scope.document = previous; }
};
const types: Record<string, string> = { ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm", ".glb": "model/gltf-binary", ".pbf": "application/x-protobuf", ".png": "image/png", ".jpg": "image/jpeg", ".woff2": "font/woff2", ".3dm": "application/octet-stream" };

function serveBelowMount(directory: string, mount: string): Server {
  return createServer((request, response) => {
    const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://host").pathname);
    const file = pathname.startsWith(mount) ? resolve(directory, normalize(pathname.slice(mount.length))) : undefined;
    if (file === undefined || relative(directory, file).startsWith("..") || !existsSync(file) || !statSync(file).isFile()) { response.writeHead(404).end(); return; }
    response.writeHead(200, { "content-type": types[extname(file)] ?? "application/octet-stream" }).end(readFileSync(file));
  });
}

describe("demonstrator bundle relocatable below any sub-path", () => {
  let root = "", server: Server, origin = "";
  beforeAll(async () => {
    root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "relocatable-"));
    const site = process.env.SEMIO_DEMONSTRATOR_DIST ?? join(root, "site");
    if (process.env.SEMIO_DEMONSTRATOR_DIST === undefined) {
      writeFileSync(join(root, "index.html"), "");
      for (const file of ["/index.html", ...corpus.files]) {
        const target = join(site, ...file.split("/"));
        mkdirSync(dirname(target), { recursive: true });
        writeFileSync(target, file);
      }
    }
    server = serveBelowMount(site, corpus.mount);
    await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
    origin = `http://127.0.0.1:${(server.address() as { port: number }).port}`;
  });
  afterAll(() => { server.close(); rmSync(root, { recursive: true, force: true }); });

  test("the corpus matches its schema", () => {
    const check = new Ajv({ strict: true }).compile(schema);
    expect(check(corpus), JSON.stringify(check.errors)).toBe(true);
  });

  for (const row of corpus.cases) test(`resolves ${row.id}`, () => {
    withDocument(row.documentBase, () => expect(publishedPageUrl(row.input, {})).toBe(row.output));
    withDocument(row.documentBase, () => expect(relocatePublishedRequestUrl(row.input, corpus.origin, {})).toBe(row.output === row.input ? row.input : row.output));
    if (!row.oracle) return;
    const expected = new OracleURL("." + row.input, row.documentBase);
    expect(decodeURI(`${expected.pathname}${expected.search}${expected.hash}`)).toBe(decodeURI(row.output));
  });

  for (const row of corpus.workerBases) test(`derives the worker base ${row.id}`, () => {
    expect(runtimeBasePath(undefined, row.moduleUrl, row.production)).toBe(row.base);
  });

  test("a same-origin absolute request lands below the mount", () => {
    withDocument(corpus.cases[0]!.documentBase, () => expect(relocatePublishedRequestUrl(`${corpus.origin}/mesh/a.glb?x=1`, corpus.origin, {})).toBe("/sub/mesh/a.glb?x=1"));
  });

  test("framework owners resolve below the runtime base route", () => {
    withDocument(corpus.owners.documentBase, () => {
      expect(shardWorkerUrl()).toBe(corpus.owners.shardWorker);
      expect(JSON.stringify(demonstratorFooterItems("en"))).toContain(`"src":"${corpus.owners.brandLogo}"`);
      expect(packageAssetUrl(corpus.owners.packageAsset.name, corpus.owners.documentBase)).toBe(corpus.owners.packageAsset.url);
    });
  });

  test("the installed fetch relocates every runtime route below the mount", async () => {
    const requests: unknown[] = [], previous = { fetch: scope.fetch, location: scope.location };
    const holder = globalThis as unknown as Record<symbol, unknown>, key = Symbol.for("semio.publishedPageFetch"), installed = holder[key];
    delete holder[key];
    scope.fetch = async (input: unknown) => { requests.push(input); return new Response(); };
    scope.location = { origin: corpus.origin };
    try {
      scope.document = { baseURI: corpus.cases[0]!.documentBase };
      installPublishedPageFetch();
      const patched = scope.fetch as (input: string) => Promise<Response>;
      for (const row of corpus.cases) if (row.serve) { await patched(row.input); expect(requests.at(-1)).toBe(row.output); }
    } finally { delete scope.document; scope.fetch = previous.fetch; scope.location = previous.location; delete holder[key]; if (installed !== undefined) holder[key] = installed; }
  });

  test("every runtime route is served below the mount and nowhere else", async () => {
    const documentBase = `${origin}${corpus.mount}index.html`;
    expect((await fetch(documentBase)).status).toBe(200);
    for (const row of corpus.cases.filter(row => row.serve)) {
      const url = withDocument(documentBase, () => new URL(publishedPageUrl(row.input, {}), documentBase));
      expect(url.pathname.startsWith(corpus.mount), row.id).toBe(true);
      const found = await fetch(url);
      expect(found.status, `${row.id} ${url.pathname}`).toBe(200);
      expect(await found.text(), row.id).toBe(row.input);
      expect((await fetch(new URL(row.input, origin))).status, `${row.id} unrelocated`).toBe(404);
    }
  });
});

const assert = require("node:assert/strict");
const { once } = require("node:events");
const { mkdirSync, mkdtempSync, readFileSync, rmSync } = require("node:fs");
const { createServer } = require("node:http");
const { resolve, join } = require("node:path");
const ts = require("typescript");
const toml = require("@iarna/toml");
const fixture = require("./🧪️native-tile-launch.json");

const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const registry = os + "/🔌️plugin/📇️registry";
const runner = os + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🟦️.ts";
const { createAssetHttpServerV1 } = require(resolve("🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts"));
const { PLAYGROUND_ASSET_PROVIDERS_V1 } = require(resolve(registry + "/🎮️playground/🖼️assets/🧩️composition/🟦️.ts"));
const manifest = "🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml";
const { assets, playground } = toml.parse(readFileSync(manifest, "utf8")).package.metadata.semio;
const gis = playground.find(row => row.variant === fixture.variant);
assert.equal(gis.app, fixture.app);
const selected = assets.filter(row => row.app === gis.app);
assert.deepEqual(selected.map(({route, upstream}) => ({route, upstream})), fixture.routes.map(({route, upstream}) => ({route, upstream})));

const source = ts.createSourceFile(runner, readFileSync(runner, "utf8"), ts.ScriptTarget.ESNext, true);
const names = ["runNativeSession", "runRunScript", "selection"];
const declarations = names.map(name => {
  const declaration = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === name);
  assert(declaration, name);
  return declaration.getText(source).replace(/^export /, "");
});
const runtime = ts.transpileModule(declarations.join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const generated = join(__dirname, "🗑️generated");
mkdirSync(generated, { recursive: true });
const root = mkdtempSync(join(generated, "native-loopback-"));
const calls = [];
let origin;
const upstream = createServer((request, response) => {
  calls.push(request.url);
  assert.equal(request.headers["user-agent"], selected[0].userAgent);
  if (request.url === "/planet") {
    response.end(JSON.stringify({ tiles: [origin + "/vector/{z}/{x}/{y}.pbf"] }));
  } else {
    response.end(request.url);
  }
});
const close = server => new Promise((fulfill, reject) => {
  server.close(error => error ? reject(error) : fulfill());
  server.closeAllConnections();
});

async function main() {
  try {
    upstream.listen(0, "127.0.0.1");
    await once(upstream, "listening");
    origin = "http://127.0.0.1:" + upstream.address().port;
    const row = { ...gis, assets: selected.map(asset => ({ ...asset, upstream: asset.route === "/osm" ? origin + "/raster/{z}/{x}/{y}.png" : origin + "/planet" })) };
    const phase = new AbortController();
    const control = { signal: phase.signal };
    let local;
    const ports = {
      readNativeCallerAuthorityV1: () => ({ original: { buildMode: "prototype" } }),
      withPlaygroundSessionV1: (_repo, _invocation, _variant, run) => run(null, { playgrounds: [row] }, control),
      withNativeRuntimeV1: (_repo, _invocation, _variant, _control, run) => run("native-runtime", null, control),
      createRequestedIssuedResourceControlV1: () => control,
      nativeLaunchDeclaration: {},
      readNativeLaunchDeclarationV1: value => value,
      selectNativeLaunchDeclarationV1: value => value,
      nativeLaunchEnvironmentV1: () => ({ SEMIO_ASSET_BASE_URL: "http://incorrect" }),
      playgroundNativeHostArtifactV1: () => "native-host",
      existsSync: () => true,
      PLAYGROUND_ASSET_PROVIDERS_V1,
      requireIssuedResourceControlInvocationV1: () => {},
      checkResourceLeaseControl: () => 3000,
      once,
      createAssetHttpServerV1: (workspace, port, assets, providers) => {
        local = createAssetHttpServerV1(workspace, port, assets, providers);
        assert(!local.listening);
        return local;
      },
      runNativeBinary: async (_exe, _args, env) => {
        assert(local.listening);
        assert.equal(local.address().address, "127.0.0.1");
        assert.equal(env.SEMIO_ASSET_BASE_URL, "http://127.0.0.1:" + local.address().port);
        for (const { request, reply } of fixture.routes) {
          const response = await fetch(env.SEMIO_ASSET_BASE_URL + request);
          assert.equal(response.status, 200);
          assert.equal(await response.text(), reply);
        }
      },
    };
    const run = new Function(...Object.keys(ports), runtime + ";return runRunScript;")(...Object.values(ports));
    await run({ repoRoot: root, invocation: { control, capabilities: {} } }, [fixture.variant, "prototype"]);
    assert(!local.listening);
    assert.deepEqual(calls, fixture.upstreamCalls);
    console.log("Native launcher waited for loopback, forwarded the raster and vector routes, and closed the server");
  } finally {
    await close(upstream);
    rmSync(root, { recursive: true, force: true });
  }
}

main().catch(error => { console.error(error); process.exitCode = 1; });

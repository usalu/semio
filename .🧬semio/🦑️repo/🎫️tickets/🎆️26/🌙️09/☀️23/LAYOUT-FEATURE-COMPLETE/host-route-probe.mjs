import assert from "node:assert/strict";
const root = new URL(process.argv[2]);
assert.ok(["http:", "https:"].includes(root.protocol));
const seen = new Set(), assets = [];
async function get(url) {
  const response = await fetch(url, { redirect: "error", signal: AbortSignal.timeout(15000) });
  assert.equal(response.status, 200, `${url}: HTTP ${response.status}`);
  const bytes = new Uint8Array(await response.arrayBuffer());
  assert.ok(bytes.length, `${url}: empty response`);
  assets.push({ url: String(url), bytes: bytes.length, type: response.headers.get("content-type") });
  return { response, bytes };
}
const { response, bytes } = await get(root);
assert.equal(response.headers.get("cross-origin-opener-policy")?.toLowerCase(), "same-origin", "COOP");
const coep = response.headers.get("cross-origin-embedder-policy")?.toLowerCase();
assert.ok(["require-corp", "credentialless"].includes(coep), "COEP");
const html = new TextDecoder().decode(bytes);
const tag = html.match(/<script\b(?=[^>]*\bid=["']semio-plugin-catalog["'])[^>]*>([\s\S]*?)<\/script>/i);
assert.ok(tag, "injected plugin catalog absent");
const catalog = JSON.parse(tag[1]), matches = [];
function visit(value) {
  if (!value || typeof value !== "object") return;
  if (value.pluginId === "layout" && typeof value.moduleUrl === "string") matches.push(value);
  for (const child of Object.values(value)) visit(child);
}
visit(catalog);
assert.equal(matches.length, 1, "expected exactly one Layout catalog entry with moduleUrl");
const bridge = new URL(matches[0].moduleUrl, root);
assert.equal(bridge.origin, root.origin, "Layout bridge is not same origin");
async function crawl(url, depth = 0) {
  if (seen.has(url.href) || depth > 8) return;
  seen.add(url.href);
  assert.ok(seen.size <= 128, "guest module graph exceeds probe limit");
  const { bytes } = await get(url);
  if (url.pathname.endsWith(".wasm")) {
    assert.deepEqual([...bytes.slice(0, 4)], [0, 97, 115, 109], `${url}: invalid wasm magic`);
    return;
  }
  const text = new TextDecoder().decode(bytes);
  assert.ok(!/^\s*<!doctype html/i.test(text), `${url}: HTML fallback`);
  const patterns = [/new URL\(\s*["']([^"']+\.(?:m?js|wasm))["']\s*,\s*import\.meta\.url/g, /(?:from\s*|import\s*\(|import\s*)["']([^"']+\.(?:m?js|wasm))["']/g];
  for (const pattern of patterns) for (const match of text.matchAll(pattern)) {
    if (!match[1].startsWith("./") && !match[1].startsWith("../")) continue;
    const child = new URL(match[1], url);
    assert.equal(child.origin, root.origin);
    await crawl(child, depth + 1);
  }
}
await crawl(bridge);
assert.ok([...seen].some(url => new URL(url).pathname.endsWith(".wasm")), "no statically discoverable guest wasm");
for (const path of ["/renderer-modules/wgpu/semio-renderer-wgpu-component.js", "/renderer-modules/wgpu/semio-renderer-wgpu-component_bg.wasm"]) {
  const { bytes } = await get(new URL(path, root));
  if (path.endsWith(".wasm")) assert.deepEqual([...bytes.slice(0, 4)], [0, 97, 115, 109], "renderer wasm magic");
  else assert.ok(!/^\s*<!doctype html/i.test(new TextDecoder().decode(bytes)), "renderer JS HTML fallback");
}
console.log(JSON.stringify({ kind: "live-host-route-probe", layout: matches[0].pluginId, moduleUrl: bridge.href, isolation: { coop: "same-origin", coep }, assets }, null, 2));

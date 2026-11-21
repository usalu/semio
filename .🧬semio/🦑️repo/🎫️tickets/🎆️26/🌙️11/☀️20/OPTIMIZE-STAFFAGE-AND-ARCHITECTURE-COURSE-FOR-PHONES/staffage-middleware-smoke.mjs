import assert from "node:assert/strict";

const origin = "http://127.0.0.1:6271";
let serve;
assert.ok(globalThis.routeTicketBrowser);
await globalThis.routeTicketBrowser({
  route: async (_pattern, handler) => { serve = handler; },
}, origin);
assert.ok(serve);

async function load(path) {
  let reply;
  await serve({
    request: () => ({
      url: () => new URL(path, origin).href,
      method: () => "GET",
      headers: () => ({ accept: path === "/" ? "text/html" : "*/*" }),
      postDataBuffer: () => null,
    }),
    fulfill: async value => { reply = value; },
    abort: async () => { throw Error(`Aborted ${path}`); },
  });
  assert.ok(reply, path);
  const body = reply.body.toString();
  console.log("[DEBUG] Staffage middleware", reply.status, decodeURIComponent(path), body.length);
  assert.equal(reply.status, 200, body.slice(0, 1200));
  return body;
}

const html = await load("/");
assert.match(html, /id="root"/u);
const index = /<script type="module" src="([^"]+)">/gu;
for (const match of html.matchAll(index)) await load(match[1]);
const entry = await load("/🟦️.ts");
const app = /from\s+"([^"]+\/📚(?:️|%EF%B8%8F)?app\/[^"]+)"/u.exec(decodeURIComponent(entry));
if (app !== null) {
  const source = await load(app[1]);
  const react = /"([^"]+\/vite-\d+\/deps\/react(?:\.js|_jsx[^"?]*\.js)[^"]*)"/gu;
  const imports = [...new Set([...source.matchAll(react)].map(match => match[1]))];
  for (const dependency of imports) await load(dependency);
}
await load("/🎨️.css?direct");
console.log("[DEBUG] Staffage entry and styles served through isolated middleware");

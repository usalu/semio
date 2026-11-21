import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import type { BrowserContext, Route } from "playwright";

const origin = `http://127.0.0.1:${process.env.COURSE_PHONE_BROWSER_PORT ?? "6272"}`;
let handleRoute: ((route: Route) => Promise<void>) | undefined;
const context = {
  route: async (_url: string, handler: (request: Route) => Promise<void>) => { handleRoute = handler; },
} as Pick<BrowserContext, "route">;
assert.ok(globalThis.routeTicketBrowser);
await globalThis.routeTicketBrowser(context, origin);

async function load(path: string): Promise<{ status: number; body: Buffer }> {
  assert.ok(handleRoute);
  let reply: { status: number; body: Buffer } | undefined;
  await handleRoute({
    request: () => ({
      url: () => new URL(path, origin).href,
      method: () => "GET",
      headers: () => ({ accept: path === "/" ? "text/html" : "*/*" }),
      postDataBuffer: () => null,
    }),
    fulfill: async (value: typeof reply) => { reply = value; },
    abort: async () => { throw Error(`Aborted ${path}`); },
  } as unknown as Route);
  assert.ok(reply, `No response for ${path}`);
  return reply;
}

const html = await load("/");
assert.equal(html.status, 200);
assert.match(html.body.toString(), /id="root"/u);
assert.match(html.body.toString(), /<meta name="viewport"/u);
const script = await load("/🟦️.ts");
assert.equal(script.status, 200);
assert.match(script.body.toString(), /mountCourse/u);
const styles = await load("/🎨️.css?direct");
assert.equal(styles.status, 200);
assert.ok(styles.body.includes(Buffer.from("@layer utilities")), "course design system stylesheet");
const reactStyles = fileURLToPath(new URL("../../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️.css", import.meta.url));
const shellStyles = await load(`/@fs${reactStyles}?direct`);
assert.equal(shellStyles.status, 200);
assert.ok(shellStyles.body.includes(Buffer.from(".course-page-picker")), "native page picker stylesheet");
assert.ok(shellStyles.body.includes(Buffer.from(".course-home-entry")), "touch deck stylesheet");
const devClient = await load("/@vite/client");
assert.equal(devClient.status, 200);
assert.ok(devClient.body.includes(Buffer.from("updateStyle")), "stylesheet updates from the Vite development client");
assert.ok(!devClient.body.includes(Buffer.from("transport.connect(createHMRHandler(handleMessage));")), "no HMR connection through synthetic HTTP");
console.info("[DEBUG] Course HTML, TypeScript entry, phone layout CSS, and HMR-free Vite client served through the isolated HTTP adapter");

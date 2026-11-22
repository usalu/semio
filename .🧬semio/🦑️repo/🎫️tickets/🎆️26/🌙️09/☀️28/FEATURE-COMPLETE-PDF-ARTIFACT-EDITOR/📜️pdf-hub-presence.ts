/** 👥️ Two signed-in Browser actors edit one PDF through the authenticated Hub: each sees the other on the page thumbnail strip while it is present and not after it left. Run with `PDF_HUB_SCRIPT=📜️pdf-hub-presence.ts bun 📜️pdf-hub-runtime.ts <out> <png>`. */
import assert from "node:assert/strict";
import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { clickUncovered, unfoldActionsRail } from "@semio-tech/framework-os-dev/testing/program-matrix";

const [hub, serve, directory, , peerServe = serve] = process.argv.slice(2);
assert(hub && serve && directory, "Expected the local Hub, React Studio and output directory");
for (const endpoint of [hub, serve, peerServe]) assert(["127.0.0.1", "localhost", "[::1]"].includes(new URL(endpoint).hostname), "Use the isolated local test services");
const output = resolve(directory), scratch = join(output, "browser-temp");
mkdirSync(scratch, { recursive: true });
const repo = process.cwd();
const collaboration = resolve(repo, "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration");
const browserOwner = resolve(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const [{ ReactShell }, { hubCollaborationHumans }, { PLAYWRIGHT_MODULE_SPECIFIER }, { ensureRepositoryBrowserPathV1 }] = await Promise.all([
  import(join(collaboration, "🖥️shells/🟦️.ts")),
  import(join(collaboration, "📋️contract/🟦️.ts")),
  import(join(browserOwner, "🟦️.ts")),
  import(join(browserOwner, "🧰️preparation/🟦️.ts")),
]);
const humans = hubCollaborationHumans();
const name = "PDF presence " + crypto.randomUUID();
const [author, reader] = await Promise.all(humans.map(human => hubProbeSignIn(hub, human.email, human.password, "pdf-presence")));
assert(author !== reader, "Expected distinct authenticated sessions");
const space = await hubProbeCreateSpace(hub, author, name);
const invite = sealDirectoryCommandRequestV1(crypto.randomUUID().replaceAll("-", ""), { kind: "upsert-member", spaceId: space, email: humans[1].email, role: "author" });
assert([200, 202].includes((await hubProbeCall(hub, "POST", "/directory/commands", author, directoryCommandRequestJson(invite))).status), "Could not seat the second author");
const catalog = await hubProbeCreationCatalog(hub, author, space);
const kind = catalog.kinds.find(row => row.schema === "stdio.pdf.1.7");
assert(kind, "Hub creation catalog lacks the base PDF 1.7 editor");
const { artifactId } = await hubProbeCreateArtifact(hub, author, space, catalog.generationId, kind.kindId, name);

ensureRepositoryBrowserPathV1(repo, process.env);
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const args = process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [];
const browser = await chromium.launch({ args, headless: true, downloadsPath: output, env: { ...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch } });
const rail = '[data-slot="window-action-pane"]';
const here = /(here|hier): /u;
try {
  const [a, b] = await Promise.all(humans.map((human, index) => ReactShell.boot(browser, human, index === 0 ? serve : peerServe, "en", () => Date.now())));
  await Promise.all([a.signIn(), b.signIn()]);
  await Promise.all([a.open(space, artifactId, reading => !reading.opening), b.open(space, artifactId, reading => !reading.opening)]);
  await unfoldActionsRail(a.page);
  for (const page of [0, 1]) {
    await clickUncovered(a.page, rail + ' [id="action.insert-page"]');
    for (const [field, value] of Object.entries({ page, width: 612, height: 792 })) {
      const input = a.page.locator(rail + ' [id="action.insert-page.arg.' + field + '"]').locator("input").first();
      await input.fill(String(value));
      await input.blur();
    }
    await a.page.locator(rail).getByRole("button", { name: "Execute", exact: true }).click();
  }
  const second = '[data-ui-node-key="pdf-thumbnail-1"]';
  await Promise.all([a, b].map(shell => shell.page.locator(second).waitFor({ timeout: 60_000 })));
  assert.equal(await b.page.locator(second).filter({ hasText: here }).count(), 0, "Nobody is on page 2 yet");
  await a.page.locator(second).click();
  await b.page.locator(second).filter({ hasText: here }).waitFor({ timeout: 30_000 });
  await a.page.close();
  await b.page.locator(second).filter({ hasText: here }).waitFor({ state: "detached", timeout: 60_000 });
} finally {
  await browser.close();
}

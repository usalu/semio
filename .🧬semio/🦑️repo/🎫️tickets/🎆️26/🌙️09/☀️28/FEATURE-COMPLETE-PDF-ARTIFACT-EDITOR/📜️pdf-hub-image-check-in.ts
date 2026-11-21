/** 🛰️ Exercises one original PNG through two signed-in Browser actors and a reopened Hub checkpoint. */
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Locator, Page } from "playwright";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { clickUncovered, unfoldActionsRail } from "@semio-tech/framework-os-dev/testing/program-matrix";

const [hub, serve, directory, pngPath, peerServe = serve] = process.argv.slice(2);
assert(hub && serve && directory && pngPath, "Expected the local Hub, React Studio, output directory and original PNG");
for (const endpoint of [hub,serve,peerServe]) assert(["127.0.0.1","localhost","[::1]"].includes(new URL(endpoint).hostname), "Use the isolated local test services");
const output = resolve(directory), scratch = join(output,"browser-temp");
mkdirSync(scratch,{recursive:true});
const repo = process.cwd();
const collaboration = resolve(repo,"🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration");
const browserOwner = resolve(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const [{ReactShell}, {hubCollaborationHumans}, {collabOpenCheckin,collabCurrentCheckpointId,collabAwaitCheckinStatus}, {PLAYWRIGHT_MODULE_SPECIFIER}, {ensureRepositoryBrowserPathV1}] = await Promise.all([
  import(join(collaboration,"🖥️shells/🟦️.ts")),
  import(join(collaboration,"📋️contract/🟦️.ts")),
  import(resolve(repo,"🌎️hub/🧪️tests/🤝️dev-collaboration/🖱️ui-driving-and-row-waits/🟦️.ts")),
  import(join(browserOwner,"🟦️.ts")),
  import(join(browserOwner,"🧰️preparation/🟦️.ts")),
]);
const humans = hubCollaborationHumans();
const fixture = JSON.parse(readFileSync(new URL("./image-authoring.json",import.meta.url),"utf8"));
const ticketName = "PDF original media " + crypto.randomUUID();
const [author,reader] = await Promise.all(humans.map(human => hubProbeSignIn(hub,human.email,human.password,"pdf-original-media")));
assert(author !== reader,"Expected distinct authenticated sessions");
const space = await hubProbeCreateSpace(hub,author,ticketName);
const invite = sealDirectoryCommandRequestV1(crypto.randomUUID().replaceAll("-",""),{kind:"upsert-member",spaceId:space,email:humans[1].email,role:"author"});
const seated = await hubProbeCall(hub,"POST","/directory/commands",author,directoryCommandRequestJson(invite));
assert([200,202].includes(seated.status),"Could not seat the second author");
const catalog = await hubProbeCreationCatalog(hub,author,space);
const kind = catalog.kinds.find(row=>row.schema==="stdio.pdf.1.7");
assert(kind,"Hub creation catalog lacks the base PDF 1.7 editor");
const {artifactId} = await hubProbeCreateArtifact(hub,author,space,catalog.generationId,kind.kindId,ticketName);

ensureRepositoryBrowserPathV1(repo,process.env);
const {chromium}: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const args = process.platform === "darwin" ? ["--in-process-gpu","--disable-accelerated-2d-canvas","--use-angle=swiftshader"] : [];
const browser = await chromium.launch({args,headless:true,downloadsPath:output,env:{...process.env,TMPDIR:scratch,TMP:scratch,TEMP:scratch}});
const rail = '[data-slot="window-action-pane"]';
const row = (id:string) => rail + ' [id="action.' + id + '"]';
const images = (page:Page) => page.locator('[data-ui-node-key^="pdf-text-layer-p0-"]').and(page.getByRole("button",{name:/^Image:/u}));
async function action(page:Page,id:string) {
  await unfoldActionsRail(page);
  await clickUncovered(page,row(id));
}
async function fill(page:Page,id:string,values:Record<string,number>) {
  for (const [field,value] of Object.entries(values)) {
    const input = page.locator(row(id+".arg."+field)).locator("input").first();
    await input.fill(String(value));
    await input.blur();
  }
}
const execute = (page:Page):Locator => page.locator(rail).getByRole("button",{name:"Execute",exact:true});
try {
  const clock = () => Date.now();
  const [a,b] = await Promise.all(humans.map((human,index)=>ReactShell.boot(browser,human,index===0?serve:peerServe,"en",clock)));
  await Promise.all([a.signIn(),b.signIn()]);
  const packets: Buffer[] = [];
  b.page.on("websocket", socket => {
    if (!new URL(socket.url()).pathname.endsWith("/document/ws")) return;
    socket.on("framereceived", frame => {
      const bytes = frame.payload;
      if (Buffer.isBuffer(bytes) && bytes[0] === 0 && bytes[1] === 16) packets.push(Buffer.from(bytes));
    });
  });
  const opened = await Promise.all([a.open(space,artifactId,reading=>!reading.opening),b.open(space,artifactId,reading=>!reading.opening)]);
  assert(opened.every(row=>row.liveAfterMs!==null),"Both separate clients must mount the same live PDF");

  const pageLayer = '[data-ui-node-key^="pdf-text-layer-page-"]';
  await action(a.page,"insert-page");
  await fill(a.page,"insert-page",{page:0,width:612,height:792});
  await execute(a.page).click();
  await Promise.all([a.page.locator(pageLayer).first().waitFor({timeout:60_000}),b.page.locator(pageLayer).first().waitFor({timeout:60_000})]);

  await action(a.page,"insert-image");
  const matrix: number[] = fixture.existing.insertPlacement;
  await fill(a.page,"insert-image",{page:0,x:matrix[4],y:matrix[5],width:matrix[0],height:matrix[3]});
  const [picker] = await Promise.all([a.page.waitForEvent("filechooser",{timeout:60_000}),execute(a.page).click()]);
  await picker.setFiles(resolve(pngPath));
  await Promise.all([images(a.page).first().waitFor({timeout:90_000}),images(b.page).first().waitFor({timeout:90_000})]);
  assert.equal(await images(b.page).count(),1,"Peer applied the complete image exactly once");
  const deliveredParts = packets.length;
  assert(deliveredParts > 1,"Peer received a paged original-media commit");

  const checkIn = await collabOpenCheckin(a.page);
  const before = await collabCurrentCheckpointId(a.page);
  await checkIn.click();
  await a.page.locator('[id="s-checkin-message"]').fill(ticketName);
  await a.page.locator('[id="s-checkin-message"]').press("Enter");
  const status = await collabAwaitCheckinStatus(a.page,90_000);
  const checkpoint = await collabCurrentCheckpointId(a.page);
  assert(checkpoint && checkpoint !== before,"Hub Check In did not advance the checkpoint: " + status);

  await b.page.goto(peerServe,{waitUntil:"domcontentloaded"});
  const reopened = await b.open(space,artifactId,reading=>!reading.opening);
  assert.notEqual(reopened.liveAfterMs,null,"The peer did not reopen the checked-in PDF");
  await images(b.page).first().waitFor({timeout:90_000});
  assert.equal(await images(b.page).count(),1,"Reopened document retained one image");
  const [download] = await Promise.all([b.page.waitForEvent("download",{timeout:60_000}),action(b.page,"saveArtifactFile")]);
  await download.saveAs(join(output,"hub-reopened.pdf"));
  assert.equal(await download.failure(),null);
  writeFileSync(join(output,"hub-original-media.json"),JSON.stringify({space,artifactId,checkpoint,status,parts:deliveredParts},null,2));
  console.log("[DEBUG] Two authenticated users reopened one checked-in original PNG");
} finally {
  await browser.close();
}

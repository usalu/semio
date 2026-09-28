/** ⏱️ SH2 probe (hub only, no shell): create + delete one space through `/directory/commands` and measure when
 * `space.deleted` appears in the delete reply and in `/directory/event-page/v1`. usage: zsh sh2-hub-env.sh bun sh2-probe-hub-delete.ts <hub> */
import { randomBytes } from "node:crypto";
import { hubProbeCall, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const [hub = "http://127.0.0.1:7800"] = process.argv.slice(2);
const token = await hubProbeSignIn(hub, process.env.OS_HUB_PROBE_EMAIL!, process.env.OS_HUB_PROBE_PASSWORD!, "sh2deleteprobe");
const command = (body: Parameters<typeof sealDirectoryCommandRequestV1>[1]) => hubProbeCall(hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), body)));
const kinds = (answer: { json: any }): string[] => (answer.json?.events ?? []).map((event: any) => `${event?.seq}:${event?.body?.kind}`);
const created = await command(createSpaceCommandV1(`SH2 hub delete ${Date.now().toString(36)}`, "studio", "private"));
const spaceId = created.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
console.log("create", created.status, kinds(created), spaceId);
const t0 = Date.now();
const deleted = await command({ kind: "delete-space", spaceId });
console.log("delete", deleted.status, Date.now() - t0, "ms", kinds(deleted), deleted.text.slice(0, 300));
const after = Math.max(0, ...((created.json?.events ?? []).map((event: any) => Number(event?.seq ?? 0)))) - 1;
for (let attempt = 0; attempt < 30; attempt += 1) {
  const page = await hubProbeCall(hub, "GET", `/directory/event-page/v1?after=${after}`, token);
  const events = (page.json?.events ?? []).filter((event: any) => event?.spaceId === spaceId || event?.body?.spaceId === spaceId).map((event: any) => `${event?.seq}:${event?.body?.kind}`);
  console.log(`page +${Date.now() - t0} ms`, page.status, events.join(" "));
  if (events.some((line: string) => line.endsWith("space.deleted"))) break;
  await new Promise((resolveDelay) => setTimeout(resolveDelay, 10_000));
}

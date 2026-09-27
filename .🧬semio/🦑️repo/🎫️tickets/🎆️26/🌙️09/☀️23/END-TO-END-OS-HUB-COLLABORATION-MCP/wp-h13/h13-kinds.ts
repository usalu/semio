/** 🗂️ H13 one-off: prints the creation catalog kinds (kindId → schema, plugin) of a hub. usage: bun h13-kinds.ts <origin> (env C12_USER1_PASSWORD) */
import { hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const origin = process.argv[2]!;
const token = await hubProbeSignIn(origin, "user1@semio.dev", process.env.C12_USER1_PASSWORD!, "h13-kinds");
const spaceId = await hubProbeCreateSpace(origin, token, `H13 kinds ${Date.now()}`);
const catalog = await hubProbeCreationCatalog(origin, token, spaceId);
for (const kind of catalog.kinds as any[]) console.log(JSON.stringify(kind).slice(0, 300));

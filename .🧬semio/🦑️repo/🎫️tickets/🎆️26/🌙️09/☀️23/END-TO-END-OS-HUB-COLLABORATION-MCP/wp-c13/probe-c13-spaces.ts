/** 🔎️ C13: does the hub list a space by name for user1 (credentials via env SEMIO_TWO_HUMAN_USER1_EMAIL/PASSWORD)?
 * usage: bun probe-c13-spaces.ts <hubOrigin> <namePrefix> */
import { hubProbeCall, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const [origin, prefix] = process.argv.slice(2) as [string, string];
const token = await hubProbeSignIn(origin, process.env.SEMIO_TWO_HUMAN_USER1_EMAIL!, process.env.SEMIO_TWO_HUMAN_USER1_PASSWORD!, "c13spaces");
const started = Date.now();
const answer = await hubProbeCall(origin, "GET", "/directory/spaces", token);
const rows = (Array.isArray(answer.json) ? answer.json : answer.json?.spaces ?? []) as { access: string; space: { id: string; name: string; updatedAtMs: number } }[];
console.log(`HTTP ${answer.status} in ${Date.now() - started} ms, ${rows.length} spaces`);
for (const row of rows.filter((entry) => entry.space.name.startsWith(prefix))) console.log(`${row.access} ${row.space.id} ${row.space.name} ${new Date(row.space.updatedAtMs).toISOString()}`);

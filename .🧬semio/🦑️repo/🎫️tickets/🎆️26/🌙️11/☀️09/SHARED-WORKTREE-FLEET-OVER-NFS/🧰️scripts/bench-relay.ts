#!/usr/bin/env bun
// usage: bench-relay.ts REPLICA_TS SCRATCH_DIR ORIGINS EVENTS_EACH BLOB_KB -- builds a peer store with ORIGINS origins of EVENTS_EACH events (one blob each) and times one relayOnceV1 into an empty store.
import {mkdirSync,mkdtempSync,readdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const [,, replica, scratch, origins, events, sizeKb] = process.argv;
const R = await import(replica!);
const peer = mkdtempSync(join(scratch!, "peer-"));
const module = join(replica!, "..", "..");
const journal = readdirSync(module).find(name => name.endsWith("journal"))!;
const { appendEventsV1, writeBlobV1, formatHlcV1 } = await import(join(module, journal, readdirSync(join(module, journal)).find(name => name.endsWith(".ts"))!));
const { createHash } = await import("node:crypto");
const N = Number(origins), E = Number(events), size = Number(sizeKb) * 1024;
const source = join(peer, "src.bin");
const names: string[] = [];
for (let o = 0; o < N; o++) {
  const name = `bench-${o}`; names.push(name);
  mkdirSync(join(peer, "v1", "origins", name), { recursive: true });
  writeFileSync(join(peer, "v1", "origins", name, "home.json"), JSON.stringify({ version: 1, group: "g9" }));
  const list: unknown[] = [];
  for (let e = 0; e < E; e++) {
    const data = Buffer.alloc(size, `${o}-${e}-`); writeFileSync(source, data);
    const sha = createHash("sha256").update(data).digest("hex");
    await writeBlobV1(peer, name, sha, source);
    list.push({ version: 1, hlc: formatHlcV1({ ms: 1794300000000 + e, counter: 0 }, name), operation: "put", path: `b/${name}/f${e}.txt`, sha256: sha, bytes: size, executable: false, base: null });
  }
  await appendEventsV1(peer, name, list as never);
}
const own = mkdtempSync(join(scratch!, "own-"));
const t = performance.now();
const r = await R.relayOnceV1(own, peer, names);
console.log(JSON.stringify({ origins: N, eventsEach: E, sizeKb: Number(sizeKb), ms: Math.round(performance.now() - t), ...r }));

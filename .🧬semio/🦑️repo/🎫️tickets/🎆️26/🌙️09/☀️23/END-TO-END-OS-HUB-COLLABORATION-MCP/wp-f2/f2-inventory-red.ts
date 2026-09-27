/** 🔴️ F2 — red check of the connection-budget oracle: the permanent `connectionInventory` over the pre-mux NetLog (SSE era,
 * `hub2`) must name the idle holds, and over the post-mux NetLog (`after2`) none. usage: bun f2-inventory-red.ts */
import { readFileSync } from "node:fs";
import { connectionInventory } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔀️connection-budget/🟦️.ts";
for (const tag of ["hub2", "after2"]) {
  const inventory = connectionInventory(readFileSync(new URL(`./generated/f2-netlog-${tag}.json`, import.meta.url), "utf8"), "http://127.0.0.1:6580");
  console.log(tag, JSON.stringify({ idleHolds: inventory.idleHolds, transfers: inventory.transfersOver10s, streamMuxSockets: inventory.streamMuxSockets, stalls: inventory.stalls }));
}

// 🔬️ c4p43: runs the command-ingress consumer probe only on the cases that name the three Package 43 channel files, once on the pre-edit snapshots and once on the live files.
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {inspectCommandIngressConsumer, type ConsumerFixture} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️testing/🧩️consumer/🟦️.ts";
const repo = "/Users/ueli/Documents/semio";
const tools = resolve(import.meta.dir);
const fixture = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🏛️ownership/📥️command-ingress/🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as ConsumerFixture;
const snap: Record<string, string> = {
  "🧵️channel/🗜️codec/🦀️.rs": "ccd.rs",
  "🧵️channel/🧩️command-decode/🦀️.rs": "cdc.rs",
  "🧵️channel/🧪️tests/🔬️unit/🦀️.rs": "cut.rs",
};
const mine = fixture.cases.filter((c: any) => Object.keys(snap).some(k => c.file?.endsWith(k)));
console.log("cases naming the three files:", mine.length, mine.map((c: any) => c.kind + ":" + c.file.split("/").slice(-2, -1)[0]).join(" "));
const sub = {...fixture, cases: mine};
for (const mode of ["snapshot", "live"]) {
  const read = (path: string) => {
    const key = Object.keys(snap).find(k => path.endsWith(k));
    if (mode === "snapshot" && key) return readFileSync(join(tools, "orig", snap[key]!), "utf8");
    return readFileSync(join(repo, path), "utf8");
  };
  try {
    const out = await inspectCommandIngressConsumer(repo, sub, read);
    console.log(mode, "PASS", JSON.stringify(out));
  } catch (error) {
    console.log(mode, "FAIL", String((error as Error).message).slice(0, 200));
  }
}

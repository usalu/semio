/** 🧮️ Summarizes one census batch from cargo's JSON messages: which hub libraries were produced, which crates are red (error
 * count + first error), and which red crate each unproduced hub library depends on (`cargo tree -i`, resolver only).
 * Usage: `bun 🧪️s5-infra-census-summary.ts <batch.jsonl> <plugin>…`; prints one line per plugin and per red crate. */
import { spawnSync } from "node:child_process";

type Message = { reason: string; package_id?: string; fresh?: boolean; target?: { name: string; kind: string[] }; message?: { level: string; rendered?: string } };
const root = "/Users/ueli/Documents/semio";
const [path, ...plugins] = process.argv.slice(2);
if (!path || plugins.length === 0) throw new Error("usage: <batch.jsonl> <plugin>…");
const crateOf = (id: string): string => (id.match(/#([^@#]+)@/u) ?? id.match(/\/([^/#]+)#/u) ?? ["", id])[1]!;
const produced = new Set<string>();
const red = new Map<string, { count: number; first: string }>();
for (const line of (await Bun.file(path).text()).split("\n")) {
  if (!line.startsWith("{")) continue;
  const row = JSON.parse(line) as Message;
  if (row.reason === "compiler-artifact" && row.target?.kind.some((kind) => kind === "lib" || kind === "rlib" || kind === "cdylib")) produced.add(crateOf(row.package_id ?? ""));
  if (row.reason !== "compiler-message" || row.message?.level !== "error" || !row.message.rendered || /^error: (could not compile|aborting)/u.test(row.message.rendered)) continue;
  const crate = crateOf(row.package_id ?? ""), entry = red.get(crate) ?? { count: 0, first: row.message.rendered.split("\n")[0]!.replace(`${root}/`, "").replace(/\/📦️packages\/🦀️rust\/(?:\.\.?\/)+/u, "/…/") };
  red.set(crate, { ...entry, count: entry.count + 1 });
}
for (const [crate, entry] of red) console.log(`RED ${crate} errors=${entry.count} first=${entry.first.slice(0, 330)}`);
for (const plugin of plugins) {
  const hub = `semio-hub-${plugin}`;
  if (produced.has(hub)) { console.log(`GREEN ${hub}`); continue; }
  if (red.has(hub)) { console.log(`RED-HUB ${hub}`); continue; }
  const blockers = [...red.keys()].filter((crate) => {
    const tree = spawnSync("cargo", ["tree", "--offline", "--locked", "--manifest-path", `${root}/🌎️hub/Cargo.toml`, "-p", hub, "--target", "wasm32-wasip2", "-e", "normal", "-i", crate], { encoding: "utf8" });
    return tree.status === 0 && tree.stdout.includes(hub);
  });
  console.log(`BLOCKED ${hub} by=[${blockers.join(", ") || "unknown"}]`);
}

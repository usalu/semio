// bins-oracle: bun bins-oracle.ts <cargo-json.jsonl> <allow-list> <report-dir> [crate-name...] : runs every cargo test executable named in a `cargo test --no-run --message-format=json` log under the read oracle (backend auto) with the declared set, one executable at a time, cwd = the package directory (as cargo does), CARGO_MANIFEST_DIR set. Streams the log; writes one report per executable and a summary line per executable on stdout.
import {createReadStream, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {createInterface} from "node:readline";
import {dirname, join, relative} from "node:path";
import {runReadOracleV1} from "./stage/🔭️read-oracle/🟦️.ts";

const W = "/Users/ueli/Documents/semio";
const [log, allowList, reportDirectory, ...only] = process.argv.slice(2);
mkdirSync(reportDirectory!, {recursive: true});
const allowed = readFileSync(allowList!, "utf8").split("\n").filter(Boolean);
const R = "🧰️framework/🛍️products/🦑️repo", T = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/keys2-g-tools";
const lines = createInterface({input: createReadStream(log!), crlfDelay: Infinity});
const executables: {name: string; target: string; executable: string; manifest: string}[] = [];
for await (const line of lines) {
  let row: any;
  try { row = JSON.parse(line); } catch { continue; }
  if (row?.reason !== "compiler-artifact" || !row.executable || !row.profile?.test) continue;
  const name = String(row.package_id).replace(/^.*#/u, "").replace(/@.*$/u, "") || row.target.name;
  const crate = /\/([^/#]+)#/u.exec(row.package_id)?.[1] ?? name;
  if (only.length && !only.some((wanted) => row.package_id.includes(wanted))) continue;
  executables.push({name: crate, target: `${row.target.kind.join("+")}:${row.target.name}`, executable: row.executable, manifest: row.manifest_path});
}
for (const entry of executables) {
  const directory = dirname(entry.manifest), cwd = relative(W, directory);
  const started = Date.now();
  try {
    const report = await runReadOracleV1({version: 1, language: "en", root: W, cwd, allowed, ambient: [T, `${R}/🔨️modules/🚀️launcher`, `${R}/tsconfig.json`, relative(W, reportDirectory!)], names: ["package.json"], command: [entry.executable, "--test-threads", "1"], environment: {CARGO_MANIFEST_DIR: directory}, rounds: 12, timeoutMs: 900000, backend: "auto", compare: "exit"});
    writeFileSync(join(reportDirectory!, `${entry.name}__${entry.target.replace(/[^a-z0-9_.+-]/giu, "_")}.json`), JSON.stringify(report, null, 1));
    console.log(JSON.stringify({crate: entry.name, target: entry.target, backend: report.backend, passed: report.passed, clean: report.clean, learned: report.learned.length, seconds: Math.round((Date.now() - started) / 1000)}));
  } catch (error) {
    console.log(JSON.stringify({crate: entry.name, target: entry.target, error: (error as Error).message.slice(0, 200)}));
  }
}

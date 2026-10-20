import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const authority = JSON.parse(readFileSync(root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json", "utf8"));
const pattern = new RegExp(authority.$defs.DeveloperInstructions.not?.pattern ?? authority.$defs.DeveloperInstructions.allOf?.[0]?.not?.pattern ?? "$^");
const src = readFileSync(root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts", "utf8");
const re = eval(/const acceptsInstructions = \(text: string\) => !(\/.*\/)\.test/.exec(src)![1]);
const out = spawnSync("git", ["-c", "core.quotepath=off", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 30 }).stdout.toString().split("\0");
const docs = out.filter(p => /(?:^|\/)(?:README|CONTRIBUTING|DEVELOPMENT|AGENTS)\.md$/i.test(p) && !/(?:🧫️fixtures|🧪️tests|🧬️schema|\.cursor\/plans)(?:\/|$)/.test(p));
let bad = 0;
for (const p of docs) { try { const t = readFileSync(root + "/" + p, "utf8").replaceAll("\r\n", "\n"); if (re.test(t.toLowerCase())) { bad++; console.log("REFUSED", p); } } catch {} }
console.log(JSON.stringify({ docs: docs.length, bad, schemaPattern: pattern.source.slice(0, 80) }));

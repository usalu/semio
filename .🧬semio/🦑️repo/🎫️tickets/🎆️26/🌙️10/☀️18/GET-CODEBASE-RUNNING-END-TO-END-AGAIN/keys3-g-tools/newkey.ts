// keys3-g: the simulated key of architecture-course:build after the batch: old key list (generation 550 measure) minus every file the batch's negations match, plus every tracked/untracked file the `tailwindScan` positives match and the negations do not. Streams git ls-files; prints counts.
// usage: bun newkey.ts <old.files> <patched project.json> <out.files>
import {readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const {inputGlobRegExpV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts");
const [oldList, projectFile, out] = process.argv.slice(2);
const scan: string[] = JSON.parse(readFileSync(projectFile, "utf8")).namedInputs.tailwindScan;
const positives = scan.filter((p) => !p.startsWith("!")).map((p) => inputGlobRegExpV1(p.replace("{workspaceRoot}/", "")));
const negatives = scan.filter((p) => p.startsWith("!")).map((p) => inputGlobRegExpV1(p.slice(1).replace("{workspaceRoot}/", "")));
const old = readFileSync(oldList, "utf8").split("\n").filter(Boolean);
const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {maxBuffer: 256 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean);
const removed = old.filter((f) => negatives.some((n) => n.test(f)));
const added = listing.filter((f) => positives.some((p) => p.test(f)) && !negatives.some((n) => n.test(f)));
const next = new Set([...old.filter((f) => !negatives.some((n) => n.test(f))), ...added]);
writeFileSync(out, [...next].sort().join("\n") + "\n");
console.log(JSON.stringify({old: old.length, removedByNegations: removed.length, added: added.filter((f) => !old.includes(f)).length, next: next.size}));
writeFileSync(out + ".removed", removed.sort().join("\n") + "\n");

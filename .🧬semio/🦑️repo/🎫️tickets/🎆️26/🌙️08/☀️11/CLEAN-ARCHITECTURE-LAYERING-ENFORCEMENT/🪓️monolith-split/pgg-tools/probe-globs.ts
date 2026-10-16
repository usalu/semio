import { spawnSync } from "node:child_process";
import { join, resolve } from "node:path";
import { relativePosix } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { dropUnlinkedCascades, unlinkedGlobModules, type OracleEntry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";

const [sandbox, ...flags] = process.argv.slice(2);
const root = join(sandbox!, "__combined.rs");
const result = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "combined", "--emit=metadata", "--error-format=json", "-o", resolve(sandbox!, ".probe.rmeta"), ...flags, root], { encoding: "utf8", maxBuffer: 1 << 30, cwd: sandbox });
const entries: OracleEntry[] = [];
for (const line of (result.stderr ?? "").split("\n")) {
  if (!line.startsWith("{")) continue;
  const message = JSON.parse(line);
  if (message.level !== "error") continue;
  const primary = message.spans?.find((span: { is_primary: boolean }) => span.is_primary);
  entries.push({ code: message.code?.code ?? "", message: (message.message ?? "").replaceAll(sandbox!, "<sandbox>"), label: primary?.label ?? "", file: primary ? relativePosix(sandbox!, resolve(sandbox!, primary.file_name)) : "", line: primary?.line_start ?? 0, sourceLine: primary?.text?.[0]?.text ?? "" });
}
const globs = unlinkedGlobModules(sandbox!, root, entries);
console.log("entries", entries.length, "globModules", JSON.stringify([...globs]));
for (const entry of entries) if (/board_set_camera/u.test(entry.message + entry.sourceLine)) console.log(JSON.stringify(entry).slice(0, 500));
const kept = dropUnlinkedCascades(entries, globs);
console.log("kept", kept.length);
for (const entry of kept) if (/board_set_camera/u.test(entry.message + entry.sourceLine)) console.log("KEPT", JSON.stringify(entry).slice(0, 500));

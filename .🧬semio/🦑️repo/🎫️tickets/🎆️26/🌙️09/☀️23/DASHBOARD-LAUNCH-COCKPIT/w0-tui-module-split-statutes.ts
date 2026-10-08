#!/usr/bin/env bun
/**
 * 🔏️ Runs the repo's own path-emoji statute and taxonomy emoji admission over the split `🖱️ui/⌨️tui` tree.
 *
 * Ticket input of 26/09/23/DASHBOARD-LAUNCH-COCKPIT, slice w0. Run from the repository root:
 * `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/w0-tui-module-split-statutes.ts`
 *
 * @see 🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { pathEmojiStatuteFindings, type PathEmojiEntry } from "../../../../../../../🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts";

const root = process.cwd();
const subtree = "🧰️framework/🔨️modules/🖱️ui/⌨️tui";
const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const entries: PathEmojiEntry[] = [];
const walk = (relative: string): void => {
  for (const name of readdirSync(join(root, relative)).sort()) {
    const path = `${relative}/${name}`;
    const directory = statSync(join(root, path)).isDirectory();
    entries.push({ path, nodeKind: directory ? "directory" : "file" });
    if (directory) walk(path);
  }
};
walk(subtree);
const findings = pathEmojiStatuteFindings(entries, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
const rgi = new RegExp("^(?:\\p{RGI_Emoji})$", "v");
const admitted = (emoji: string): boolean => emoji === emoji.normalize("NFC") && (rgi.test(emoji) || /^\p{Extended_Pictographic}️$/u.test(emoji));
const longest = Math.max(...entries.map((entry) => Buffer.byteLength(entry.path)));
const rejected = entries.filter((entry) => {
  const name = entry.path.slice(entry.path.lastIndexOf("/") + 1);
  const emoji = name.match(/^(\p{Extended_Pictographic}️?)/u)?.[1] ?? "";
  return !admitted(emoji) || name !== name.normalize("NFC");
});
const untouched = `${subtree}/🧪️tests/`;
const inherited = findings.filter((finding) => finding.path.startsWith(untouched));
const introduced = findings.filter((finding) => !finding.path.startsWith(untouched));
console.log(JSON.stringify({ entries: entries.length, directories: entries.filter((entry) => entry.nodeKind === "directory").length, introduced, inheritedUnderTests: inherited, emojiRejected: rejected.map((entry) => entry.path), longestPathBytes: longest, maxPathBytes: taxonomy.collisionPolicy.maxPathBytes }, null, 2));
if (introduced.length > 0 || rejected.length > 0 || longest > taxonomy.collisionPolicy.maxPathBytes) process.exit(1);

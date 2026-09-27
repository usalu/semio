/** 🧭️ Z3 one-off: runs the repository's path-emoji statutes over every path this slice created (git-visible form). */
import { pathEmojiStatuteFindings } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { readFileSync } from "node:fs";
const taxonomy = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", "utf8"));
const paths = process.argv.slice(2);
const entries = paths.flatMap((path) => { const parts = path.split("/"); return parts.map((_, index) => ({ path: parts.slice(0, index + 1).join("/"), nodeKind: index === parts.length - 1 && /\.[a-z]+$/.test(parts[index]!) ? "file" : "directory" })); });
const unique = [...new Map(entries.map((entry) => [entry.path, entry])).values()] as any;
const findings = pathEmojiStatuteFindings(unique, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
console.log(`${unique.length} path nodes; ${findings.length} findings`);
for (const finding of findings) console.log(JSON.stringify(finding));

/** 🎨️ Deploy-readiness helper: gives every docstring of the operator module its own emoji. Each entry names the start of
 * one docstring (after its emoji) and the emoji it gets; the script fails when a start is not found exactly once, and
 * prints every emoji that still opens more than one docstring. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const file = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts");
const renames: ReadonlyArray<readonly [string, string, string]> = [
  ["🌐️", "The site origin: the one browser origin", "🪪️"],
  ["🛂️", "The proctor origin a release build of the site talks to", "🔗️"],
  ["🏷️", "The labels of the image", "🪧️"],
  ["🩺️", "The image's `HEALTHCHECK`", "💓️"],
  ["🩺️", "The daemon's server version", "🐋️"],
  ["🧳️", "Where the image carries the stack files", "🎒️"],
  ["🧱️", "The whole footprint of a host", "👣️"],
  ["⚖️", "What a release of the site may weigh", "🏋️"],
  ["🧾️", "`_headers` for CDNs that honour it", "📑️"],
  ["🐳️", "Runs one `docker` argv and fails with its output", "🚢️"],
  ["📏️", "A Caddy size", "🧮️"],
  ["📏️", "The request body Caddy lets through", "📨️"],
  ["🚚️", "`docker-image-publish [--tag <tag>]`", "📤️"],
  ["🌐️", "One HTTP answer of the proctor", "📬️"],
  ["🌐️", "One request to `url`", "📡️"],
  ["🔎️", "Whether the proctor knows `learner`", "🕵️"],
  ["👥️", "One presence socket of a check", "🪢️"],
  ["🗃️", "How many events the proctor database", "🧫️"],
];
let text = readFileSync(file, "utf8");
for (const [from, start, to] of renames) {
  const before = `/** ${from} ${start}`;
  if (text.split(before).length !== 2) throw new Error(`${JSON.stringify(before)} occurs ${text.split(before).length - 1} times`);
  text = text.replace(before, `/** ${to} ${start}`);
}
writeFileSync(file, text);
const counts = new Map<string, number>();
for (const [, emoji] of text.matchAll(/^\s*\/\*\* (\S+) /gmu)) counts.set(emoji!, (counts.get(emoji!) ?? 0) + 1);
console.log(`[DEBUG] ${counts.size} docstrings emojis; repeated: ${[...counts].filter(([, count]) => count > 1).map(([emoji, count]) => `${emoji}×${count}`).join(" ") || "none"}`);

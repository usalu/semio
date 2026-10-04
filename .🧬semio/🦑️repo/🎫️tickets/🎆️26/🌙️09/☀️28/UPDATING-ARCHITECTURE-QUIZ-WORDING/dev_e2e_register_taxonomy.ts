/** 🗂️ Idempotent taxonomy registration of the directories the dev-stack and end-to-end work package added to the
 * architecture quiz site: the local stack, the end-to-end gate with its learner driver (contextual kinds under the
 * teaching quiz), and every spec directory under `🧪️tests` (members of the tests kind). The registry is edited as text
 * at two anchors, so nothing else in the file moves. `bun dev_e2e_register_taxonomy.ts [--dry]`. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type Json = Record<string, any>;
const VS = "️";
const root = join(import.meta.dir, "../../../../../../..");
const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const dry = process.argv.includes("--dry");
let text = readFileSync(path, "utf8");
const taxonomy = JSON.parse(text) as Json;

const kinds: Readonly<Record<string, readonly [emoji: string, slug: string, parent: string]>> = {
  "teaching-quiz-stack": ["🧱", "stack", "teaching-quiz"],
  "teaching-quiz-e2e": ["🎭", "e2e", "teaching-quiz"],
  "teaching-quiz-e2e-learner": ["🚶", "learner", "teaching-quiz-e2e"],
};
const kindAnchor = `    "deploy": {\n      "emoji": "🚀${VS}",`;
const missingKinds = Object.entries(kinds).filter(([id]) => !(id in taxonomy.semanticDirectoryKinds));
if (missingKinds.length > 0) {
  if (text.split(kindAnchor).length !== 2) throw new Error("the deploy kind anchor is not unique");
  const blocks = missingKinds.map(([id, [emoji, slug, parent]]) => `    "${id}": {\n      "emoji": "${emoji}${VS}",\n      "slugPattern": "^${slug}$",\n      "allowEmojiOnly": false,\n      "parentKindIds": [\n        "${parent}"\n      ]\n    },\n`).join("");
  text = text.replace(kindAnchor, `${blocks}${kindAnchor}`);
}

const tests = join(root, "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests");
const members = taxonomy.semanticDirectoryMemberKinds["members-of-tests"].memberNames as string[];
const fresh = readdirSync(tests, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name.normalize("NFC"))
  .filter((name) => !members.includes(name) && !name.startsWith(`🧪${VS}`) && !name.startsWith(`🎚${VS}`));
if (fresh.length > 0) {
  const start = text.indexOf(`    "members-of-tests": {\n`);
  const last = members[members.length - 1]!;
  const tail = `        ${JSON.stringify(last)}\n      ],`;
  const at = text.indexOf(tail, start);
  if (start < 0 || at < 0) throw new Error("the end of the members-of-tests list was not found");
  text = `${text.slice(0, at)}        ${JSON.stringify(last)},\n${fresh.map((name, index) => `        ${JSON.stringify(name)}${index === fresh.length - 1 ? "" : ","}\n`).join("")}      ],${text.slice(at + tail.length)}`;
}

const after = JSON.parse(text) as Json;
for (const id of Object.keys(kinds)) if (!(id in after.semanticDirectoryKinds)) throw new Error(`kind ${id} did not land`);
for (const name of fresh) if (!after.semanticDirectoryMemberKinds["members-of-tests"].memberNames.includes(name)) throw new Error(`member ${name} did not land`);
console.log(`[DEBUG] kinds += ${missingKinds.map(([id]) => id).join(" ") || "(nothing new)"}; members-of-tests += ${fresh.join(" ") || "(nothing new)"}`);
if (!dry) writeFileSync(path, text);
console.log(`[DEBUG] ${dry ? "dry run" : "written"}`);

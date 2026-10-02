/** 🎫️ Ticket tool: closes the QUIZ-PETS ticket on disk (the repo MCP was down): sets the status, the summary and the files this ticket created or updated.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/close_ticket.ts"
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticketRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticketRoot, "../../../../../../..");
const ticketPath = resolve(ticketRoot, "🎫️ticket.json");
const ticketFolder = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS";

const createdScopes = [
  "🧰️framework/🛍️products/🐾️pets",
  "🎓️teaching/🏛️architecture/🐾️pets",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets",
  "🧰️framework/🛍️products/❓️quiz/🧪️tests/🐾️pet-companions",
  "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐾️pet-cast",
  "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk",
  ticketFolder,
];

const updated = [
  ".claude/launch.json",
  ".vscode/launch.json",
  ".vscode/🧩️launch.seed.jsonc",
  "Cargo.lock",
  "Cargo.toml",
  "bun.lock",
  "package.json",
  "🎓️teaching/🏛️architecture/README.md",
  "🎓️teaching/🏛️architecture/❓️quiz/README.md",
  "🎓️teaching/🏛️architecture/❓️quiz/🎭️e2e/🎚️config/🟦️.ts",
  "🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts",
  "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/package.json",
  "🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json",
  "🎓️teaching/🏛️architecture/❓️quiz/🟦️.ts",
  "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/❓️quiz/README.md",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/⚖️legal/🟦️.tsx",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx",
  "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts",
  "🧰️framework/🛍️products/❓️quiz/🧪️tests/🏠️home-grid/🟦️.tsx",
  "🧰️framework/🛍️products/❓️quiz/🧪️tests/📡️presence-client/🟦️.tsx",
  "🧰️framework/🛍️products/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md",
];

const summary =
  "Added pets to the quiz UI. New domain-neutral product framework/products/pets: schema-first menageries of rigged species (bones, parts, face, keyframed clips), a pure deterministic stage (64 ticks per second, own sine and cosine, counter-based randomness equal to numpy SeedSequence) with TypeScript and bit-exact Rust cores (510 TypeScript and 184 Rust unit tests; twelve Protocol v2 cases with numpy, scipy and jsonschema oracles, parity 183/183; 57 600 long-run checkpoints without a mismatch), and a React target (inline SVG depiction, DOM survey of card tabs, body edges and the footer line, paced loop that sleeps when nothing moves, decorative aria-hidden pointer-transparent layer, stories gallery). Pets are slightly active by default, blink and fidget, follow the pointer with pupils, head and body, walk, hop and float on top of UI elements, ride and fall with them, keep off text, controls and the focused element, and meet each other: greet, cuddle, squabble, sulk and mend, by authored bonds and a drifting rapport. The quiz gets QuizOptions.pets (lazy chunk), a preference (off, still, calm, lively), a footer switch on every screen, notes for reduced motion and forced colours, and rests its pets during a run. The architecture site ships twenty hand-crafted species grounded in its quiz items (sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery, windy, boily, roofy, insuly, shady, venty, chilly, kettly, flamy, thermy, servy), 67 bonds and casts for home and each quiz; a site test keeps every cast fitting its topic and a Playwright project proves standing on cards, gaze, blinking, walking and encounters. Open points are listed in 📓️closing-summary.md.";

const lines = Bun.spawnSync(["git", "-c", "core.quotepath=false", "status", "--porcelain", "--untracked-files=all", "--", ...createdScopes], { cwd: repoRoot }).stdout.toString().split("\n").filter((line) => line.length > 3);
const created = lines
  .filter((line) => line.startsWith("??") || line.startsWith("A"))
  .map((line) => line.slice(3).replace(/^"|"$/g, ""))
  .filter((path) => !path.includes("🗑️generated"))
  .sort();
const missing = updated.filter((path) => !existsSync(resolve(repoRoot, path)));
if (missing.length > 0) throw new Error(`updated paths that do not exist: ${missing.join(", ")}`);

const ticket = JSON.parse(readFileSync(ticketPath, "utf8"));
const closed = { ...ticket, status: "closed", summary, files: { created, updated, removed: [] }, note: `${ticket.note} Closed manually on 2026-10-02 (repo MCP still down).` };
writeFileSync(ticketPath, `${JSON.stringify(closed, null, 2)}\n`);
console.log(`closed: ${created.length} created, ${updated.length} updated`);

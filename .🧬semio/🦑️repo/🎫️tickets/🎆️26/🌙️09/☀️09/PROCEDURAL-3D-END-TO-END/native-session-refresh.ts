import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { readGeneratedCatalogProjection } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { renderPlaygroundSessionTypeScript } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";

/** ♻️ Graph-free twin of `session-generation3d`: rewrites the staged generation3d playground session from the committed generated catalog (same renderer and file set as `playgroundSessionPublicationV1`). */
const root = process.cwd(), out = join(root, ".cache/b/p/m/browser/prototype/sessions/generation3d");
const projection = readGeneratedCatalogProjection(join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"));
mkdirSync(join(out, "🎮️playground-session"), { recursive: true });
writeFileSync(join(out, "🎮️playground-session/🟦️.ts"), renderPlaygroundSessionTypeScript("generation3d", projection));
writeFileSync(join(out, "🔌️plugins.json"), JSON.stringify(projection.entries) + "\n");
writeFileSync(join(out, "🎠️playgrounds.json"), JSON.stringify(projection.playgrounds) + "\n");
console.log("[g3d-native] session refreshed", projection.entries.length, "plugins", projection.playgrounds.length, "playgrounds");

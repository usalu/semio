/** ✂️ The carve wave: every module that the cycle-break design turns into its own workspace package. */
import { Plan } from "./plan.ts";
import { Carver, type CarveSpec } from "./carve.ts";
import { finish } from "./direct.ts";
const M = "🧰️framework/🔨️modules";
const R = "🧰️framework/🛍️products/🦑️repo";
const OS = "🧰️framework/🛍️products/💻️os/🔨️modules";
const FW = "@semio-tech/framework";
const plan = new Plan();
const foundation = (slug: string, dir: string, description: string, wave = "w3-foundation"): CarveSpec => ({ wave, name: `@semio-tech/framework-${slug}`, dir: `${M}/${dir}`, description, moveFrom: [{ pkg: FW }] });
const specs: CarveSpec[] = [
  { wave: "w2a-publication", name: "@semio-tech/plugin-registry-publication", dir: `${OS}/🔌️plugin/📇️registry/📣️publication`, description: "plugin catalog · publication contract and build contribution (leaf)", moveFrom: [{ pkg: "@semio-tech/plugin-registry", keys: ["./publication", "./publication/build"] }] },
  { wave: "w2b-server-library", name: "@semio-tech/repo-server-library", dir: `${R}/🔨️modules/🖥️server/📚️library`, description: "repo · server library (persistence, authentication, events, source index, worker execution) and its ports", moveFrom: [{ pkg: FW }, { pkg: "@semio-tech/repo-coordinator", keys: ["./ports"] }], anchorDir: "🧰️framework/📦️packages/🟦️typescript", extraThirdParty: ["next", "pg", "pg-boss"] },
  { wave: "w2c-repo-product", name: "@semio-tech/repo-product", dir: R, description: "repo · product modules (dashboard, client, lint) outside the library, coordinator, test and server-library packages", moveFrom: [{ pkg: FW, skip: ["./products/repo/modules/dashboard/installation"] }], ignorePrefixes: [`${R}/🔨️modules/🎛️dashboard/📦️installation`] },
  foundation("hash", "🔏️hash", "framework · content hashing (leaf)"),
  foundation("trace", "⏱️trace", "framework · trace, clock and memory sinks (leaf)"),
  foundation("interaction", "🕹️interaction", "framework · interaction vocabulary"),
  foundation("transient", "🫧️transient", "framework · ephemeral state boxes and maps (leaf)"),
  foundation("filesystem", "📁️filesystem", "framework · filesystem observation contracts (leaf)"),
  foundation("onboarding", "🎓️onboarding", "framework · onboarding declarations, window layouts and tutorials"),
  foundation("manifest", "🛂️manifest", "framework · manifest: app, plugin and action definitions with the ui contract and canvas types"),
  foundation("action-argument-resolution", "🧩️action-argument-resolution", "framework · action argument resolution"),
  { wave: "w3-effect", name: "@semio-tech/framework-effect", dir: `${M}/🎠️kernel/⚡️effect`, description: "framework · effect vocabulary (effects, spawned jobs, video render programs) shared by kernel, actor and manifest", moveFrom: [{ pkg: "@semio-tech/framework-kernel", keys: ["./effect", "./spawned-job", "./video-render-program"] }] },
  { wave: "w3-ui", name: "@semio-tech/ui-contract", dir: `${M}/🖱️ui/🧬️contract`, description: "ui · retained scene, wire, resident and component contract shared by actor, kernel and the ui", moveFrom: [{ pkg: FW }], extraExports: { "./testing/source-text/retained-fixtures": "./🧵️retained/🧪️testing/📖️source-text/🟦️.ts" }, semio: { dependencyRole: "ui" } },
  { wave: "w3-ui", name: "@semio-tech/ui", dir: `${M}/🖱️ui`, description: "ui · elements, modules, scene, host and styling-neutral core shared by every ui target", moveFrom: [{ pkg: FW }], extraExports: { "./i18n/source-text": "./🌐️i18n/📖️source-text/🟦️.ts" }, semio: { dependencyRole: "ui", role: "framework", id: "ui" } },
];
new Carver(plan, specs).run();
finish(plan, "carves", process.argv);

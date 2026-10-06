#!/usr/bin/env bun
/** lowpoly TypeScript package */
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

//#region 🔖️InteractiveJobExamples
/** 🔤️ The Rust variant name of one kebab lane/disposition from `framework.ui`'s shared vocabulary. */
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");

type Route = {
  toolId: string;
  classification: "migrated" | "batch-only-pending-rewrite";
  lanes: ("host-only" | "artifact" | "config" | "transient")[];
  preparation: ("Artifact" | "Config")[];
  blocker: string | null;
};

type Fixture = {
  version: number;
  owner: string;
  maximumPollMicros: number;
  maximumRawBytes: number;
  maximumWorkItems: number;
  artifactStoreMaximumBytes: number;
  configStoreMaximumBytes: number;
  routes: Route[];
};

const publicationPolicyAccepted = (lanes: Route["lanes"], preparation: Route["preparation"]): boolean => {
  if (!lanes.length || new Set(lanes).size !== lanes.length || lanes.some(lane => !["host-only", "artifact", "config", "transient"].includes(lane))) return false;
  if (lanes.includes("host-only")) return lanes.length === 1 && preparation.length === 0;
  const order = ["artifact", "config", "transient"];
  if (lanes.join("+") !== [...lanes].sort((left, right) => order.indexOf(left) - order.indexOf(right)).join("+")) return false;
  return preparation.join("+") === lanes.filter(lane => lane === "artifact" || lane === "config").map(variant).join("+");
};

const reject = (condition: boolean, message: string): void => {
  if (!condition) throw new Error(message);
};
//#endregion 🔖️InteractiveJobExamples

//#region 🧪️InteractiveJobSourceTest
class TestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", ...["✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts","✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts"].map(path => resolve(this.repoRoot, path))], { cwd: this.repoRoot });

    const root = resolve(import.meta.dir, "../..");
    const policySchema = JSON.parse(readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json"), "utf8")).$defs.LowpolyPublicationPolicyV1;
    const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🧪️interactive-job/🔣️.json"), "utf8")) as Fixture;
    const source = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), "utf8");
    const schemaSource = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"), "utf8");
    const sessionSource = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs"), "utf8");
    const sourceCollapsed = source.replace(/\s+/g, " ").replace(/,\s*\}/g, " }");
    const registered = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => ({ toolId: match[1]!, classification: match[2]! }));
    reject(registered.length === fixture.routes.length, `Lowpoly source must register exactly ${fixture.routes.length} classified actions (found ${registered.length})`);
    reject([...registered.map(route => route.toolId)].sort().join("\0") === [...fixture.routes.map(route => route.toolId)].sort().join("\0"), "Lowpoly source must declare every expected action exactly once");
    for (const route of fixture.routes) {
      reject(registered.some((row) => row.toolId === route.toolId && row.classification === variant(route.classification)), `Lowpoly source classification drift: ${route.toolId}`);
      if (route.classification === "migrated") {
        const lanes = route.lanes.map((lane) => `semio_framework_plugin::ArtifactToolPublicationLane::${variant(lane)}`).join(", ");
        // 🧹️ rustfmt wraps the longest contract row over several lines; compare with whitespace collapsed.
        reject(sourceCollapsed.includes(`ArtifactToolPublicationContract { tool_id: "${route.toolId}", lanes: &[${lanes}] }`.replace(/\s+/g, " ")), `Lowpoly publication lane drift: ${route.toolId}`);
        reject(source.includes(`"${route.toolId}" => ToolExecutionContract::resumable`), `Lowpoly proof drift: ${route.toolId}`);
      }
    }
    const structural = [
      "operation.operation_id != self.operation_id",
      "operation.generation != self.generation",
      "operation.canonical_base_revision != self.base_revision",
      "context.identity_digest() != self.context_identity",
      "ArtifactCommandWorkStep::Progress",
      "ArtifactCommandWorkStep::Replay",
      "copy_from_slice(b\"LPC3\")",
      "fn begin_close(&mut self)",
      "fn close_step(&mut self",
      "build_artifact_store_one_item_preparation_factory",
      "build_config_store_one_item_preparation_factory",
      "authority.prepare_one_item",
      "const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384",
      "const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258",
      "const LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 16 * 1024 * 1024",
      "const LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384",
      "paint::lowpoly_paint_step(",
    ];
    for (const needle of structural) reject(source.includes(needle), `Lowpoly retained source missing ${needle}`);
    reject(!schemaSource.includes("OnceLock<(crate::artifacts::lowpoly::LowpolySnapshot"), "Lowpoly schema still owns a process-global ArtifactChild payload cache");
    reject(schemaSource.includes("pub fn default_owned_document() -> LowpolyOwnedDefaultDocument"), "Lowpoly schema lacks caller-owned default child payload construction");
    reject(sessionSource.includes("pub fn lowpoly_paint_drive("), "Lowpoly session lacks the paint tool drive");
    reject(sessionSource.includes("pub fn paint_preview(&self"), "Lowpoly session lacks the paint gesture preview");
    console.log(`lowpoly interactive-job actual source actions agree with examples: ${fixture.routes.length} Migrated, 0 BatchOnlyPendingRewrite`);

    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    const validatePolicy = ajv.compile(policySchema);
    for (const route of fixture.routes) {
      const policy = { lanes: route.lanes, preparation: route.preparation };
      reject(publicationPolicyAccepted(policy.lanes, policy.preparation), `Invalid actual Lowpoly publication policy: ${route.toolId}`);
      reject(validatePolicy(policy), `Ajv rejected actual publication policy: ${route.toolId}`);
    }
    for (const policy of [{ lanes: [], preparation: [] }, { lanes: ["artifact", "artifact"], preparation: ["Artifact"] }, { lanes: ["artifact"], preparation: ["Config"] }, { lanes: ["host-only", "config"], preparation: ["Config"] }]) {
      reject(!publicationPolicyAccepted(policy.lanes as Route["lanes"], policy.preparation as Route["preparation"]), "Owned domain policy accepted invalid lanes or preparation");
      reject(!validatePolicy(policy), "Ajv domain policy accepted invalid lanes or preparation");
    }
    console.log(`Lowpoly actual publication policies agree with Ajv: ${fixture.routes.length} source actions, four invalid domain policies`);
  }
}
//#endregion 🧪️InteractiveJobSourceTest

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });

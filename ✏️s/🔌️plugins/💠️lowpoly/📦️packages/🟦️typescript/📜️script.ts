#!/usr/bin/env bun
/** lowpoly TypeScript package */
import { BundleScript, ScriptRouter, runCmd, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import Ajv2020 from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

//#region 🔖️InteractiveJobFixture
type Route = {
  toolId: string;
  classification: "Migrated" | "BatchOnlyPendingRewrite";
  lanes: ("HostOnly" | "Artifact" | "Config" | "Transient")[];
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

const ownKeys = (value: object, expected: string[]): boolean =>
  Object.keys(value).sort().join("\u0000") === [...expected].sort().join("\u0000");

const validateOwnedFixture = (value: unknown): value is Fixture => {
  if (typeof value !== "object" || value === null || !ownKeys(value, ["version", "owner", "maximumPollMicros", "maximumRawBytes", "maximumWorkItems", "artifactStoreMaximumBytes", "configStoreMaximumBytes", "routes"])) return false;
  const fixture = value as Fixture;
  if (fixture.version !== 1 || fixture.owner !== "LowpolyPlayApp" || fixture.maximumRawBytes !== 16_384 || fixture.maximumWorkItems !== 258 || fixture.artifactStoreMaximumBytes !== 16_777_216 || fixture.configStoreMaximumBytes !== 16_384 || !Number.isInteger(fixture.maximumPollMicros) || fixture.maximumPollMicros < 1 || fixture.maximumPollMicros > 8_000 || !Array.isArray(fixture.routes) || fixture.routes.length !== 47) return false;
  const ids = new Set<string>();
  let migrated = 0;
  let batch = 0;
  for (const route of fixture.routes) {
    if (typeof route !== "object" || route === null || !ownKeys(route, ["toolId", "classification", "lanes", "preparation", "blocker"]) || typeof route.toolId !== "string" || route.toolId.length === 0 || ids.has(route.toolId) || !Array.isArray(route.lanes) || !Array.isArray(route.preparation)) return false;
    ids.add(route.toolId);
    if (route.classification === "Migrated") {
      migrated += 1;
      const signature = `${route.lanes.join("+")}|${route.preparation.join("+")}`;
      if (!["Artifact|Artifact", "Config|Config", "HostOnly|", "Transient|", "Config+Transient|Config", "Artifact+Transient|Artifact", "Artifact+Config|Artifact+Config", "Artifact+Config+Transient|Artifact+Config"].includes(signature) || route.blocker !== null) return false;
    } else if (route.classification === "BatchOnlyPendingRewrite") {
      batch += 1;
      if (route.lanes.length !== 0 || route.preparation.length !== 0 || typeof route.blocker !== "string" || route.blocker.length === 0) return false;
    } else {
      return false;
    }
  }
  return migrated === 47 && batch === 0;
};

const reject = (condition: boolean, message: string): void => {
  if (!condition) throw new Error(message);
};
//#endregion 🔖️InteractiveJobFixture

//#region 🧪️InteractiveJobSourceTest
class TestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", ...["✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts","✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts"].map(path => resolve(this.repoRoot, path))], { cwd: this.repoRoot });

    const root = resolve(import.meta.dir, "../..");
    const schema = JSON.parse(readFileSync(resolve(root, "🧪️interactive-job/🧬️.schema.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(resolve(root, "🧪️interactive-job/🔣️.json"), "utf8")) as Fixture;
    const source = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), "utf8");
    const schemaSource = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"), "utf8");
    const sessionSource = readFileSync(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs"), "utf8");
    reject(validateOwnedFixture(fixture), "owned Lowpoly fixture validation failed");
    const registered = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => ({ toolId: match[1]!, classification: match[2]! }));
    reject(registered.length === 47, "Lowpoly source must register exactly 47 classified actions");
    for (const route of fixture.routes) {
      reject(registered.some((row) => row.toolId === route.toolId && row.classification === route.classification), `Lowpoly source classification drift: ${route.toolId}`);
      if (route.classification === "Migrated") {
        const lanes = route.lanes.map((lane) => `semio_framework_plugin::ArtifactToolPublicationLane::${lane}`).join(", ");
        reject(source.includes(`ArtifactToolPublicationContract { tool_id: "${route.toolId}", lanes: &[${lanes}] }`), `Lowpoly publication lane drift: ${route.toolId}`);
        reject(source.includes(`"${route.toolId}" => semio_framework::ToolExecutionContract::resumable`), `Lowpoly proof drift: ${route.toolId}`);
      }
    }
    const structural = [
      "operation.operation_id != self.operation_id",
      "operation.generation != self.generation",
      "operation.canonical_base_revision != self.base_revision",
      "context.identity_digest() != self.context_identity",
      "ArtifactCommandWorkStep::Progress",
      "ArtifactCommandWorkStep::Replay",
      "copy_from_slice(b\"LPC2\")",
      "fn begin_close(&mut self)",
      "fn close_step(&mut self",
      "build_artifact_store_one_item_preparation_factory",
      "build_config_store_one_item_preparation_factory",
      "authority.prepare_one_item",
      "const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384",
      "const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258",
      "const LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 16 * 1024 * 1024",
      "const LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384",
      "fn paint_end_step(&mut self",
      "paint_replay_target",
      "LOWPOLY_RETAINED_PAINT_CHUNK_BYTES",
    ];
    for (const needle of structural) reject(source.includes(needle), `Lowpoly retained source missing ${needle}`);
    reject(!schemaSource.includes("OnceLock<(crate::artifacts::lowpoly::LowpolySnapshot"), "Lowpoly schema still owns a process-global ArtifactChild payload cache");
    reject(schemaSource.includes("pub fn default_owned_document() -> LowpolyOwnedDefaultDocument"), "Lowpoly schema lacks caller-owned default child payload construction");
    reject(sessionSource.includes("pub(crate) fn stroke_diff_parts(&self)"), "Lowpoly session lacks borrowed paint diff ownership");
    reject(sessionSource.includes("pub(crate) fn finish_stroke_drag(&self)"), "Lowpoly session lacks bounded transient completion");
    console.log("lowpoly interactive-job owned source/fixture ok: 47 Migrated, 0 BatchOnlyPendingRewrite");

    const ajv = new Ajv2020({ allErrors: true, strict: true, allowUnionTypes: true });
    const validateOracle = ajv.compile(schema);
    reject(validateOracle(fixture), `Ajv oracle rejected canonical fixture: ${ajv.errorsText(validateOracle.errors)}`);
    const hostiles: Fixture[] = [
      { ...structuredClone(fixture), routes: fixture.routes.map((route, index) => index === 1 ? structuredClone(fixture.routes[0]!) : route) },
      { ...structuredClone(fixture), routes: fixture.routes.map((route) => route.classification === "Migrated" ? { ...route, lanes: [] } : route) },
      // 🧬️ Every route is now Migrated (0 BatchOnlyPendingRewrite) — a non-null blocker on a Migrated
      // route is the equivalent hostile mutation the old "empty blocker on BatchOnly" case exercised.
      { ...structuredClone(fixture), routes: fixture.routes.map((route, index) => index === 0 ? { ...route, blocker: "unexpected" } : route) },
      { ...structuredClone(fixture), routes: fixture.routes.map((route) => route.toolId === "paintStrokeEnd" ? { ...route, preparation: ["Config"] as ("Artifact" | "Config")[] } : route) },
    ];
    for (const hostile of hostiles) {
      reject(!validateOwnedFixture(hostile), "owned validator accepted hostile fixture");
      reject(!validateOracle(hostile), "Ajv oracle accepted hostile fixture");
    }
    console.log("lowpoly interactive-job Ajv hostile oracle ok: duplicate, missing lane, non-null blocker on migrated, lane/preparation mismatch rejected");
  }
}
//#endregion 🧪️InteractiveJobSourceTest

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });

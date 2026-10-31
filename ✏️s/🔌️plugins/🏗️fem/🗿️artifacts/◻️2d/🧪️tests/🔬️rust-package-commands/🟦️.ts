import AjvDraft07 from "ajv";
import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/framework/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/framework/process/cargo-laws";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/framework/process/cargo-test";
import { readCargoTestPolicyV1 } from "@semio-tech/framework/process/cargo-test";
import { scriptInvocationBudget } from "@semio-tech/framework/process/routing";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { resolveTypescriptCompilerV1 } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length !== 2 || segments[0] !== "fem2d-invariants" || !["source", "native", "mounted", "numerical", "presence"].includes(segments[1]!)) throw new Error("fem2d-invariants requires source, native or mounted");
  const sources = ["../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-contract/🟦️.ts", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts"].map(path => resolve(context.root, path));
  await runOwnedCommand(process.execPath, [resolveTypescriptCompilerV1(context.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...sources], context.repoRoot, "fem2d:source-typecheck", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
  const { testFem2dWindowConfigContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-contract/🟦️.ts");
  const { testFem2dEventMechanisms } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts");
  testFem2dWindowConfigContract(); testFem2dEventMechanisms();
  const Ajv = (await import("ajv/dist/2020.js")).default;
  const canvasSchema = await Bun.file(resolve(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🧬️schema/🔣️.json")).json();
  const fixture = await Bun.file(resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🧫️fixtures/🔣️.json")).json();
  const validate = new AjvDraft07({ strict: true }).compile(canvasSchema);
  const pagePolicy = { snapshotCapacity: 8, pageCapacity: fixture.visual.pageCapacity, pageBytes: fixture.visual.pageBytes };
  assert(validate(pagePolicy)); assert(!validate({ ...pagePolicy, pageCapacity: pagePolicy.pageCapacity + 1 }));
  const collision = fixture.generatedIdentityCollision;
  const uniqueElementIds = new Ajv({ strict: true }).compile({ type: "array", items: { type: "string" }, uniqueItems: true });
  assert(!uniqueElementIds([collision.authoredElementId, `${collision.regionId}_t${collision.triangleIndex}`]));
  const rustSources = ["../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🔬️unit/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🗣️terminology/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📊️results/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/👁️set-result-display/🧪️tests/🔬️unit/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️unit/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"].map(path => resolve(context.root, path));
  rustSources.push(resolve(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🦀️.rs"), resolve(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️canvas2d-snapshot-value-round-trip/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs"));
  rustSources.push(resolve(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"));
  await runOwnedCommand("rustfmt", ["--check", "--edition", "2021", "--config", "disable_all_formatting=true,skip_children=true", ...rustSources], context.repoRoot, "fem2d:rust-source-parse", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 1048576, maximumLines: 8192 } });
  if (segments[1] === "source") return;
  const policy = readCargoTestPolicyV1(process.env), manifest = resolve(context.root, "Cargo.toml");
  assert.equal(resolve(policy.manifestPath), manifest, "FEM2d exact laws require the original selected manifest");
  type LawTarget = { kind: "lib"; name: string } | { kind: "test"; name: string };
  type LawGroup = { package: string; manifest?: string; target: LawTarget; laws: string[] };
  const lawSets = await Bun.file(resolve(context.root, "🧪️tests/🧫️fixtures/⚖️verification-laws/🔣️.json")).json() as Record<"native" | "mounted" | "numerical" | "presence", LawGroup[]>;
  const lawGroups = lawSets[segments[1] as keyof typeof lawSets];
  const groups: { package: string; target: LawTarget; cargoArgs: string[]; laws: string[] }[] = lawGroups.map((group) => ({ package: group.package, target: group.target, cargoArgs: ["--locked"], laws: [...group.laws] }));
  const manifestPaths: Record<string, string> = Object.fromEntries(lawGroups.map((group) => [group.package, group.manifest ? resolve(context.repoRoot, group.manifest) : manifest]));
  if (segments[1] === "numerical") {
    manifestPaths["semio-framework-ui-scene"] = resolve(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml");
    groups.push({ package: "semio-framework-ui-scene", target: { kind: "lib", name: "semio_framework_ui_scene" }, cargoArgs: ["--locked"], laws: ["canvas2d_snapshot::value_round_trip_tests::canvas2d_page_policy_matches_schema_and_retires_exactly_one_page"] });
  }
  const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: policy.buildBudgetMs },manifestPaths,cwd: context.repoRoot,groups});
  assert.equal(receipts.length, groups.length);
  for (let index = 0; index < groups.length; index++) assert.equal(receipts[index]!.assertions, groups[index]!.laws.length);

}

/** 🏃️ Runs the `MaterialNativeScript` command under the caller's roots and invocation. */
export async function runMaterialNativeScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
assert(args.length===1&&["source","native"].includes(args[0]!),"test-material requires source or native");
const oracle=await import("../../../../⚙️engine/🧱️material/📋️table/🧪️tests/🟦️.ts");oracle.materialTableOracle();
if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env),pkg="semio-s-fem-material";await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(context.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(context.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:context.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{[pkg]:resolve(context.root,"../../../../⚙️engine/🧱️material/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:pkg,target:{kind:"lib",name:"semio_s_fem_material"},cargoArgs:["--offline"],laws:["planar::native_material_tests::native_material_receives_canonical_and_authored_original_values","planar::native_material_tests::native_material_receives_actual_declared_text_ordinals","planar::native_material_tests::native_material_refuses_invalid_parts_without_dropping_partial_fields","planar::native_material_tests::table::native_material_table_receives_original_order_and_closes_cancelled_frontiers"]}]});}
 }

/** 🏃️ Runs the `FixedDofScript` command under the caller's roots and invocation. */
export async function runFixedDofScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
assert(args.length===1&&["source","native"].includes(args[0]!),"test-fixed-dofs requires source or native");
const source=resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🎟️fixed-dofs/🧪️tests/🟦️.ts");
await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(context.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],context.repoRoot,"fem:fixed-dofs:types",scriptInvocationBudget(context.invocation,30000),{signal:context.invocation.control.signal});
await runOwnedCommand(process.execPath,["test",source],context.repoRoot,"fem:fixed-dofs:source",scriptInvocationBudget(context.invocation,30000),{signal:context.invocation.control.signal});
if(args[0]==="source")return;
const manifestPaths={"semio-s-artifact-fem-2d-model":resolve(context.root,"../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml"),"semio-s-artifact-fem-3d-model":resolve(context.root,"../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml")};
await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:context.repoRoot,manifestPaths,groups:[["semio-s-artifact-fem-2d-model","structure_laws"],["semio-s-artifact-fem-3d-model","fixed_dofs_laws"]].map(([packageName,name])=>({package:packageName,target:{kind:"test" as const,name:name!},cargoArgs:["--locked"],laws:["fixed_dof_tests::fixed_dof_boundary_matches_neutral_canonical_sets"]}))});
 }

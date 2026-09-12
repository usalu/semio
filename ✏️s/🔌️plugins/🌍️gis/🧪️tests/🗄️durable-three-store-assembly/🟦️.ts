import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { compileGisScopeExport } from "../../🧬️schema/🟦️.ts";

const GIS_SCHEMA_MODULE = "✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json";

/** 🏗️ Proves the exact fixed-three GIS assembly schema, typed factory ports, and private Store bind. */
export async function proveGisDurableThreeStoreAssembly(repoRoot: string): Promise<void> {
  const fixtureRoot = join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🧫️fixtures/🗄️durable-three-store-assembly");
  const fixtureBytes = readFileSync(join(fixtureRoot, "🔣️.json"));
  const fixture = JSON.parse(fixtureBytes.toString("utf8"));
  const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisDurableThreeStoreAssembly");
  if (!validate(fixture)) throw new Error(`invalid GIS durable three-Store assembly corpus: ${JSON.stringify(validate.errors)}`);
  const identity = [
    fixture.schema,
    fixture.shape,
    fixture.operation,
    fixture.actor,
    `parent:${fixture.members[0].document}`,
    `drawing:${fixture.members[1].document}:${fixture.members[1].owner}`,
    `value:${fixture.members[2].document}:${fixture.members[2].owner}`,
  ].join("|");
  const nodeHash = createHash("sha256").update(identity).digest("hex");
  const webHash = Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(identity))).toString("hex");
  if (identity !== fixture.canonicalIdentity || nodeHash !== fixture.canonicalIdentitySha256 || webHash !== nodeHash) throw new Error("GIS durable assembly identity oracle mismatch");
  if (JSON.stringify(fixture.members.map((member: any) => member.role)) !== JSON.stringify(["parent", "drawing", "value"])) throw new Error("GIS durable assembly role order changed");
  if (JSON.stringify(fixture.preparationOrder) !== JSON.stringify(["parent", "drawing", "value"])) throw new Error("GIS durable assembly preparation order changed");
  if (fixture.invariants.callerSuppliesGroupId || fixture.invariants.exposesPreparedSeal || fixture.invariants.publishBeforeAllPrepared || fixture.invariants.terminalReturnsStores !== 3 || fixture.invariants.journalBegins !== 1) {
    throw new Error("GIS durable assembly ownership invariants changed");
  }
  if (fixture.cancellationCases.some((row: any) => row.journalBegins !== 0 || row.terminalStores !== 3) || fixture.rejectionCases.some((row: any) => row.terminalStores !== 3)) {
    throw new Error("GIS durable assembly terminal owner trace changed");
  }
  const storeSource = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs"), "utf8");
  const assembly = storeSource.slice(storeSource.indexOf("pub struct DurableOwnedThreeStoreMapAssemblyV1"), storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreBoundV1"));
  for (const marker of ["DurableOwnedMapMemberAdmissionV1", "AdmittingParent", "AdmittingDrawing", "AdmittingValue", "PreparingParent", "PreparingDrawing", "PreparingValue", "take_assembly_prepared", "bind_store_owned", "mount_map", "take_mounted_host", "take_terminal_owners"]) {
    if (!assembly.includes(marker)) throw new Error(`Store durable assembly missing ${marker}`);
  }
  if (assembly.includes("begin_member_apply_batch") || assembly.includes("group_id:") || !assembly.includes("begin_apply_batch(")) throw new Error("Store durable assembly admits a caller group or catalog factory shortcut");
  const journal = storeSource.slice(storeSource.indexOf("DurableOwnedThreeStoreCommitPhaseV1::StartingJournal =>"), storeSource.indexOf("DurableOwnedThreeStoreCommitPhaseV1::Journal =>"));
  if (!journal.includes("sink.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?") || (journal.match(/\.begin_commit\(/gu) ?? []).length !== 1) throw new Error("Store durable journal does not begin exactly once");
  const gisSource = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), "utf8");
  for (const builder of ["gis_map_parent_one_item_preparation_factory", "gis_map_drawing_one_item_preparation_factory", "gis_map_value_one_item_preparation_factory"]) {
    if (!gisSource.includes(builder)) throw new Error(`GIS exact preparation port missing ${builder}`);
  }
  console.log(`gis-durable-three-store-assembly-oracle: AJV=1 SHA256=node+webcrypto roles=3 cancellation=${fixture.cancellationCases.length} rejection=${fixture.rejectionCases.length}; no WAL recovery or Hub publication claim`);
}


export class DurableThreeStoreAssemblyCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisDurableThreeStoreAssembly(this.repoRoot);
  }
}

export class DurableThreeStoreAssemblyNativeCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisDurableThreeStoreAssembly(this.repoRoot);
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      groups: [
        { package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: [
          "durable_group::tests::durable_map_three_store_assembly_uses_exact_gis_factories_and_binds_one_decision",
          "durable_group::tests::durable_map_three_store_assembly_late_member_rejection_closes_prior_publications_before_owner_handoff",
          "durable_group::tests::durable_map_three_store_assembly_cancellation_before_journal_restores_all_three_frontiers",
          "durable_group::tests::durable_map_three_store_assembly_uncertain_journal_retains_same_host_until_committed_or_proven_absent",
        ] },
        { package: "semio-s-artifact-gis-gismap", target: { kind: "lib" }, cargoArgs: ["--no-default-features", "--features", "component-app-assembly"], laws: [
          "editor::gis2d::component::tests::gis_map_durable_three_store_factory_builders_are_exact_role_ports",
        ] },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`gis-durable-three-store-assembly ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`gis-durable-three-store-assembly-receipt: ${JSON.stringify(receipt)}`);
  }
}


#!/usr/bin/env bun
/** 🌊️ `@semio-tech/flow-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runCargo, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";

//#region 🧪️Validation
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "-p", "semio-s-plugin-flow", ...(segments.length ? segments : ["--lib"])], this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-flow"], this.repoRoot, rest, { ...process.env, RUST_TEST_THREADS: "1" });
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts");
  }
}

class ChildIdentityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await import("../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts");
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      cargoArgs: segments,
      groups: [
        {
          package: "semio-s-artifact-flow-flow",
          target: { kind: "lib" },
          laws: [
            "command_ids_are_unique_and_match_the_declared_manifest_actions",
            "every_command_round_trips_through_text_and_binary",
            "every_printed_op_line_starts_with_the_rows_wire_keyword",
            "scene_identity_matches_node_crypto_and_adopts_the_exact_root",
            "every_artifact_variant_matches_serde_bytes_including_nested_chrome",
            "large_unicode_key_and_label_scene_matches_serde_without_an_ordinal_map_scan",
            "flow_parent_projection_and_child_identity_match_neutral_corpus",
            "flow_store_owners_retire_all_durable_lanes_with_neutral_byte_grants",
            "flow_presence_store_owners_preserve_readers_and_retire_neutral_byte_grants",
            "flow_empty_transient_close_matches_neutral_trace_and_exact_owner",
            "flow_viewer_member_factory_and_full_store_close_match_neutral_contract",
            "flow_render_fixture_projection_retires_populated_and_rejected_pages",
          ],
        },
        { package: "semio-s-plugin-flow", target: { kind: "lib" }, laws: ["flow_actual_surface_factories_close_all_owners_under_neutral_grants"] },
      ],
    });
    console.log(`[DEBUG] Flow child identity native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}

class ChildEditCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await import("../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts");
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      cargoArgs: segments,
      groups: [
        {
          package: "semio-s-artifact-flow-flow",
          target: { kind: "lib" },
          laws: [
            "child_add_widget_uses_the_smallest_available_identity_and_the_descriptor_default_payload",
            "child_add_widget_preserves_every_descriptor_payload_and_neuron_port_default",
            "child_add_widget_rejects_an_explicit_identity_collision",
            "add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content",
          ],
        },
      ],
    });
    console.log(`[DEBUG] Flow typed child edit native law: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}

class AddWidgetRetainedCheckScript extends BundleScript {
  async oracle(): Promise<void> {
    const root = new URL("../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/", import.meta.url);
    const fixture = await Bun.file(new URL("🧫️fixtures/🧵️add-widget-retained/🔣️.json", root)).json();
    const schemaModule = await Bun.file(new URL("🧫️fixtures/🧬️schema/🔣️.json", root)).json();
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    for (const keyword of ["x-semio-state", "x-semio-child-kind", "x-semio-child-standard", "x-semio-child-subset"]) ajv.addKeyword({ keyword, metaSchema: { type: "string" } });
    for (const numeric of ["double", "float", "int32", "int64", "uint32", "uint64"]) ajv.addFormat(numeric, true);
    for (const schema of ["🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json"]) ajv.addSchema(await Bun.file(join(this.repoRoot, schema)).json());
    const validate = ajv.addSchema(schemaModule).compile({ $ref: `${schemaModule.$id}#/$defs/FlowAddWidgetRetained` });
    assert(validate(fixture), JSON.stringify(validate.errors));
    const deny = { accepted: false, sessionCalls: 0, parentMutations: 0, childGroups: 0, visibleGroups: 0 };
    const model = (request: any) => {
      const textBytes = Buffer.byteLength(request.command.kind) + Buffer.byteLength(request.command.neuronKind ?? "");
      const finite = (value: unknown) => typeof value === "number" && Number.isFinite(value);
      const dialect = request.child.dialect;
      const admitted =
        request.controller === fixture.registration.controller &&
        request.tool === fixture.registration.tool &&
        request.schema === fixture.registration.schema &&
        request.rawBytes <= fixture.limits.rawBytes &&
        request.checkpointBytes <= fixture.limits.checkpointBytes &&
        request.wire === "canonical" &&
        textBytes <= fixture.limits.textBytes &&
        finite(request.command.x) &&
        finite(request.command.y) &&
        request.child.present &&
        request.child.slot === "content" &&
        request.child.id === request.parent.childId &&
        dialect.artifactKind === "s.stdio.semio" &&
        dialect.standard === "v1" &&
        dialect.subset === "flow" &&
        request.child.nodes <= fixture.limits.childItems &&
        request.child.edges <= fixture.limits.childItems &&
        request.owner === "ready" &&
        request.cancel !== "before-work";
      if (!admitted) return deny;
      if (!["neuron", "inputSlider", "inputNote", "inputImage", "outputPreview", "outputAction", "outputExport", "variable"].includes(request.command.kind)) {
        return { accepted: false, sessionCalls: 1, parentMutations: 0, childGroups: 0, visibleGroups: 0 };
      }
      const visible = request.cancel !== "after-plan" && request.parent.revisionCurrent && request.parent.rootCurrent && request.child.revisionCurrent && !request.duplicateTarget;
      return { accepted: visible, sessionCalls: 1, parentMutations: 0, childGroups: 1, visibleGroups: visible ? 1 : 0 };
    };
    for (const row of fixture.accepted) {
      assert.deepEqual(row.inserted, { kind: row.command.kind, x: row.command.x, y: row.command.y }, `${row.id}: typed inserted node`);
      assert.deepEqual(model({ ...structuredClone(fixture.canonical), command: row.command }), row.expected, row.id);
    }
    const change = (request: any, kind: string): void => {
      if (kind === "wrong-controller") request.controller = "s.flow.other@1/*#editor";
      else if (kind === "wrong-tool") request.tool = "removeWidget";
      else if (kind === "wrong-schema") request.schema = "flow.other";
      else if (kind === "oversize-raw") request.rawBytes = fixture.limits.rawBytes + 1;
      else if (kind === "oversize-checkpoint") request.checkpointBytes = fixture.limits.checkpointBytes + 1;
      else if (kind === "malformed-wire") request.wire = "malformed";
      else if (kind === "trailing-wire") request.wire = "trailing";
      else if (kind === "oversize-kind") request.command.kind = "k".repeat(fixture.limits.textBytes + 1);
      else if (kind === "oversize-neuron-kind") request.command.neuronKind = "n".repeat(fixture.limits.textBytes + 1);
      else if (kind === "unknown-descriptor") request.command.kind = "notAFlowWidget";
      else if (kind === "nonfinite-x") request.command.x = "nan";
      else if (kind === "nonfinite-y") request.command.y = "positive-infinity";
      else if (kind === "missing-child") request.child.present = false;
      else if (kind === "wrong-slot") request.child.slot = "preview";
      else if (kind === "wrong-child-id") request.child.id = "other-child";
      else if (kind === "wrong-dialect") request.child.dialect.subset = "text";
      else if (kind === "stale-parent") request.parent.revisionCurrent = false;
      else if (kind === "stale-root") request.parent.rootCurrent = false;
      else if (kind === "stale-child") request.child.revisionCurrent = false;
      else if (kind === "duplicate-target") request.duplicateTarget = true;
      else if (kind === "owner-busy") request.owner = "busy";
      else if (kind === "owner-closing") request.owner = "closing";
      else if (kind === "cancel-before-work") request.cancel = "before-work";
      else if (kind === "cancel-after-plan") request.cancel = "after-plan";
      else assert.fail(`unknown retained addWidget change ${kind}`);
    };
    assert.equal(new Set(fixture.denials.map((row: any) => row.change)).size, 24);
    for (const row of fixture.denials) {
      const request = structuredClone(fixture.canonical);
      change(request, row.change);
      assert.deepEqual(model(request), row.expected, row.id);
    }
    const source = await Bun.file(new URL("🦀️.rs", root)).text();
    for (const witness of [
      'const FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget"]',
      "struct FlowChildGroupWork",
      "impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowChildGroupWork",
      "struct FlowChildGroupJobFactory",
      "ArtifactToolPublicationLane::Child",
      "FlowChildGroupJobFactoryProofs::bounded_first_step_tool_proofs()",
      "registry.register(FlowChildGroupJobFactory::new(&controller))",
      "ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES",
      "FlowCommand::AddWidget(payload)",
      "semio_framework_plugin::InteractiveJobClassification::Migrated",
    ])
      assert(source.includes(witness), `missing retained addWidget source witness: ${witness}`);
    assert(source.includes('dialect.artifact_kind != "s.stdio.semio"') && source.includes('dialect.standard != "v1"') && source.includes('dialect.subset != "flow"'), "retained addWidget must bind the exact captured child dialect");
    assert(source.includes("add_widget::handle(payload, &view") && source.includes("FlowInstanceOperationOwner"), "retained addWidget must reuse the typed child planner under the instance session owner");
    const retainedCommandSource = await Bun.file(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs")).text();
    assert.deepEqual(fixture.progress, { en: "Applying command", de: "Befehl wird angewendet" });
    assert(retainedCommandSource.includes('{"en":"Applying command","de":"Befehl wird angewendet"}'), "retained addWidget progress must bind the production bilingual preview");
    assert(source.includes('action_interactive_job("addWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)'), "the manifest must expose one migrated addWidget route");
    assert(source.includes("FLOW_CHILD_GROUP_TOOL_IDS.contains(&command.command_id())"), "legacy addWidget dispatch must fail closed");
    console.log(`[DEBUG] Flow retained addWidget oracle: ${fixture.accepted.length} accepted + ${fixture.denials.length} hostile cases; session/publication model is independent; runtimeClaims=0`);
  }

  async run(segments: string[]): Promise<void> {
    await this.oracle();
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      cargoArgs: segments,
      groups: [{ package: "semio-s-artifact-flow-flow", target: { kind: "lib" }, laws: ["retained_add_widget_factory_is_exact_child_only_and_legacy_closed", "retained_add_widget_dispatches_one_acknowledged_child_group_and_retires"] }],
    });
    console.log(`[DEBUG] Flow retained addWidget native laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}
//#endregion 🧪️Validation

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-flow", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("test-source", SourceTestScript)
  .register("child-identity-check", ChildIdentityCheckScript)
  .register("child-edit-check", ChildEditCheckScript)
  .register("add-widget-retained-check", AddWidgetRetainedCheckScript)
  .register("describe", DescribeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });

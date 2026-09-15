/**
 * @emoji 🧭 Language-neutral guardrails for snapshot / fixture / asset naming in generation3d.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

function read(relative: string): string {
  return readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8");
}

/** 🧭 Scans generation3d sources for forbidden fixture-as-host-document symbols. */
export function generation3dSnapshotFixtureAssetSelfTests(): number {
  const sources = [
    read("../../../../🧬️schema/🦀️.rs"),
    read("../../../../🧬️schema/🧬️mutations/🦀️.rs"),
    read("../../../../🧬️schema/📸️snapshot/🦀️.rs"),
    read("../../../../🧬️schema/🔗️.graphql"),
    read("../../../../🧬️schema/🟦️.ts"),
  ];
  for (const source of sources) {
    assert(!/\bfixture:\s*&\w+Snapshot\b/.test(source), "snapshot parameters must not be named fixture");
    assert(!/\bgeneration3d_fixture_operations\b/.test(source), "use generation3d_host_snapshot_operations");
    assert(!/\bgeneration_fixture_for\b/.test(source), "use generation_host_snapshot_for");
    assert(!/\bflow_fixture_to_form_spec\b/.test(source), "use flow_host_snapshot_to_form_spec");
    assert(!/pub fixture:\s*FlowHostSnapshot/.test(source), "Generation3dSnapshot fields must be host_snapshot");
    assert(!/\bdag_fixture_/.test(source), "DAG host snapshot helpers must use dag_host_snapshot_* names");
    assert(!/\bevaluate_fixture\b/.test(source), "use evaluate_host_snapshot for flow host snapshot eval");
    assert(!/\bFlowFixture\b/.test(source), "schema twins must use FlowHostSnapshot, not FlowFixture");
    assert(!/\bfixture:\s*FlowHostSnapshot/.test(source), "schema twins must use hostSnapshot field name");
  }
  console.log("[DEBUG] Generation3d snapshot/fixture/asset terminology lint: 5 sources, 0 forbidden patterns");
  return sources.length;
}

/**
 * @emoji 🧭 Language-neutral guardrails for snapshot / fixture / asset naming in generation2d.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

function read(relative: string): string {
  return readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8");
}

/** 🧭 Scans generation2d sources for forbidden fixture-as-host-document symbols. */
export function generation2dSnapshotFixtureAssetSelfTests(): number {
  const sources = [
    read("../../../../🧬️schema/🦀️.rs"),
    read("../../../../🧬️schema/🧬️mutations/🦀️.rs"),
    read("../../../../🧬️schema/📸️snapshot/🦀️.rs"),
    read("../../../../🧬️schema/🔗️.graphql"),
    read("../../../../🧬️schema/🟦️.ts"),
  ];
  for (const source of sources) {
    assert(!/\bfixture:\s*&\w+Snapshot\b/.test(source), "snapshot parameters must not be named fixture");
    assert(!/\bdag_fixture_/.test(source), "DAG host snapshot helpers must use dag_host_snapshot_* names");
    assert(!/\bgeneration2d_fixture_operations\b/.test(source), "use generation2d_host_snapshot_operations");
    assert(!/\bflow_fixture_to_form_spec\b/.test(source), "use flow_host_snapshot_to_form_spec");
    assert(!/pub fixture:\s*FlowHostSnapshot/.test(source), "Generation2dSnapshot fields must be host_snapshot");
    assert(!/\.replace_fixture\s*\(/.test(source), "FlowHost must use replace_host_snapshot");
    assert(!/\bFlowFixture\b/.test(source), "schema twins must use FlowHostSnapshot, not FlowFixture");
    assert(!/\bfixture:\s*FlowHostSnapshot/.test(source), "schema twins must use hostSnapshot field name");
  }
  console.log("[DEBUG] Generation2d snapshot/fixture/asset terminology lint: 5 sources, 0 forbidden patterns");
  return sources.length;
}

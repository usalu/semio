/**
 * @emoji 🧭 Language-neutral guardrails for snapshot / fixture / asset naming in sequence.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

function read(relative: string): string {
  return readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8");
}

/** 🧭 Scans sequence sources for forbidden fixture-as-host-document symbols. */
export function sequenceSnapshotFixtureAssetSelfTests(): number {
  const sources = [
    read("../../../../🧬️schema/🦀️.rs"),
    read("../../../../🧬️schema/📸️snapshot/🦀️.rs"),
    read("../../../../🧬️schema/🧬️mutations/🦀️.rs"),
  ];
  for (const source of sources) {
    assert(!/\bfixture:\s*&\w+Snapshot\b/.test(source), "snapshot parameters must not be named fixture");
    assert(!/\bSequenceFixture\b/.test(source), "use SequenceHostDocument");
    assert(!/\.to_fixture\s*\(/.test(source), "SequenceSnapshot bridges must call to_host_document");
  }
  console.log("[DEBUG] Sequence snapshot/fixture/asset terminology lint: 3 sources, 0 forbidden patterns");
  return sources.length;
}

import { expect, test } from "bun:test";
import { policyReadRustPolicySource } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";

const root = process.cwd();
const classifier = policyReadRustPolicySource(root, '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑\u200d🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs');
const spawn = policyReadRustPolicySource(root, '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑\u200d🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs');

test("deadline is not a maintenance terminal fault", () => {
  expect(classifier).toContain("pub fn frame_maintenance_terminal_fault(cancelled: bool, stale: bool)");
  expect(classifier).toContain("frame maintenance cancelled");
  expect(classifier).toContain("frame maintenance generation became stale");
  expect(classifier).not.toContain("frame maintenance deadline expired");
  expect(classifier).not.toContain("deadline_exceeded");
  expect(spawn).toContain("deadline_ms.is_some_and(|deadline| worker_pool.now_ms() > deadline)");
  expect(spawn).toContain("frame_maintenance_terminal_fault(cancelled, stale)");
  expect(spawn).not.toContain("frame_maintenance_terminal_fault(cancelled, stale,");
  expect(spawn).toContain("} else if deadline_passed {");
  expect(spawn).toContain("cursor.shell_maintenance = false;");
  expect(spawn).toContain("cursor.begin_close();");
  expect(spawn).toContain("let cancelled = cursor.cancel.as_ref().is_none_or(|cancel| cancel.is_cancelled_now());");
});

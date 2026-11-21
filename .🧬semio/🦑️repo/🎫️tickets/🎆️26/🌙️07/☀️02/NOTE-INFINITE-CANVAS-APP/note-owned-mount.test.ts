import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json", "utf8"));
const mount = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs", "utf8").split("async fn plugin_mount_surface")[1].split("async fn ")[0];
const registry = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs", "utf8").split("pub fn insert(")[1].split("pub fn ")[0];

test("the host window supplies the mounted utility and kind", () => {
  using database = new Database(":memory:");
  const route = database.query(`
    SELECT
      json_extract(window.value, '$.windowKindId') AS kind,
      (SELECT value FROM json_each($utilities) WHERE key = $window) AS utility
    FROM (SELECT 1)
    LEFT JOIN json_each($windows) AS window ON json_extract(window.value, '$.id') = $window
  `);
  for (const surface of fixture.surfaces) {
    expect(route.get({
      $windows: JSON.stringify(fixture.view.windowInstances),
      $utilities: JSON.stringify(fixture.view.activeUtilityByWindowId),
      $window: surface.windowId,
    })).toEqual({ kind: surface.windowId ? surface.bodyKey : null, utility: surface.activeUtilityId });
  }
});

test("the exact instance owns the decoded view before any window projection", () => {
  expect(mount.indexOf("find_instance(")).toBeGreaterThan(-1);
  expect(mount.indexOf("find_instance(")).toBeLessThan(mount.indexOf("decode_wire_serialized::<ViewModel>"));
  expect(mount).toContain("instance.surface_contexts.insert(surface, body_key, view)");
  expect(mount).not.toContain(".for_window_instance(");
  expect(registry).toContain("role.project(&view_state)");
  expect(registry).toContain("self.displaced.push(Displaced::View(view_state))");
  expect(registry).toContain("self.view_state.replace(view_state)");
  expect(registry).toContain("view_state.active_utility_id.clone_from(&projected.active_utility_id)");
});

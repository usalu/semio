import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const manifest = "🧰️framework/🔨️modules/🛂️manifest";
const value = "🧰️framework/🔨️modules/🌱️value";
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const fixture = JSON.parse(readFileSync(`${manifest}/🧪️tests/🧫️fixtures/🔬️window-view-context/🔣️.json`, "utf8"));

test("window section inputs keep distinct same-kind utilities and cursor identities", () => {
  using db = new Database(":memory:");
  const scope = db.query(`SELECT
    json_extract(window.value,'$.id') AS windowId,
    json_extract(window.value,'$.windowKindId') AS kindId,
    utility.value AS activeUtilityId,
    trace.value ->> '$.run' AS traceRun
    FROM json_each($view,'$.windowInstances') AS window
    LEFT JOIN json_each($view,'$.activeUtilityByWindowId') AS utility ON utility.key = json_extract(window.value,'$.id')
    LEFT JOIN json_each($view,'$.toolRunTraceCursorByWindowId') AS trace ON trace.key = json_extract(window.value,'$.id')
    WHERE json_extract(window.value,'$.id') = $id`);
  for (const clamp of fixture.cases) {
    const row = scope.get({ $view: JSON.stringify(fixture.view), $id: clamp.windowId });
    expect(row).toEqual(clamp.absent ? null : {
      windowId: clamp.windowId,
      kindId: "graph",
      activeUtilityId: clamp.activeUtilityId,
      traceRun: clamp.traceRun ?? null,
    });
  }
});

test("the section runtime scopes both maps to the exact live donor through early return", () => {
  const view = readFileSync(`${manifest}/🖼️view-model/🦀️.rs`, "utf8");
  const ordered = readFileSync(`${value}/🗂️ordered/🦀️.rs`, "utf8");
  const render = readFileSync(`${plugin}/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`, "utf8");
  const guard = view.split("impl Drop for ScopedWindowView")[1]?.split("\n}")[0] ?? "";
  expect(guard).toContain("active_utility_by_window_id.return_scoped_alias(&self.original.active_utility_by_window_id)");
  expect(guard).toContain("tool_run_trace_cursor_by_window_id.return_scoped_alias(&self.original.tool_run_trace_cursor_by_window_id)");
  const close = ordered.split("pub fn return_scoped_alias")[1]?.split("\n    }")[0] ?? "";
  expect(close).toContain("alias.same_original(parent)");
  expect(close).toContain("alias.elect_cold().is_none()");
  for (const [section, next] of [["window_engagements", "window_measures"], ["window_measures", "tool_measures"]]) {
    const method = render.split(`pub async fn ${section}`)[1].split(`pub async fn ${next}`)[0];
    expect(method).toContain("view_state.scope_window_instance(&window.id)");
    expect(method).toContain("Err(_) => continue");
    expect(method).not.toContain("view_state.for_window_instance");
  }
});

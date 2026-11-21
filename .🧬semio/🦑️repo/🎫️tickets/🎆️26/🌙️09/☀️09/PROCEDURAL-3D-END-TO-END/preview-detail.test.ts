import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { JSDOM } from "jsdom";

const fixture = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️preview/🧪️tests/🧫️fixtures/🔁️replacement.json").json();

test("a request reserves one slot for each handle and skips only an exact resident detail", () => {
  const cache = new Database(":memory:", { strict: true });
  cache.exec("CREATE TABLE resident (handle TEXT PRIMARY KEY, tolerance REAL); CREATE TABLE pending (handle TEXT PRIMARY KEY)");
  for (const row of fixture.admission) {
    cache.exec("DELETE FROM resident; DELETE FROM pending");
    if (row.resident !== null) cache.run("INSERT INTO resident VALUES ('surface', ?)", [row.resident]);
    if (row.inFlight) cache.run("INSERT INTO pending VALUES ('surface')");
    const result = cache.run(
      "INSERT OR IGNORE INTO pending SELECT 'surface' WHERE NOT EXISTS (SELECT 1 FROM resident WHERE handle = 'surface' AND tolerance = ?)",
      [row.requested],
    );
    expect(result.changes === 1).toBe(row.admitted);
    expect(cache.query("SELECT tolerance FROM resident").get()).toEqual(row.resident === null ? null : { tolerance: row.resident });
  }
  cache.close();
});

test("committing new detail replaces one resident pack and keeps the prior pack until commit", () => {
  const cache = new Database(":memory:", { strict: true });
  cache.exec("CREATE TABLE resident (handle TEXT PRIMARY KEY, tolerance REAL, packed_revision INTEGER)");
  for (const [index, tolerance] of fixture.toleranceRounds.entries()) {
    expect(cache.query("SELECT packed_revision FROM resident WHERE handle = 'surface'").get()).toEqual(index ? { packed_revision: index - 1 } : null);
    cache.run("INSERT OR REPLACE INTO resident VALUES ('surface', ?, ?)", [tolerance, index]);
    expect(cache.query("SELECT COUNT(*) AS count FROM resident").get()).toEqual({ count: fixture.expectedEntries });
    expect(cache.query("SELECT packed_revision FROM resident WHERE tolerance = ?").get(tolerance)).toEqual({ packed_revision: index });
  }
  cache.close();
});

test("a reply without an active lease preserves current detail and other pending requests", async () => {
  const law = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/🎚️preview-detail-ownership.json").json();
  const cache = new Database(":memory:", { strict: true });
  cache.exec("CREATE TABLE resident (handle TEXT PRIMARY KEY, tolerance REAL); CREATE TABLE pending (id INTEGER PRIMARY KEY, handle TEXT)");
  cache.run("INSERT INTO resident VALUES (?, ?)", [law.handle, law.cacheTolerance]);
  cache.run("INSERT INTO pending VALUES (?, ?)", [law.liveHash, law.handle]);
  const result = cache.run("INSERT OR REPLACE INTO resident SELECT handle, ? FROM pending WHERE id = ?", [law.tolerance, law.staleHash]);
  expect(result.changes).toBe(0);
  expect(cache.query("SELECT id FROM pending").values().flat()).toEqual(law.remainingPending);
  expect(cache.query("SELECT tolerance FROM resident").get()).toEqual({ tolerance: law.cacheTolerance });
  cache.close();
});

test("a failed detail settles only its original request and leaves other tolerances available", async () => {
  const law = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🧫️fixtures/🎚️preview-detail-ownership.json").json();
  const row = law.fault;
  const cache = new Database(":memory:", { strict: true });
  cache.exec("CREATE TABLE requests (handle TEXT, tolerance REAL, state TEXT, PRIMARY KEY (handle, tolerance))");
  cache.run("INSERT INTO requests VALUES (?, ?, 'pending')", [law.handle, row.tolerance]);
  cache.run("UPDATE requests SET state = 'failed' WHERE handle = ? AND tolerance = ?", [law.handle, row.tolerance]);
  expect(cache.query("SELECT COUNT(*) AS count FROM requests WHERE state = 'pending'").get()).toEqual({ count: 0 });
  expect(cache.query("SELECT COUNT(*) AS count FROM requests WHERE state = 'failed' AND tolerance = ?").get(row.tolerance)).toEqual({ count: row.diagnostics });
  expect(cache.run("INSERT OR IGNORE INTO requests VALUES (?, ?, 'pending')", [law.handle, row.tolerance]).changes).toBe(0);
  expect(cache.run("INSERT OR IGNORE INTO requests VALUES (?, ?, 'pending')", [law.handle, row.nextTolerance]).changes).toBe(1);
  cache.close();
});

const modeFixture = await Bun.file("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎛️generate-mode-interactions.json").json();
const detail = modeFixture.previewDetail;

for (const language of ["en", "de"]) {
  test(`editor preview detail in ${language}`, () => {
    const browser = new JSDOM("<label></label><select></select>");
    try {
      const { document, Event, Option } = browser.window;
      const select = document.querySelector("select")!;
      select.id = detail.id;
      const label = document.querySelector("label")!;
      label.htmlFor = detail.id;
      label.textContent = detail[language];
      for (const preset of detail.presets) select.add(new Option(preset[language], preset.value));
      select.value = detail.default;
      expect(select.labels?.[0].textContent).toBe(detail[language]);
      expect([...select.options].map(option => option.value)).toEqual(detail.presets.map(preset => preset.value));
      expect(select.selectedOptions[0].textContent).toBe(detail.presets.find(preset => preset.value === detail.default)[language]);
      const actions: { action: string; args: { value: string } }[] = [];
      select.addEventListener("change", () => actions.push({ action: detail.action, args: { value: select.value } }));
      for (const preset of detail.presets) {
        select.value = preset.value;
        select.dispatchEvent(new Event("change"));
      }
      expect(actions).toEqual(detail.presets.map(preset => ({ action: detail.action, args: { value: preset.value } })));
      expect(modeFixture.windows.filter(window => modeFixture.previewParity.includes(window.id)).every(window => window.actions.includes(detail.action))).toBe(true);
      expect(detail.presets.map(preset => preset.tolerance)).toEqual([0.15, 0.05, 0.02]);
    } finally {
      browser.window.close();
    }
  });
}

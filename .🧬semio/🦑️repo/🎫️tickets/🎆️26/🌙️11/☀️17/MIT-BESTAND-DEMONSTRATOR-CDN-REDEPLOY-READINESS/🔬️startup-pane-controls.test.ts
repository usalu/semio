import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import fixture from "./🔣️pane-startup-control-cases.json";
import { controlIsUnnamed } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🟦️.ts";

import { geometryFromMesh } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🎨️mesh-render/🔺️mesh-geometry-and-appearance/🟦️.ts";

const producer = readFileSync("✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs", "utf8");
const terminology = readFileSync("✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs", "utf8");
const preflight = process.env.SEMIO_VERIFY_UNNAMED_BASELINE === "1";

test("the authored AUSSUCHEN startup filters satisfy retained control naming", () => {
  const localeCodes = ["en", "de"] as const;
  const statement = new Database(":memory:").query<{ unnamed: number }, [string, string | null, string]>(
    "select case when ?1 in ('input', 'select') and length(trim(coalesce(?2, ''))) = 0 and ?3 <> 'field' then 1 else 0 end as unnamed",
  );
  expect(producer).toContain('ui::row().try_id("sourcing-pool-filters")');
  expect(producer).toContain('children.try_push(filter_bar(cfg, labels)?)');
  const unnamed: string[] = [];
  const observedLabels: (string | undefined)[] = [];
  const expectedLabels: string[] = [];
  for (const control of fixture.controls) {
    const line = producer.split("\n").find((line) => line.includes('try_id("' + control.key + '")'));
    expect(line).toBeDefined();
    const labelKey = preflight ? undefined : line?.match(/\.try_label\(labels\.(\w+)\.as_str\(\)\)/)?.[1];
    const labelSource = terminology.split("\n").find((line) => line.trimStart().startsWith(labelKey + ":"));
    for (const locale of localeCodes) {
      const label = labelSource?.match(new RegExp('native_' + locale + ' "([^"]+)"'))?.[1];
      const parent = { component: fixture.parent, accessibility: {} } as Parameters<typeof controlIsUnnamed>[0];
      const node = { component: { type: control.type }, accessibility: { label } } as Parameters<typeof controlIsUnnamed>[0];
      const oracle = statement.get(control.type, label ?? null, fixture.parent.role)?.unnamed;
      expect(controlIsUnnamed(node, parent)).toBe(oracle === 1);
      if (controlIsUnnamed(node, parent)) unnamed.push(control.key + ":" + locale);
      observedLabels.push(label);
      expectedLabels.push(control.labels[locale]);
    }
  }
  expect(unnamed).toEqual([]);
  expect(observedLabels).toEqual(expectedLabels);
});

test("selected FEM entities copy their row name onto the interactive child", () => {
  const inspector = readFileSync("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs", "utf8");
  const femTerminology = readFileSync("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🗣️terminology/🦀️.rs", "utf8");
  const attachesName = !preflight && inspector.includes("control.accessibility.label = Some(label.clone());");
  const statement = new Database(":memory:").query<{ unnamed: number }, [string, string | null, string | null]>(
    "select case when ?1 in ('input', 'select', 'slider', 'toggle') and length(trim(coalesce(?2, ''))) = 0 and length(trim(coalesce(?3, ''))) = 0 then 1 else 0 end as unnamed",
  );
  const missing: string[] = [];
  for (const control of fixture.femControls) {
    const producerRow = inspector.split("\n").find((line) => line.includes('("' + control.row + '", labels.'));
    const labelKey = producerRow?.match(/labels\.(\w+)\.as_str\(\)/)?.[1];
    expect(labelKey).toBeDefined();
    const labelSource = femTerminology.split("\n").find((line) => line.trimStart().startsWith(labelKey + ":"));
    for (const locale of ["en", "de"] as const) {
      const text = labelSource?.match(new RegExp('native_' + locale + ' "([^"]+)"'))?.[1];
      expect(text).toBe(control.labels[locale]);
      const label = attachesName ? text : undefined;
      const toggleText = control.type === "toggle" ? text : undefined;
      const parent = { component: { type: "treeItem", label: text }, accessibility: {} } as Parameters<typeof controlIsUnnamed>[0];
      const node = { component: { type: control.type, text: toggleText }, accessibility: { label } } as Parameters<typeof controlIsUnnamed>[0];
      const oracle = statement.get(control.type, label ?? null, toggleText ?? null)?.unnamed;
      expect(controlIsUnnamed(node, parent)).toBe(oracle === 1);
      if (label !== text) missing.push(control.row + ":" + locale);
    }
  }
  expect(missing).toEqual([]);
});

test("FEM solid color output binds as opaque RGBA in the real renderer", () => {
  const scene = readFileSync("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/🦀️.rs", "utf8");
  const suffix = scene.match(/colors\.extend_from_slice\(&\[r, g, bl(?:, (1\.0))?\]\)/)?.[1];
  const alpha = preflight || suffix === undefined ? [] : [Number(suffix)];
  const { positions, indices, rgb } = fixture.solidMesh;
  const colors = rgb.flatMap((vertex) => [...vertex, ...alpha]);
  const statement = new Database(":memory:").query<{ valid: number }, [number, number]>(
    "select case when ?1 % 3 = 0 and (?1 / 3) * 4 = ?2 then 1 else 0 end as valid",
  );
  const valid = statement.get(positions.length, colors.length)?.valid === 1;
  const create = () => geometryFromMesh({ positions, indices, colors } as Parameters<typeof geometryFromMesh>[0]);
  if (valid) {
    const geometry = create();
    const color = geometry.getAttribute("color");
    expect(color.itemSize).toBe(4);
    expect(color.count).toBe(positions.length / 3);
    expect([...Array(color.count).keys()].map((vertex) => color.getW(vertex))).toEqual(rgb.map(() => 1));
    geometry.dispose();
  } else {
    expect(create).toThrow("color buffer cardinality differs from the vertex count");
  }
  expect(valid).toBe(true);
});

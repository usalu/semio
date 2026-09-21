import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import Ajv from "ajv";

type WidthCase = {
  readonly id: "minimumDefaultRoot" | "maximumDefaultRoot" | "minimumCustomRoot" | "intermediateCustomRoot" | "maximumCustomRoot";
  readonly rootRemPixels: number;
  readonly availablePixels: number;
  readonly expectedPixels: number;
  readonly expectedReservationPixels: number;
  readonly expectedPaintPixels: number;
};

const fixture = JSON.parse(
  readFileSync(resolve(process.cwd(), "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔝️navbar-centered-band/🔣️.json"), "utf8"),
) as {
  readonly exampleControlMetrics: {
    readonly controlId: string;
    readonly productionRootRemPixels: number;
    readonly minimumRem: number;
    readonly maximumRem: number;
    readonly forbidPostLayoutXOffset: boolean;
    readonly cases: readonly WidthCase[];
  };
  readonly taskManagerFooter: {
    readonly id: string;
    readonly anchor: string;
    readonly order: number;
    readonly iconId: string;
    readonly labels: readonly { readonly locale: string; readonly key: string; readonly label: string }[];
  };
};

const schema = JSON.parse(
  readFileSync(resolve(process.cwd(), "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔝️navbar-centered-band/🔣️.json"), "utf8"),
) as object;
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);

const resolveWidth = (available: number, rootRem: number, minimumRem: number, maximumRem: number): number =>
  Math.min(Math.max(available, minimumRem * rootRem), maximumRem * rootRem);

test("the neutral chrome metrics fixture satisfies its strict schema", () => {
  expect(validate(fixture), JSON.stringify(validate.errors)).toBeTrue();
});

test("the schema refuses unknown members, substituted metric rows, and duplicate locale rows", () => {
  const rows = fixture.exampleControlMetrics.cases;
  const labels = fixture.taskManagerFooter.labels;
  const invalid = [
    { ...fixture, exampleControlMetrics: { ...fixture.exampleControlMetrics, xOffsetPixels: 82 } },
    { ...fixture, exampleControlMetrics: { ...fixture.exampleControlMetrics, cases: [rows[0], rows[0], rows[2], rows[3], rows[4]] } },
    {
      ...fixture,
      exampleControlMetrics: {
        ...fixture.exampleControlMetrics,
        cases: rows.map((row) => row.id === "minimumCustomRoot" ? { ...row, expectedPaintPixels: 241 } : row),
      },
    },
    { ...fixture, taskManagerFooter: { ...fixture.taskManagerFooter, labels: [labels[0], labels[0]] } },
  ];
  for (const candidate of invalid) expect(validate(candidate)).toBeFalse();
});

test("example control bounds resolve before both reservation and paint", () => {
  const metrics = fixture.exampleControlMetrics;
  expect(metrics.controlId).toBe("playground.navbar.fixture");
  expect(metrics.productionRootRemPixels).toBe(16);
  expect(metrics.forbidPostLayoutXOffset).toBeTrue();
  expect(metrics.cases.map(({ id }) => id)).toEqual(["minimumDefaultRoot", "maximumDefaultRoot", "minimumCustomRoot", "intermediateCustomRoot", "maximumCustomRoot"]);
  for (const row of metrics.cases) {
    const width = resolveWidth(row.availablePixels, row.rootRemPixels, metrics.minimumRem, metrics.maximumRem);
    expect(width).toBe(row.expectedPixels);
    expect(width).toBe(row.expectedReservationPixels);
    expect(width).toBe(row.expectedPaintPixels);
  }
});

test("task manager keeps its footer identity, order, icon, and localized toggle labels", () => {
  const footer = fixture.taskManagerFooter;
  expect([footer.id, footer.anchor, footer.order, footer.iconId]).toEqual(["os.task-manager", "bottomRight", 2, "cpu"]);
  expect(footer.labels).toEqual([
    { locale: "en", key: "ui.panelToggle.taskManager", label: "Tasks" },
    { locale: "de", key: "ui.panelToggle.taskManager", label: "Aufgaben" },
  ]);
});

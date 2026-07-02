export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { formsPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for forms. */
export function buildFormsProgramDefinition(): PlatformDefinition {
	const app = formsPlayAppDefinition;
	return {
		id: "forms",
		name: "Forms",
		apiVersion: "1",
		apps: [{ id: "forms", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("forms-core", () => {
	const sampleSpec: FormSpec = {
		schema: "forms.form",
		id: "sample",
		version: "1",
		steps: [
			{
				id: "step-a",
				title: "Step A",
				questions: [
					{ id: "name", kind: "text", label: "Name", required: true },
					{ id: "show-extra", kind: "boolean", label: "Show extra", default: false },
					{
						id: "extra",
						kind: "text",
						label: "Extra",
						condition: { kind: "truthy", expr: { kind: "var", name: "show-extra" } },
					},
				],
			},
			{
				id: "step-b",
				title: "Step B",
				questions: [{ id: "count", kind: "slider", label: "Count", min: 0, max: 10, default: 3 }],
			},
		],
	};

	it("parses form spec", () => {
		const parsed = parseFormSpec(sampleSpec);
		expect(parsed.id).toBe("sample");
		expect(parsed.steps).toHaveLength(2);
	});

	it("rejects forbidden keys", () => {
		expect(() => parseFormSpec({ schema: "forms.form", id: "x", version: "1", steps: [], code: "bad" })).toThrow();
	});

	it("evaluates visibility conditions", () => {
		const runtime = new FormRuntime(sampleSpec);
		expect(runtime.getVisibleQuestions().map((q) => q.id)).toEqual(["name", "show-extra"]);
		runtime.setValue("show-extra", true);
		expect(runtime.getVisibleQuestions().map((q) => q.id)).toEqual(["name", "show-extra", "extra"]);
	});

	it("validates required fields on submit", () => {
		const runtime = new FormRuntime(sampleSpec);
		runtime.setValue("name", "");
		const result = runtime.submit();
		expect(result.ok).toBe(false);
		expect(result.errors.some((error) => error.questionId === "name")).toBe(true);
	});

	it("applies edit ops including cross-step move", () => {
		let spec = sampleSpec;
		spec = applyFormEditOp(spec, {
			op: "addQuestion",
			stepId: "step-a",
			question: { id: "temp", kind: "number", label: "Temp" },
		});
		spec = applyFormEditOp(spec, {
			op: "moveQuestion",
			questionId: "temp",
			fromStepId: "step-a",
			toStepId: "step-b",
			index: 0,
		});
		expect(findQuestionLocation(spec, "temp")?.stepId).toBe("step-b");
	});

	it("exposes question kind catalogue", () => {
		expect(formsExtensionHost.catalogueEntries().map((entry) => entry.kind)).toContain("slider");
		expect(defaultQuestionForKind("boolean", "q1").kind).toBe("boolean");
	});

	it("parses extension question params", () => {
		const spec = parseFormSpec({
			schema: "forms.form",
			id: "ext",
			version: "1",
			steps: [
				{
					id: "s1",
					title: "Step",
					questions: [{ id: "col", kind: "buildingComponent", label: "Column", fixtureSlug: "hexagonal-mushroom-column", params: { height: 8 } }],
				},
			],
		});
		const question = spec.steps[0]?.questions[0];
		expect(question?.kind).toBe("buildingComponent");
		if (question && isExtensionFormQuestion(question)) {
			expect(question.params?.height).toBe(8);
		}
	});

	it("registers extension kinds in the host", () => {
		expect(formsExtensionHost.findQuestionKind("buildingComponent")?.preview?.surface).toBe("flow3d");
	});

	it("materializes inline form specs via app VCS handler", () => {
		const projection = createFormsAppVcsHandler().materializeProjection({ inline: formSpecToJson(sampleSpec) });
		expect(projection.id).toBe("sample");
	});

	it("maps flow fixture widgets to form spec", () => {
		const json = JSON.stringify({
			schema: "flow.fixture",
			widgets: [{ kind: "inputSlider", id: "width", label: "Span Width", value: 4, min: 0, max: 10, unit: "m" }],
			synapses: [],
		});
		const mapped = flowFixtureToFormSpec(json);
		const question = mapped.steps[0]?.questions[0];
		expect(question?.kind).toBe("slider");
		if (question?.kind === "slider") {
			expect(question.label).toBe("Span Width");
			expect(question.unit).toBe("m");
		}
	});

	it("infers slider labels from synapse target ports", () => {
		const json = JSON.stringify({
			schema: "flow.fixture",
			widgets: [{ kind: "inputSlider", id: "slider_7", value: 6, min: 0, max: 10 }],
			synapses: [{ id: "e1", from: "slider_7", to: "vector", fromPort: "number", toPort: "z" }],
		});
		const mapped = flowFixtureToFormSpec(json);
		expect(mapped.steps[0]?.questions[0]?.label).toBe("Height");
	});

	it("applies generation values to fixture", () => {
		const json = JSON.stringify({ schema: "flow.fixture", widgets: [{ kind: "inputSlider", id: "width", value: 1 }], synapses: [] });
		const next = applyGenerationValuesToFixture(json, { width: 7 });
		const parsed = JSON.parse(next) as { widgets: { value: number }[] };
		expect(parsed.widgets[0]?.value).toBe(7);
	});
	});
}
// #endregion 🧪Tests

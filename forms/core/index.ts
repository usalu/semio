// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 📋 `@semio-tech/forms-core` — declarative form specs, runtime, and edit operations. */
// #endregion 🧲Header

// #region 📐Types
export type FormQuestionKind =
	| "text"
	| "longText"
	| "number"
	| "slider"
	| "boolean"
	| "single"
	| "multi"
	| "date"
	| "color"
	| "vector"
	| "note"
	| "image"
	| "file";

export interface FormSelectOption {
	readonly value: string;
	readonly label: string;
}

export interface FormVectorField {
	readonly key: string;
	readonly label?: string;
	readonly value?: number;
}

export interface FormQuestionBase {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly required?: boolean;
	readonly placeholder?: string;
	readonly condition?: FormExpr;
}

export interface FormQuestionText extends FormQuestionBase {
	readonly kind: "text";
	readonly default?: string;
}

export interface FormQuestionLongText extends FormQuestionBase {
	readonly kind: "longText";
	readonly default?: string;
}

export interface FormQuestionNumber extends FormQuestionBase {
	readonly kind: "number";
	readonly default?: number;
	readonly min?: number;
	readonly max?: number;
	readonly step?: number;
}

export interface FormQuestionSlider extends FormQuestionBase {
	readonly kind: "slider";
	readonly default?: number;
	readonly min?: number;
	readonly max?: number;
	readonly step?: number;
	readonly unit?: string;
}

export interface FormQuestionBoolean extends FormQuestionBase {
	readonly kind: "boolean";
	readonly default?: boolean;
}

export interface FormQuestionSingle extends FormQuestionBase {
	readonly kind: "single";
	readonly default?: string;
	readonly options: readonly FormSelectOption[];
}

export interface FormQuestionMulti extends FormQuestionBase {
	readonly kind: "multi";
	readonly default?: readonly string[];
	readonly options: readonly FormSelectOption[];
}

export interface FormQuestionDate extends FormQuestionBase {
	readonly kind: "date";
	readonly default?: string;
}

export interface FormQuestionColor extends FormQuestionBase {
	readonly kind: "color";
	readonly default?: string;
}

export interface FormQuestionVector extends FormQuestionBase {
	readonly kind: "vector";
	readonly schema?: string;
	readonly fields: readonly FormVectorField[];
	readonly step?: number;
}

export interface FormQuestionNote extends FormQuestionBase {
	readonly kind: "note";
	readonly text: string;
}

export interface FormQuestionImage extends FormQuestionBase {
	readonly kind: "image";
	readonly src?: string;
}

export interface FormQuestionFile extends FormQuestionBase {
	readonly kind: "file";
	readonly accept?: string;
}

export interface FormQuestionExtension extends FormQuestionBase {
	readonly kind: string;
	readonly fixtureSlug?: string;
}

export type FormQuestion =
	| FormQuestionText
	| FormQuestionLongText
	| FormQuestionNumber
	| FormQuestionSlider
	| FormQuestionBoolean
	| FormQuestionSingle
	| FormQuestionMulti
	| FormQuestionDate
	| FormQuestionColor
	| FormQuestionVector
	| FormQuestionNote
	| FormQuestionImage
	| FormQuestionFile
	| FormQuestionExtension;

export interface FormStep {
	readonly id: string;
	readonly title: string;
	readonly description?: string;
	readonly questions: readonly FormQuestion[];
}

export interface FormSpec {
	readonly schema: "forms.form/v1";
	readonly id: string;
	readonly version: string;
	readonly title?: string;
	readonly steps: readonly FormStep[];
}

export type FormScalarValue = string | number | boolean | readonly string[] | readonly number[];

export type FormValue = FormScalarValue | FormValues | null;

export type FormValues = Readonly<Record<string, FormValue>>;

export interface QuestionKindCatalogueEntry {
	readonly kind: string;
	readonly label: string;
	readonly iconId: string;
	readonly defaults: Omit<FormQuestion, "id" | "label">;
}

export type FormsQuestionValueShape = "scalar" | "list" | "record";

export interface FormsQuestionKindPreview {
	readonly surface: "flow3d";
	readonly fixtureSlug: string;
}

export interface FormsQuestionKindControls {
	readonly source: "flowFixture";
	readonly fixtureSlug: string;
}

export interface FormsQuestionKindContribution {
	readonly kind: string;
	readonly label: string;
	readonly iconId: string;
	readonly group?: string;
	readonly value: FormsQuestionValueShape;
	readonly defaults: Record<string, unknown>;
	readonly preview?: FormsQuestionKindPreview;
	readonly controls?: FormsQuestionKindControls;
}

export interface FormsExtensionManifestV1 {
	readonly schema: "forms.module/v1";
	readonly id: string;
	readonly name: string;
	readonly version: string;
	readonly activationEvents: readonly string[];
	readonly contributes: {
		readonly questionKinds: readonly FormsQuestionKindContribution[];
	};
}

export interface FormsExtensionEntry {
	readonly id: string;
	readonly manifest: FormsExtensionManifestV1;
	readonly active: boolean;
}

export type FormExpr =
	| { readonly kind: "const"; readonly value: FormValue }
	| { readonly kind: "var"; readonly name: string }
	| { readonly kind: "eq"; readonly left: FormExpr; readonly right: FormExpr }
	| { readonly kind: "and"; readonly items: readonly FormExpr[] }
	| { readonly kind: "or"; readonly items: readonly FormExpr[] }
	| { readonly kind: "truthy"; readonly expr: FormExpr };

export type FormEditOp =
	| { readonly op: "addStep"; readonly step: FormStep; readonly index?: number }
	| { readonly op: "removeStep"; readonly stepId: string }
	| { readonly op: "moveStep"; readonly stepId: string; readonly index: number }
	| { readonly op: "addQuestion"; readonly stepId: string; readonly question: FormQuestion; readonly index?: number }
	| { readonly op: "removeQuestion"; readonly stepId: string; readonly questionId: string }
	| {
			readonly op: "moveQuestion";
			readonly questionId: string;
			readonly fromStepId: string;
			readonly toStepId: string;
			readonly index: number;
	  }
	| { readonly op: "updateQuestion"; readonly stepId: string; readonly question: FormQuestion }
	| { readonly op: "updateStep"; readonly step: FormStep };

export interface FormValidationError {
	readonly questionId: string;
	readonly message: string;
}

export interface FormSubmitResult {
	readonly ok: boolean;
	readonly values: FormValues;
	readonly errors: readonly FormValidationError[];
}
// #endregion 📐Types

// #region 📚Catalogue
export const QUESTION_KIND_CATALOGUE: readonly QuestionKindCatalogueEntry[] = [
	{ kind: "text", label: "Text", iconId: "ui.input.text", defaults: { kind: "text", placeholder: "Enter text" } },
	{ kind: "longText", label: "Long Text", iconId: "ui.input.textarea", defaults: { kind: "longText", placeholder: "Enter long text" } },
	{ kind: "number", label: "Number", iconId: "ui.input.number", defaults: { kind: "number", step: 1 } },
	{ kind: "slider", label: "Slider", iconId: "ui.input.slider", defaults: { kind: "slider", min: 0, max: 100, step: 1, default: 50 } },
	{ kind: "boolean", label: "Boolean", iconId: "ui.input.toggle", defaults: { kind: "boolean", default: false } },
	{
		kind: "single",
		label: "Single Select",
		iconId: "ui.input.select",
		defaults: {
			kind: "single",
			options: [
				{ value: "a", label: "Option A" },
				{ value: "b", label: "Option B" },
			],
		},
	},
	{
		kind: "multi",
		label: "Multi Select",
		iconId: "ui.input.multiselect",
		defaults: {
			kind: "multi",
			options: [
				{ value: "a", label: "Option A" },
				{ value: "b", label: "Option B" },
			],
			default: [],
		},
	},
	{ kind: "date", label: "Date", iconId: "ui.input.date", defaults: { kind: "date" } },
	{ kind: "color", label: "Color", iconId: "ui.input.color", defaults: { kind: "color", default: "#336699" } },
	{
		kind: "vector",
		label: "Vector",
		iconId: "ui.input.stepper",
		defaults: {
			kind: "vector",
			schema: "vec3",
			fields: [
				{ key: "x", label: "X", value: 0 },
				{ key: "y", label: "Y", value: 0 },
				{ key: "z", label: "Z", value: 0 },
			],
			step: 0.1,
		},
	},
	{ kind: "note", label: "Note", iconId: "ui.input.note", defaults: { kind: "note", text: "Informational note" } },
	{ kind: "image", label: "Image", iconId: "ui.input.image", defaults: { kind: "image" } },
	{ kind: "file", label: "File", iconId: "ui.input.file", defaults: { kind: "file" } },
];

const BUILTIN_FORM_QUESTION_KINDS = new Set<string>(QUESTION_KIND_CATALOGUE.map((entry) => entry.kind));

function contributionValueShape(kind: FormQuestionKind): FormsQuestionValueShape {
	if (kind === "multi" || kind === "vector") return "list";
	if (kind === "note" || kind === "image" || kind === "file") return "scalar";
	return "scalar";
}

function buildBuiltinFormsExtensionManifest(): FormsExtensionManifestV1 {
	return {
		schema: "forms.module/v1",
		id: "builtin",
		name: "Built-in",
		version: "1.0.0",
		activationEvents: ["*"],
		contributes: {
			questionKinds: QUESTION_KIND_CATALOGUE.map((entry) => ({
				kind: entry.kind,
				label: entry.label,
				iconId: entry.iconId,
				value: contributionValueShape(entry.kind),
				defaults: entry.defaults as Record<string, unknown>,
			})),
		},
	};
}

/** @emoji 🏗 Procedural forms extension manifest contributing flow-backed question kinds. */
export const PROCEDURAL_FORMS_EXTENSION_MANIFEST: FormsExtensionManifestV1 = {
	schema: "forms.module/v1",
	id: "procedural",
	name: "Procedural",
	version: "1.0.0",
	activationEvents: ["*"],
	contributes: {
		questionKinds: [
			{
				kind: "buildingComponent",
				label: "Building Component",
				iconId: "ui.input.3d",
				group: "Procedural",
				value: "record",
				defaults: { kind: "buildingComponent", fixtureSlug: "hexagonal-mushroom-column" },
				preview: { surface: "flow3d", fixtureSlug: "hexagonal-mushroom-column" },
				controls: { source: "flowFixture", fixtureSlug: "hexagonal-mushroom-column" },
			},
		],
	},
};

export const FORMS_DEFAULT_EXTENSION_IDS = ["builtin", "procedural"] as const;

interface ActiveFormsExtension {
	readonly manifest: FormsExtensionManifestV1;
}

/** @emoji 🧩 Controlled, DOM-free forms extension host. */
export class FormsExtensionHost {
	private readonly active = new Map<string, ActiveFormsExtension>();
	private readonly kinds = new Map<string, FormsQuestionKindContribution & { readonly moduleId: string }>();
	private revision = 0;
	private readonly listeners = new Set<() => void>();

	constructor() {
		this.registerManifest(buildBuiltinFormsExtensionManifest());
		this.registerManifest(PROCEDURAL_FORMS_EXTENSION_MANIFEST);
	}

	getRevision(): number {
		return this.revision;
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	private notify(): void {
		this.revision += 1;
		for (const listener of this.listeners) {
			listener();
		}
	}

	registerManifest(manifest: FormsExtensionManifestV1): void {
		this.active.set(manifest.id, { manifest });
		for (const kind of manifest.contributes.questionKinds) {
			this.kinds.set(kind.kind, { ...kind, moduleId: manifest.id });
		}
		this.notify();
	}

	async activateDefaults(): Promise<void> {
		for (const id of FORMS_DEFAULT_EXTENSION_IDS) {
			if (!this.active.has(id)) {
				throw new Error(`forms extension not registered: ${id}`);
			}
		}
	}

	async setActive(id: string, enabled: boolean): Promise<void> {
		if (!enabled) {
			const entry = this.active.get(id);
			if (!entry) return;
			for (const kind of entry.manifest.contributes.questionKinds) {
				this.kinds.delete(kind.kind);
			}
			this.active.delete(id);
			this.notify();
			return;
		}
		if (id === "builtin") this.registerManifest(buildBuiltinFormsExtensionManifest());
		else if (id === "procedural") this.registerManifest(PROCEDURAL_FORMS_EXTENSION_MANIFEST);
		else throw new Error(`unknown forms extension: ${id}`);
	}

	listEntries(): readonly FormsExtensionEntry[] {
		return FORMS_DEFAULT_EXTENSION_IDS.map((id) => ({
			id,
			manifest: this.active.get(id)?.manifest ?? buildBuiltinFormsExtensionManifest(),
			active: this.active.has(id),
		}));
	}

	findQuestionKind(kind: string): (FormsQuestionKindContribution & { readonly moduleId: string }) | undefined {
		return this.kinds.get(kind);
	}

	catalogueEntries(): readonly QuestionKindCatalogueEntry[] {
		return [...this.kinds.values()].map((contribution) => ({
			kind: contribution.kind,
			label: contribution.label,
			iconId: contribution.iconId,
			defaults: contribution.defaults as Omit<FormQuestion, "id" | "label">,
		}));
	}
}

export const formsExtensionHost = new FormsExtensionHost();

const formsFlowFixtureResolvers = new Map<string, () => string>();

/** @emoji 📦 Registers a resolver for flow fixture JSON referenced by extension question kinds. */
export function registerFormsFlowFixtureResolver(slug: string, resolver: () => string): void {
	formsFlowFixtureResolvers.set(slug, resolver);
}

/** @emoji 📦 Resolves flow fixture JSON for a registered slug. */
export function resolveFormsFlowFixtureJson(slug: string): string | undefined {
	return formsFlowFixtureResolvers.get(slug)?.();
}

export function isBuiltinFormQuestionKind(kind: string): kind is FormQuestionKind {
	return BUILTIN_FORM_QUESTION_KINDS.has(kind);
}

export function isExtensionFormQuestion(question: FormQuestion): question is FormQuestionExtension {
	return !isBuiltinFormQuestionKind(question.kind);
}

export function questionKindContribution(question: FormQuestion): FormsQuestionKindContribution | undefined {
	return formsExtensionHost.findQuestionKind(question.kind);
}

export function resolveQuestionFixtureSlug(question: FormQuestion): string | undefined {
	if (!isExtensionFormQuestion(question)) return undefined;
	const contribution = formsExtensionHost.findQuestionKind(question.kind);
	return question.fixtureSlug ?? contribution?.controls?.fixtureSlug ?? contribution?.preview?.fixtureSlug;
}

export function questionKindCatalogueEntry(kind: string): QuestionKindCatalogueEntry {
	const entry = formsExtensionHost.catalogueEntries().find((item) => item.kind === kind);
	if (!entry) throw new Error(`Unknown question kind: ${kind}`);
	return entry;
}

export function defaultQuestionForKind(kind: string, id: string, label?: string): FormQuestion {
	const entry = questionKindCatalogueEntry(kind);
	return { id, label: label ?? entry.label, ...entry.defaults, kind } as FormQuestion;
}
// #endregion 📚Catalogue

// #region 🔧Helpers
const FORBIDDEN_KEYS = new Set(["run", "code", "handler", "eval", "Function"]);

export function createFormId(prefix = "form"): string {
	return `${prefix}-${Math.random().toString(36).slice(2, 10)}`;
}

export function formSpecToJson(spec: FormSpec): string {
	return JSON.stringify(spec);
}

export function parseFormSpec(raw: unknown): FormSpec {
	if (raw == null || typeof raw !== "object") throw new Error("FormSpec must be an object");
	const obj = raw as Record<string, unknown>;
	assertNoForbiddenKeys(obj);
	if (obj.schema !== "forms.form/v1") throw new Error(`Invalid schema: ${String(obj.schema)}`);
	if (typeof obj.id !== "string" || !obj.id.trim()) throw new Error("FormSpec.id is required");
	if (typeof obj.version !== "string" || !obj.version.trim()) throw new Error("FormSpec.version is required");
	if (!Array.isArray(obj.steps) || obj.steps.length === 0) throw new Error("FormSpec.steps must be a non-empty array");
	const steps = obj.steps.map(parseFormStep);
return {
		schema: "forms.form/v1",
		id: obj.id,
		version: obj.version,
		title: typeof obj.title === "string" ? obj.title : undefined,
		steps,
	};
}

function parseFormStep(raw: unknown, index: number): FormStep {
	if (raw == null || typeof raw !== "object") throw new Error(`Step ${index} must be an object`);
	const obj = raw as Record<string, unknown>;
	assertNoForbiddenKeys(obj);
	if (typeof obj.id !== "string" || !obj.id.trim()) throw new Error(`Step ${index}.id is required`);
	if (typeof obj.title !== "string" || !obj.title.trim()) throw new Error(`Step ${index}.title is required`);
	if (!Array.isArray(obj.questions)) throw new Error(`Step ${index}.questions must be an array`);
	return {
		id: obj.id,
		title: obj.title,
		description: typeof obj.description === "string" ? obj.description : undefined,
		questions: obj.questions.map((q, qi) => parseFormQuestion(q, index, qi)),
	};
}

function parseFormQuestion(raw: unknown, stepIndex: number, questionIndex: number): FormQuestion {
	if (raw == null || typeof raw !== "object") throw new Error(`Question ${stepIndex}.${questionIndex} must be an object`);
	const obj = raw as Record<string, unknown>;
	assertNoForbiddenKeys(obj);
	if (typeof obj.id !== "string" || !obj.id.trim()) throw new Error(`Question ${stepIndex}.${questionIndex}.id is required`);
	if (typeof obj.label !== "string" || !obj.label.trim()) throw new Error(`Question ${stepIndex}.${questionIndex}.label is required`);
	if (typeof obj.kind !== "string") throw new Error(`Question ${stepIndex}.${questionIndex}.kind is required`);
	const base = {
		id: obj.id,
		label: obj.label,
		description: typeof obj.description === "string" ? obj.description : undefined,
		required: obj.required === true ? true : undefined,
		placeholder: typeof obj.placeholder === "string" ? obj.placeholder : undefined,
		condition: obj.condition != null ? parseFormExpr(obj.condition) : undefined,
	};
	switch (obj.kind) {
		case "text":
		case "longText":
			return { ...base, kind: obj.kind, default: typeof obj.default === "string" ? obj.default : undefined };
		case "number":
		case "slider":
			return {
				...base,
				kind: obj.kind,
				default: typeof obj.default === "number" ? obj.default : undefined,
				min: typeof obj.min === "number" ? obj.min : undefined,
				max: typeof obj.max === "number" ? obj.max : undefined,
				step: typeof obj.step === "number" ? obj.step : undefined,
				unit: obj.kind === "slider" && typeof obj.unit === "string" ? obj.unit : undefined,
			};
		case "boolean":
			return { ...base, kind: "boolean", default: typeof obj.default === "boolean" ? obj.default : undefined };
		case "single":
		case "multi":
			return {
				...base,
				kind: obj.kind,
				default: obj.kind === "multi" ? parseStringArray(obj.default) : typeof obj.default === "string" ? obj.default : undefined,
				options: parseSelectOptions(obj.options, stepIndex, questionIndex),
			};
		case "date":
		case "color":
			return { ...base, kind: obj.kind, default: typeof obj.default === "string" ? obj.default : undefined };
		case "vector":
			return {
				...base,
				kind: "vector",
				schema: typeof obj.schema === "string" ? obj.schema : undefined,
				fields: parseVectorFields(obj.fields, stepIndex, questionIndex),
				step: typeof obj.step === "number" ? obj.step : undefined,
			};
		case "note":
			if (typeof obj.text !== "string") throw new Error(`Question ${stepIndex}.${questionIndex}.text is required for note`);
			return { ...base, kind: "note", text: obj.text };
		case "image":
			return { ...base, kind: "image", src: typeof obj.src === "string" ? obj.src : undefined };
		case "file":
			return { ...base, kind: "file", accept: typeof obj.accept === "string" ? obj.accept : undefined };
		default: {
			const contribution = formsExtensionHost.findQuestionKind(obj.kind);
			if (!contribution) throw new Error(`Unknown question kind: ${String(obj.kind)}`);
			const fixtureSlug =
				typeof obj.fixtureSlug === "string"
					? obj.fixtureSlug
					: typeof contribution.defaults.fixtureSlug === "string"
						? contribution.defaults.fixtureSlug
						: contribution.controls?.fixtureSlug ?? contribution.preview?.fixtureSlug;
			return { ...base, kind: obj.kind, fixtureSlug };
		}
	}
}

function parseSelectOptions(raw: unknown, stepIndex: number, questionIndex: number): readonly FormSelectOption[] {
	if (!Array.isArray(raw) || raw.length === 0) throw new Error(`Question ${stepIndex}.${questionIndex}.options is required`);
	return raw.map((entry, index) => {
		if (entry == null || typeof entry !== "object") throw new Error(`Option ${index} must be an object`);
		const obj = entry as Record<string, unknown>;
		if (typeof obj.value !== "string" || typeof obj.label !== "string") throw new Error(`Option ${index} requires value and label`);
		return { value: obj.value, label: obj.label };
	});
}

function parseVectorFields(raw: unknown, stepIndex: number, questionIndex: number): readonly FormVectorField[] {
	if (!Array.isArray(raw) || raw.length === 0) throw new Error(`Question ${stepIndex}.${questionIndex}.fields is required`);
	return raw.map((entry, index) => {
		if (entry == null || typeof entry !== "object") throw new Error(`Vector field ${index} must be an object`);
		const obj = entry as Record<string, unknown>;
		if (typeof obj.key !== "string") throw new Error(`Vector field ${index}.key is required`);
		return {
			key: obj.key,
			label: typeof obj.label === "string" ? obj.label : undefined,
			value: typeof obj.value === "number" ? obj.value : undefined,
		};
	});
}

function parseStringArray(raw: unknown): readonly string[] {
	if (!Array.isArray(raw)) return [];
	return raw.filter((entry): entry is string => typeof entry === "string");
}

function parseFormExpr(raw: unknown): FormExpr {
	if (raw == null || typeof raw !== "object") throw new Error("FormExpr must be an object");
	const obj = raw as Record<string, unknown>;
	assertNoForbiddenKeys(obj);
	if (typeof obj.kind !== "string") throw new Error("FormExpr.kind is required");
	switch (obj.kind) {
		case "const":
			return { kind: "const", value: obj.value as FormValue };
		case "var":
			if (typeof obj.name !== "string") throw new Error("FormExpr.var.name is required");
			return { kind: "var", name: obj.name };
		case "eq":
			return { kind: "eq", left: parseFormExpr(obj.left), right: parseFormExpr(obj.right) };
		case "and":
			if (!Array.isArray(obj.items)) throw new Error("FormExpr.and.items is required");
			return { kind: "and", items: obj.items.map(parseFormExpr) };
		case "or":
			if (!Array.isArray(obj.items)) throw new Error("FormExpr.or.items is required");
			return { kind: "or", items: obj.items.map(parseFormExpr) };
		case "truthy":
			return { kind: "truthy", expr: parseFormExpr(obj.expr) };
		default:
			throw new Error(`Unknown FormExpr kind: ${String(obj.kind)}`);
	}
}

function assertNoForbiddenKeys(obj: Record<string, unknown>): void {
	for (const key of Object.keys(obj)) {
		if (FORBIDDEN_KEYS.has(key)) throw new Error(`Forbidden key in form data: ${key}`);
	}
}

/** @emoji 🔀 Maps flow input widgets into a {@link FormSpec}. */
export function flowFixtureToFormSpec(fixtureJson: string, formId = "flow-generate"): FormSpec {
	const fixture = JSON.parse(fixtureJson) as { readonly widgets?: readonly Record<string, unknown>[] };
	const questions: FormQuestion[] = [];
	for (const widget of fixture.widgets ?? []) {
		const kind = widget.kind;
		const id = String(widget.id ?? createFormId("widget"));
		if (kind === "inputSlider") {
			questions.push({
				id,
				kind: "slider",
				label: id,
				min: typeof widget.min === "number" ? widget.min : 0,
				max: typeof widget.max === "number" ? widget.max : 100,
				step: typeof widget.step === "number" ? widget.step : 1,
				default: typeof widget.value === "number" ? widget.value : 0,
			});
		} else if (kind === "inputStepper") {
			const fields = Array.isArray(widget.fields)
				? widget.fields.map((field: Record<string, unknown>) => ({
						key: String(field.key ?? "v"),
						label: String(field.key ?? "v"),
						value: typeof field.value === "number" ? field.value : 0,
					}))
				: [{ key: "x", label: "X", value: 0 }];
			questions.push({ id, kind: "vector", label: id, schema: String(widget.schema ?? "vec"), fields, step: typeof widget.step === "number" ? widget.step : 0.1 });
		} else if (kind === "inputNote") {
			questions.push({ id, kind: "note", label: id, text: String(widget.text ?? "") });
		} else if (kind === "inputImage") {
			questions.push({ id, kind: "image", label: id, src: typeof widget.src === "string" ? widget.src : undefined });
		} else if (kind === "variable") {
			const schema = String(widget.schema ?? "text");
			if (schema.includes("enum") && Array.isArray((widget as { options?: unknown }).options)) {
				const options = ((widget as { options: readonly { value: string; label: string }[] }).options ?? []).map((option) => ({
					value: option.value,
					label: option.label,
				}));
				questions.push({ id, kind: "single", label: String(widget.name ?? id), options, default: options[0]?.value });
			} else {
				questions.push({ id, kind: "text", label: String(widget.name ?? id), default: "" });
			}
		}
	}
	return {
		schema: "forms.form/v1",
		id: formId,
		version: "1",
		title: "Generate",
		steps: [{ id: "inputs", title: "Inputs", questions }],
	};
}

/** @emoji 🔧 Applies generation values back onto a flow fixture JSON blob. */
export function applyGenerationValuesToFixture(fixtureJson: string, values: FormValues): string {
	const fixture = JSON.parse(fixtureJson) as { readonly widgets?: Record<string, unknown>[] };
	const widgets = (fixture.widgets ?? []).map((widget) => {
		const id = String(widget.id ?? "");
		if (!(id in values)) return widget;
		const value = values[id];
		if (widget.kind === "inputSlider" && typeof value === "number") return { ...widget, value };
		if (widget.kind === "inputStepper" && Array.isArray(value)) {
			const fields = Array.isArray(widget.fields)
				? widget.fields.map((field: Record<string, unknown>, index: number) => ({ ...field, value: Number(value[index] ?? field.value ?? 0) }))
				: [];
			return { ...widget, fields };
		}
		if (widget.kind === "inputNote" && typeof value === "string") return { ...widget, text: value };
		if (widget.kind === "inputImage" && typeof value === "string") return { ...widget, src: value };
		if (widget.kind === "variable" && typeof value === "string") return { ...widget, name: value };
		return widget;
	});
	return JSON.stringify({ ...fixture, widgets });
}

export function defaultValueForQuestion(question: FormQuestion): FormValue {
	switch (question.kind) {
		case "text":
		case "longText":
			return question.default ?? "";
		case "number":
		case "slider":
			return question.default ?? question.min ?? 0;
		case "boolean":
			return question.default ?? false;
		case "single":
			return question.default ?? question.options[0]?.value ?? "";
		case "multi":
			return question.default ? [...question.default] : [];
		case "date":
		case "color":
			return question.default ?? "";
		case "vector":
			return question.fields.map((field) => field.value ?? 0);
		case "note":
		case "image":
		case "file":
			return null;
		default: {
			if (!isExtensionFormQuestion(question)) return null;
			const contribution = formsExtensionHost.findQuestionKind(question.kind);
			if (contribution?.value !== "record") return null;
			const slug = resolveQuestionFixtureSlug(question);
			if (!slug) return {};
			const fixtureJson = resolveFormsFlowFixtureJson(slug);
			if (!fixtureJson) return {};
			const subSpec = flowFixtureToFormSpec(fixtureJson, `${question.id}-params`);
			return new FormRuntime(subSpec).getValues();
		}
	}
}

export function flattenFormQuestions(spec: FormSpec): FormQuestion[] {
	return spec.steps.flatMap((step) => [...step.questions]);
}

export function findQuestionLocation(
	spec: FormSpec,
	questionId: string,
): { readonly stepId: string; readonly index: number; readonly question: FormQuestion } | undefined {
	for (const step of spec.steps) {
		const index = step.questions.findIndex((question) => question.id === questionId);
		if (index !== -1) return { stepId: step.id, index, question: step.questions[index]! };
	}
	return undefined;
}
// #endregion 🔧Helpers

// #region ✏️EditOps
export function applyFormEditOp(spec: FormSpec, op: FormEditOp): FormSpec {
	switch (op.op) {
		case "addStep": {
			const steps = [...spec.steps];
			const index = op.index ?? steps.length;
			steps.splice(index, 0, op.step);
			return { ...spec, steps };
		}
		case "removeStep":
			return { ...spec, steps: spec.steps.filter((step) => step.id !== op.stepId) };
		case "moveStep": {
			const steps = [...spec.steps];
			const fromIndex = steps.findIndex((step) => step.id === op.stepId);
			if (fromIndex === -1) return spec;
			const [step] = steps.splice(fromIndex, 1);
			steps.splice(Math.max(0, Math.min(op.index, steps.length)), 0, step!);
			return { ...spec, steps };
		}
		case "addQuestion":
			return {
				...spec,
				steps: spec.steps.map((step) => {
					if (step.id !== op.stepId) return step;
					const questions = [...step.questions];
					questions.splice(op.index ?? questions.length, 0, op.question);
					return { ...step, questions };
				}),
			};
		case "removeQuestion":
			return {
				...spec,
				steps: spec.steps.map((step) =>
					step.id === op.stepId ? { ...step, questions: step.questions.filter((question) => question.id !== op.questionId) } : step,
				),
			};
		case "moveQuestion": {
			let moving: FormQuestion | undefined;
			const without = spec.steps.map((step) => {
				if (step.id !== op.fromStepId) return step;
				const questions = [...step.questions];
				const index = questions.findIndex((question) => question.id === op.questionId);
				if (index === -1) return step;
				moving = questions[index];
				questions.splice(index, 1);
				return { ...step, questions };
			});
			if (!moving) return spec;
			return {
				...spec,
				steps: without.map((step) => {
					if (step.id !== op.toStepId) return step;
					const questions = [...step.questions];
					questions.splice(Math.max(0, Math.min(op.index, questions.length)), 0, moving!);
					return { ...step, questions };
				}),
			};
		}
		case "updateQuestion":
			return {
				...spec,
				steps: spec.steps.map((step) =>
					step.id === op.stepId
						? {
								...step,
								questions: step.questions.map((question) => (question.id === op.question.id ? op.question : question)),
							}
						: step,
				),
			};
		case "updateStep":
			return {
				...spec,
				steps: spec.steps.map((step) => (step.id === op.step.id ? op.step : step)),
			};
		default:
			return spec;
	}
}
// #endregion ✏️EditOps

// #region 🧮Expr
export function evalFormExpr(expr: FormExpr, values: FormValues): FormValue {
	switch (expr.kind) {
		case "const":
			return expr.value;
		case "var":
			return values[expr.name] ?? null;
		case "eq":
			return evalFormExpr(expr.left, values) === evalFormExpr(expr.right, values);
		case "and":
			return expr.items.every((item) => Boolean(evalFormExpr(item, values)));
		case "or":
			return expr.items.some((item) => Boolean(evalFormExpr(item, values)));
		case "truthy":
			return Boolean(evalFormExpr(expr.expr, values));
		default:
			return null;
	}
}

export function isQuestionVisible(question: FormQuestion, values: FormValues): boolean {
	if (!question.condition) return true;
	return Boolean(evalFormExpr(question.condition, values));
}
// #endregion 🧮Expr

// #region 🏃Runtime
export class FormRuntime {
	private readonly spec: FormSpec;
	private values: Record<string, FormValue>;
	private stepIndex = 0;

	constructor(spec: FormSpec, initialValues?: FormValues) {
		this.spec = spec;
		this.values = {};
		for (const question of flattenFormQuestions(spec)) {
			this.values[question.id] = initialValues?.[question.id] ?? defaultValueForQuestion(question);
		}
		if (initialValues) {
			for (const [key, value] of Object.entries(initialValues)) {
				if (key in this.values) this.values[key] = value;
			}
		}
	}

	getSpec(): FormSpec {
		return this.spec;
	}

	getValues(): FormValues {
		return { ...this.values };
	}

	getCurrentStepIndex(): number {
		return this.stepIndex;
	}

	getCurrentStep(): FormStep {
		return this.spec.steps[this.stepIndex] ?? this.spec.steps[0]!;
	}

	setValue(questionId: string, value: FormValue): void {
		if (!(questionId in this.values)) return;
		this.values[questionId] = value;
	}

	getVisibleQuestions(step: FormStep = this.getCurrentStep()): FormQuestion[] {
		return step.questions.filter((question) => isQuestionVisible(question, this.values));
	}

	getStepErrors(step: FormStep = this.getCurrentStep()): FormValidationError[] {
		const errors: FormValidationError[] = [];
		for (const question of this.getVisibleQuestions(step)) {
			if (question.kind === "note" || question.kind === "image") continue;
			if (!question.required) continue;
			const value = this.values[question.id];
			if (isExtensionFormQuestion(question)) {
				const contribution = formsExtensionHost.findQuestionKind(question.kind);
				if (contribution?.value === "record") {
					if (value == null || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length === 0) {
						errors.push({ questionId: question.id, message: `${question.label} is required` });
					}
					continue;
				}
			}
			if (value == null || value === "" || (Array.isArray(value) && value.length === 0)) {
				errors.push({ questionId: question.id, message: `${question.label} is required` });
			}
		}
		return errors;
	}

	canAdvance(): boolean {
		return this.getStepErrors().length === 0;
	}

	goToStep(index: number): void {
		this.stepIndex = Math.max(0, Math.min(index, this.spec.steps.length - 1));
	}

	nextStep(): boolean {
		if (!this.canAdvance()) return false;
		if (this.stepIndex >= this.spec.steps.length - 1) return false;
		this.stepIndex += 1;
		return true;
	}

	previousStep(): boolean {
		if (this.stepIndex <= 0) return false;
		this.stepIndex -= 1;
		return true;
	}

	submit(): FormSubmitResult {
		const errors: FormValidationError[] = [];
		for (const step of this.spec.steps) {
			errors.push(...this.getStepErrors(step));
		}
		const values: Record<string, FormValue> = {};
		for (const question of flattenFormQuestions(this.spec)) {
			if (!isQuestionVisible(question, this.values)) continue;
			if (question.kind === "note" || question.kind === "image") continue;
			values[question.id] = this.values[question.id] ?? null;
		}
		return { ok: errors.length === 0, values, errors };
	}
}
// #endregion 🏃Runtime

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("forms-core", () => {
	const sampleSpec: FormSpec = {
		schema: "forms.form/v1",
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
		expect(() => parseFormSpec({ schema: "forms.form/v1", id: "x", version: "1", steps: [], code: "bad" })).toThrow();
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

	it("parses extension question kinds", () => {
		const spec = parseFormSpec({
			schema: "forms.form/v1",
			id: "ext",
			version: "1",
			steps: [
				{
					id: "s1",
					title: "Step",
					questions: [{ id: "col", kind: "buildingComponent", label: "Column", fixtureSlug: "hexagonal-mushroom-column" }],
				},
			],
		});
		expect(spec.steps[0]?.questions[0]?.kind).toBe("buildingComponent");
	});

	it("registers extension kinds in the host", () => {
		expect(formsExtensionHost.findQuestionKind("buildingComponent")?.preview?.surface).toBe("flow3d");
	});

	it("maps flow fixture widgets to form spec", () => {
		const json = JSON.stringify({
			schema: "flow.fixture/v1",
			widgets: [{ kind: "inputSlider", id: "width", value: 4, min: 0, max: 10 }],
			synapses: [],
		});
		const mapped = flowFixtureToFormSpec(json);
		expect(mapped.steps[0]?.questions[0]?.kind).toBe("slider");
	});

	it("applies generation values to fixture", () => {
		const json = JSON.stringify({ schema: "flow.fixture/v1", widgets: [{ kind: "inputSlider", id: "width", value: 1 }], synapses: [] });
		const next = applyGenerationValuesToFixture(json, { width: 7 });
		const parsed = JSON.parse(next) as { widgets: { value: number }[] };
		expect(parsed.widgets[0]?.value).toBe(7);
	});
	});
}
// #endregion 🧪Tests

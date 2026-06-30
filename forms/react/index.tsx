// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 📋 `@semio-tech/forms-react` — form renderer and builder UI. */
// #endregion 🧲Header

import {
	createFormId,
	formSpecToJson,
	FormRuntime,
	parseFormSpec,
	type FormQuestion,
	type FormSpec,
	type FormStep,
	type FormValue,
	type FormValues,
} from "@semio-tech/forms-core";
import {
	Button,
	Field,
	Input,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	Stepper,
	Textarea,
	Toggle,
	type TreeDragAndDropController,
	cn,
} from "@semio-tech/ui-react";
import React from "react";

// #region 📐Contracts
export const FORMS_QUESTION_DRAG_MIME = "application/x-semio-forms-question-kind";

export interface FormRendererProps {
	readonly spec: FormSpec;
	readonly values?: FormValues;
	readonly onChange?: (values: FormValues) => void;
	readonly onSubmit?: (values: FormValues) => void;
	readonly className?: string;
}

export interface FormBuilderProps {
	readonly spec: FormSpec;
	readonly onChange: (spec: FormSpec) => void;
	readonly selectedIds?: readonly string[];
	readonly onSelectionChange?: (selectedIds: readonly string[]) => void;
	readonly className?: string;
}

export interface FlowGeneration {
	readonly id: string;
	readonly name: string;
	readonly values: FormValues;
}

export interface FlowGenerateSurfaceProps {
	readonly formSpec: FormSpec;
	readonly generations: readonly FlowGeneration[];
	readonly selectedGenerationId: string | null;
	readonly previewText: string;
	readonly onSelectGeneration: (generationId: string) => void;
	readonly onAddGeneration: () => void;
	readonly onRemoveGeneration: (generationId: string) => void;
	readonly onGenerationValuesChange: (generationId: string, values: FormValues) => void;
	readonly onRenameGeneration: (generationId: string, name: string) => void;
	readonly className?: string;
}
// #endregion 📐Contracts

// #region 🔧Helpers
export function formSpecFromJson(json: string): FormSpec {
	return parseFormSpec(JSON.parse(json));
}

export function defaultFormSpec(id = "default"): FormSpec {
	return {
		schema: "forms.form/v1",
		id,
		version: "1",
		title: "New Form",
		steps: [{ id: "step-1", title: "Step 1", questions: [{ id: "q-text", kind: "text", label: "Name" }] }],
	};
}

function questionControl(
	question: FormQuestion,
	value: FormValue,
	onValue: (next: FormValue) => void,
): React.ReactNode {
	const id = `form-${question.id}`;
	switch (question.kind) {
		case "text":
			return <Input id={id} value={String(value ?? "")} placeholder={question.placeholder} onChange={(event) => onValue(event.target.value)} />;
		case "longText":
			return <Textarea id={id} value={String(value ?? "")} placeholder={question.placeholder} onChange={(event) => onValue(event.target.value)} />;
		case "number":
			return (
				<Input
					id={id}
					type="number"
					value={typeof value === "number" ? value : Number(value ?? 0)}
					min={question.min}
					max={question.max}
					step={question.step}
					onChange={(event) => onValue(Number(event.target.value))}
				/>
			);
		case "slider":
			return (
				<Slider
					id={id}
					min={question.min ?? 0}
					max={question.max ?? 100}
					step={question.step ?? 1}
					value={[typeof value === "number" ? value : Number(value ?? 0)]}
					onValueChange={(values) => onValue(values[0] ?? 0)}
				/>
			);
		case "boolean":
			return <Toggle id={id} pressed={Boolean(value)} onPressedChange={(pressed) => onValue(pressed)} />;
		case "single":
			return (
				<Select id={id} value={String(value ?? "")} onValueChange={(next) => onValue(next)}>
					<SelectTrigger>
						<SelectValue placeholder={question.placeholder ?? "Select"} />
					</SelectTrigger>
					<SelectContent>
						{question.options.map((option) => (
							<SelectItem key={option.value} value={option.value}>
								{option.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			);
		case "multi":
			return (
				<div className="flex flex-wrap gap-single">
					{question.options.map((option) => {
						const selected = Array.isArray(value) ? value.includes(option.value) : false;
						return (
							<Toggle
								key={option.value}
								pressed={selected}
								onPressedChange={(pressed) => {
									const current = Array.isArray(value) ? [...value] : [];
									onValue(pressed ? [...current, option.value] : current.filter((entry) => entry !== option.value));
								}}
							>
								{option.label}
							</Toggle>
						);
					})}
				</div>
			);
		case "date":
			return <Input id={id} type="date" value={String(value ?? "")} onChange={(event) => onValue(event.target.value)} />;
		case "color":
			return <Input id={id} type="color" value={String(value ?? "#336699")} onChange={(event) => onValue(event.target.value)} />;
		case "vector": {
			const numbers = Array.isArray(value) ? value.map((entry) => Number(entry)) : question.fields.map((field) => field.value ?? 0);
			return (
				<div className="flex flex-col gap-single">
					{question.fields.map((field, index) => (
						<Stepper
							key={field.key}
							id={`${id}-${field.key}`}
							value={numbers[index] ?? 0}
							step={question.step ?? 0.1}
							onChange={(next) => {
								const copy = [...numbers];
								copy[index] = next;
								onValue(copy);
							}}
						/>
					))}
				</div>
			);
		}
		case "note":
			return <p className="text-sm text-muted-foreground">{question.text}</p>;
		case "image":
			return question.src ? <img src={question.src} alt={question.label} className="max-h-32 rounded-md border border-border" /> : <p className="text-xs text-muted-foreground">No image</p>;
		case "file":
			return <Input id={id} type="file" accept={question.accept} onChange={(event) => onValue(event.target.files?.[0]?.name ?? "")} />;
		default:
			return null;
	}
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for workbench rows that carry forms question palette `dragData`. */
export function formsQuestionPaletteTreeDragController(
	dragDataByItemId: ReadonlyMap<string, Record<string, string>>,
): TreeDragAndDropController {
	const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
		const payload = dragData?.[FORMS_QUESTION_DRAG_MIME];
		return payload?.trim() ? payload : undefined;
	};
	return {
		getDragData: ({ sourceItem }) => dragDataByItemId.get(sourceItem.id),
		pointerPaletteDrag: {
			readEncodedDragPayload: readEncoded,
			begin: () => {},
			cancel: () => {},
		},
		onDragStart: () => {},
		onDragEnd: () => {},
	};
}
// #endregion 🔧Helpers

// #region 🖼️FormRenderer
/** @emoji 🖼️ Interactive multi-step form renderer. */
export const FormRenderer: React.FC<FormRendererProps> = ({ spec, values, onChange, onSubmit, className }) => {
	const runtimeRef = React.useRef<FormRuntime | null>(null);
	const [, bump] = React.useState(0);
	if (!runtimeRef.current || runtimeRef.current.getSpec() !== spec) {
		runtimeRef.current = new FormRuntime(spec, values);
	}
	const runtime = runtimeRef.current;
	const step = runtime.getCurrentStep();
	const errors = runtime.getStepErrors();
	const errorById = Object.fromEntries(errors.map((error) => [error.questionId, error.message]));

	const setValue = (questionId: string, value: FormValue) => {
		runtime.setValue(questionId, value);
		onChange?.(runtime.getValues());
		bump((value) => value + 1);
	};

	return (
		<div className={cn("flex flex-col gap-medium p-double min-w-0", className)} data-slot="form-renderer">
			<div className="flex items-center justify-between gap-double">
				<h2 className="text-lg font-semibold">{spec.title ?? spec.id}</h2>
				<p className="text-xs text-muted-foreground">
					Step {runtime.getCurrentStepIndex() + 1} / {spec.steps.length}
				</p>
			</div>
			<div>
				<h3 className="text-base font-medium">{step.title}</h3>
				{step.description ? <p className="text-sm text-muted-foreground">{step.description}</p> : null}
			</div>
			<div className="flex flex-col gap-medium">
				{runtime.getVisibleQuestions().map((question) => (
					<Field key={question.id} id={question.id} label={question.label} description={question.description} required={question.required} error={errorById[question.id]}>
						{questionControl(question, runtime.getValues()[question.id] ?? null, (value) => setValue(question.id, value))}
					</Field>
				))}
			</div>
			<div className="flex items-center gap-single">
				<Button icon="chevron-left" text="Back" disabled={runtime.getCurrentStepIndex() <= 0} onClick={() => { runtime.previousStep(); bump((value) => value + 1); }} />
				{runtime.getCurrentStepIndex() < spec.steps.length - 1 ? (
					<Button
						icon="chevron-right"
						text="Next"
						disabled={!runtime.canAdvance()}
						onClick={() => {
							runtime.nextStep();
							bump((value) => value + 1);
						}}
					/>
				) : (
					<Button
						icon="check"
						text="Submit"
						disabled={!runtime.canAdvance()}
						onClick={() => {
							const result = runtime.submit();
							if (result.ok) onSubmit?.(result.values);
							bump((value) => value + 1);
						}}
					/>
				)}
			</div>
		</div>
	);
};
// #endregion 🖼️FormRenderer

// #region 🛠️FormBuilder
/** @emoji 🛠️ Embeddable form builder canvas; playground hosts use side-panel hierarchy, catalogue, and inspection tabs. */
export const FormBuilder: React.FC<FormBuilderProps> = ({ spec, onChange, className }) => {
	void onChange;
	return (
		<div className={cn("min-h-0 min-w-0", className)} data-slot="form-builder">
			<FormRenderer spec={spec} />
		</div>
	);
};
// #endregion 🛠️FormBuilder

// #region ⚡FlowGenerate
/** @emoji ⚡ Flow generate surface listing editable generations with preview. */
export const FlowGenerateSurface: React.FC<FlowGenerateSurfaceProps> = ({
	formSpec,
	generations,
	selectedGenerationId,
	previewText,
	onSelectGeneration,
	onAddGeneration,
	onRemoveGeneration,
	onGenerationValuesChange,
	onRenameGeneration,
	className,
}) => {
	const selected = generations.find((generation) => generation.id === selectedGenerationId) ?? generations[0];
	return (
		<div className={cn("grid min-h-0 min-w-0 grid-cols-[minmax(10rem,14rem)_minmax(0,1fr)_minmax(10rem,16rem)] gap-double p-double", className)} data-slot="flow-generate-surface">
			<div className="flex min-h-0 flex-col gap-single border border-border rounded-md p-single overflow-auto">
				<div className="flex items-center justify-between gap-single">
					<h3 className="text-sm font-medium">Generations</h3>
					<Button icon="plus" text="Add" onClick={onAddGeneration} />
				</div>
				{generations.map((generation) => (
					<div key={generation.id} className={cn("flex flex-col gap-half rounded-md border p-single", selected?.id === generation.id ? "border-primary" : "border-border")}>
						<button type="button" className="text-left text-sm font-medium truncate" onClick={() => onSelectGeneration(generation.id)}>
							{generation.name}
						</button>
						<Input value={generation.name} onChange={(event) => onRenameGeneration(generation.id, event.target.value)} />
						<Button icon="trash-2" text="Remove" onClick={() => onRemoveGeneration(generation.id)} />
					</div>
				))}
			</div>
			<div className="min-h-0 overflow-auto border border-border rounded-md">
				{selected ? (
					<FormRenderer spec={formSpec} values={selected.values} onChange={(values) => onGenerationValuesChange(selected.id, values)} />
				) : (
					<p className="p-double text-sm text-muted-foreground">Add a generation to begin.</p>
				)}
			</div>
			<div className="min-h-0 overflow-auto border border-border rounded-md p-double">
				<h3 className="text-sm font-medium mb-single">Preview</h3>
				<pre className="text-xs whitespace-pre-wrap break-words">{previewText}</pre>
			</div>
		</div>
	);
};

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
// #endregion ⚡FlowGenerate

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("forms-react", () => {
	it("maps flow fixture widgets to form spec", () => {
		const json = JSON.stringify({
			schema: "flow.fixture/v1",
			widgets: [{ kind: "inputSlider", id: "width", value: 4, min: 0, max: 10 }],
			synapses: [],
		});
		const spec = flowFixtureToFormSpec(json);
		expect(spec.steps[0]?.questions[0]?.kind).toBe("slider");
	});

	it("applies generation values to fixture", () => {
		const json = JSON.stringify({ schema: "flow.fixture/v1", widgets: [{ kind: "inputSlider", id: "width", value: 1 }], synapses: [] });
		const next = applyGenerationValuesToFixture(json, { width: 7 });
		const parsed = JSON.parse(next) as { widgets: { value: number }[] };
		expect(parsed.widgets[0]?.value).toBe(7);
	});

	it("serializes default form spec", () => {
		expect(formSpecFromJson(formSpecToJson(defaultFormSpec())).id).toBe("default");
	});
	});
}
// #endregion 🧪Tests

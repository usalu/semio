// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 📋 `@semio-tech/forms-react` — form renderer and builder UI. */
// #endregion 🧲Header

import {
	applyGenerationValuesToFixture,
	createFormId,
	defaultValueForQuestion,
	flowFixtureToFormSpec,
	formSpecToJson,
	FormRuntime,
	formsExtensionHost,
	isExtensionFormQuestion,
	parseFormSpec,
	questionKindContribution,
	registerFormsFlowFixtureResolver,
	resolveFormsFlowFixtureJson,
	resolveQuestionFixtureSlug,
	type FormQuestion,
	type FormQuestionExtension,
	type FormSelectOption,
	type FormSpec,
	type FormStep,
	type FormValue,
	type FormValues,
	type FormVectorField,
} from "@semio-tech/forms-core";
export {
	applyGenerationValuesToFixture,
	flowFixtureToFormSpec,
	formsExtensionHost,
	type FormsExtensionEntry,
	type FormsQuestionKindContribution,
} from "@semio-tech/forms-core";
import { FlowOrchestratorClient } from "@semio-tech/flow-react";
import {
	ensureProceduralBrepBridge,
	extractChannelPreviewItems,
	preferGeometryPreviewItems,
	attachPreviewMeshesToItems,
	ProceduralPreview,
	type ProceduralPreviewItem,
} from "@semio-tech/procedural-3d-react";
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
import { createPortal } from "react-dom";
import hexColumnFixture from "../../procedural/3d/example/hexagonal-mushroom-column.procedural.json";

function registerDefaultFormsFlowFixtures(): void {
	registerFormsFlowFixtureResolver("hexagonal-mushroom-column", () =>
		typeof hexColumnFixture === "string" ? hexColumnFixture : JSON.stringify(hexColumnFixture),
	);
}

registerDefaultFormsFlowFixtures();
void formsExtensionHost.activateDefaults();

let formsFlowEvalClient: FlowOrchestratorClient | null = null;

function getFormsFlowEvalClient(): FlowOrchestratorClient {
	if (!formsFlowEvalClient) formsFlowEvalClient = new FlowOrchestratorClient();
	return formsFlowEvalClient;
}

// #region 📐Contracts
export const FORMS_QUESTION_DRAG_MIME = "application/x-semio-forms-question-kind";

export interface FormRendererProps {
	readonly spec: FormSpec;
	readonly values?: FormValues;
	readonly onChange?: (values: FormValues) => void;
	readonly onSubmit?: (values: FormValues) => void;
	readonly interactive?: boolean;
	readonly className?: string;
}

export interface FormEditSurfaceProps {
	readonly spec: FormSpec;
	readonly onChange: (spec: FormSpec) => void;
	readonly selectedIds?: readonly string[];
	readonly onSelectionChange?: (selectedIds: readonly string[]) => void;
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
		schema: "forms.form",
		id,
		version: "1",
		title: "New Form",
		steps: [{ id: "step-1", title: "Step 1", questions: [{ id: "q-text", kind: "text", label: "Name" }] }],
	};
}

function FormBooleanToggle({
	id,
	pressed,
	onPressedChange,
	text,
}: {
	readonly id?: string;
	readonly pressed: boolean;
	onPressedChange: (pressed: boolean) => void;
	readonly text?: string;
}): React.ReactElement {
	return <Toggle id={id} icon="check" text={text ?? (pressed ? "Yes" : "No")} pressed={pressed} onPressedChange={onPressedChange} />;
}

function FormMultiSelectControl({
	id,
	options,
	value,
	onValue,
}: {
	readonly id: string;
	readonly options: readonly FormSelectOption[];
	readonly value: FormValue;
	onValue: (next: FormValue) => void;
}): React.ReactElement {
	const selected = Array.isArray(value) ? value : [];
	return (
		<div className="flex flex-wrap gap-single">
			{options.map((option) => {
				const active = selected.includes(option.value);
				return (
					<Toggle
						key={option.value}
						id={`${id}-${option.value}`}
						icon="hash"
						text={option.label}
						pressed={active}
						onPressedChange={(pressed) => {
							const current = [...selected];
							onValue(pressed ? [...current, option.value] : current.filter((entry) => entry !== option.value));
						}}
					/>
				);
			})}
		</div>
	);
}

function patchStepInSpec(spec: FormSpec, stepId: string, patch: Partial<FormStep>): FormSpec {
	return {
		...spec,
		steps: spec.steps.map((step) => (step.id === stepId ? { ...step, ...patch } : step)),
	};
}

function editorGridClass(): string {
	return "grid gap-single sm:grid-cols-2 lg:grid-cols-3";
}

function EditorMetaField({
	label,
	children,
	className,
}: {
	readonly label: string;
	readonly children: React.ReactNode;
	readonly className?: string;
}): React.ReactElement {
	return (
		<label className={cn("flex flex-col gap-half text-xs", className)}>
			<span className="font-medium text-muted-foreground">{label}</span>
			{children}
		</label>
	);
}

function FormSelectOptionsEditor({
	options,
	onChange,
}: {
	readonly options: readonly FormSelectOption[];
	onChange: (options: FormSelectOption[]) => void;
}): React.ReactElement {
	return (
		<div className="flex flex-col gap-single">
			<div className="flex items-center justify-between gap-single">
				<span className="text-xs font-medium text-muted-foreground">Options</span>
				<Button
					icon="plus"
					text="Add Option"
					onClick={() => onChange([...options, { value: createFormId("opt"), label: "New Option" }])}
				/>
			</div>
			{options.length === 0 ? <p className="text-xs text-muted-foreground">No options yet.</p> : null}
			{options.map((option, index) => (
				<div key={`${option.value}-${index}`} className="grid gap-single rounded-md border border-border p-single sm:grid-cols-[1fr_1fr_auto]">
					<EditorMetaField label="Value">
						<Input
							value={option.value}
							onChange={(event) => {
								const next = [...options];
								next[index] = { ...option, value: event.target.value };
								onChange(next);
							}}
						/>
					</EditorMetaField>
					<EditorMetaField label="Label">
						<Input
							value={option.label}
							onChange={(event) => {
								const next = [...options];
								next[index] = { ...option, label: event.target.value };
								onChange(next);
							}}
						/>
					</EditorMetaField>
					<div className="flex items-end">
						<Button
							icon="trash-2"
							text="Remove"
							disabled={options.length <= 1}
							onClick={() => onChange(options.filter((_, entryIndex) => entryIndex !== index))}
						/>
					</div>
				</div>
			))}
		</div>
	);
}

function FormVectorFieldsEditor({
	fields,
	onChange,
}: {
	readonly fields: readonly FormVectorField[];
	onChange: (fields: FormVectorField[]) => void;
}): React.ReactElement {
	return (
		<div className="flex flex-col gap-single">
			<div className="flex items-center justify-between gap-single">
				<span className="text-xs font-medium text-muted-foreground">Vector Fields</span>
				<Button icon="plus" text="Add Field" onClick={() => onChange([...fields, { key: createFormId("axis"), label: "Axis", value: 0 }])} />
			</div>
			{fields.map((field, index) => (
				<div key={`${field.key}-${index}`} className="grid gap-single rounded-md border border-border p-single sm:grid-cols-[1fr_1fr_1fr_auto]">
					<EditorMetaField label="Key">
						<Input
							value={field.key}
							onChange={(event) => {
								const next = [...fields];
								next[index] = { ...field, key: event.target.value };
								onChange(next);
							}}
						/>
					</EditorMetaField>
					<EditorMetaField label="Label">
						<Input
							value={field.label ?? ""}
							onChange={(event) => {
								const next = [...fields];
								next[index] = { ...field, label: event.target.value };
								onChange(next);
							}}
						/>
					</EditorMetaField>
					<EditorMetaField label="Default">
						<Input
							type="number"
							value={String(field.value ?? 0)}
							onChange={(event) => {
								const next = [...fields];
								next[index] = { ...field, value: Number(event.target.value) };
								onChange(next);
							}}
						/>
					</EditorMetaField>
					<div className="flex items-end">
						<Button icon="trash-2" text="Remove" disabled={fields.length <= 1} onClick={() => onChange(fields.filter((_, entryIndex) => entryIndex !== index))} />
					</div>
				</div>
			))}
		</div>
	);
}

function FormQuestionEditorCard({
	question,
	selected,
	onSelect,
	onPatch,
	onPatchParams,
}: {
	readonly question: FormQuestion;
	readonly selected: boolean;
	onSelect: () => void;
	onPatch: (patch: Partial<FormQuestion>) => void;
	onPatchParams: (params: FormValues) => void;
}): React.ReactElement {
	const contribution = isExtensionFormQuestion(question) ? questionKindContribution(question) : undefined;

	return (
		<div
			className={cn(
				"flex flex-col gap-medium rounded-md border border-border bg-background p-double transition-shadow",
				selected && "ring-2 ring-primary border-primary",
			)}
			data-slot="form-question-editor-card"
			onClick={onSelect}
			onKeyDown={(event) => {
				if (event.key === "Enter" || event.key === " ") {
					event.preventDefault();
					onSelect();
				}
			}}
			role="button"
			tabIndex={0}
		>
			<div className="flex flex-wrap items-center justify-between gap-single">
				<div className="flex flex-wrap items-center gap-single">
					<span className="rounded-md bg-muted px-single py-half text-xs font-medium uppercase tracking-wide">{question.kind}</span>
					<span className="text-xs text-muted-foreground">{question.id}</span>
				</div>
				<div className="flex items-center gap-single" onClick={(event) => event.stopPropagation()}>
					<span className="text-xs text-muted-foreground">Required</span>
					<FormBooleanToggle pressed={Boolean(question.required)} text="Required" onPressedChange={(pressed) => onPatch({ required: pressed || undefined })} />
				</div>
			</div>

			<div className={editorGridClass()} onClick={(event) => event.stopPropagation()}>
				<EditorMetaField label="Label" className="sm:col-span-2 lg:col-span-3">
					<Input value={question.label} onChange={(event) => onPatch({ label: event.target.value })} />
				</EditorMetaField>
				<EditorMetaField label="Description" className="sm:col-span-2 lg:col-span-3">
					<Textarea value={question.description ?? ""} onChange={(event) => onPatch({ description: event.target.value || undefined })} />
				</EditorMetaField>
			</div>

			<div onClick={(event) => event.stopPropagation()}>
				{question.kind === "text" || question.kind === "longText" ? (
					<div className={editorGridClass()}>
						<EditorMetaField label="Placeholder">
							<Input value={question.placeholder ?? ""} onChange={(event) => onPatch({ placeholder: event.target.value || undefined })} />
						</EditorMetaField>
						<EditorMetaField label="Default">
							{question.kind === "longText" ? (
								<Textarea value={String(question.default ?? "")} onChange={(event) => onPatch({ default: event.target.value })} />
							) : (
								<Input value={String(question.default ?? "")} onChange={(event) => onPatch({ default: event.target.value })} />
							)}
						</EditorMetaField>
					</div>
				) : null}

				{question.kind === "number" || question.kind === "slider" ? (
					<div className={editorGridClass()}>
						<EditorMetaField label="Min">
							<Input type="number" value={String(question.min ?? 0)} onChange={(event) => onPatch({ min: Number(event.target.value) })} />
						</EditorMetaField>
						<EditorMetaField label="Max">
							<Input type="number" value={String(question.max ?? 100)} onChange={(event) => onPatch({ max: Number(event.target.value) })} />
						</EditorMetaField>
						<EditorMetaField label="Step">
							<Input type="number" value={String(question.step ?? 1)} onChange={(event) => onPatch({ step: Number(event.target.value) })} />
						</EditorMetaField>
						<EditorMetaField label="Default">
							<Input type="number" value={String(question.default ?? question.min ?? 0)} onChange={(event) => onPatch({ default: Number(event.target.value) })} />
						</EditorMetaField>
						{question.kind === "slider" ? (
							<EditorMetaField label="Unit">
								<Input value={question.unit ?? ""} onChange={(event) => onPatch({ unit: event.target.value || undefined })} />
							</EditorMetaField>
						) : null}
					</div>
				) : null}

				{question.kind === "boolean" ? (
					<EditorMetaField label="Default">
						<FormBooleanToggle pressed={Boolean(question.default)} onPressedChange={(pressed) => onPatch({ default: pressed })} />
					</EditorMetaField>
				) : null}

				{question.kind === "single" || question.kind === "multi" ? (
					<div className="flex flex-col gap-medium">
						<FormSelectOptionsEditor options={[...question.options]} onChange={(options) => onPatch({ options })} />
						{question.kind === "single" ? (
							<EditorMetaField label="Default">
								<Select value={String(question.default ?? question.options[0]?.value ?? "")} onValueChange={(value) => onPatch({ default: value })}>
									<SelectTrigger>
										<SelectValue placeholder="Default option" />
									</SelectTrigger>
									<SelectContent>
										{question.options.map((option) => (
											<SelectItem key={option.value} value={option.value}>
												{option.label}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</EditorMetaField>
						) : (
							<EditorMetaField label="Default selections">
								<FormMultiSelectControl
									id={`${question.id}-default`}
									options={question.options}
									value={question.default ?? []}
									onValue={(next) => onPatch({ default: next as string[] })}
								/>
							</EditorMetaField>
						)}
					</div>
				) : null}

				{question.kind === "date" || question.kind === "color" ? (
					<EditorMetaField label="Default">
						<Input type={question.kind} value={String(question.default ?? "")} onChange={(event) => onPatch({ default: event.target.value })} />
					</EditorMetaField>
				) : null}

				{question.kind === "vector" ? (
					<div className="flex flex-col gap-medium">
						<div className={editorGridClass()}>
							<EditorMetaField label="Schema">
								<Input value={question.schema ?? ""} onChange={(event) => onPatch({ schema: event.target.value || undefined })} />
							</EditorMetaField>
							<EditorMetaField label="Step">
								<Input type="number" value={String(question.step ?? 0.1)} onChange={(event) => onPatch({ step: Number(event.target.value) })} />
							</EditorMetaField>
						</div>
						<FormVectorFieldsEditor fields={[...question.fields]} onChange={(fields) => onPatch({ fields })} />
					</div>
				) : null}

				{question.kind === "note" ? (
					<EditorMetaField label="Note text">
						<Textarea value={question.text} onChange={(event) => onPatch({ text: event.target.value })} />
					</EditorMetaField>
				) : null}

				{question.kind === "image" ? (
					<EditorMetaField label="Image URL">
						<Input value={question.src ?? ""} onChange={(event) => onPatch({ src: event.target.value || undefined })} />
					</EditorMetaField>
				) : null}

				{question.kind === "file" ? (
					<EditorMetaField label="Accept">
						<Input value={question.accept ?? ""} onChange={(event) => onPatch({ accept: event.target.value || undefined })} placeholder=".pdf,.png" />
					</EditorMetaField>
				) : null}

				{isExtensionFormQuestion(question) && usesFlow3dQuestionSurface(question, "edit") ? (
					<div className="mt-medium flex flex-col gap-medium border-t border-border pt-medium">
						<div className="flex flex-wrap items-center justify-between gap-single">
							<span className="text-xs font-medium text-muted-foreground">Procedural parameters</span>
							<span className="text-xs text-muted-foreground">{resolveQuestionFixtureSlug(question) ?? contribution?.edit?.fixtureSlug}</span>
						</div>
						<Flow3dQuestionControl
							question={question}
							value={question.params ?? (defaultValueForQuestion(question) as FormValues)}
							onValue={(next) => onPatchParams(next as FormValues)}
							interactive
						/>
					</div>
				) : null}
			</div>
		</div>
	);
}

function patchQuestionInSpec(spec: FormSpec, questionId: string, patch: Partial<FormQuestion>): FormSpec {
	return {
		...spec,
		steps: spec.steps.map((step) => ({
			...step,
			questions: step.questions.map((question) => (question.id === questionId ? ({ ...question, ...patch } as FormQuestion) : question)),
		})),
	};
}

function usesFlow3dQuestionSurface(question: FormQuestion, surface: "edit" | "try"): boolean {
	if (!isExtensionFormQuestion(question)) return false;
	const contribution = questionKindContribution(question);
	if (surface === "edit") return contribution?.edit?.surface === "flow3d" || contribution?.preview?.surface === "flow3d";
	return contribution?.preview?.surface === "flow3d";
}

function Flow3dQuestionControl({
	question,
	value,
	onValue,
	interactive,
}: {
	readonly question: FormQuestionExtension;
	readonly value: FormValue;
	readonly onValue: (next: FormValue) => void;
	readonly interactive: boolean;
}): React.ReactElement {
	const slug = resolveQuestionFixtureSlug(question);
	const fixtureJson = slug ? resolveFormsFlowFixtureJson(slug) : undefined;
	const paramSpec = React.useMemo(
		() => (fixtureJson ? flowFixtureToFormSpec(fixtureJson, `${question.id}-params`) : null),
		[fixtureJson, question.id],
	);
	const paramValues = (typeof value === "object" && value != null && !Array.isArray(value) ? value : {}) as FormValues;
	const paramValuesKey = React.useMemo(() => JSON.stringify(paramValues), [paramValues]);
	const [previewItems, setPreviewItems] = React.useState<readonly ProceduralPreviewItem[]>([]);
	const previewItemsRef = React.useRef<readonly ProceduralPreviewItem[]>([]);
	const [kernel, setKernel] = React.useState<Awaited<ReturnType<typeof ensureProceduralBrepBridge>>>();
	const evalGenRef = React.useRef(0);

	React.useEffect(() => {
		void ensureProceduralBrepBridge().then(setKernel);
	}, []);

	React.useEffect(() => {
		if (!fixtureJson || !interactive) return;
		const gen = ++evalGenRef.current;
		const timer = globalThis.setTimeout(() => {
			void (async () => {
				try {
					const client = getFormsFlowEvalClient();
					const patched = applyGenerationValuesToFixture(fixtureJson, paramValues);
					await client.loadFixtureJson(patched);
					const result = await client.evaluate();
					if (gen !== evalGenRef.current) return;
					const previewMeshes = await client.tessellatePreviews(result.outputsJson);
					if (gen !== evalGenRef.current) return;
					const nextItems = attachPreviewMeshesToItems(
						preferGeometryPreviewItems(extractChannelPreviewItems(result.outputsJson)),
						previewMeshes,
						previewItemsRef.current,
					);
					previewItemsRef.current = nextItems;
					setPreviewItems(nextItems);
				} catch (error) {
					console.log("[DEBUG] forms flow3d preview eval failed", error);
				}
			})();
		}, 200);
		return () => globalThis.clearTimeout(timer);
	}, [fixtureJson, interactive, paramValuesKey]);

	if (!paramSpec || !fixtureJson) {
		return <p className="text-xs text-muted-foreground">Flow fixture unavailable.</p>;
	}

	return (
		<div className="grid min-h-0 gap-double lg:grid-cols-[minmax(0,1fr)_minmax(12rem,18rem)]" data-slot="forms-flow3d-question">
			<FormRenderer spec={paramSpec} values={paramValues} interactive={interactive} onChange={(next) => onValue(next)} />
			<div className="relative min-h-[12rem] rounded-md border border-border">
				<ProceduralPreview items={previewItems} kernel={kernel ?? undefined} className="h-full min-h-[12rem]" />
			</div>
		</div>
	);
}

function questionControl(
	question: FormQuestion,
	value: FormValue,
	onValue: (next: FormValue) => void,
	interactive = true,
	surface: "edit" | "try" = "try",
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
				<div className="flex flex-col gap-single">
					<Slider
						id={id}
						min={question.min ?? 0}
						max={question.max ?? 100}
						step={question.step ?? 1}
						value={[typeof value === "number" ? value : Number(value ?? 0)]}
						onValueChange={(values) => onValue(values[0] ?? 0)}
					/>
					{question.unit ? (
						<span className="text-xs text-muted-foreground tabular-nums">
							{typeof value === "number" ? value : Number(value ?? 0)} {question.unit}
						</span>
					) : null}
				</div>
			);
		case "boolean":
			return <FormBooleanToggle id={id} pressed={Boolean(value)} onPressedChange={(pressed) => onValue(pressed)} />;
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
			return <FormMultiSelectControl id={id} options={question.options} value={value} onValue={onValue} />;
		case "date":
			return <Input id={id} type="date" value={String(value ?? "")} onChange={(event) => onValue(event.target.value)} />;
		case "color":
			return <Input id={id} type="color" value={String(value ?? "#336699")} onChange={(event) => onValue(event.target.value)} />;
		case "vector": {
			const numbers = Array.isArray(value) ? value.map((entry) => Number(entry)) : question.fields.map((field) => field.value ?? 0);
			return (
				<div className="flex flex-col gap-single">
					{question.fields.map((field, index) => (
						<EditorMetaField key={field.key} label={field.label ?? field.key.toUpperCase()}>
							<Stepper
								id={`${id}-${field.key}`}
								value={numbers[index] ?? 0}
								step={question.step ?? 0.1}
								onChange={(next) => {
									const copy = [...numbers];
									copy[index] = next;
									onValue(copy);
								}}
							/>
						</EditorMetaField>
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
			if (usesFlow3dQuestionSurface(question, surface)) {
				return <Flow3dQuestionControl question={question as FormQuestionExtension} value={value} onValue={onValue} interactive={interactive} />;
			}
			return null;
	}
}

/** @emoji 🖱️ Forms question palette drag session (workbench catalogue → hierarchy / builder). */
export const formsQuestionPaletteDragRef = { active: false };
export const formsQuestionPalettePointerDragRef = { active: false, encoded: null as string | null };
export const formsQuestionPaletteDragEncodedRef = { current: null as string | null };
export const formsQuestionPaletteDragClientRef = { clientX: 0, clientY: 0 };
export const formsQuestionPaletteDropCommittedRef = { current: false };

let formsQuestionPaletteDragPreviewRafId: number | null = null;

function formsStopQuestionPaletteDragPreviewLoop(): void {
	if (formsQuestionPaletteDragPreviewRafId !== null) {
		globalThis.cancelAnimationFrame?.(formsQuestionPaletteDragPreviewRafId);
		formsQuestionPaletteDragPreviewRafId = null;
	}
}

function formsTickQuestionPaletteDragPreview(): void {
	if (!formsReadActiveQuestionPaletteDragEncoded()) {
		formsStopQuestionPaletteDragPreviewLoop();
		return;
	}
	window.dispatchEvent(new CustomEvent("forms-question-drag-preview", { detail: { clientX: formsQuestionPaletteDragClientRef.clientX, clientY: formsQuestionPaletteDragClientRef.clientY } }));
	const requestFrame = globalThis.requestAnimationFrame?.bind(globalThis);
	if (!requestFrame) {
		formsQuestionPaletteDragPreviewRafId = null;
		return;
	}
	formsQuestionPaletteDragPreviewRafId = requestFrame(formsTickQuestionPaletteDragPreview);
}

function formsStartQuestionPaletteDragPreviewLoop(): void {
	if (formsQuestionPaletteDragPreviewRafId !== null) return;
	formsTickQuestionPaletteDragPreview();
}

/** @emoji 📦 Reads the encoded palette drag payload when a catalogue question drag is active. */
export function formsReadActiveQuestionPaletteDragEncoded(): string | null {
	const pointer = formsQuestionPalettePointerDragRef.encoded?.trim();
	if (pointer) return pointer;
	const shared = formsQuestionPaletteDragEncodedRef.current?.trim();
	return shared ? shared : null;
}

/** @emoji 🔍 Parses a catalogue drag payload into a question kind. */
export function parseFormsQuestionDragPayload(encoded: string): { kind?: string } | null {
	try {
		return JSON.parse(encoded) as { kind?: string };
	} catch {
		return null;
	}
}

/** @emoji 🏷️ Preview label for the floating palette drag ghost. */
export function formsQuestionPaletteDragPreviewLabel(encoded: string): string {
	const payload = parseFormsQuestionDragPayload(encoded);
	return payload?.kind ?? "Question";
}

/** @emoji 👻 Notes palette-drag client coordinates for preview refresh. */
export function formsNoteQuestionPaletteDragClient(clientX: number, clientY: number): void {
	formsQuestionPaletteDragClientRef.clientX = clientX;
	formsQuestionPaletteDragClientRef.clientY = clientY;
	if (!formsReadActiveQuestionPaletteDragEncoded()) return;
	formsStartQuestionPaletteDragPreviewLoop();
}

/** @emoji ⎋ Aborts an in-flight catalogue question palette drag. */
export function abortFormsQuestionPaletteDrag(): void {
	const wasActive = formsQuestionPalettePointerDragRef.active || formsQuestionPaletteDragRef.active;
	formsQuestionPalettePointerDragRef.active = false;
	formsQuestionPalettePointerDragRef.encoded = null;
	formsQuestionPaletteDragEncodedRef.current = null;
	formsQuestionPaletteDragRef.active = false;
	if (wasActive) {
		window.dispatchEvent(new CustomEvent("forms-question-drag-session", { detail: null }));
	}
	formsStopQuestionPaletteDragPreviewLoop();
	window.dispatchEvent(new CustomEvent("forms-question-drag-preview", { detail: null }));
}

/** @emoji 🖱️ Begins pointer palette drag with an encoded question kind payload. */
export function beginFormsQuestionPalettePointerDrag(encoded: string): void {
	formsQuestionPaletteDropCommittedRef.current = false;
	formsQuestionPalettePointerDragRef.active = true;
	formsQuestionPalettePointerDragRef.encoded = encoded;
	formsQuestionPaletteDragEncodedRef.current = encoded;
	formsQuestionPaletteDragRef.active = true;
	window.dispatchEvent(new CustomEvent("forms-question-drag-session", { detail: { encoded } }));
	formsStartQuestionPaletteDragPreviewLoop();
}

/** @emoji 🖱️ Ends pointer palette drag without committing a drop. */
export function cancelFormsQuestionPalettePointerDrag(): void {
	if (!formsQuestionPalettePointerDragRef.active && !formsQuestionPaletteDragRef.active) return;
	formsQuestionPalettePointerDragRef.active = false;
	formsQuestionPalettePointerDragRef.encoded = null;
	formsQuestionPaletteDragEncodedRef.current = null;
	formsQuestionPaletteDragRef.active = false;
	formsStopQuestionPaletteDragPreviewLoop();
	window.dispatchEvent(new CustomEvent("forms-question-drag-session", { detail: null }));
	window.dispatchEvent(new CustomEvent("forms-question-drag-preview", { detail: null }));
}

/** @emoji 🔍 Whether a drag gesture carries a forms question palette payload. */
export function formsQuestionDragAcceptsTransfer(types: readonly string[]): boolean {
	if (formsQuestionPalettePointerDragRef.active) return true;
	if (types.includes(FORMS_QUESTION_DRAG_MIME)) return true;
	return Boolean(formsQuestionPaletteDragRef.active && formsReadActiveQuestionPaletteDragEncoded());
}

/** @emoji 📥 Commits a palette question drop at client coordinates. */
export function endFormsQuestionPalettePointerDrag(
	clientX: number,
	clientY: number,
	onDrop: (detail: { kind: string; clientX: number; clientY: number }) => boolean,
): void {
	if (!formsQuestionPalettePointerDragRef.active) return;
	const encoded = formsQuestionPalettePointerDragRef.encoded;
	cancelFormsQuestionPalettePointerDrag();
	if (!encoded) return;
	const payload = parseFormsQuestionDragPayload(encoded);
	if (!payload?.kind) return;
	if (onDrop({ kind: payload.kind, clientX, clientY })) {
		formsQuestionPaletteDropCommittedRef.current = true;
	}
}

/** @emoji 👻 Floating label following the cursor during catalogue question drags. */
export const FormsQuestionPaletteDragGhost: React.FC = () => {
	const [tick, setTick] = React.useState(0);
	React.useEffect(() => {
		const onPreview = () => setTick((value) => value + 1);
		const onSession = () => setTick((value) => value + 1);
		window.addEventListener("forms-question-drag-preview", onPreview);
		window.addEventListener("forms-question-drag-session", onSession);
		return () => {
			window.removeEventListener("forms-question-drag-preview", onPreview);
			window.removeEventListener("forms-question-drag-session", onSession);
		};
	}, []);
	const encoded = formsReadActiveQuestionPaletteDragEncoded();
	if (!encoded || typeof document === "undefined" || !document.body) return null;
	const { clientX, clientY } = formsQuestionPaletteDragClientRef;
	return createPortal(
		<div
			className="border-primary bg-panel text-foreground pointer-events-none fixed z-tutorial rounded-md border px-2 py-1 text-xs shadow-md"
			style={{ left: clientX + 12, top: clientY + 12 }}
		>
			{formsQuestionPaletteDragPreviewLabel(encoded)}
		</div>,
		document.body,
	);
};

/** @emoji 📍 Global pointer / HTML5 drag bridge for catalogue question drops. */
export function FormsQuestionPaletteDragBridge(props: {
	readonly enabled?: boolean;
	readonly onCommitDrop: (detail: { kind: string; clientX: number; clientY: number }) => boolean;
}): null {
	const enabled = props.enabled ?? true;
	const onCommitDrop = props.onCommitDrop;
	React.useEffect(() => {
		if (!enabled) return;
		const onDragOver = (event: DragEvent): void => {
			if (!formsQuestionDragAcceptsTransfer([...event.dataTransfer!.types])) return;
			formsNoteQuestionPaletteDragClient(event.clientX, event.clientY);
		};
		window.addEventListener("dragover", onDragOver);
		return () => window.removeEventListener("dragover", onDragOver);
	}, [enabled]);
	React.useEffect(() => {
		if (!enabled) return;
		const onPointerMove = (event: PointerEvent): void => {
			if (!formsQuestionPalettePointerDragRef.active) return;
			formsNoteQuestionPaletteDragClient(event.clientX, event.clientY);
		};
		const onPointerUp = (event: PointerEvent): void => {
			endFormsQuestionPalettePointerDrag(event.clientX, event.clientY, onCommitDrop);
		};
		const onPointerCancel = (): void => {
			if (!formsQuestionPalettePointerDragRef.active) return;
			abortFormsQuestionPaletteDrag();
		};
		const onKeyDown = (event: KeyboardEvent): void => {
			if (event.key !== "Escape" || !formsReadActiveQuestionPaletteDragEncoded()) return;
			event.preventDefault();
			abortFormsQuestionPaletteDrag();
		};
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp, true);
		window.addEventListener("pointercancel", onPointerCancel);
		window.addEventListener("keydown", onKeyDown, true);
		return () => {
			window.removeEventListener("pointermove", onPointerMove);
			window.removeEventListener("pointerup", onPointerUp, true);
			window.removeEventListener("pointercancel", onPointerCancel);
			window.removeEventListener("keydown", onKeyDown, true);
		};
	}, [enabled, onCommitDrop]);
	return null;
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
			begin: beginFormsQuestionPalettePointerDrag,
			cancel: cancelFormsQuestionPalettePointerDrag,
		},
		onDragStart: ({ sourceItem }) => {
			if (formsQuestionPalettePointerDragRef.active) return;
			formsQuestionPaletteDropCommittedRef.current = false;
			const payload = readEncoded(dragDataByItemId.get(sourceItem.id));
			if (!payload) return;
			formsQuestionPaletteDragRef.active = true;
			formsQuestionPaletteDragEncodedRef.current = payload;
			window.dispatchEvent(new CustomEvent("forms-question-drag-session", { detail: { encoded: payload } }));
			formsStartQuestionPaletteDragPreviewLoop();
		},
		onDragEnd: () => {
			if (formsQuestionPalettePointerDragRef.active) return;
			formsQuestionPaletteDragEncodedRef.current = null;
			formsQuestionPaletteDragRef.active = false;
			if (!formsQuestionPaletteDropCommittedRef.current) {
				formsStopQuestionPaletteDragPreviewLoop();
				window.dispatchEvent(new CustomEvent("forms-question-drag-preview", { detail: null }));
			}
			formsQuestionPaletteDropCommittedRef.current = false;
			window.dispatchEvent(new CustomEvent("forms-question-drag-session", { detail: null }));
		},
	};
}
// #endregion 🔧Helpers

// #region 🖼️FormRenderer
/** @emoji 🖼️ Interactive multi-step form renderer. */
export const FormRenderer: React.FC<FormRendererProps> = ({ spec, values, onChange, onSubmit, interactive = true, className }) => {
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
			<div className={cn("flex flex-col gap-medium", !interactive && "pointer-events-none opacity-90")}>
				{runtime.getVisibleQuestions().map((question) => (
					<Field key={question.id} id={question.id} label={question.label} description={question.description} required={question.required} error={errorById[question.id]}>
						{questionControl(question, runtime.getValues()[question.id] ?? null, (next) => setValue(question.id, next), interactive, "try")}
					</Field>
				))}
			</div>
			{interactive ? (
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
			) : (
				<p className="text-xs text-muted-foreground">Switch to Try mode to fill out this form.</p>
			)}
		</div>
	);
};
// #endregion 🖼️FormRenderer

// #region ✏️FormEditSurface
/** @emoji ✏️ Structural form editor with inline label, option, and range editing. */
export const FormEditSurface: React.FC<FormEditSurfaceProps> = ({ spec, onChange, selectedIds = [], onSelectionChange, className }) => {
	const patchQuestion = (questionId: string, patch: Partial<FormQuestion>) => {
		onChange(patchQuestionInSpec(spec, questionId, patch));
	};

	return (
		<div className={cn("flex flex-col gap-double p-double min-w-0 overflow-auto", className)} data-slot="form-edit-surface">
			<div className="flex flex-col gap-single border-b border-border pb-medium">
				<span className="text-xs font-medium uppercase tracking-wide text-muted-foreground">Form</span>
				<Input value={spec.title ?? spec.id} onChange={(event) => onChange({ ...spec, title: event.target.value || undefined })} />
			</div>
			{spec.steps.map((step) => (
				<section key={step.id} className="flex flex-col gap-medium">
					<div className="flex flex-col gap-single rounded-md border border-border bg-muted/20 p-double">
						<span className="text-xs font-medium uppercase tracking-wide text-muted-foreground">Step</span>
						<Input value={step.title} onChange={(event) => onChange(patchStepInSpec(spec, step.id, { title: event.target.value }))} />
						<Textarea
							value={step.description ?? ""}
							placeholder="Step description"
							onChange={(event) => onChange(patchStepInSpec(spec, step.id, { description: event.target.value || undefined }))}
						/>
					</div>
					{step.questions.map((question) => (
						<FormQuestionEditorCard
							key={question.id}
							question={question}
							selected={selectedIds.includes(question.id)}
							onSelect={() => onSelectionChange?.([question.id])}
							onPatch={(patch) => patchQuestion(question.id, patch)}
							onPatchParams={(params) => patchQuestion(question.id, { params } as Partial<FormQuestion>)}
						/>
					))}
				</section>
			))}
		</div>
	);
};
// #endregion ✏️FormEditSurface

// #region 🛠️FormBuilder
/** @emoji 🛠️ Embeddable form builder canvas; playground hosts use side-panel hierarchy, catalogue, and inspection tabs. */
export const FormBuilder: React.FC<FormBuilderProps> = ({ spec, onChange, selectedIds, onSelectionChange, className }) => {
	return (
		<div className={cn("min-h-0 min-w-0", className)} data-slot="form-builder">
			<FormEditSurface spec={spec} onChange={onChange} selectedIds={selectedIds} onSelectionChange={onSelectionChange} />
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

// #endregion ⚡FlowGenerate

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("forms-react", () => {
	it("maps flow fixture widgets to form spec", () => {
		const json = JSON.stringify({
			schema: "flow.fixture",
			widgets: [{ kind: "inputSlider", id: "width", value: 4, min: 0, max: 10 }],
			synapses: [],
		});
		const spec = flowFixtureToFormSpec(json);
		expect(spec.steps[0]?.questions[0]?.kind).toBe("slider");
	});

	it("applies generation values to fixture", () => {
		const json = JSON.stringify({ schema: "flow.fixture", widgets: [{ kind: "inputSlider", id: "width", value: 1 }], synapses: [] });
		const next = applyGenerationValuesToFixture(json, { width: 7 });
		const parsed = JSON.parse(next) as { widgets: { value: number }[] };
		expect(parsed.widgets[0]?.value).toBe(7);
	});

	it("serializes default form spec", () => {
		expect(formSpecFromJson(formSpecToJson(defaultFormSpec())).id).toBe("default");
	});

	it("parses question palette drag payloads", () => {
		const encoded = JSON.stringify({ kind: "slider" });
		expect(parseFormsQuestionDragPayload(encoded)?.kind).toBe("slider");
		expect(formsQuestionDragAcceptsTransfer([FORMS_QUESTION_DRAG_MIME])).toBe(true);
	});

	it("registers procedural building component kind", () => {
		expect(formsExtensionHost.findQuestionKind("buildingComponent")?.preview?.surface).toBe("flow3d");
	});
	});
}
// #endregion 🧪Tests

/** @emoji ⚙️ `@semio-tech/imperative-react` — step list editor components. */
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import initImperativeWasm, { ImperativeSession, initSync } from "../../core/rs/pkg/imperative_core.js";
import {
	DEFAULT_IMPERATIVE_DOCUMENT,
	DEFAULT_IMPERATIVE_CATALOGUE,
	CONTROL_MODULE_CATALOGUE_SECTION,
	imperativeDocumentToJson,
	parseImperativeDocumentJson,
	performImperativeEffects,
	type EffectLogEntry,
	type ImperativeCatalogueItem,
	type ImperativeCatalogueSection,
	type ImperativeDocument,
	type ImperativePathRef,
	type ImperativeStep,
	type RunResult,
} from "@semio-tech/imperative-core";
import { ImperativeRunClient } from "@semio-tech/imperative-core";

// #region 🔖WasmBridge
if (import.meta.env.VITEST) {
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../../core/rs/pkg/imperative_core_bg.wasm");
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initImperativeWasm();
}

export async function ensureImperativeWasmLoaded(): Promise<void> {
	await initImperativeWasm();
}

export { ImperativeSession };
// #endregion 🔖WasmBridge

export type { EffectLogEntry, ImperativeCatalogueItem, ImperativeCatalogueSection, ImperativeDocument, ImperativePathRef, ImperativeStep, RunResult };

export interface ImperativeEditorProps {
	readonly documentJson?: string;
	readonly className?: string;
	readonly onDocumentChange?: (json: string) => void;
	readonly onRunResult?: (result: RunResult) => void;
}

function parseCatalogue(raw: string): readonly ImperativeCatalogueSection[] {
	try {
		const parsed = JSON.parse(raw) as { sections?: ImperativeCatalogueSection[] };
		return parsed.sections ?? [];
	} catch {
		return [];
	}
}

function parseRunResult(raw: string): RunResult | null {
	try {
		return JSON.parse(raw) as RunResult;
	} catch {
		return null;
	}
}

function isControlKind(kind: string): boolean {
	return kind.startsWith("control.");
}

function controlSlots(kind: string): readonly string[] {
	if (kind === "control.if") return ["then", "else"];
	if (kind === "control.while" || kind === "control.repeat") return ["body"];
	return [];
}

function pathRefKey(pathRef: ImperativePathRef): string {
	return pathRef.owner && pathRef.slot ? `${pathRef.owner}/${pathRef.slot}` : "root";
}

function pathRefJson(pathRef: ImperativePathRef): string {
	return JSON.stringify(pathRef);
}

export interface StepListEditorProps {
	readonly session: ImperativeSession;
	readonly document: ImperativeDocument;
	readonly catalogue: readonly ImperativeCatalogueSection[];
	readonly pathRef: ImperativePathRef;
	readonly depth: number;
	readonly selectedStepId: string | null;
	readonly onSelect: (stepId: string) => void;
	readonly onSync: () => void;
}

/** @emoji 📝 Recursive imperative step list editor. */
export function StepListEditor({
	session,
	document,
	catalogue,
	pathRef,
	depth,
	selectedStepId,
	onSelect,
	onSync,
}: StepListEditorProps): React.JSX.Element {
	const steps = useMemo(() => {
		if (!pathRef.owner || !pathRef.slot) return document.path.steps;
		const owner = document.path.steps.find((step) => step.id === pathRef.owner);
		return owner?.bodies?.[pathRef.slot]?.steps ?? [];
	}, [document, pathRef]);

	const addStep = (kind: string) => {
		if (pathRef.owner && pathRef.slot) {
			session.addStepAt(pathRefJson(pathRef), kind, undefined);
		} else {
			session.addStep(kind, undefined);
		}
		onSync();
		onSelect(session.pathJson().includes(kind) ? selectedStepId ?? "" : selectedStepId ?? "");
	};

	const moveStep = (id: string, direction: -1 | 1) => {
		const index = steps.findIndex((step) => step.id === id);
		if (index < 0) return;
		const target = index + direction;
		if (target < 0 || target >= steps.length) return;
		if (pathRef.owner && pathRef.slot) {
			session.moveStepAt(pathRefJson(pathRef), id, target);
		} else {
			session.moveStep(id, target);
		}
		onSync();
	};

	return (
		<ul className="min-h-0 flex-1 overflow-auto p-2 text-sm" style={{ paddingLeft: `${depth * 12 + 8}px` }}>
			{pathRef.owner && pathRef.slot ? (
				<li className="mb-2 text-xs font-medium text-[var(--muted-foreground)]">{pathRef.slot}</li>
			) : null}
			{steps.map((step, index) => (
				<li key={step.id} className="mb-2">
					<div className={`flex items-center gap-1 rounded border px-2 py-1 ${selectedStepId === step.id ? "bg-[var(--accent)]/10" : ""}`}>
						<button type="button" className="flex-1 text-left" onClick={() => onSelect(step.id)}>
							{index + 1}. {step.kind}
						</button>
						<button type="button" className="text-xs" onClick={() => moveStep(step.id, -1)} aria-label="Move up">
							↑
						</button>
						<button type="button" className="text-xs" onClick={() => moveStep(step.id, 1)} aria-label="Move down">
							↓
						</button>
					</div>
					{isControlKind(step.kind)
						? controlSlots(step.kind).map((slotName) => (
								<StepListEditor
									key={`${step.id}-${slotName}`}
									session={session}
									document={document}
									catalogue={catalogue}
									pathRef={{ owner: step.id, slot: slotName }}
									depth={depth + 1}
									selectedStepId={selectedStepId}
									onSelect={onSelect}
									onSync={onSync}
								/>
							))
						: null}
				</li>
			))}
			<li className="mt-2 flex flex-wrap gap-1">
				{catalogue.flatMap((section) =>
					section.items.slice(0, 2).map((item) => (
						<button key={`${pathRefKey(pathRef)}-${item.kind}`} type="button" className="rounded border px-2 py-0.5 text-xs" onClick={() => addStep(item.kind)}>
							+ {item.abbreviation}
						</button>
					)),
				)}
			</li>
		</ul>
	);
}

/** @emoji 📝 Ordered step-list imperative editor. */
export function ImperativeEditor({ documentJson, className, onDocumentChange, onRunResult }: ImperativeEditorProps): React.JSX.Element {
	const session = useMemo(() => new ImperativeSession(), []);
	const runClientRef = useRef<ImperativeRunClient | null>(null);
	const [document, setDocument] = useState<ImperativeDocument>(DEFAULT_IMPERATIVE_DOCUMENT);
	const [catalogue, setCatalogue] = useState<readonly ImperativeCatalogueSection[]>([]);
	const [selectedStepId, setSelectedStepId] = useState<string | null>(null);
	const [compiledText, setCompiledText] = useState("");
	const [effectLog, setEffectLog] = useState<readonly EffectLogEntry[]>([]);
	const [scope, setScope] = useState<Record<string, unknown>>({});
	const [running, setRunning] = useState(false);

	const syncFromSession = useCallback(() => {
		const json = session.pathJson();
		const parsed = parseImperativeDocumentJson(json);
		if (parsed) setDocument(parsed);
		setCompiledText(session.compileText());
		onDocumentChange?.(json);
	}, [onDocumentChange, session]);

	useEffect(() => {
		const client = new ImperativeRunClient();
		runClientRef.current = client;
		return () => {
			client.terminate();
			runClientRef.current = null;
		};
	}, []);

	useEffect(() => {
		const json = documentJson ?? imperativeDocumentToJson(DEFAULT_IMPERATIVE_DOCUMENT);
		session.loadPathJson(json);
		setCatalogue(parseCatalogue(session.catalogueJson()));
		syncFromSession();
		setSelectedStepId(parseImperativeDocumentJson(json)?.path.steps[0]?.id ?? null);
	}, [documentJson, session, syncFromSession]);

	const selectedStep = useMemo(() => {
		const findStep = (steps: readonly ImperativeStep[]): ImperativeStep | null => {
			for (const step of steps) {
				if (step.id === selectedStepId) return step;
				if (step.bodies) {
					for (const body of Object.values(step.bodies)) {
						const nested = findStep(body.steps);
						if (nested) return nested;
					}
				}
			}
			return null;
		};
		return findStep(document.path.steps);
	}, [document.path.steps, selectedStepId]);

	const addStep = (kind: string) => {
		const id = session.addStep(kind, undefined);
		syncFromSession();
		setSelectedStepId(id);
	};

	const removeSelected = () => {
		if (!selectedStepId) return;
		session.removeStep(selectedStepId);
		syncFromSession();
		setSelectedStepId(document.path.steps[0]?.id ?? null);
	};

	const updateParam = (key: string, value: unknown) => {
		if (!selectedStep) return;
		const params = { ...selectedStep.params, [key]: value };
		session.setStepParamsJson(selectedStep.id, JSON.stringify(params));
		syncFromSession();
	};

	const runPath = async () => {
		const client = runClientRef.current;
		if (!client) return;
		setRunning(true);
		try {
			const result = await client.runDocument(session.pathJson());
			setEffectLog(result.effects);
			setScope(result.scope);
			onRunResult?.(result);
			const logs: string[] = [];
			await performImperativeEffects(result.effects, {
				onLog: (message) => logs.push(message),
				onStateChange: (key, value) => setScope((prev) => ({ ...prev, [key]: value })),
			});
			if (logs.length > 0) {
				setEffectLog((prev) =>
					prev.map((entry) =>
						entry.kind === "log.print" && entry.output?.message
							? { ...entry, output: { ...entry.output, performed: logs.shift() ?? "" } }
							: entry,
					),
				);
			}
		} finally {
			setRunning(false);
		}
	};

	const stopRun = () => {
		runClientRef.current?.stop();
		setRunning(false);
		setEffectLog((prev) => [...prev, { stepId: "", kind: "control.stop", input: {}, error: "Stopped by user" }]);
	};

	return (
		<div className={className ?? "flex h-full min-h-0 flex-col gap-3 p-3"}>
			<header className="flex flex-wrap items-center gap-2">
				<h2 className="mr-2 text-sm font-semibold">Imperative Path</h2>
				{catalogue.flatMap((section) =>
					section.items.map((item) => (
						<button key={item.kind} type="button" className="rounded border px-2 py-1 text-xs" onClick={() => addStep(item.kind)}>
							+ {item.name}
						</button>
					)),
				)}
				<button type="button" className="rounded border px-2 py-1 text-xs" disabled={running} onClick={() => void runPath()}>
					{running ? "Running…" : "Run"}
				</button>
				<button type="button" className="rounded border px-2 py-1 text-xs" disabled={!running} onClick={stopRun}>
					Stop
				</button>
			</header>
			<div className="grid min-h-0 flex-1 grid-cols-[minmax(12rem,1fr)_minmax(12rem,1fr)] gap-3">
				<section className="flex min-h-0 flex-col rounded border">
					<div className="border-b px-2 py-1 text-xs font-medium">Steps</div>
					<StepListEditor
						session={session}
						document={document}
						catalogue={catalogue.length ? catalogue : [...DEFAULT_IMPERATIVE_CATALOGUE.sections, CONTROL_MODULE_CATALOGUE_SECTION]}
						pathRef={{}}
						depth={0}
						selectedStepId={selectedStepId}
						onSelect={setSelectedStepId}
						onSync={syncFromSession}
					/>
					{selectedStep ? (
						<div className="border-t p-2 text-xs">
							<div className="mb-2 font-medium">Params · {selectedStep.kind}</div>
							<StepParamForm step={selectedStep} catalogue={catalogue} onChange={updateParam} onRemove={removeSelected} />
						</div>
					) : null}
				</section>
				<div className="flex min-h-0 flex-col gap-3">
					<section className="min-h-0 flex-1 rounded border">
						<div className="border-b px-2 py-1 text-xs font-medium">Compiled Text</div>
						<pre className="overflow-auto p-2 text-xs whitespace-pre-wrap">{compiledText || "—"}</pre>
					</section>
					<section className="min-h-0 flex-1 rounded border">
						<div className="border-b px-2 py-1 text-xs font-medium">Effect Log</div>
						<ul className="overflow-auto p-2 text-xs">
							{effectLog.length === 0 ? <li className="text-[var(--muted-foreground)]">Run to see effects</li> : null}
							{effectLog.map((entry, index) => (
								<li key={`${entry.stepId}-${index}`} className="mb-1 rounded border px-2 py-1">
									<strong>{entry.kind}</strong>
									{entry.error ? <span className="text-red-500"> · {entry.error}</span> : null}
								</li>
							))}
						</ul>
					</section>
					<section className="rounded border">
						<div className="border-b px-2 py-1 text-xs font-medium">Scope</div>
						<pre className="overflow-auto p-2 text-xs">{JSON.stringify(scope, null, 2)}</pre>
					</section>
				</div>
			</div>
		</div>
	);
}

export interface StepParamFormProps {
	readonly step: ImperativeStep;
	readonly catalogue?: readonly ImperativeCatalogueSection[];
	readonly onChange: (key: string, value: unknown) => void;
	readonly onRemove: () => void;
}

function catalogueFieldsForStep(step: ImperativeStep, catalogue: readonly ImperativeCatalogueSection[]): readonly { readonly key: string; readonly label: string; readonly type: "text" | "number" }[] {
	const item = catalogue.flatMap((section) => section.items).find((entry) => entry.kind === step.kind);
	if (!item?.inputs.length) {
		return Object.keys(step.params).map((key) => ({
			key,
			label: key,
			type: typeof step.params[key] === "number" ? ("number" as const) : ("text" as const),
		}));
	}
	return item.inputs.map((input) => ({
		key: input.name,
		label: input.name,
		type: input.code === "N" ? ("number" as const) : ("text" as const),
	}));
}

/** @emoji 🎛️ Edits params for one imperative step. */
export function StepParamForm({ step, catalogue = DEFAULT_IMPERATIVE_CATALOGUE.sections, onChange, onRemove }: StepParamFormProps): React.JSX.Element {
	const fields = catalogueFieldsForStep(step, catalogue);

	return (
		<div className="flex flex-col gap-2">
			{fields.map((field) => (
				<label key={field.key} className="flex flex-col gap-1">
					<span>{field.label}</span>
					<input
						className="rounded border px-2 py-1"
						type={field.type}
						value={String(step.params[field.key] ?? "")}
						onChange={(event) => onChange(field.key, field.type === "number" ? Number(event.target.value) : event.target.value)}
					/>
				</label>
			))}
			<button type="button" className="self-start rounded border px-2 py-1 text-red-600" onClick={onRemove}>
				Remove step
			</button>
		</div>
	);
}

/** @emoji 📋 Read-only effect log panel. */
export function EffectLogPanel({ entries }: { readonly entries: readonly EffectLogEntry[] }): React.JSX.Element {
	return (
		<ul className="overflow-auto p-2 text-xs">
			{entries.map((entry, index) => (
				<li key={`${entry.stepId}-${index}`} className="mb-1 rounded border px-2 py-1">
					<strong>{entry.kind}</strong>
					{entry.error ? <span className="text-red-500"> · {entry.error}</span> : null}
				</li>
			))}
		</ul>
	);
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("StepParamForm fields", () => {
		it("derives log.print fields from catalogue", () => {
			const fields = catalogueFieldsForStep(
				{ id: "s1", kind: "log.print", params: { message: "hi" } },
				DEFAULT_IMPERATIVE_CATALOGUE.sections,
			);
			expect(fields.map((field) => field.key)).toEqual(["message"]);
		});
		it("derives wait.delay number field from catalogue", () => {
			const fields = catalogueFieldsForStep(
				{ id: "s1", kind: "wait.delay", params: { ms: 10 } },
				DEFAULT_IMPERATIVE_CATALOGUE.sections,
			);
			expect(fields[0]?.type).toBe("number");
		});
	});
	describe("controlSlots", () => {
		it("returns then/else for control.if", () => {
			expect(controlSlots("control.if")).toEqual(["then", "else"]);
		});
	});
}

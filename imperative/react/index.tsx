/** @emoji ⚙️ `@semio-tech/imperative-react` — step list editor components. */
import React, { useCallback, useEffect, useMemo, useState } from "react";
import initImperativeWasm, { ImperativeSession, initSync } from "../core/pkg/imperative_core.js";
import {
	DEFAULT_IMPERATIVE_DOCUMENT,
	DEFAULT_IMPERATIVE_CATALOGUE,
	imperativeDocumentToJson,
	parseImperativeDocumentJson,
	performImperativeEffects,
	type EffectLogEntry,
	type ImperativeCatalogueItem,
	type ImperativeCatalogueSection,
	type ImperativeDocumentV1,
	type ImperativeStepV1,
	type RunResult,
} from "@semio-tech/imperative-core";

// #region 🔖WasmBridge
if (import.meta.env.VITEST) {
	const { readFileSync } = await import("node:fs");
	const { dirname, join } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const wasmPath = join(dirname(fileURLToPath(import.meta.url)), "../core/pkg/imperative_core_bg.wasm");
	initSync({ module: readFileSync(wasmPath) });
} else {
	await initImperativeWasm();
}

export async function ensureImperativeWasmLoaded(): Promise<void> {
	await initImperativeWasm();
}

export { ImperativeSession };
// #endregion 🔖WasmBridge

export type { EffectLogEntry, ImperativeCatalogueItem, ImperativeCatalogueSection, ImperativeDocumentV1, ImperativeStepV1, RunResult };

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

/** @emoji 📝 Ordered step-list imperative editor. */
export function ImperativeEditor({ documentJson, className, onDocumentChange, onRunResult }: ImperativeEditorProps): React.JSX.Element {
	const session = useMemo(() => new ImperativeSession(), []);
	const [document, setDocument] = useState<ImperativeDocumentV1>(DEFAULT_IMPERATIVE_DOCUMENT);
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
		const json = documentJson ?? imperativeDocumentToJson(DEFAULT_IMPERATIVE_DOCUMENT);
		session.loadPathJson(json);
		setCatalogue(parseCatalogue(session.catalogueJson()));
		syncFromSession();
		setSelectedStepId(parseImperativeDocumentJson(json)?.path.steps[0]?.id ?? null);
	}, [documentJson, session, syncFromSession]);

	const selectedStep = document.path.steps.find((step) => step.id === selectedStepId) ?? null;

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

	const moveStep = (id: string, direction: -1 | 1) => {
		const index = document.path.steps.findIndex((step) => step.id === id);
		if (index < 0) return;
		const target = index + direction;
		if (target < 0 || target >= document.path.steps.length) return;
		session.moveStep(id, target);
		syncFromSession();
	};

	const updateParam = (key: string, value: unknown) => {
		if (!selectedStep) return;
		const params = { ...selectedStep.params, [key]: value };
		session.setStepParamsJson(selectedStep.id, JSON.stringify(params));
		syncFromSession();
	};

	const runPath = async () => {
		setRunning(true);
		try {
			const raw = session.run();
			const result = parseRunResult(raw);
			if (!result) return;
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
			</header>
			<div className="grid min-h-0 flex-1 grid-cols-[minmax(12rem,1fr)_minmax(12rem,1fr)] gap-3">
				<section className="flex min-h-0 flex-col rounded border">
					<div className="border-b px-2 py-1 text-xs font-medium">Steps</div>
					<ul className="min-h-0 flex-1 overflow-auto p-2 text-sm">
						{document.path.steps.map((step, index) => (
							<li key={step.id} className={`mb-1 flex items-center gap-1 rounded border px-2 py-1 ${selectedStepId === step.id ? "bg-[var(--accent)]/10" : ""}`}>
								<button type="button" className="flex-1 text-left" onClick={() => setSelectedStepId(step.id)}>
									{index + 1}. {step.kind}
								</button>
								<button type="button" className="text-xs" onClick={() => moveStep(step.id, -1)} aria-label="Move up">
									↑
								</button>
								<button type="button" className="text-xs" onClick={() => moveStep(step.id, 1)} aria-label="Move down">
									↓
								</button>
							</li>
						))}
					</ul>
					{selectedStep ? (
						<div className="border-t p-2 text-xs">
							<div className="mb-2 font-medium">Params · {selectedStep.kind}</div>
							<StepParamForm step={selectedStep} onChange={updateParam} onRemove={removeSelected} />
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
							{effectLog.map((entry) => (
								<li key={entry.stepId} className="mb-1 rounded border px-2 py-1">
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
	readonly step: ImperativeStepV1;
	readonly catalogue?: readonly ImperativeCatalogueSection[];
	readonly onChange: (key: string, value: unknown) => void;
	readonly onRemove: () => void;
}

function catalogueFieldsForStep(step: ImperativeStepV1, catalogue: readonly ImperativeCatalogueSection[]): readonly { readonly key: string; readonly label: string; readonly type: "text" | "number" }[] {
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
			{entries.map((entry) => (
				<li key={entry.stepId} className="mb-1 rounded border px-2 py-1">
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
}

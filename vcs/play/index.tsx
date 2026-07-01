// #region 🧲Header
/** @emoji 🗄️ VCS play — history table playground on `@semio-tech/vcs-react`. */
// #endregion 🧲Header

import React, { useSyncExternalStore } from "react";
import { createRoot } from "react-dom/client";
import { HistoryTable } from "@semio-tech/vcs-react";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { createVcsDemoStore, seedVcsDemoHistory, VCS_DEMO_AUTHORS, type VcsDemoOp } from "./demo.ts";
import "./globals.css";

const store = createVcsDemoStore();
seedVcsDemoHistory(store);

function useVcsDemoStore(): ReturnType<typeof createVcsDemoStore> {
	return useSyncExternalStore(
		(listener) => store.subscribe(listener),
		() => store,
		() => store,
	);
}

function VcsPlayApp(): React.ReactElement {
	const demoStore = useVcsDemoStore();
	const projection = demoStore.projection();
	const columns = demoStore.historyColumns();
	const dispatchApply = (operations: readonly VcsDemoOp[]) => {
		demoStore.dispatch({ kind: "apply", operations });
	};
	return (
		<div className="vcs-play-root flex flex-col gap-4 p-4 min-h-screen bg-[var(--background)] text-[var(--foreground)]">
			<header className="flex flex-wrap items-center gap-2">
				<h1 className="text-lg font-semibold mr-4">VCS History</h1>
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => dispatchApply([{ op: "setCounter", counter: projection.counter + 1 }])}>
					+ Counter ({projection.counter})
				</button>
				<button
					type="button"
					className="rounded border px-2 py-1 text-xs"
					onClick={() => demoStore.dispatch({ kind: "commitCheckpoint", message: `Checkpoint @ ${projection.counter}`, authors: [VCS_DEMO_AUTHORS[0]!] })}
				>
					Commit checkpoint
				</button>
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => demoStore.dispatch({ kind: "undo" })}>
					Undo
				</button>
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => demoStore.dispatch({ kind: "redo" })}>
					Redo
				</button>
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => demoStore.dispatch({ kind: "createAlternative", name: `alt-${demoStore.getEnvelope().vcs.alternatives.length + 1}` })}>
					New alternative
				</button>
			</header>
			<section className="rounded border p-3 text-sm">
				<div>
					<strong>{projection.title}</strong> · counter {projection.counter}
				</div>
				<div className="text-[var(--muted-foreground)]">{projection.notes || "—"}</div>
			</section>
			<section className="overflow-x-auto">
				<HistoryTable columns={columns} />
			</section>
		</div>
	);
}

if (typeof document !== "undefined" && document.getElementById("root")) {
	bootstrapElementsSurfaceChromeDocument("system");
	createRoot(document.getElementById("root")!).render(<VcsPlayApp />);
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("seedVcsDemoHistory", () => {
		it("creates checkpoints and alternatives", () => {
			const local = createVcsDemoStore();
			seedVcsDemoHistory(local);
			expect(local.getEnvelope().vcs.checkpoints.length).toBeGreaterThanOrEqual(3);
			expect(local.getEnvelope().vcs.alternatives.length).toBeGreaterThanOrEqual(2);
			expect(local.historyColumns().length).toBeGreaterThanOrEqual(3);
		});
	});
}
// #endregion 🧪Tests

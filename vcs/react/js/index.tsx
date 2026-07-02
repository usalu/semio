// #region 🧲Header
/** @emoji 🗄️ `@semio-tech/vcs-react` — version control history UI (HistoryTable). */
// #endregion 🧲Header

import React, { useLayoutEffect, useMemo, useRef, useState } from "react";
import { TableAvatar, cn } from "@semio-tech/ui-react";
import type { HistoryColumn } from "@semio-tech/vcs-core";

export interface HistoryTableProps {
	readonly columns: readonly HistoryColumn[];
	readonly className?: string;
	readonly laneWidth?: number;
}

const GRAPH_GUIDE_CLASS = "bg-muted-foreground/40";
const ROW_SHELL_CLASS = "relative h-workbench min-h-workbench max-h-workbench select-none overflow-hidden min-w-0";
const LANE_PITCH = 16;
const LANE_PAD = 8;
const AUTHOR_SLOT = 40;
const GRAPH_STROKE = "color-mix(in oklab, var(--muted-foreground) 40%, transparent)";

function historyLaneCount(columns: readonly HistoryColumn[]): number {
	return Math.max(1, ...columns.map((column) => column.lane + 1));
}

function historyGraphWidth(laneCount: number, laneWidth?: number): number {
	return laneWidth ?? Math.max(56, LANE_PAD * 2 + laneCount * LANE_PITCH);
}

function historyGraphColumnWidth(laneCount: number, laneWidth?: number): number {
	return historyGraphWidth(laneCount, laneWidth) + AUTHOR_SLOT;
}

function laneX(lane: number, laneCount: number, graphWidth: number): number {
	if (laneCount <= 1) return graphWidth / 2;
	return LANE_PAD + lane * LANE_PITCH + LANE_PITCH / 2;
}

function rowIndexByCheckpointId(columns: readonly HistoryColumn[]): Map<string, number> {
	return new Map(columns.map((column, index) => [column.checkpointId, index]));
}

function rowLaneGuides(columns: readonly HistoryColumn[], laneCount: number): boolean[][] {
	const guides = Array.from({ length: columns.length }, () => Array.from({ length: laneCount }, () => false));
	const rowById = rowIndexByCheckpointId(columns);
	for (const [rowIndex, column] of columns.entries()) {
		guides[rowIndex]![column.lane] = true;
		const parentRow = column.parentCheckpointId ? rowById.get(column.parentCheckpointId) : undefined;
		if (parentRow === undefined) continue;
		const parentLane = columns[parentRow]!.lane;
		if (column.lane === parentLane) {
			for (let row = rowIndex + 1; row < parentRow; row += 1) guides[row]![column.lane] = true;
			continue;
		}
		const elbowRow = rowIndex + 1 < parentRow ? rowIndex + 1 : parentRow;
		for (let row = rowIndex + 1; row <= elbowRow; row += 1) guides[row]![column.lane] = true;
		for (let row = elbowRow; row < parentRow; row += 1) guides[row]![parentLane] = true;
	}
	return guides;
}

function HistoryGraphSvg({
	columns,
	graphWidth,
	rowHeight,
}: {
	readonly columns: readonly HistoryColumn[];
	readonly graphWidth: number;
	readonly rowHeight: number;
}): React.ReactElement {
	const laneCount = historyLaneCount(columns);
	const rowById = rowIndexByCheckpointId(columns);
	const guides = rowLaneGuides(columns, laneCount);
	const height = Math.max(columns.length, 1) * rowHeight;
	return (
		<svg width={graphWidth} height={height} className="absolute inset-0 block pointer-events-none" aria-hidden>
			{guides.map((rowGuides, rowIndex) =>
				rowGuides.map((active, lane) => {
					if (!active) return null;
					const x = laneX(lane, laneCount, graphWidth);
					const y0 = rowIndex * rowHeight;
					const y1 = y0 + rowHeight;
					return (
						<line
							key={`guide-${rowIndex}-${lane}`}
							x1={x}
							y1={y0}
							x2={x}
							y2={y1}
							stroke={GRAPH_STROKE}
							strokeWidth={1}
						/>
					);
				}),
			)}
			{columns.map((column, rowIndex) => {
				const parentRow = column.parentCheckpointId ? rowById.get(column.parentCheckpointId) : undefined;
				if (parentRow === undefined) return null;
				const x0 = laneX(column.lane, laneCount, graphWidth);
				const x1 = laneX(columns[parentRow]!.lane, laneCount, graphWidth);
				const y0 = rowIndex * rowHeight + rowHeight / 2;
				const y1 = parentRow * rowHeight + rowHeight / 2;
				if (x0 === x1) {
					return (
						<line
							key={`${column.checkpointId}-stem`}
							x1={x0}
							y1={y0}
							x2={x1}
							y2={y1}
							stroke={GRAPH_STROKE}
							strokeWidth={1.5}
						/>
					);
				}
				const elbowY = (rowIndex + 1) * rowHeight;
				return (
					<path
						key={`${column.checkpointId}-stem`}
						d={`M ${x0} ${y0} L ${x0} ${elbowY} L ${x1} ${elbowY} L ${x1} ${y1}`}
						fill="none"
						stroke={GRAPH_STROKE}
						strokeWidth={1.5}
					/>
				);
			})}
			{columns.map((column, rowIndex) => (
				<circle
					key={`${column.checkpointId}-node`}
					cx={laneX(column.lane, laneCount, graphWidth)}
					cy={rowIndex * rowHeight + rowHeight / 2}
					r={3}
					fill="var(--foreground)"
				/>
			))}
		</svg>
	);
}

function HistoryRowAuthors({ column }: { readonly column: HistoryColumn }): React.ReactElement {
	return (
		<div className="flex shrink-0 -space-x-2">
			{column.authors.length > 0 ? (
				column.authors.map((author) => (
					<TableAvatar key={author.id} id={author.id} name={author.name} icon={author.avatar} />
				))
			) : (
				<TableAvatar id="unknown" name="?" />
			)}
		</div>
	);
}

function HistoryRowLabels({ column }: { readonly column: HistoryColumn }): React.ReactElement {
	return (
		<div className="flex min-w-0 flex-wrap items-center gap-1">
			{column.labels.length > 0 ? (
				column.labels.map((label) => (
					<span key={`${column.checkpointId}-${label}`} className="rounded px-1.5 py-0.5 text-2xs bg-[var(--accent)] text-[var(--accent-foreground)]">
						{label}
					</span>
				))
			) : (
				<span className="text-2xs text-muted-foreground">checkpoint</span>
			)}
		</div>
	);
}

export const HistoryTable: React.FC<HistoryTableProps> = ({ columns, className, laneWidth }) => {
	const sorted = useMemo(() => columns, [columns]);
	const laneCount = historyLaneCount(sorted);
	const graphWidth = historyGraphWidth(laneCount, laneWidth);
	const graphColumnWidth = historyGraphColumnWidth(laneCount, laneWidth);
	const rowProbeRef = useRef<HTMLDivElement>(null);
	const [rowHeight, setRowHeight] = useState(30);
	useLayoutEffect(() => {
		const height = rowProbeRef.current?.offsetHeight;
		if (height && height > 0) setRowHeight(height);
	}, [sorted.length]);
	const graphHeight = Math.max(sorted.length, 1) * rowHeight;
	return (
		<div className={cn("text-xs", className)} data-testid="vcs-history-table">
			{sorted.length === 0 ? (
				<div className="px-single py-single text-muted-foreground">—</div>
			) : (
				<div
					className="relative grid min-w-0"
					style={{
						gridTemplateColumns: `auto ${graphColumnWidth}px minmax(0, 1fr)`,
						gridTemplateRows: `repeat(${sorted.length}, minmax(0, auto))`,
					}}
				>
					<div className="pointer-events-none relative" style={{ gridColumn: 2, gridRow: `1 / ${sorted.length + 1}`, height: graphHeight }}>
						<HistoryGraphSvg columns={sorted} graphWidth={graphWidth} rowHeight={rowHeight} />
					</div>
					{sorted.map((column, index) => {
						const nodeX = laneX(column.lane, laneCount, graphWidth);
						const authorLeft = Math.max(nodeX - 12, 0);
						return (
							<React.Fragment key={column.checkpointId}>
								<div
									ref={index === 0 ? rowProbeRef : undefined}
									className={cn(ROW_SHELL_CLASS, "grid items-center border-b border-[var(--border)]")}
									style={{ gridColumn: "1 / 3", gridRow: index + 1, gridTemplateColumns: `auto ${graphColumnWidth}px` }}
								>
									<div className="flex items-center px-single">
										<HistoryRowLabels column={column} />
									</div>
									<div className="relative">
										<div
											className={cn("pointer-events-none absolute top-1/2 h-px -translate-y-1/2", GRAPH_GUIDE_CLASS)}
											style={{ left: 0, width: authorLeft }}
											aria-hidden
										/>
										<div className="absolute top-1/2 -translate-y-1/2" style={{ left: authorLeft }}>
											<HistoryRowAuthors column={column} />
										</div>
									</div>
								</div>
								<div
									className={cn(ROW_SHELL_CLASS, "flex items-center border-b border-[var(--border)] px-single")}
									style={{ gridColumn: 3, gridRow: index + 1 }}
								>
									<span className="min-w-0 truncate text-muted-foreground" title={column.description ?? ""}>
										{column.description ?? ""}
									</span>
								</div>
							</React.Fragment>
						);
					})}
				</div>
			)}
		</div>
	);
};

HistoryTable.displayName = "HistoryTable";

export type { HistoryColumn } from "@semio-tech/vcs-core";

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	const { renderToStaticMarkup } = await import("react-dom/server");

	describe("HistoryTable", () => {
		it("renders swimlane guides and fork elbows", () => {
			const columns: HistoryColumn[] = [
				{
					checkpointId: "c3",
					timestamp: "3",
					labels: ["feature-b"],
					authors: [],
					parentCheckpointId: "c2",
					description: "branch b",
					lane: 2,
					alternativeIds: ["b"],
				},
				{
					checkpointId: "c2",
					timestamp: "2",
					labels: ["feature-a"],
					authors: [],
					parentCheckpointId: "c1",
					description: "branch a",
					lane: 1,
					alternativeIds: ["a"],
				},
				{
					checkpointId: "c1",
					timestamp: "1",
					labels: ["main"],
					authors: [],
					parentCheckpointId: undefined,
					description: "root",
					lane: 0,
					alternativeIds: [],
				},
			];
			const markup = renderToStaticMarkup(<HistoryTable columns={columns} />);
			expect(markup).toContain('data-testid="vcs-history-table"');
			expect(markup).toContain('d="M ');
			expect(markup.match(/<line /g)?.length ?? 0).toBeGreaterThanOrEqual(3);
			expect(markup.match(/<circle /g)?.length).toBe(3);
			expect(markup).toMatch(/grid-template-columns:auto \d+px minmax\(0, 1fr\)/);
			expect(markup).toContain("branch b");
			expect(markup).toContain("feature-b");
			expect(markup).toMatch(/width:\d+px/);
		});
	});
}
// #endregion 🧪Tests

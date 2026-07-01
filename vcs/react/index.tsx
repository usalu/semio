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

function laneX(lane: number, laneCount: number, laneWidth: number): number {
	const padding = 8;
	const span = Math.max(laneWidth - padding * 2, 1);
	if (laneCount <= 1) return laneWidth / 2;
	return padding + (span * lane) / (laneCount - 1);
}

function rowIndexByCheckpointId(columns: readonly HistoryColumn[]): Map<string, number> {
	return new Map(columns.map((column, index) => [column.checkpointId, index]));
}

function HistoryGraphSvg({
	columns,
	laneWidth,
	rowHeight,
}: {
	readonly columns: readonly HistoryColumn[];
	readonly laneWidth: number;
	readonly rowHeight: number;
}): React.ReactElement {
	const laneCount = Math.max(1, ...columns.map((column) => column.lane + 1));
	const rowById = rowIndexByCheckpointId(columns);
	const height = Math.max(columns.length, 1) * rowHeight;
	return (
		<svg width={laneWidth} height={height} className="absolute inset-0 block pointer-events-none" aria-hidden>
			{columns.map((column, rowIndex) => {
				const parentRow = column.parentCheckpointId ? rowById.get(column.parentCheckpointId) : undefined;
				if (parentRow === undefined) return null;
				const x0 = laneX(column.lane, laneCount, laneWidth);
				const x1 = laneX(columns[parentRow]!.lane, laneCount, laneWidth);
				const y0 = rowIndex * rowHeight + rowHeight / 2;
				const y1 = parentRow * rowHeight + rowHeight / 2;
				if (x0 === x1) {
					return <line key={`${column.checkpointId}-stem`} x1={x0} y1={y0} x2={x1} y2={y1} stroke="var(--border)" strokeWidth={1.5} />;
				}
				const midY = (y0 + y1) / 2;
				return (
					<path
						key={`${column.checkpointId}-stem`}
						d={`M ${x0} ${y0} L ${x0} ${midY} L ${x1} ${midY} L ${x1} ${y1}`}
						fill="none"
						stroke="var(--border)"
						strokeWidth={1.5}
					/>
				);
			})}
			{columns.map((column, rowIndex) => (
				<circle
					key={`${column.checkpointId}-node`}
					cx={laneX(column.lane, laneCount, laneWidth)}
					cy={rowIndex * rowHeight + rowHeight / 2}
					r={3}
					fill="var(--foreground)"
				/>
			))}
		</svg>
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

export const HistoryTable: React.FC<HistoryTableProps> = ({ columns, className, laneWidth = 48 }) => {
	const sorted = useMemo(() => columns, [columns]);
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
				<div className="relative min-w-0">
					<div className="absolute left-0 top-0" style={{ width: laneWidth, height: graphHeight }}>
						<HistoryGraphSvg columns={sorted} laneWidth={laneWidth} rowHeight={rowHeight} />
					</div>
					<div className="relative min-w-0">
						{sorted.map((column, index) => (
							<div
								key={column.checkpointId}
								ref={index === 0 ? rowProbeRef : undefined}
								className={cn(ROW_SHELL_CLASS, "grid items-center")}
								style={{ gridTemplateColumns: `${laneWidth}px minmax(0, 1fr)` }}
							>
								<div className="relative h-full" />
								<div className="flex h-full min-w-0 items-center gap-single border-b border-[var(--border)] px-single">
									<div className={cn("h-px w-3 shrink-0", GRAPH_GUIDE_CLASS)} aria-hidden />
									<div className="flex min-w-0 flex-1 items-center gap-single overflow-hidden">
										<HistoryRowLabels column={column} />
										<div className="flex shrink-0 -space-x-2">
											{column.authors.length > 0 ? (
												column.authors.map((author) => (
													<TableAvatar key={author.id} id={author.id} name={author.name} icon={author.avatar} />
												))
											) : (
												<TableAvatar id="unknown" name="?" />
											)}
										</div>
										<span className="min-w-0 flex-1 truncate text-muted-foreground" title={column.description ?? ""}>
											{column.description ?? ""}
										</span>
									</div>
								</div>
							</div>
						))}
					</div>
				</div>
			)}
		</div>
	);
};

HistoryTable.displayName = "HistoryTable";

export type { HistoryColumn } from "@semio-tech/vcs-core";

// #region 🧲Header
/** @emoji 🗄️ `@semio-tech/vcs-react` — version control history UI (HistoryTable). */
// #endregion 🧲Header

import React, { useMemo } from "react";
import { TableAvatar } from "@semio-tech/ui-react";
import type { HistoryColumn } from "@semio-tech/vcs-core";

export interface HistoryTableProps {
	readonly columns: readonly HistoryColumn[];
	readonly className?: string;
	readonly rowLabelWidth?: number;
	readonly columnWidth?: number;
	readonly rowHeight?: number;
}

const ROW_LABELS = ["labels", "parent", "description"] as const;

function parentColumnIndex(columns: readonly HistoryColumn[], parentId?: string): number {
	if (!parentId) return -1;
	return columns.findIndex((column) => column.checkpointId === parentId);
}

function HistoryLaneSvg({
	columns,
	columnWidth,
	rowHeight,
}: {
	readonly columns: readonly HistoryColumn[];
	readonly columnWidth: number;
	readonly rowHeight: number;
}): React.ReactElement {
	const width = columns.length * columnWidth;
	const height = rowHeight;
	const centerY = height / 2;
	const laneCount = Math.max(1, ...columns.map((column) => column.lane + 1));
	const laneStep = height / (laneCount + 1);
	return (
		<svg width={width} height={height} className="block" aria-hidden>
			{columns.map((column, index) => {
				const parentIndex = parentColumnIndex(columns, column.parentCheckpointId);
				if (parentIndex < 0) return null;
				const x0 = parentIndex * columnWidth + columnWidth / 2;
				const x1 = index * columnWidth + columnWidth / 2;
				const y0 = laneStep * (column.lane + 1);
				const y1 = centerY;
				const midX = (x0 + x1) / 2;
				return (
					<path
						key={`${column.checkpointId}-lane`}
						d={`M ${x0} ${y0} L ${x0} ${y1} L ${midX} ${y1} L ${x1} ${y1} L ${x1} ${centerY}`}
						fill="none"
						stroke="var(--border)"
						strokeWidth={1.5}
					/>
				);
			})}
		</svg>
	);
}

export const HistoryTable: React.FC<HistoryTableProps> = ({
	columns,
	className,
	rowLabelWidth = 72,
	columnWidth = 140,
	rowHeight = 56,
}) => {
	const sorted = useMemo(() => [...columns].sort((a, b) => a.timestamp.localeCompare(b.timestamp)), [columns]);
	const gridTemplateColumns = `${rowLabelWidth}px repeat(${Math.max(sorted.length, 1)}, ${columnWidth}px)`;
	return (
		<div className={className} data-testid="vcs-history-table">
			<div
				className="grid border border-[var(--border)] rounded-md overflow-hidden text-xs"
				style={{ gridTemplateColumns }}
			>
				{ROW_LABELS.map((rowLabel) => (
					<React.Fragment key={rowLabel}>
						<div className="px-2 py-2 font-medium text-[var(--muted-foreground)] border-b border-r border-[var(--border)] bg-[var(--muted)] capitalize">
							{rowLabel}
						</div>
						{sorted.length === 0 ? (
							<div className="px-2 py-2 border-b border-[var(--border)] text-[var(--muted-foreground)] col-span-1">—</div>
						) : (
							sorted.map((column) => {
								if (rowLabel === "labels") {
									return (
										<div
											key={`${column.checkpointId}-labels`}
											className="px-2 py-2 border-b border-r border-[var(--border)] flex flex-wrap gap-1 items-center min-h-[2.5rem]"
										>
											{column.labels.length > 0 ? (
												column.labels.map((label) => (
													<span
														key={`${column.checkpointId}-${label}`}
														className="rounded px-1.5 py-0.5 bg-[var(--accent)] text-[var(--accent-foreground)]"
													>
														{label}
													</span>
												))
											) : (
												<span className="text-[var(--muted-foreground)]">checkpoint</span>
											)}
										</div>
									);
								}
								if (rowLabel === "parent") {
									return (
										<div
											key={`${column.checkpointId}-parent`}
											className="relative px-2 py-1 border-b border-r border-[var(--border)] flex items-center justify-center min-h-[3.5rem]"
										>
											<div className="absolute inset-0 flex items-center justify-center pointer-events-none opacity-80">
												<HistoryLaneSvg columns={sorted} columnWidth={columnWidth} rowHeight={rowHeight} />
											</div>
											<div className="relative z-10 flex -space-x-2">
												{column.authors.length > 0 ? (
													column.authors.map((author) => (
														<TableAvatar key={author.id} id={author.id} name={author.name} icon={author.avatar} />
													))
												) : (
													<TableAvatar id="unknown" name="?" />
												)}
											</div>
										</div>
									);
								}
								return (
									<div
										key={`${column.checkpointId}-description`}
										className="px-2 py-2 border-b border-r border-[var(--border)] text-[var(--muted-foreground)] truncate"
										title={column.description ?? ""}
									>
										{column.description ?? ""}
									</div>
								);
							})
						)}
					</React.Fragment>
				))}
			</div>
		</div>
	);
};

HistoryTable.displayName = "HistoryTable";

export type { HistoryColumn } from "@semio-tech/vcs-core";

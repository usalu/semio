import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function TableHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.table;
	if (!scene) return <div className="semio-table-empty">No table scene</div>;
	const columns = JSON.parse(scene.columnsJson) as { id: string; label: string }[];
	const rows = JSON.parse(scene.rowsJson) as Record<string, unknown>[];
	return (
		<table className="semio-table-host" data-surface-id={node.surfaceId}>
			<thead>
				<tr>
					{columns.map((column) => (
						<th key={column.id}>{column.label}</th>
					))}
				</tr>
			</thead>
			<tbody>
				{rows.map((row, index) => (
					<tr
						key={index}
						onClick={() =>
							onCommand({
								controllerId: node.controllerId,
								command: "selectRow",
								args: { row },
							})
						}
					>
						{columns.map((column) => (
							<td key={column.id}>{String(row[column.id] ?? "")}</td>
						))}
					</tr>
				))}
			</tbody>
		</table>
	);
}

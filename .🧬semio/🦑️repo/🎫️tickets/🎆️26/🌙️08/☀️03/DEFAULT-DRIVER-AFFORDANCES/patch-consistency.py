from pathlib import Path

ui = next(Path("/Users/ueli/Documents/semio").rglob("**/🖱️ui/⚛️react/**/📦️index.tsx"))
text = ui.read_text()

# --- 1) Insert useNativeDragArm after useUiDriverDragSurface ---
marker = "export function useUiDriverDragSurface(): boolean {"
idx = text.find(marker)
assert idx != -1
brace = text.find("{", idx)
depth = 0
i = brace
while i < len(text):
    ch = text[i]
    if ch == "{":
        depth += 1
    elif ch == "}":
        depth -= 1
        if depth == 0:
            end = i + 1
            break
    i += 1

if "export function useNativeDragArm" not in text:
    helper = """

/** @emoji Arm native HTML5 `draggable` only while a drag handle is pressed. */
export function useNativeDragArm(): { readonly armed: boolean; readonly arm: () => void } {
  const [armed, setArmed] = reactHostPort.useState(false);
  const arm = reactHostPort.useCallback(() => {
    setArmed(true);
    window.addEventListener("pointerup", () => setArmed(false), { once: true });
  }, []);
  return { armed, arm };
}
"""
    # keep docstring emoji-free in patch source; add emoji via separate replace
    helper = helper.replace(
        "/** @emoji Arm native HTML5",
        "/** @emoji \U0001f91d Arm native HTML5",
    )
    text = text[:end] + helper + text[end:]
    print("inserted useNativeDragArm")
else:
    print("useNativeDragArm already present")

# --- 2) Replace panel tree unit header block with driver-aware component usage ---
old_header_start = '            {showUnitHeader ? (\n              <div\n                data-slot="panel-tree-unit-header"'
if "PanelTreeUnitHeader" not in text:
    # Insert component before PanelLeafTabTrees / the units map function
    insert_at = text.find("/** @emoji \U0001f332 Leaf-tab tree body shared by {@link Panel}")
    if insert_at == -1:
        insert_at = text.find('data-slot="panel-tree-unit-header"')
        # back up to function start
        insert_at = text.rfind("export function", 0, insert_at)
        insert_at = text.rfind("//", 0, insert_at)
    assert insert_at != -1

    component = r'''
/** @emoji Panel tree-unit dock header — handle-only under the default driver, whole-header under surface drag. */
function PanelTreeUnitHeader({
  anchor,
  tabId,
  unit,
  index,
  unitDragActive,
}: {
  readonly anchor?: Anchor;
  readonly tabId: string;
  readonly unit: PanelTreeUnit;
  readonly index: number;
  readonly unitDragActive: boolean;
}) {
  const dock = usePanelDockContext();
  const surfaceDrag = useUiDriverDragSurface();
  const { armed, arm } = useNativeDragArm();
  const unitDockDraggable = Boolean(dock && anchor);
  const effectiveDraggable = unitDockDraggable && (surfaceDrag || armed);
  const UnitIcon = unit.icon;
  return (
    <div
      data-slot="panel-tree-unit-header"
      draggable={effectiveDraggable}
      onDragStart={
        unitDockDraggable
          ? (event) => {
              event.dataTransfer.effectAllowed = "move";
              event.dataTransfer.setData(PANEL_TREE_UNIT_MIME, unit.id);
              beginPanelTreeUnitDrag({ tabId, unitId: unit.id, label: unit.label ?? tabId });
            }
          : undefined
      }
      onDragEnd={unitDockDraggable ? () => endPanelTreeUnitDrag() : undefined}
      onDragOver={
        unitDockDraggable
          ? (event) => {
              if (event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME)) event.preventDefault();
            }
          : undefined
      }
      onDrop={
        unitDockDraggable
          ? (event) => {
              if (!event.dataTransfer.types.includes(PANEL_TREE_UNIT_MIME) || !dock || !anchor) return;
              event.preventDefault();
              const session = readActivePanelTreeUnitDrag();
              if (!session) return;
              dock.onTreeUnitDockDrop({ unitId: session.unitId, fromTabId: session.tabId, target: { anchor, tabId, index } });
              endPanelTreeUnitDrag();
            }
          : undefined
      }
      className={cn(
        "flex shrink-0 items-center gap-single px-single py-half text-2xs",
        unitDragActive ? "text-emphasized" : "text-muted-foreground",
        unitDockDraggable && surfaceDrag && "cursor-grab active:cursor-grabbing",
        unitDragActive && dropZoneReadyFillClass,
      )}
    >
      {UnitIcon ? <UnitIcon size={12} /> : null}
      <span className="min-w-0 truncate">{unit.label}</span>
      {unitDockDraggable && !surfaceDrag ? (
        <DragHandle labelId="ui.tree.drag.sort" className="ms-auto" onPointerDown={arm} emphasized={unitDragActive} />
      ) : null}
    </div>
  );
}

'''
    component = component.replace(
        "/** @emoji Panel tree-unit",
        "/** @emoji \U0001f332 Panel tree-unit",
    )
    # Find a safe insertion point: just before the leaf-tab tree body function that contains the header
    needle = 'data-slot="panel-tree-unit-header"'
    header_idx = text.find(needle)
    assert header_idx != -1
    # find the enclosing function start - look for "export function" or "function " before
    fn_start = text.rfind("\nfunction ", 0, header_idx)
    if fn_start == -1 or text.rfind("\nexport function ", 0, header_idx) > fn_start:
        fn_start = text.rfind("\nexport function ", 0, header_idx)
    assert fn_start != -1
    # also check for const X = (
    alt = text.rfind("\nexport const ", 0, header_idx)
    # Prefer the closest function/const that contains showUnitHeader
    region = text[max(0, header_idx - 800):header_idx]
    print("region prelude:", region[-200:])

    text = text[:fn_start + 1] + component + text[fn_start + 1 :]
    print("inserted PanelTreeUnitHeader component")

    # Now replace the inline header JSX with the component
    # Re-find after insertion
    header_idx = text.find('data-slot="panel-tree-unit-header"')
    # Skip the one inside the component definition - find the second occurrence (usage)
    second = text.find('data-slot="panel-tree-unit-header"', header_idx + 1)
    assert second != -1, "usage site missing"
    # Expand to the showUnitHeader ternary
    usage_start = text.rfind("{showUnitHeader ? (", 0, second)
    assert usage_start != -1
    # Find matching close: </div>\n            ) : null}
    usage_end = text.find("</div>\n            ) : null}", second)
    assert usage_end != -1
    usage_end = text.find("}", usage_end) + 1
    replacement = """{showUnitHeader ? (
              <PanelTreeUnitHeader anchor={anchor} tabId={tabId} unit={unit} index={index} unitDragActive={unitDragActive} />
            ) : null}"""
    text = text[:usage_start] + replacement + text[usage_end:]
    print("replaced inline panel unit header")
else:
    print("PanelTreeUnitHeader already present")

# --- 3) TableDraggableRow: handle-only listeners ---
old_tr_listeners = "{...(canDragRow ? { ...attributes, ...listeners } : {})}"
if old_tr_listeners in text and "driverSurfaceDrag" not in text[text.find("function TableDraggableRow"): text.find("function TableDraggableRow") + 2000]:
    # Insert hook and change listeners + first cell handle
    fn_idx = text.find("function TableDraggableRow")
    assert fn_idx != -1
    insert_hook_at = text.find("const canDragRow", fn_idx)
    assert insert_hook_at != -1
    text = (
        text[:insert_hook_at]
        + "const driverSurfaceDrag = useUiDriverDragSurface();\n  "
        + text[insert_hook_at:]
    )
    text = text.replace(
        "{...(canDragRow ? { ...attributes, ...listeners } : {})}",
        "{...(canDragRow && driverSurfaceDrag ? { ...attributes, ...listeners } : canDragRow ? { ...attributes } : {})}",
        1,
    )
    # Inject sort handle into first cell of TableDraggableRow
    cell_map = text.find("{visibleColumns.map((column) => (", fn_idx)
    # There may be multiple - find within TableDraggableRow only
    fn_end_approx = text.find("\nfunction ", fn_idx + 10)
    if fn_end_approx == -1:
        fn_end_approx = text.find("\nconst Table =", fn_idx)
    cell_map = text.find("{visibleColumns.map((column) => (", fn_idx, fn_end_approx)
    assert cell_map != -1
    old_cells = """{visibleColumns.map((column) => (
        <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
          <div className="flex items-center h-full min-w-0">{column.accessor(row)}</div>
        </td>
      ))}"""
    # Get exact snippet from file
    snippet_start = cell_map
    snippet_end = text.find("))}", snippet_start) + 3
    old_snippet = text[snippet_start:snippet_end]
    print("TableDraggableRow cells snippet:\\n", old_snippet[:300])
    new_snippet = """{visibleColumns.map((column, columnIndex) => (
        <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
          <div className="flex items-center h-full min-w-0 gap-1">
            {canDragRow && !driverSurfaceDrag && columnIndex === 0 ? (
              <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(event) => event.stopPropagation()} />
            ) : null}
            {column.accessor(row)}
          </div>
        </td>
      ))}"""
    text = text[:snippet_start] + new_snippet + text[snippet_end:]
    print("updated TableDraggableRow")
else:
    print("TableDraggableRow skip or already updated", old_tr_listeners in text)

# --- 4) Extract TableHtml5DragRow for native rowDragProps ---
if "function TableHtml5DragRow" not in text:
    insert_before = text.find("function TableDraggableRow")
    assert insert_before != -1
    component = r'''
/** @emoji Native HTML5 table-row drag that honors the UI driver (handle vs surface). */
function TableHtml5DragRow<T>({
  row,
  rowId,
  index,
  isSelected,
  customRowClassName,
  isDragging,
  rowHeightClass,
  visibleColumns,
  rowDragProps,
  onRowClick,
  onRowContextMenu,
  onRowDoubleClick,
  onRowMouseEnter,
  onRowMouseLeave,
}: {
  readonly row: T;
  readonly rowId: string;
  readonly index: number;
  readonly isSelected: boolean;
  readonly customRowClassName: string;
  readonly isDragging: boolean;
  readonly rowHeightClass: string;
  readonly visibleColumns: TableColumn<T>[];
  readonly rowDragProps?: (row: T, index: number) => React.HTMLAttributes<HTMLTableRowElement>;
  readonly onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  readonly onRowContextMenu?: (row: T, index: number, event: React.MouseEvent) => void;
  readonly onRowDoubleClick?: (row: T, index: number) => void;
  readonly onRowMouseEnter?: (row: T, index: number) => void;
  readonly onRowMouseLeave?: (row: T, index: number) => void;
}) {
  const driverSurfaceDrag = useUiDriverDragSurface();
  const { armed, arm } = useNativeDragArm();
  const dragProps = rowDragProps?.(row, index) ?? {};
  const wantsDrag = Boolean(dragProps.draggable || dragProps.onDragStart);
  const effectiveDraggable = wantsDrag && (driverSurfaceDrag || armed);
  const baseRowClassName = cn(
    borderNormalBottomClass,
    rowHeightClass,
    tableRowInteractiveClass,
    isSelected && tableRowSelectedClass,
    wantsDrag && driverSurfaceDrag && "cursor-grab active:cursor-grabbing",
    isDragging && "opacity-50",
    onRowClick && "cursor-selectable",
    customRowClassName,
  );
  const { draggable: _ignoredDraggable, className: dragClassName, ...restDragProps } = dragProps;
  return (
    <tr
      className={cn(baseRowClassName, dragClassName)}
      draggable={effectiveDraggable || undefined}
      {...restDragProps}
      onClick={(e) => {
        if (e.detail >= 2) {
          onRowDoubleClick?.(row, index);
          return;
        }
        onRowClick?.(row, index, e);
      }}
      onContextMenu={(e) => {
        onRowContextMenu?.(row, index, e);
      }}
      onMouseEnter={() => onRowMouseEnter?.(row, index)}
      onMouseLeave={() => onRowMouseLeave?.(row, index)}
      role={onRowClick ? "button" : undefined}
      tabIndex={onRowClick ? 0 : undefined}
      data-row-id={rowId}
    >
      {visibleColumns.map((column, columnIndex) => (
        <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
          <div className="flex items-center h-full min-w-0 gap-1">
            {wantsDrag && !driverSurfaceDrag && columnIndex === 0 ? (
              <DragHandle labelId="ui.tree.drag.transfer" iconKind="move" onPointerDown={arm} onClick={(event) => event.stopPropagation()} />
            ) : null}
            {column.accessor(row)}
          </div>
        </td>
      ))}
    </tr>
  );
}

'''
    component = component.replace(
        "/** @emoji Native HTML5",
        "/** @emoji \U0001f5b1 Native HTML5",
    )
    text = text[:insert_before] + component + text[insert_before:]
    print("inserted TableHtml5DragRow")

    # Replace the inline tr return when !dragDrop
    old_return = """                const baseRowClassName = cn(borderNormalBottomClass, rowHeightClass, tableRowInteractiveClass, isSelected && tableRowSelectedClass);
                const isDragging = activeId === rowId;

                return (
                  <tr
                    key={key}
                    className={cn(baseRowClassName, customRowClassName, isDragging && "opacity-50", onRowClick && "cursor-selectable")}
                    onClick={(e) => {
                      if (e.detail >= 2) {
                        onRowDoubleClick?.(row, index);
                        return;
                      }
                      onRowClick?.(row, index, e);
                    }}
                    onContextMenu={(e) => {
                      onRowContextMenu?.(row, index, e);
                    }}
                    onMouseEnter={() => onRowMouseEnter?.(row, index)}
                    onMouseLeave={() => onRowMouseLeave?.(row, index)}
                    role={onRowClick ? "button" : undefined}
                    tabIndex={onRowClick ? 0 : undefined}
                    data-row-id={rowId}
                    {...rowDragProps?.(row, index)}
                  >
                    {visibleColumns.map((column) => (
                      <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
                        <div className="flex items-center h-full min-w-0">{column.accessor(row)}</div>
                      </td>
                    ))}
                  </tr>
                );"""
    new_return = """                const isDragging = activeId === rowId;

                return (
                  <TableHtml5DragRow
                    key={key}
                    row={row}
                    rowId={rowId}
                    index={index}
                    isSelected={isSelected}
                    customRowClassName={customRowClassName}
                    isDragging={isDragging}
                    rowHeightClass={rowHeightClass}
                    visibleColumns={visibleColumns}
                    rowDragProps={rowDragProps}
                    onRowClick={onRowClick}
                    onRowContextMenu={onRowContextMenu}
                    onRowDoubleClick={onRowDoubleClick}
                    onRowMouseEnter={onRowMouseEnter}
                    onRowMouseLeave={onRowMouseLeave}
                  />
                );"""
    if old_return not in text:
        raise SystemExit("Table inline row block not found for replacement")
    text = text.replace(old_return, new_return, 1)
    print("replaced Table inline row with TableHtml5DragRow")
else:
    print("TableHtml5DragRow already present")

ui.write_text(text)
print("wrote", ui)
print("size delta", len(text) - len(ui.read_text()) + len(text))  # nonsense check
print("final size", len(text))
ENDSCRIPT
#!/usr/bin/env python3
"""📊️ U5 6b — one-off: the React host's windowed Table (Interpreter `TableView`) and the window-body tree-window
channel (ShellHost), plus the host strings (en + de). Exact single-match edits only."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
INTERPRETER = ENGINE / "🗣️Interpreter/🟦️.tsx"
SHELL_HOST = ENGINE / "🏛️ShellHost/🟦️.tsx"
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
I18N = UI / "🧱️elements/📚️I18n/🟦️.tsx"
UI_REACT = UI / "🎯️targets/⚛️react/🟦️.tsx"

edits: list[tuple[pathlib.Path, str, str]] = []


def edit(path: pathlib.Path, old: str, new: str) -> None:
    edits.append((path, old, new))


edit(INTERPRETER, '''  treeWindowPathOf,
  treeWindowRequestsForViewport,
  treeWindowVisibleRowsForViewport,
  useLabel,''', '''  treeRowHeightPx,
  treeWindowDomAttributes,
  treeWindowPathOf,
  treeWindowRequestsForViewport,
  treeWindowSpacerRows,
  treeWindowVisibleRowsForViewport,
  useLabel,''')

TABLE_VIEW = '''//#region 📊️TableWindow
/** 📊️ Keys that move the active row of a windowed table, and how far. `page` is the viewport's own row
 * count, so PageDown lands on the row the reader sees at the bottom — the ARIA grid pattern's row
 * navigation over the table's WHOLE logical extent, not over the rows that happen to be materialised. */
export function tableWindowNextRowV1(key: string, current: number, total: number, page: number): number | null {
  if (total <= 0) return null;
  const last = total - 1;
  const clamp = (value: number) => Math.min(last, Math.max(0, value));
  switch (key) {
    case "ArrowDown":
      return clamp(current + 1);
    case "ArrowUp":
      return clamp(current - 1);
    case "PageDown":
      return clamp(current + Math.max(1, page));
    case "PageUp":
      return clamp(current - Math.max(1, page));
    case "Home":
      return 0;
    case "End":
      return last;
    default:
      return null;
  }
}

/** 📊️ The scroll position that brings logical row `index` fully into a viewport of `height` pixels at
 * `scrollTop`, rows pitched `rowPx` apart — unchanged when it already is. Spacers keep every row at
 * `index * rowPx` whether or not it is materialised, so this is exact for rows the host has not streamed. */
export function tableWindowScrollTopForRowV1(index: number, rowPx: number, scrollTop: number, height: number): number {
  const top = index * rowPx;
  if (top < scrollTop) return top;
  if (top + rowPx > scrollTop + height) return Math.max(0, top + rowPx - height);
  return scrollTop;
}

/** 📊️ The shared column track of a table's header and rows: the first (name) column twice as wide as the
 * others, the actions column exactly as wide as the widest materialised action strip — header and rows
 * read the SAME string, so their columns line up however the rows stream. */
function tableWindowColumnTemplate(columns: number, actions: number): string {
  const data = columns > 0 ? ["minmax(0, 2fr)", ...Array.from({ length: columns - 1 }, () => "minmax(0, 1fr)")] : [];
  return [...data, ...(actions > 0 ? [`calc(${actions} * var(--size-medium) + ${actions} * var(--spacing-single))`] : [])].join(" ");
}

/** 📊️ One `Component::Table`: a keyboard- and screen-reader-accessible grid whose rows are the host's
 * WINDOW of a logically `window.total`-long row list. It speaks the tree windows' own streaming protocol —
 * the same `data-tree-window-*` stamps, the same spacers, the same {@link useTreeWindowObserver} — so the
 * body's one scheduler asks the guest for exactly the rows the viewport shows, on the same node ledger.
 *
 * ⌨️ One tab stop (the active row). Up/Down, PageUp/PageDown and Home/End move it over the whole logical
 * extent, scrolling rows the host has not streamed yet into view and focusing them the moment they arrive;
 * Enter/Space fire the row's own activation; Right/Left walk the row's actions; Escape returns to the row. */
function TableView({ store, record, context }: { readonly store: UiDocumentStore; readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const revision = useUiDocumentRevision(store);
  const windows = useTreeWindowContext();
  const rootRef = useRef<HTMLDivElement | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const focusRowRef = useRef<number | null>(null);
  const component = record.component as Extract<Component, { type: "table" }>;
  const rows = useMemo(() => {
    void revision;
    const state = store.getState();
    return (record.children ?? []).map((id) => state.nodes.get(id)).filter((row): row is UiNodeRecord => !!row && row.component.type === "tableRow");
  }, [store, record, revision]);
  const window = component.window ?? undefined;
  const total = window ? Math.max(rows.length, Math.floor(window.total)) : rows.length;
  const { leading, trailing } = treeWindowSpacerRows(window ?? { total, offset: 0, rowExtent: "standard" }, rows.length);
  const rowPx = treeRowHeightPx;
  const actionColumns = rows.reduce((widest, row) => Math.max(widest, (row.component as Extract<Component, { type: "tableRow" }>).rowActions.length), 0);
  const hasActions = actionColumns > 0;
  const template = tableWindowColumnTemplate(component.columns.length, actionColumns);
  const [active, setActive] = useState(0);
  const activeRow = active >= leading && active < leading + rows.length ? active : leading;
  const range = useLabel("ui.host.tableRowRange", { from: rows.length > 0 ? leading + 1 : 0, to: leading + rows.length, total });
  useTreeWindowObserver(rootRef, windows, revision, store);
  useEffect(() => {
    const wanted = focusRowRef.current;
    if (wanted === null || wanted < leading || wanted >= leading + rows.length) return;
    const element = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${wanted}"]`);
    if (!element) return;
    focusRowRef.current = null;
    element.focus({ preventScroll: true });
  }, [leading, rows]);
  const moveTo = (index: number) => {
    setActive(index);
    focusRowRef.current = index;
    const scroller = scrollRef.current;
    if (scroller) scroller.scrollTop = tableWindowScrollTopForRowV1(index, rowPx, scroller.scrollTop, scroller.clientHeight);
    const element = rootRef.current?.querySelector<HTMLElement>(`[data-table-row-index="${index}"]`);
    if (element) {
      focusRowRef.current = null;
      element.focus({ preventScroll: true });
    }
  };
  const activateRow = (row: UiNodeRecord) => {
    if ((row.bindings ?? []).some((binding) => binding.trigger === "activate")) {
      void dispatchTrigger(context, row, "activate");
      return;
    }
    const first = (row.component as Extract<Component, { type: "tableRow" }>).rowActions[0];
    if (first) context.onIntent(context.store.buildIntent(row, first.action));
  };
  const onRowKeyDown = (event: import("react").KeyboardEvent<HTMLDivElement>, row: UiNodeRecord, index: number) => {
    if (event.target !== event.currentTarget) {
      const buttons = Array.from(event.currentTarget.querySelectorAll<HTMLElement>("[data-table-row-action]"));
      const position = buttons.indexOf(event.target as HTMLElement);
      if (event.key === "ArrowRight" && position >= 0 && position < buttons.length - 1) buttons[position + 1]!.focus();
      else if (event.key === "ArrowLeft" && position > 0) buttons[position - 1]!.focus();
      else if (event.key === "Escape" || (event.key === "ArrowLeft" && position === 0)) event.currentTarget.focus();
      else return;
      event.preventDefault();
      return;
    }
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activateRow(row);
      return;
    }
    if (event.key === "ArrowRight") {
      const first = event.currentTarget.querySelector<HTMLElement>("[data-table-row-action]");
      if (first) {
        event.preventDefault();
        first.focus();
      }
      return;
    }
    const page = Math.max(1, Math.floor((scrollRef.current?.clientHeight ?? rowPx) / rowPx));
    const next = tableWindowNextRowV1(event.key, index, total, page);
    if (next === null) return;
    event.preventDefault();
    moveTo(next);
  };
  const cellClass = "flex min-w-0 items-center truncate px-single";
  return (
    <div
      id={nodeDomId(store, record)}
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      role="grid"
      aria-label={component.label}
      aria-rowcount={total + 1}
      aria-colcount={component.columns.length + (hasActions ? 1 : 0)}
      aria-busy={record.activity === "loading" || record.activity === "waiting" || undefined}
      className={cn("flex min-h-0 min-w-0 flex-1 flex-col", activityBorderClass(record))}
      style={layoutSpecStyle(record.layout)}
    >
      <div role="rowgroup" className={cn("shrink-0", borderNormalTopClass)}>
        <div role="row" aria-rowindex={1} className="grid min-w-0 text-2xs font-semibold uppercase tracking-wide text-muted-foreground" style={{ gridTemplateColumns: template, height: rowPx }}>
          {component.columns.map((column, position) => (
            <div key={position} role="columnheader" aria-colindex={position + 1} className={cellClass}>
              {column}
            </div>
          ))}
          {hasActions ? (
            <div role="columnheader" aria-colindex={component.columns.length + 1} className={cellClass}>
              {component.actionsLabel ?? ""}
            </div>
          ) : null}
        </div>
      </div>
      <div ref={rootRef} className="contents">
        <div ref={scrollRef} role="rowgroup" data-slot="table-window-scroll" className="min-h-0 min-w-0 flex-1 overflow-auto">
          <div {...(treeWindowDomAttributes(window, rows.length, record.key) ?? {})} className="min-w-0">
            {leading > 0 ? <div aria-hidden="true" data-tree-window-spacer="leading" style={{ height: leading * rowPx }} /> : null}
            {rows.map((row, position) => {
              const index = leading + position;
              const props = row.component as Extract<Component, { type: "tableRow" }>;
              const name = props.cells[0] ?? row.key;
              return (
                <div
                  key={row.key}
                  id={nodeDomId(store, row)}
                  data-ui-node-id={row.id}
                  data-ui-node-key={row.key}
                  data-tree-window-row={index}
                  data-table-row-index={index}
                  role="row"
                  aria-rowindex={index + 2}
                  tabIndex={index === activeRow ? 0 : -1}
                  onFocus={(event) => {
                    if (event.target === event.currentTarget) setActive(index);
                  }}
                  onClick={(event) => {
                    if (event.target === event.currentTarget || !(event.target as HTMLElement).closest("[data-table-row-action]")) moveTo(index);
                  }}
                  onDoubleClick={() => activateRow(row)}
                  onKeyDown={(event) => onRowKeyDown(event, row, index)}
                  className="grid min-w-0 cursor-default border-b border-border/40 text-xs outline-none hover:bg-muted/40 focus-visible:bg-muted/60 focus-visible:ring-1 focus-visible:ring-primary"
                  style={{ gridTemplateColumns: template, height: rowPx }}
                >
                  {component.columns.map((column, cell) => (
                    <div key={cell} role="gridcell" aria-colindex={cell + 1} className={cellClass} title={props.cells[cell] ?? ""}>
                      {props.cells[cell] ?? ""}
                    </div>
                  ))}
                  {hasActions ? (
                    <div role="gridcell" aria-colindex={component.columns.length + 1} className="flex min-w-0 items-center gap-single px-single">
                      {props.rowActions.map((action, actionIndex) => {
                        const label = action.label ? `${wireLabel(action.label)}: ${name}` : name;
                        return (
                          <Button
                            key={actionIndex}
                            type="button"
                            variant="ghost"
                            data-table-row-action=""
                            tabIndex={-1}
                            icon={resolveControlIconNode(action.icon)}
                            aria-label={label}
                            title={label}
                            onClick={() => context.onIntent(context.store.buildIntent(row, action.action))}
                          />
                        );
                      })}
                    </div>
                  ) : null}
                </div>
              );
            })}
            {trailing > 0 ? <div aria-hidden="true" data-tree-window-spacer="trailing" style={{ height: trailing * rowPx }} /> : null}
          </div>
        </div>
      </div>
      <div role="status" aria-live="polite" className="shrink-0 px-single text-2xs text-muted-foreground">
        {range}
      </div>
    </div>
  );
}
//#endregion 📊️TableWindow

/** 🌳️ Renders one record's component, recursing into children via {@link UiNodeView} — never reads a'''
edit(INTERPRETER, '''/** 🌳️ Renders one record's component, recursing into children via {@link UiNodeView} — never reads a''', TABLE_VIEW)
edit(INTERPRETER, '''    case "extension":
      return <ExtensionView record={record} />;
    default:
      return <UnknownComponentView record={record} />;''', '''    case "extension":
      return <ExtensionView record={record} />;
    case "table":
      return <TableView store={store} record={record} context={context} />;
    default:
      return <UnknownComponentView record={record} />;''')

edit(I18N, '''    readonly host: {
      readonly emptyScene: UiLabelValue;''', '''    readonly host: {
      readonly emptyScene: UiLabelValue;
      /** ♿️ The polite status under a windowed table: which of its rows are materialised. */
      readonly tableRowRange: UiLabelValue;''')
edit(UI_REACT, '''        host: {
          emptyScene: { label: { normal: "Keine Szene", beginner: "Keine Szene" } },''', '''        host: {
          emptyScene: { label: { normal: "Keine Szene", beginner: "Keine Szene" } },
          tableRowRange: { label: { normal: "Zeilen {{from}}–{{to}} von {{total}}", beginner: "Zeilen {{from}} bis {{to}} von {{total}}" } },''')
edit(UI_REACT, '''        host: {
          emptyScene: { label: { normal: "No scene", beginner: "No scene" } },''', '''        host: {
          emptyScene: { label: { normal: "No scene", beginner: "No scene" } },
          tableRowRange: { label: { normal: "Rows {{from}}–{{to}} of {{total}}", beginner: "Rows {{from}} to {{to}} of {{total}}" } },''')

edit(SHELL_HOST, '''        void refresh(live, { kind: "partial", panelBodies: [bodyKey] })''', '''        const windowBody = live.app.windowKinds.some((kind) => kind.bodyKey === bodyKey);
        void refresh(live, windowBody ? { kind: "partial", windowBodies: [bodyKey] } : { kind: "partial", panelBodies: [bodyKey] })''')

edit(SHELL_HOST, '''  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  wireLabel,
} from "../🗣️Interpreter/🟦️.tsx";''', '''  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  TreeWindowContext,
  wireLabel,
  type TreeWindowContextValue,
} from "../🗣️Interpreter/🟦️.tsx";''')
edit(SHELL_HOST, '''  //#endregion 🪟️TreeWindows
  /** 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-A — diagnostic + defensive''', '''  /** 🪟️ The streaming channel of one WINDOW body — the same host-owned windows and the same scheduler the
   * panel bodies speak, addressed by the window kind's `bodyKey`, so a windowed table (or tree) in a window
   * streams its rows exactly like a panel tree does. One stable value per body key. */
  const windowTreeContextsRef = useRef(new Map<string, TreeWindowContextValue>());
  const windowTreeContext = useCallback(
    (bodyKey: string): TreeWindowContextValue => {
      const known = windowTreeContextsRef.current.get(bodyKey);
      if (known) return known;
      const created: TreeWindowContextValue = {
        bodyKey,
        openStates: {},
        setOpen: (nodeKey, open) => treeWindowHost.setOpen(bodyKey, nodeKey, open),
        reportWindows: (requests, viewportRows) => treeWindowHost.reportWindows(bodyKey, requests, viewportRows),
      };
      windowTreeContextsRef.current.set(bodyKey, created);
      return created;
    },
    [treeWindowHost],
  );
  //#endregion 🪟️TreeWindows
  /** 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-A — diagnostic + defensive''')
edit(SHELL_HOST, '''                <InterpretedUiNode store={browserActorStore ?? builtNodeStoreFor(`window:${kind.id}`, windowUiByWindowId[kind.id] ?? PENDING_WINDOW_UI_NODE)}''', '''                <TreeWindowContext.Provider value={windowTreeContext(kind.bodyKey)}>
                <InterpretedUiNode store={browserActorStore ?? builtNodeStoreFor(`window:${kind.id}`, windowUiByWindowId[kind.id] ?? PENDING_WINDOW_UI_NODE)}''')
edit(SHELL_HOST, '''onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, intent); }} />
              </ShellFaultBoundary>''', '''onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, intent); }} />
                </TreeWindowContext.Provider>
              </ShellFaultBoundary>''')
edit(SHELL_HOST, '''                  <InterpretedUiNode store={builtNodeStoreFor(`window:${instance.id}`, windowUiByWindowId[instance.id] ?? PENDING_WINDOW_UI_NODE)} onAction={onActionStable} onIntent={onIntentStable} />''', '''                  <TreeWindowContext.Provider value={windowTreeContext(kind.bodyKey)}>
                    <InterpretedUiNode store={builtNodeStoreFor(`window:${instance.id}`, windowUiByWindowId[instance.id] ?? PENDING_WINDOW_UI_NODE)} onAction={onActionStable} onIntent={onIntentStable} />
                  </TreeWindowContext.Provider>''')

by_path: dict[pathlib.Path, str] = {}
for path, old, new in edits:
    text = by_path.get(path) or path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        sys.exit(f"refused: {path.relative_to(ROOT)} expects one match, found {count}: {old[:100]!r}")
    by_path[path] = text.replace(old, new)
for path, text in by_path.items():
    path.write_text(text, encoding="utf-8")
    print(f"edited {path.relative_to(ROOT)}")

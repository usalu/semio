#!/usr/bin/env python3
"""🪪️ LB2 prepared patch p8 — a row's inline controls and cells get DOM ids under their row (host React renderer).

Measured (S18 served matrix, 2026-09-28 17:1x): every tree edit input rendered `id="…::framework.window.tree/edit"` and every
windowed editable table row's cells `…/cell-0`, `…/cell-1` — duplicate DOM ids (an a11y defect: `aria-labelledby`, label
`for`, focus restoration and every id-based lookup resolve the FIRST row's control). Cause is the host, not the keys: the
ui contract states a node key is "unique only among this node's own siblings (not surface-wide)" (`🧬️contract/📃️document`
+ the TS schema), and the SDK's `edit` / `cell-<col>` children are sibling-unique; `uiNodeDomId` = `surface/key` assumed
surface-wide keys. Tree ROWS already get a path-unique DOM id (their window path); their controls and a table row's cells
did not. The SDK keys (and the p3/WG11 TableRow contract: one `cell-<col>` child per materialised cell) stay unchanged.

Hunks (`📺️renderer/…/🗣️Interpreter/🟦️.tsx`, host TS — open during the freeze, lands with a rule-20 tsc + Home boot):
`UiInterpreterContext.domScope` (the DOM id of the row a subtree renders in); `uiNodeDomId(surface, key, fallback, scope?)`
namespaces a keyed node by that scope; `nodeDomId` forwards the context's scope at every call site; tree rows hand their own
DOM id to their inline controls, table rows to their cells. Laws: `🧪️tests/🪟️tree-windows` (two rows' `edit` inputs: distinct
ids, each under its row's id) and `🧪️tests/📊️table` (two editable rows: no id rendered twice in the grid; cells under their
row) — Testing Library's DOM is the oracle.

Usage: lb2-p8-row-scoped-dom-ids.py [--dry-run | --write | --revert] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
BACKUP = Path(__file__).resolve().parent / "generated" / "p8-backup"
BASE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/"
INTERPRETER = BASE + "🟦️.tsx"
TREE_TESTS = BASE + "🧪️tests/🪟️tree-windows/🟦️.tsx"
TABLE_TESTS = BASE + "🧪️tests/📊️table/🟦️.tsx"

CONTEXT_OLD = """  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion UiInterpreterContext
"""
CONTEXT_NEW = """  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  /** 🪪️ The DOM id of the row this subtree renders inside — a tree row or a table row. A node key is unique only among its
   * siblings, so a row's inline controls and cells are addressed under their row ({@link uiNodeDomId}). */
  readonly domScope?: string;
};
//#endregion UiInterpreterContext
"""
DOC_OLD = """ * one app can render the same authored key. Falls back to the volatile id only for a keyless node
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function uiNodeDomId(surface: SurfaceId, key: string, fallbackNodeId: UiNodeId): string {
  return key ? `${surface}/${key}` : `node-${fallbackNodeId}`;
}

/** 🪪️ {@link uiNodeDomId} for a record held by a live store — the surface comes off the store's own state. */
function nodeDomId(store: UiDocumentStore, record: UiNodeRecord): string {
  return uiNodeDomId(store.getState().surface, record.key, record.id);
}
"""
DOC_NEW = """ * one app can render the same authored key. A key is unique only among its siblings (the ui contract), so a node
 * rendered inside a row — a tree row's inline controls, a table row's cells — is namespaced by that row's own DOM id
 * (`scope`) instead: two rows' `edit` inputs or `cell-0` cells never share an id. Falls back to the volatile id only for
 * a keyless node (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function uiNodeDomId(surface: SurfaceId, key: string, fallbackNodeId: UiNodeId, scope?: string): string {
  return key ? `${scope ?? surface}/${key}` : `node-${fallbackNodeId}`;
}

/** 🪪️ {@link uiNodeDomId} for a record held by a live store — the surface comes off the store's own state, the row scope
 * off the rendering context. */
function nodeDomId(store: UiDocumentStore, record: UiNodeRecord, scope?: string): string {
  return uiNodeDomId(store.getState().surface, record.key, record.id, scope);
}
"""
CALL_SITES = [
    ("nodeDomId(context.store, record)", "nodeDomId(context.store, record, context.domScope)"),
    ("      <Section id={nodeDomId(store, record)}", "      <Section id={nodeDomId(store, record, context.domScope)}"),
    ("      <Field id={nodeDomId(store, record)}", "      <Field id={nodeDomId(store, record, context.domScope)}"),
    ("      id={nodeDomId(store, record)}\n", "      id={nodeDomId(store, record, context.domScope)}\n"),
]
CONTROLS_OLD = """    control: controlRecords.length > 0 && controlRecords.length !== (activatableControl ? 1 : 0) ? <>{renderTreeItemControls(store, controlRecords.filter((child) => child !== activatableControl), context)}</> : undefined,
"""
CONTROLS_NEW = """    control: controlRecords.length > 0 && controlRecords.length !== (activatableControl ? 1 : 0) ? <>{renderTreeItemControls(store, controlRecords.filter((child) => child !== activatableControl), { ...context, domScope: domId })}</> : undefined,
"""
ROW_OLD = """              const name = props.cells[0] ?? row.key;
              return (
                <div
                  key={row.key}
                  id={nodeDomId(store, row)}
"""
ROW_NEW = """              const name = props.cells[0] ?? row.key;
              const rowDomId = nodeDomId(store, row, context.domScope);
              const cellContext: UiInterpreterContext = { ...context, domScope: rowDomId };
              return (
                <div
                  key={row.key}
                  id={rowDomId}
"""
CELL_OLD = """{cellNodes[cell] ? <UiNodeView store={store} id={cellNodes[cell]!} context={context} /> : (props.cells[cell] ?? "")}"""
CELL_NEW = """{cellNodes[cell] ? <UiNodeView store={store} id={cellNodes[cell]!} context={cellContext} /> : (props.cells[cell] ?? "")}"""

TREE_LAW_ANCHOR = """    it("gives the same node key under two different parents two independent windows", async () => {
"""
TREE_LAW = """    it("gives every row's inline control its own DOM id — a node key is unique only among its siblings", () => {
      const commit = { trigger: "commit", action: { scope: "s.stdio.json@rfc8259/*#editor", name: "set-node", version: 1 }, args: { nodeId: "$", revision: "r" }, capability: null };
      const edit = (id: number, value: string) => node(id, "edit", { type: "input", kind: "text", value, commit: "blur" }, [], [commit]);
      const view = mount(
        [
          node(1, "framework.window.tree", { type: "tree", interactionDomain: "outliner.objects" }, [2]),
          node(2, "framework.window.tree-root", { type: "treeSection", label: "", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 2, offset: 0 } }, [3, 4]),
          treeItem(3, "i=0", "first: 1", {}, [5]),
          treeItem(4, "i=1", "second: 2", {}, [6]),
          edit(5, "1"),
          edit(6, "2"),
        ],
        1,
        () => {},
      );
      const inputs = Array.from(view.container.querySelectorAll<HTMLInputElement>("input"));
      expect(inputs).toHaveLength(2);
      const ids = inputs.map((input) => input.id);
      expect(new Set(ids).size).toBe(2);
      for (const id of ids) {
        expect(id.endsWith("/edit")).toBe(true);
        expect(id).not.toBe(`${SURFACE}/edit`);
      }
    });

"""
TABLE_LAW_ANCHOR = """    it("windows columns independently and preserves their logical addresses", () => {
"""
TABLE_LAW = """    it("renders no DOM id twice: two editable rows' cells live under their own row", () => {
      const two = structuredClone(snapshot);
      const editable = two.nodes.find((node: any) => node.key === "space:sp-1");
      const cells = two.nodes.filter((node: any) => (editable.children ?? []).includes(node.id));
      const offset = Math.max(...two.nodes.map((node: any) => node.id)) + 1;
      const copies = cells.map((cell: any, index: number) => ({ ...structuredClone(cell), id: offset + index + 1 }));
      two.nodes.push({ ...structuredClone(editable), id: offset, key: "space:sp-3", component: { ...structuredClone(editable.component), cells: ["Atelier Bea", "atelier"] }, children: copies.map((cell: any) => cell.id) }, ...copies);
      const table = two.nodes.find((node: any) => node.id === two.root);
      table.children = [...table.children, offset];
      const view = mount(() => {}, null, two);
      const ids = Array.from(view.container.querySelectorAll<HTMLElement>("[id]")).map((element) => element.id);
      expect(ids.filter((id, index) => ids.indexOf(id) !== index)).toEqual([]);
      const rows = Array.from(screen.getByRole("grid", { name: "Spaces" }).querySelectorAll<HTMLElement>("[role='row'][data-ui-node-key^='space:']"));
      for (const row of rows) for (const input of Array.from(row.querySelectorAll<HTMLInputElement>("input"))) expect(input.id.startsWith(`${row.id}/`)).toBe(true);
    });

"""

problems, changed = [], {}
if "--revert" in sys.argv:
    for backup in sorted(path for path in BACKUP.rglob("*") if path.is_file()):
        rel = backup.relative_to(BACKUP)
        if backup.name.endswith(".absent"):
            target = ROOT / str(rel)[: -len(".absent")]
            if target.exists():
                target.unlink()
                print("removed", target.relative_to(ROOT))
                parent = target.parent
                while parent != ROOT and not any(parent.iterdir()):
                    parent.rmdir()
                    parent = parent.parent
        else:
            (ROOT / rel).write_bytes(backup.read_bytes())
            print("restored", rel)
    sys.exit(0)


def text(rel):
    return changed[rel] if rel in changed else (ROOT / rel).read_text(encoding="utf-8")


def replace(rel, label, old, new, count=1):
    current = text(rel)
    if new in current and old not in current:
        print(f"already applied: {rel} ({label})")
        return
    found = current.count(old)
    if (count is not None and found != count) or found == 0:
        problems.append(f"{rel}: expected {count if count is not None else '>=1'}x {label}, found {found}")
        return
    changed[rel] = current.replace(old, new)


replace(INTERPRETER, "context domScope", CONTEXT_OLD, CONTEXT_NEW)
replace(INTERPRETER, "uiNodeDomId scope", DOC_OLD, DOC_NEW)
for old, new in CALL_SITES:
    replace(INTERPRETER, old.strip()[:40], old, new, None)
replace(INTERPRETER, "tree row control scope", CONTROLS_OLD, CONTROLS_NEW)
replace(INTERPRETER, "table row scope", ROW_OLD, ROW_NEW)
replace(INTERPRETER, "table cell scope", CELL_OLD, CELL_NEW)
replace(TREE_TESTS, "tree law", TREE_LAW_ANCHOR, TREE_LAW + TREE_LAW_ANCHOR)
replace(TABLE_TESTS, "table law", TABLE_LAW_ANCHOR, TABLE_LAW + TABLE_LAW_ANCHOR)
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} problems={len(problems)} write={WRITE}")
for rel in changed:
    print("  ", rel)
if WRITE and not problems:
    for rel, content in changed.items():
        target = ROOT / rel
        backup = BACKUP / (rel if target.exists() else rel + ".absent")
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(target.read_bytes() if target.exists() else b"")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)

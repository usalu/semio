#!/usr/bin/env python3
"""📊️ U5 6b — one-off: the UI contract's windowed Table (Component::Table + Component::TableRow).

A table row is ONE node record (cells and row actions are props), the header lives on the Table node, and the Table
carries the same `TreeWindow` stamp a tree section does — so tables and trees share one windowing contract, one
body-wide node ledger and one host request channel (`ViewModel::tree_windows`). Every edit is an exact, single-match
replacement; nothing is written unless every edit matches."""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
C = UI / "🧬️contract"
GENERATED = ROOT / "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts"

edits: list[tuple[pathlib.Path, str, str]] = []


def edit(path: pathlib.Path, old: str, new: str) -> None:
    edits.append((path, old, new))


TABLE_RUST = '''
/// 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
/// every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
/// row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
/// section carries: the children are the rows `[offset, offset + children.len())` of a logically
/// `total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
/// viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
/// ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TableProps {
    /// 🏷️ The table's accessible name — what assistive technology announces on entering it.
    pub label: Label,
    /// 🗂️ The column headers, in cell order.
    pub columns: crate::UiFixedList<Label>,
    /// 🎬️ Header of the trailing actions column a renderer adds when any row carries row actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub actions_label: Option<Label>,
    /// 🪟️ The materialised slice of the logical row list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}

/// 📊️ Props for `Component::TableRow` — one row of a [`TableProps`] table: its cells in column order and
/// its row-scoped actions, both as props, so a row is ONE node record. The row's primary activation (open,
/// select) is the record's own `Trigger::Activate` binding. No `ToValue`/`FromValue`: [`RowAction`] embeds
/// `UiValue`, the same deliberate exception [`TreeItemProps`] documents.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps {
    /// 📝️ The row's cells, positional to the table's `columns`.
    pub cells: crate::UiFixedList<crate::UiText>,
    /// 🎬️ Row-scoped actions, rendered in the table's trailing actions column.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
}

impl TableRowProps {
    fn credited_clone(&self) -> Option<Self> {
        let mut row_actions = crate::UiFixedList::default();
        for action in self.row_actions.iter() {
            row_actions.try_push(action.credited_clone()?).ok()?;
        }
        Some(Self { cells: self.cells.clone(), row_actions })
    }
}
'''

component = C / "🧩️component/🦀️.rs"
edit(component, '''            granularity: self.granularity.clone(),
            row_actions,
        })
    }
}
''', '''            granularity: self.granularity.clone(),
            row_actions,
        })
    }
}
''' + TABLE_RUST)
edit(component, '''    Surface(crate::SurfaceProps),
    Extension(ExtensionProps),
}''', '''    Surface(crate::SurfaceProps),
    Extension(ExtensionProps),
    Table(TableProps),
    TableRow(TableRowProps),
}''')
edit(component, '''            Self::Extension(value) => Self::Extension(ExtensionProps { extension: value.extension.clone(), props: value.props.credited_clone()? }),
        })''', '''            Self::Extension(value) => Self::Extension(ExtensionProps { extension: value.extension.clone(), props: value.props.credited_clone()? }),
            Self::Table(value) => Self::Table(value.clone()),
            Self::TableRow(value) => Self::TableRow(value.credited_clone()?),
        })''')

builder = C / "🏗️builder/🦀️.rs"
edit(builder, '''                granularity: builder.granularity,
                row_actions: builder.row_actions,
            }),
        )
    }
}
//#endregion 🌲️Tree
''', '''                granularity: builder.granularity,
                row_actions: builder.row_actions,
            }),
        )
    }
}
//#endregion 🌲️Tree

//#region 📊️Table
/// 📊️ A column-headed data table — `Component::Table`. Its rows are [`table_row`] children. Build with
/// [`table`].
pub struct TableBuilder {
    base: NodeBase,
    label: crate::Label,
    columns: crate::UiFixedList<crate::Label>,
    actions_label: Option<crate::Label>,
    window: Option<crate::TreeWindow>,
}

/// 📊️ A table named `label` whose header reads `columns`. Its accessible name defaults to `label`.
pub fn table(label: crate::Label, columns: crate::UiFixedList<crate::Label>) -> TableBuilder {
    let mut base = NodeBase::stack(crate::Axis::Vertical);
    base.accessibility.label = Some(label.clone());
    TableBuilder { base, label, columns, actions_label: None, window: None }
}

impl TableBuilder {
    /// 🎬️ Names the trailing actions column.
    pub fn actions_label(mut self, label: crate::Label) -> Self {
        self.actions_label = Some(label);
        self
    }

    /// 🪟️ Declares which slice of the logical row list the built rows actually are — see
    /// [`crate::TreeWindow`].
    pub fn window(mut self, window: crate::TreeWindow) -> Self {
        self.window = Some(window);
        self
    }
}

impl HasBase for TableBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}
impl HasChildren for TableBuilder {}
impl HasStackLayout for TableBuilder {}

impl From<TableBuilder> for BuiltNode {
    fn from(builder: TableBuilder) -> Self {
        assemble(builder.base, crate::Component::Table(crate::TableProps { label: builder.label, columns: builder.columns, actions_label: builder.actions_label, window: builder.window }))
    }
}

/// 📊️ One table row — `Component::TableRow`. Cells and row actions are props, never child records.
/// Build with [`table_row`].
pub struct TableRowBuilder {
    base: NodeBase,
    cells: crate::UiFixedList<crate::UiText>,
    row_actions: crate::UiFixedList<crate::RowAction>,
}

/// 📊️ A table row reading `cells`, positional to its table's columns.
pub fn table_row(cells: crate::UiFixedList<crate::UiText>) -> TableRowBuilder {
    TableRowBuilder { base: NodeBase::stack(crate::Axis::Horizontal), cells, row_actions: crate::UiFixedList::default() }
}

impl TableRowBuilder {
    /// 🎬️ Appends one row action.
    #[expect(clippy::result_large_err, reason = "A full row-action list returns its builder and original action owner for caller-directed retirement.")]
    pub fn try_row_action(mut self, row_action: crate::RowAction) -> Result<Self, (Self, crate::RowAction)> {
        match self.row_actions.try_push(row_action) {
            Ok(()) => Ok(self),
            Err(row_action) => Err((self, row_action)),
        }
    }
}

impl HasBase for TableRowBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<TableRowBuilder> for BuiltNode {
    fn from(builder: TableRowBuilder) -> Self {
        assemble(builder.base, crate::Component::TableRow(crate::TableRowProps { cells: builder.cells, row_actions: builder.row_actions }))
    }
}
//#endregion 📊️Table
''')

limits = C / "🛡️limits/🦀️.rs"
edit(limits, '''        Extension(props) => props.extension.len(),
        Separator(_)''', '''        Extension(props) => props.extension.len(),
        Table(props) => props.label.0.len() + props.columns.iter().map(|column| column.0.len()).sum::<usize>() + label_bytes(&props.actions_label),
        TableRow(props) => props.cells.iter().map(|cell| cell.len()).sum(),
        Separator(_)''')

a11y = C / "♿️accessibility/🦀️.rs"
edit(a11y, '''        crate::Component::Extension(_) => "region",
    }
}''', '''        crate::Component::Extension(_) => "region",
        crate::Component::Table(_) => "grid",
        crate::Component::TableRow(_) => "row",
    }
}''')
edit(a11y, '''        | crate::Component::Surface(_)
        | crate::Component::TreeItem(_) => true,''', '''        | crate::Component::Surface(_)
        | crate::Component::TreeItem(_)
        | crate::Component::TableRow(_) => true,''')
edit(a11y, '''        crate::Component::TreeItem(props) => Some(&props.label),
        _ => None,
    });''', '''        crate::Component::TreeItem(props) => Some(&props.label),
        crate::Component::Table(props) => Some(&props.label),
        _ => None,
    });''')

typed = C / "🧾️typed/🦀️.rs"
edit(typed, '''        $visitor!(ImageProps { 0 => src: UiText, 1 => alt: Option<Label> });''', '''        $visitor!(TableProps { 0 => label: Label, 1 => columns: UiFixedList<Label>, 2 => actions_label: Option<Label>, 3 => window: Option<TreeWindow> });
        $visitor!(TableRowProps { 0 => cells: UiFixedList<UiText>, 1 => row_actions: UiFixedList<RowAction> });
        $visitor!(ImageProps { 0 => src: UiText, 1 => alt: Option<Label> });''')

for path in (C / "🪞️copy/🦀️.rs", C / "⚖️compare/🦀️.rs"):
    edit(path, '''    Surface: SurfaceProps,
    Extension: ExtensionProps
});''', '''    Surface: SurfaceProps,
    Extension: ExtensionProps,
    Table: TableProps,
    TableRow: TableRowProps
});''')

retire = C / "♻️retirement/🌳️typed/🧱️component/🦀️.rs"
edit(retire, '''        SurfaceProps::DEPTH,
        ExtensionProps::DEPTH,
    ]);''', '''        SurfaceProps::DEPTH,
        ExtensionProps::DEPTH,
        TableProps::DEPTH,
        TableRowProps::DEPTH,
    ]);''')
edit(retire, '''            Self::Extension(field) => field.retire_typed(path, value, bytes),
        }
    }
}

impl UiTypedRetire for LayoutSpec {''', '''            Self::Extension(field) => field.retire_typed(path, value, bytes),
            Self::Table(field) => field.retire_typed(path, value, bytes),
            Self::TableRow(field) => field.retire_typed(path, value, bytes),
        }
    }
}

impl UiTypedRetire for LayoutSpec {''')

TABLE_TS = '''    SchemaMetadata {
        name: "TableProps",
        version: 1,
        typescript: r####"/**
 * 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
 * every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
 * row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
 * section carries: the children are the rows `[offset, offset + children.len())` of a logically
 * `total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
 * viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
 * ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.
 */
export type TableProps = {
/**
 * 🏷️ The table's accessible name — what assistive technology announces on entering it.
 */
label: Label,
/**
 * 🗂️ The column headers, in cell order.
 */
columns: Array<Label>,
/**
 * 🎬️ Header of the trailing actions column a renderer adds when any row carries row actions.
 */
actionsLabel: Label | null,
/**
 * 🪟️ The materialised slice of the logical row list — see [`TreeWindow`].
 */
window: TreeWindow | null, };"####,
    },
    SchemaMetadata {
        name: "TableRowProps",
        version: 1,
        typescript: r####"/**
 * 📊️ Props for `Component::TableRow` — one row of a [`TableProps`] table: its cells in column order and
 * its row-scoped actions, both as props, so a row is ONE node record. The row's primary activation (open,
 * select) is the record's own `Trigger::Activate` binding.
 */
export type TableRowProps = {
/**
 * 📝️ The row's cells, positional to the table's `columns`.
 */
cells: Array<string>,
/**
 * 🎬️ Row-scoped actions, rendered in the table's trailing actions column.
 */
rowActions: Array<RowAction>, };"####,
    },
'''
schema = C / "🧬️schema/🦀️.rs"
edit(schema, '''    SchemaMetadata {
        name: "TextProps",''', TABLE_TS + '''    SchemaMetadata {
        name: "TextProps",''')
UNION_OLD = '''{ "type": "surface" } & SurfaceProps | { "type": "extension" } & ExtensionProps;'''
UNION_NEW = '''{ "type": "surface" } & SurfaceProps | { "type": "extension" } & ExtensionProps | { "type": "table" } & TableProps | { "type": "tableRow" } & TableRowProps;'''
edit(schema, UNION_OLD, UNION_NEW)
typegen_test = C / "🧪️tests/🧬️typegen-export/🦀️.rs"
edit(typegen_test, "assert_eq!(schema_metadata::TYPES.len(), 84);", "assert_eq!(schema_metadata::TYPES.len(), 86);")

GEN_TS = TABLE_TS.split('typescript: r####"')[1].split('"####')[0] + "\n\n" + TABLE_TS.split('typescript: r####"')[2].split('"####')[0] + "\n\n"
text_start = '''/**
 * 📝️ Props for `Component::Text`.
 */
export type TextProps'''
edit(GENERATED, text_start, GEN_TS + text_start)
edit(GENERATED, UNION_OLD, UNION_NEW)

census = UI / "🧠️runtime/♻️reconcile/🦀️.rs"
edit(census, '''            Extension(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.extension))
                }''', '''            Table(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.label.0))
                }
                1 => {
                    self.container = 2;
                    progress(props.actions_label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0)))
                }
                2 => {
                    self.container = 3;
                    progress(self.backing::<ui_contract::Label>(props.columns.capacity()))
                }
                3 => {
                    let Some(column) = props.columns.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    self.entry += 1;
                    progress(self.inline_text(&column.0))
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            TableRow(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.backing::<ui_contract::UiText>(props.cells.capacity()))
                }
                1 => {
                    let Some(cell) = props.cells.get(self.entry) else {
                        self.container = 2;
                        self.entry = 0;
                        return progress(SurfaceSemanticUsage::default());
                    };
                    self.entry += 1;
                    progress(self.inline_text(cell))
                }
                2 => {
                    self.container = 3;
                    progress(self.backing::<ui_contract::RowAction>(props.row_actions.capacity()))
                }
                3 => {
                    let Some(action) = props.row_actions.get(self.entry) else { return SurfaceSemanticCensusStep::Complete };
                    let step = match self.data_attribute {
                        0 => progress(self.inline_text(&action.icon)),
                        1 => progress(action.label.as_ref().map_or_else(SurfaceSemanticUsage::default, |value| self.inline_text(&value.0))),
                        _ => self.binding_step(&action.action),
                    };
                    if matches!(step, SurfaceSemanticCensusStep::Complete) {
                        self.data_attribute = 0;
                        self.entry += 1;
                        return progress(SurfaceSemanticUsage::default());
                    }
                    self.data_attribute += 1;
                    step
                }
                _ => SurfaceSemanticCensusStep::Complete,
            },
            Extension(props) => match self.container {
                0 => {
                    self.container = 1;
                    progress(self.inline_text(&props.extension))
                }''')

wgpu = UI / "🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs"
edit(wgpu, '''        ui_contract::Component::Surface(props) => UiNode::ComponentScene(surface_scene_node(document, record, props, surface, controller)),''', '''        ui_contract::Component::Table(_) => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: None,
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::TableRow(props) => UiNode::Button(UiButtonNode {
            id: Some(record.key.as_str().to_string()),
            icon_id: props.row_actions.get(0).map_or(IconName::ChevronRight, |action| icon_name(&action.icon)),
            label: Label::data(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>().join(" · ")),
            action: record_action(record, ui_contract::Trigger::Activate, controller).or_else(|| props.row_actions.get(0).map(|action| row_action(action, controller).action)).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None }),
            style: None,
            presence,
            menu,
        }),
        ui_contract::Component::Surface(props) => UiNode::ComponentScene(surface_scene_node(document, record, props, surface, controller)),''')

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

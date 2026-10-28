from pathlib import Path
import re

root = Path('🧰️framework/🛍️products/📚️course')
def edit(name, transform):
    path = root / name
    path.write_text(transform(path.read_text(encoding='utf-8')), encoding='utf-8')

types = '''
/** 🧾️ Localized cell content, including meaningful empty cells. */
export type MockupText = Text;
/** 🔲️ Zero-based selected rectangle of the reference grid. */
export type MockupSelection = { readonly row: number; readonly column: number; readonly rowEnd?: number; readonly columnEnd?: number };
/** 🎯️ The highlighted Excel interface part. */
export type ExcelTarget = "cell" | "range" | "row" | "column" | "worksheet" | "workbook" | "formula" | "name-box";
/** 📗️ A workbook showing the given input of an exercise. */
export type ExcelMockup = { readonly kind: "excel"; readonly caption: Text; readonly workbook: Text; readonly sheets: readonly Text[]; readonly activeSheet: number; readonly columns: readonly string[]; readonly rows: readonly (readonly MockupText[])[]; readonly selection: MockupSelection; readonly formula: MockupText; readonly target: ExcelTarget };
/** 🔌️ A labelled node port. */
export type MockupPort = { readonly id: Slug; readonly label: Text };
/** 🧩️ A computation or parameter with identified ports. */
export type MockupNode = { readonly id: Slug; readonly label: Text; readonly inputs: readonly MockupPort[]; readonly outputs: readonly MockupPort[] };
/** 📍️ A port reference within a workspace. */
export type MockupEndpoint = { readonly node: Slug; readonly port: Slug };
/** 🧵️ A directed output-to-input connection. */
export type MockupWire = { readonly from: MockupEndpoint; readonly to: MockupEndpoint };
/** 🌿️ A tree path and its ordered reference values. */
export type MockupBranch = { readonly path: string; readonly values: readonly MockupText[] };
/** 🔦️ The highlighted Grasshopper interface part. */
export type GrasshopperTargetKind = "node" | "input" | "output" | "wire" | "tree" | "branch" | "item";
/** 🪧️ A referenced node/port or zero-based wire, branch or first-branch item. */
export type MockupTarget = { readonly kind: GrasshopperTargetKind; readonly index: number; readonly node?: Slug; readonly port?: Slug };
/** 🦗️ A bounded node graph and tree of given data. */
export type GrasshopperMockup = { readonly kind: "grasshopper"; readonly caption: Text; readonly nodes: readonly MockupNode[]; readonly wires: readonly MockupWire[]; readonly tree: readonly MockupBranch[]; readonly target: MockupTarget };
/** 🪟️ A solution-free workspace preview, independent of grading truth. */
export type Mockup = ExcelMockup | GrasshopperMockup;
'''
def ts(source):
    source = source.replace('WireVersion = 8', 'WireVersion = 9').replace('WIRE_VERSION: WireVersion = 8', 'WIRE_VERSION: WireVersion = 9')
    source = '\n'.join(line.replace('readonly icon?: Icon;', 'readonly icon?: Icon; readonly mockup?: Mockup;') if line.startswith(('export type ClassificationItem =', 'export type SortingItem =', 'export type MatchingItem =', 'export type SheetItem =')) else line for line in source.splitlines()) + '\n'
    return source.replace('//#endregion 🔖️Quiz', types + '\n//#endregion 🔖️Quiz', 1)
edit('🧬️schema/🟦️.ts', ts)
edit('🧬️schema/🦀️.rs', lambda source: source + '\n#[path = "🖥️mockups/🦀️.rs"]\nmod mockups;\npub use mockups::*;\n')
attribute = '#[serde(default, skip_serializing_if = "Option::is_none")]'
edit('🧬️schema/📚️quiz-documents/🦀️.rs', lambda source: re.sub(r'(pub struct (?:ClassificationItem|SortingItem|MatchingItem) \{)', r'\1\n    ' + attribute + '\n    pub mockup: Option<Mockup>,', source.replace('use std::collections::BTreeMap;', 'use std::collections::BTreeMap;\nuse super::Mockup;').replace('WIRE_VERSION: WireVersion = 8', 'WIRE_VERSION: WireVersion = 9')))
edit('🧬️schema/🃏️sheets-and-results/🦀️.rs', lambda source: source.replace('use std::collections::BTreeMap;', 'use std::collections::BTreeMap;\nuse super::Mockup;').replace('pub struct SheetItem {', 'pub struct SheetItem {\n    ' + attribute + '\n    pub mockup: Option<Mockup>,'))
edit('🔨️modules/🃏️sheet/🟦️.ts', lambda source: source.replace('Challenge, Icon,', 'Challenge, Icon, Mockup,').replace('readonly icon?: Icon }): SheetItem', 'readonly icon?: Icon; readonly mockup?: Mockup }): SheetItem').replace('label: item.label, ...shortOf(item)', 'label: item.label, ...(item.mockup ? { mockup: item.mockup } : {}), ...shortOf(item)'))
edit('🦀️.rs', lambda source: source.replace('    Answer, Badge,', '    Mockup, ExcelMockup, GrasshopperMockup, MockupText, MockupSelection, ExcelTarget, MockupPort, MockupNode, MockupEndpoint, MockupWire, MockupBranch, MockupTarget, GrasshopperTargetKind, Answer, Badge,'))

path = root / '🔨️modules/🃏️sheet/🦀️.rs'
source = path.read_text(encoding='utf-8')
print(source[source.index('fn present'):])

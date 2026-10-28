from pathlib import Path
import json,re,runpy
root = Path('🧰️framework/🛍️products/📚️course')
path = root / '🧬️schema/🔣️.json'
schema = json.loads(path.read_text(encoding='utf-8'))
defs = schema['$defs']
def obj(props): return {'type':'object', 'additionalProperties': False, 'required': list(props), 'properties': props}
defs['MockupWidget'] = {'oneOf': [obj({'kind': {'const': kind}}) for kind in ['parameter','component']] + [obj({'kind': {'const':'slider'}, 'min': {'type':'number'}, 'max': {'type':'number'}, 'value': {'type':'number'}}), obj({'kind':{'const':'panel'}, 'lines': {'type':'array','minItems':1,'maxItems':8,'items':{'$ref':'#/$defs/MockupText'}}}),obj({'kind':{'const':'plugin'},'label':{'$ref':'#/$defs/Text'}})]}
defs['MockupNode']['properties']['widget'] = {'$ref': '#/$defs/MockupWidget'}
path.write_text(json.dumps(schema,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
path = root/'🧬️schema/🟦️.ts'
source=path.read_text(encoding='utf-8').replace('readonly icon?: Icon };','readonly icon?: Icon; readonly mockup?: Mockup };')
source=source.replace('readonly outputs: readonly MockupPort[] };','readonly outputs: readonly MockupPort[]; readonly widget?: MockupWidget };')
source+='\n/** 🎛️ Static software-native node appearance, independent of grading truth. */\nexport type MockupWidget = { readonly kind: "parameter" | "component" } | { readonly kind: "slider"; readonly min: number; readonly max: number; readonly value: number } | { readonly kind: "panel"; readonly lines: readonly MockupText[] } | { readonly kind: "plugin"; readonly label: Text };\n'
path.write_text(source,encoding='utf-8')
path=root/'🧬️schema/🖥️mockups/🦀️.rs'
source=path.read_text(encoding='utf-8').replace('PartialEq, Eq, Serialize','PartialEq, Serialize')
source=source.replace('pub struct MockupNode { pub id: Slug, pub label: Text, pub inputs: Vec<MockupPort>, pub outputs: Vec<MockupPort> }','pub struct MockupNode {\n    pub id: Slug, pub label: Text, pub inputs: Vec<MockupPort>, pub outputs: Vec<MockupPort>,\n    #[serde(default, skip_serializing_if = "Option::is_none")]\n    pub widget: Option<MockupWidget>,\n}')
source+='''
/// 🎛️ Static native widget appearance, independent of grading truth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum MockupWidget {
    Parameter,
    Component,
    Slider { min: f64, max: f64, value: f64 },
    Panel { lines: Vec<MockupText> },
    Plugin { label: Text },
}
'''
path.write_text(source,encoding='utf-8')
path=root/'🧬️schema/🃏️sheets-and-results/🦀️.rs'
source=path.read_text(encoding='utf-8').replace('#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]\n#[serde(deny_unknown_fields)]\npub struct SheetItem','#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n#[serde(deny_unknown_fields)]\npub struct SheetItem')
path.write_text(source,encoding='utf-8')
path=root/'🦀️.rs'
source=path.read_text(encoding='utf-8').replace('    Mockup, ExcelMockup,','    MockupWidget, Mockup, ExcelMockup,')
path.write_text(source,encoding='utf-8')
path=root/'🔨️modules/✅️validation/🟦️.ts'
source=path.read_text(encoding='utf-8').replace('    uniqueIds(node.inputs, at(at(nodesPath, index), "inputs"), report);','    const widget = node.widget;\n    if (widget?.kind === "slider" && (widget.max <= widget.min || widget.value < widget.min || widget.value > widget.max)) report(at(at(nodesPath, index), "widget"), "value-invalid");\n    uniqueIds(node.inputs, at(at(nodesPath, index), "inputs"), report);')
path.write_text(source,encoding='utf-8')
path=root/'🔨️modules/✅️validation/🖥️mockups/🦀️.rs'
source=path.read_text(encoding='utf-8').replace('MockupNode, Text','MockupNode, MockupWidget, Text')
source=source.replace('                    for (side, ports)', '''                    if let Some(widget) = &node.widget {
                        let widget_path = format!("{path}/widget");
                        match widget {
                            MockupWidget::Slider { min, max, value } => {
                                if !min.is_finite() || !max.is_finite() || !value.is_finite() || max <= min || value < min || value > max { self.push(widget_path, IssueCode::ValueInvalid); }
                            }
                            MockupWidget::Panel { lines } => {
                                self.bounded(format!("{widget_path}/lines"), lines.len(), 1, 8);
                                for (index, line) in lines.iter().enumerate() { self.cell(&format!("{widget_path}/lines/{index}"), line); }
                            }
                            MockupWidget::Plugin { label } => self.text(&format!("{widget_path}/label"), label),
                            _ => {}
                        }
                    }
                    for (side, ports)''')
path.write_text(source,encoding='utf-8')
path=root/'🎯️targets/⚛️react/🔨️modules/🃏️matching/🟦️.tsx'
source=path.read_text(encoding='utf-8').replace('                      <ItemMockup item={item} locale={locale} />\n                  </li>', '                  </li>',1)
path.write_text(source,encoding='utf-8')
reference=runpy.run_path(str(root/'🧪️tests/🤝️wire-version/🐍️.py'))['reference']()
(root/'🧫️fixtures/🤝️wire-version/🔣️.json').write_text(json.dumps({'$comment':'Printed by 🧪️tests/🤝️wire-version/🐍️.py run directly — never edit by hand.',**reference},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

import importlib.util, os, sys
spec = importlib.util.spec_from_file_location("ed", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r12-w2-f3-editor-edit.py"))
ed = importlib.util.module_from_spec(spec); spec.loader.exec_module(ed)
E = ["editor"]
ed.edit(ed.rs("editor", "entities"), [
 ('#[path = "\U0001f9ec️families/\U0001f980️.rs"]\npub mod families;\n', '#[path = "\U0001f9ec️families/\U0001f980️.rs"]\npub mod families;\n\n#[path = "\U0001fa91️components/\U0001f980️.rs"]\npub mod components;\n\n#[path = "\U0001f300️mep/\U0001f980️.rs"]\npub mod mep;\n'),
 ('    kind!("space", "square-dashed", false, kind_space,', '    components::COMPONENT,\n    components::COMPONENT_OVERRIDE,\n    mep::MEP_ELEMENT,\n    kind!("space", "square-dashed", false, kind_space,'),
 ('"opening" | "wall-sweep" => (row.parent)(snapshot, id).and_then(|host| storey_of(snapshot, &host)),', '"opening" | "wall-sweep" | "component-override" => (row.parent)(snapshot, id).and_then(|host| storey_of(snapshot, &host)),'),
])
ed.edit(ed.rs("editor", "interaction"), [
 ('"opening" | "curtain-panel-override")) {', '"opening" | "curtain-panel-override" | "component-override")) {'),
 ('''                            ordered.push(node("curtain-panel-override", panel, Some(&id)));
                        }
''', '''                            ordered.push(node("curtain-panel-override", panel, Some(&id)));
                        }
                        for overridden in snapshot.component_overrides.iter().filter(|(_, row)| row.component == id).map(|(key, _)| key) {
                            ordered.push(node("component-override", overridden, Some(&id)));
                        }
'''),
])

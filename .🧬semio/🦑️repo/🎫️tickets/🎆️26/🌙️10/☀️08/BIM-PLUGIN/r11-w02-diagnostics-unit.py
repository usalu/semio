import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep('''        BimCommand::PlaceAt(place_elements::PlaceAt { ids: vec!["w-south".into()], at: "1, 2".into() }),
        BimCommand::EngagementInput(''', '''        BimCommand::PlaceAt(place_elements::PlaceAt { ids: vec!["w-south".into()], at: "1, 2".into() }),
        BimCommand::SelectFindings(select_findings::SelectFindings { ids: vec!["w-south".into(), "st-ground".into()] }),
        BimCommand::SetClassification(set_classification::SetClassification { ids: vec!["w-south".into()], system: "Uniclass".into(), code: "Ss_25".into(), title: "Walls".into() }),
        BimCommand::RemoveClassification(remove_classification::RemoveClassification { ids: vec!["w-south".into()] }),
        BimCommand::CursorLeft(cursor_keys::CursorLeft {}),
        BimCommand::CursorRight(cursor_keys::CursorRight {}),
        BimCommand::CursorUp(cursor_keys::CursorUp {}),
        BimCommand::CursorDown(cursor_keys::CursorDown {}),
        BimCommand::CursorLeftFar(cursor_keys::CursorLeftFar {}),
        BimCommand::CursorRightFar(cursor_keys::CursorRightFar {}),
        BimCommand::CursorUpFar(cursor_keys::CursorUpFar {}),
        BimCommand::CursorDownFar(cursor_keys::CursorDownFar {}),
        BimCommand::CursorPlace(cursor_keys::CursorPlace {}),
        BimCommand::EngagementInput(''')
rep('''    assert_eq!(bridge("armSlabWalls", json!({})), BimCommand::ArmSlabWalls(arm_utility::ArmSlabWalls {}));
''', '''    assert_eq!(bridge("armSlabWalls", json!({})), BimCommand::ArmSlabWalls(arm_utility::ArmSlabWalls {}));
    assert_eq!(bridge("selectFindings", json!({ "id": "w-south" })), BimCommand::SelectFindings(select_findings::SelectFindings { ids: vec!["w-south".into()] }));
    assert_eq!(
        bridge("setClassification", json!({ "ids": ["w-south"], "system": "Uniclass", "code": "Ss_25" })),
        BimCommand::SetClassification(set_classification::SetClassification { ids: vec!["w-south".into()], system: "Uniclass".into(), code: "Ss_25".into(), title: String::new() })
    );
    assert_eq!(bridge("removeClassification", json!({ "id": "w-south" })), BimCommand::RemoveClassification(remove_classification::RemoveClassification { ids: vec!["w-south".into()] }));
    assert_eq!(bridge("cursorRightFar", json!({})), BimCommand::CursorRightFar(cursor_keys::CursorRightFar {}));
''')
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)

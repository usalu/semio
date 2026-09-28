
    /// 🏠️ Row actions travel ONLY as `RowAction` props — the one representation every renderer paints in the table's
    /// trailing actions column — never as `row-action-<i>` child records, so a Home-shaped row (six cells, five labelled
    /// row actions and a row activation, each binding carrying its own argument map) stays cheap enough that the host's
    /// default window of rows is never cut by the body's item budget (ticket 26/09/23 S18: the duplicated representation
    /// capped Home at 27 rows < a 32-row viewport, leaving visible rows blank mid-scroll). Fixture and schema
    /// `🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity`; oracle: the rows read back are compared with the fixture as
    /// serde_json values.
    #[semio_framework_async_macros::async_test]
    async fn home_shaped_rows_carry_actions_as_props_and_fill_the_default_window() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity/🔣️.json")).expect("row-capacity fixture");
        let text = |value: &serde_json::Value| value.as_str().expect("fixture text").to_owned();
        let controller = text(&fixture["controller"]);
        let row = &fixture["row"];
        let cells = row["cells"].as_array().expect("cells").iter().map(text).collect::<Vec<_>>();
        let actions = row["actions"].as_array().expect("actions");
        let argument = text(&row["argument"]);
        let id_length = row["idLength"].as_u64().expect("id length") as usize;
        let columns = fixture["columns"].as_array().expect("columns").iter().map(text).collect::<Vec<_>>();
        let build = |index: &usize| -> UiAssemblyResult<BuiltNode> {
            let id = format!("{index:0id_length$}");
            let bind = |verb: &str| -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
                let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("fixture.args"))?;
                args.push(argument.clone(), UiValue::Text(UiText::try_from_str(&id).ok_or_else(|| ui_assembly_error("fixture.arg"))?)).map_err(|_| ui_assembly_error("fixture.arg"))?;
                Ok((ActionId::try_v1(&controller, verb).ok_or_else(|| ui_assembly_error("fixture.action"))?, Some(UiValue::Map(args.finish()))))
            };
            let row_actions = actions.iter().map(|action| table_row_action(action["icon"].as_str().expect("icon"), action["label"].as_str().expect("label"), bind(action["action"].as_str().expect("verb"))?)).collect::<UiAssemblyResult<Vec<_>>>()?;
            table_window_row(&format!("{}{id}", text(&row["keyPrefix"])), &cells.iter().map(String::as_str).collect::<Vec<_>>(), row_actions, Some(bind(row["activation"].as_str().expect("activation"))?))
        };
        let rows = fixture["window"]["rows"].as_u64().expect("window rows") as u32;
        assert_eq!(rows, TREE_WINDOW_DEFAULT_ROWS, "the fixture pins the host's default window");
        let windows = TreeWindows::for_body(&requested(rows), "body");
        let entries: Vec<usize> = (0..fixture["window"]["total"].as_u64().expect("total") as usize).collect();
        let node = TableWindowKit::render_rows(&windows, &text(&fixture["tableLabel"]), &columns.iter().map(String::as_str).collect::<Vec<_>>(), Some(text(&fixture["actionsLabel"]).as_str()), &entries, &build).expect("windowed table");
        assert_eq!(node.children.len(), rows as usize, "the default window of Home-shaped rows is materialised whole");
        for row_node in node.children.iter() {
            assert_eq!(row_node.children.len() as u64, fixture["expect"]["rowChildren"].as_u64().expect("row children"), "{}: cells and row actions are props, never child records", row_node.key.as_str());
            let Component::TableRow(props) = &row_node.component else { panic!("expected TableRow") };
            assert_eq!(serde_json::Value::from(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>()), row["cells"], "cells read back as the fixture's");
            let read_back = props.row_actions.iter().map(|action| serde_json::json!({ "icon": action.icon.as_str(), "label": action.label.as_ref().map(|label| label.0.as_str()), "action": action.action.action.name.as_str() })).collect::<Vec<_>>();
            assert_eq!(serde_json::Value::from(read_back), row["actions"], "every row action keeps its icon, accessible label and verb");
        }
        let row_items = semio_framework_ui_runtime::surface_subtree_items(&build(&0).expect("fixture row")).expect("bounded census");
        let capacity = TREE_WINDOW_BODY_ITEM_BUDGET / row_items;
        let minimum = fixture["expect"]["minRowsPerWindow"].as_u64().expect("minimum rows") as usize;
        assert!(capacity >= minimum, "a Home-shaped row prices {row_items} items: {capacity} rows per window < {minimum}");
    }

from pathlib import Path
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc');p=root/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-multiple-tests.rs';s=p.read_text(encoding='utf8').replace('multiple_value','scalar_controller_value').replace('multiple_required_references_use_the_original_controller_and_keep_supplied_fields_editable','scalar_type_inputs_use_the_original_controller_and_keep_empty_strings_and_signed_values_editable').replace('type-multiple-input.json','type-scalar-input.json').replace('"original multiple reference body"','"original scalar input body"').replace('multiple neutral guidance refuses','scalar neutral guidance refuses')
s=s.replace('''        let mut request = staged_request(&serde_json::json!({"path":"/entities/0","value":{"$ifcType":row["target"],"$ifcInputs":row["initial"]}}));''','''        let rows=serde_json::Value::Array(row["required"].as_array().unwrap().iter().map(|item|serde_json::json!({"index":item["index"].as_u64().unwrap().to_string(),"value":row["args"][item["index"].as_u64().unwrap()as usize]})).collect());
        let mut request = staged_request(&serde_json::json!({"path":"/entities/0","value":{"$ifcType":row["target"],"$ifcInputs":rows}}));''')
s=s.replace('''            let index = &row["required"][field];
            let raw = row["initial"].as_array().unwrap().iter().find(|item| item["index"].as_str().unwrap() == index.as_u64().unwrap().to_string());
            assert_eq!(props.value.as_ref(), raw.and_then(|item| item["value"]["value"].as_str()).unwrap_or(""));''','''            let index=row["required"][field]["index"].as_u64().unwrap()as usize;
            assert_eq!(props.value.as_ref(),row["args"][index]["value"].as_str().unwrap());''')
s=s.replace('''        "unset" => IfcValue::Unset,''','''        "unset" => IfcValue::Unset,
        "derived"=>IfcValue::Derived,
        "integer"=>IfcValue::Integer(input["value"].as_str().unwrap().parse().unwrap()),''')
p=p.with_name('guided-type-scalar-tests.rs');p.write_text(s,encoding='utf8')

from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
s=(p/'type_scalar_input_tests.rs').read_text(encoding='utf8').replace('scalar_contract','enum_contract').replace('scalar_value','enum_value').replace('scalar_source','enum_source').replace('scalar_request','enum_request').replace('type-scalar-input.json','type-enum-input.json').replace('Typed scalar drafts use the same original source, receiving and central inverse.','Declared enum choices retain full source domains, real pages and strict actual argument changes.').replace('original_scalar_type_guidance_admits_complete_exact_strings_and_signed_integers_without_defaults','original_enum_type_guidance_keeps_declared_options_bounded_and_supplied_values_editable').replace('original_scalar_type_inputs_keep_exact_complete_central_output_and_inverse','original_enum_same_type_repair_emits_only_changed_args_and_exact_central_inverse').replace('original_scalar_guidance_cancels_and_retires_every_string_copy_and_binding_prefix','original_enum_guidance_cancels_and_retires_every_option_page_and_binding_prefix')
s=s.replace('assert_eq!(output.as_ref().unwrap().controls(), row["required"].as_array().unwrap().len());','''let page = output.as_ref().unwrap();
            let required = row["required"].as_array().unwrap();
            assert!(page.controls() >= required.len() && page.controls() <= required.len() + 2);
            for (index, field) in required.iter().enumerate() {
                let control = page.control(index).unwrap();
                let semio_framework_plugin::app::staged_form::StagedScalarKind::Choice { value, choices } = control.scalar.kind else { panic!("declared choice") };
                assert!(choices.len() <= 32);
                if complete { assert_eq!(value, row["args"][field["index"].as_u64().unwrap()as usize]["value"].as_str().unwrap()); assert!(choices.iter().any(|choice|choice.key==value)); }
                for choice in choices { assert!(field["members"].as_array().unwrap().iter().any(|word|word.as_str()==Some(choice.key))); }
            }''')
s=s.replace('let mut next = snapshot.as_ref().clone();','assert!(emit.artifact_mutations.iter().all(|mutation|matches!(mutation,IfcMutation::SetEntityArg(_))));\n            let mut next = snapshot.as_ref().clone();')
a=s.index('    let request = || {',s.index('fn original_enum_guidance_cancels'))
b=s.index('    let mut settled',a)
s=s[:a]+'''    let request = || enum_request(row, true);
'''+s[b:]
s=s.replace('let row = &law["cases"][0];','let row = &law["cases"][2];')
s+='''
#[test]
fn original_enum_guidance_genuine_second_window_has_every_remaining_member_and_paid_close() {
    use semio_framework_plugin::app::staged_form::{StagedBodySource,StagedScalarKind,StagedBodyBinding};
    let law=enum_contract();
    for row in law["cases"].as_array().unwrap().iter().filter(|row|row["required"][0]["members"].as_array().unwrap().len()>32) {
        let snapshot=enum_source(&law,row["source"].as_str().unwrap());
        let mut caller=OriginalWorkFixture::new();
        let mut first=Guidance::default();let mut donor=Some(enum_request(row,false));let mut output:Option<PreparedGuidance>=None;
        assert!(caller.work_turn(grant(),1,|cx|first.receive(&mut donor,&snapshot,[0;32],cx)).unwrap());
        for turn in 0..100000 {if caller.work_turn(grant(),1,|cx|first.advance(&snapshot,[0;32],cx)).unwrap(){break;}assert!(turn<99999);}
        assert!(caller.work_turn(grant(),1,|cx|first.take(&mut output,cx)).unwrap());
        let page=output.as_ref().unwrap();let mut keys=std::collections::BTreeSet::new();
        let StagedScalarKind::Choice{choices,..}=page.control(0).unwrap().scalar.kind else{panic!("first choice")};
        for choice in choices{keys.insert(choice.key.to_owned());}
        let binding=(row["required"].as_array().unwrap().len()..page.controls()).find_map(|index|match page.control(index).unwrap().binding{Some(StagedBodyBinding::Page(binding))if binding.page.choice.as_ref().is_some_and(|choice|choice.offset=="31")=>Some(binding),_=>None}).expect("actual second page");
        let mut request=enum_request(row,false);request.page=Some(binding.destination(page.status(),&request.token).unwrap().clone());
        request.token.generation="2".into();
        let mut second=Guidance::default();let mut donor=Some(request);let mut next:Option<PreparedGuidance>=None;
        assert!(caller.work_turn(grant(),1,|cx|second.receive(&mut donor,&snapshot,[0;32],cx)).unwrap());
        for turn in 0..100000 {if caller.work_turn(grant(),1,|cx|second.advance(&snapshot,[0;32],cx)).unwrap(){break;}assert!(turn<99999);}
        assert!(caller.work_turn(grant(),1,|cx|second.take(&mut next,cx)).unwrap());
        let StagedScalarKind::Choice{choices,..}=next.as_ref().unwrap().control(0).unwrap().scalar.kind else{panic!("second choice")};
        for choice in choices{keys.insert(choice.key.to_owned());}
        assert_eq!(keys.len(),row["required"][0]["members"].as_array().unwrap().len());
        retire_guidance(&mut caller,&mut next);retire_guidance(&mut caller,&mut Some(second));retire_guidance(&mut caller,&mut output);retire_guidance(&mut caller,&mut Some(first));caller.finish();
    }
}
'''
(p/'type_enum_input_tests.rs').write_text(s,encoding='utf8')

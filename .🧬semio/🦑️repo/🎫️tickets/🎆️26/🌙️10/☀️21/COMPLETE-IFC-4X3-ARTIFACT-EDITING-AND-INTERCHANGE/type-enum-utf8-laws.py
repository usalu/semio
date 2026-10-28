from pathlib import Path
import json
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
f=p/'🧪️tests/type-enum-input.json';d=json.loads(f.read_text(encoding='utf8'));d['draftSibling']={'source':'logical','index':4,'missing':3,'text':'a'*63+'🧭'*20+'ß'};f.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
f=p/'🧪️tests/type-enum-input.schema.json';s=json.loads(f.read_text(encoding='utf8'));s['required'].append('draftSibling');s['properties']['draftSibling']={'type':'object','additionalProperties':False,'required':['source','index','missing','text'],'properties':{'source':{'const':'logical'},'index':{'const':4},'missing':{'const':3},'text':{'type':'string','minLength':64}}};f.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
f=p/'type_enum_input.test.ts';s=f.read_text(encoding='utf8');s+='''
test('inserting an earlier declared choice preserves raw UTF8 enum sibling draft exactly',async()=>{const law=contract.draftSibling,snapshot=source('logical'),before=exact(snapshot),current=request('IFCBSPLINECURVE');(current.args.value as any).$ifcInputs=[{index:String(law.index),value:{kind:'enum',value:law.text}}];const form=await typeInputForm(snapshot,'source-revision',current,control());expect(form.response.ready).toBe(false);const args=applyStagedFormBinding(current.args,stagedFormStatus(form.response),form.bindings[0]!,'U'),rows=(args.value as any).$ifcInputs;expect(rows.map((row:any)=>row.index)).toEqual([String(law.missing),String(law.index)]);expect(rows[1].value).toEqual({kind:'enum',value:law.text});const next=await typeInputForm(snapshot,'source-revision',advance(current,{args}),control());expect(next.response.ready).toBe(false);expect(exact(snapshot)).toBe(before);});
''';f.write_text(s,encoding='utf8')
f=p/'type_enum_input_tests.rs';s=f.read_text(encoding='utf8');s+='''
#[test]
fn original_enum_guidance_retains_multibyte_raw_sibling_when_an_earlier_choice_builds_sorted_rows(){
    use semio_framework_plugin::app::staged_form::{StagedBodySource,StagedBodyBinding};
    let law=enum_contract();let row=&law["cases"][1];let snapshot=enum_source(&law,"logical");let mut request=enum_request(row,false);
    *enum_field_mut(enum_field_mut(&mut request.args,"value"),"$ifcInputs")=raw(&serde_json::json!([{"index":"4","value":{"kind":"enum","value":law["draftSibling"]["text"]}}]));
    let mut caller=OriginalWorkFixture::new();let mut guide=Guidance::default();let mut donor=Some(request);let mut output:Option<PreparedGuidance>=None;
    assert!(caller.work_turn(grant(),1,|cx|guide.receive(&mut donor,&snapshot,[0;32],cx)).unwrap());
    for turn in 0..100000{if caller.work_turn(grant(),1,|cx|guide.advance(&snapshot,[0;32],cx)).unwrap(){break;}assert!(turn<99999);}
    assert!(caller.work_turn(grant(),1,|cx|guide.take(&mut output,cx)).unwrap());let page=output.as_ref().unwrap();assert!(!page.status().ready);
    let Some(StagedBodyBinding::Action(binding))=page.control(0).unwrap().binding else{panic!("typed field binding")};let semio_framework_plugin::action_staging::StagedFormBindingEdit::Fixed{edit:semio_framework_plugin::action_staging::StagedFormEdit::Set{value:DslValue::Array(rows),..}}=&binding.edits[0]else{panic!("sorted rows")};
    assert_eq!(rows[0].get("index").unwrap().as_str(),Some("3"));assert_eq!(rows[1].get("index").unwrap().as_str(),Some("4"));assert_eq!(rows[1].get("value").unwrap().get("value").unwrap().as_str(),law["draftSibling"]["text"].as_str());
    caller.cancel().cancel_now();retire_guidance(&mut caller,&mut output);retire_guidance(&mut caller,&mut Some(guide));caller.finish();
}
''';f.write_text(s,encoding='utf8')
f=p/'type_input_choices_tests.rs';s=f.read_text(encoding='utf8').replace('{RetainedCloneClose, RetainedCloneStep}','RetainedCloneClose').replace('    let _ = std::mem::size_of::<RetainedCloneStep>();\n','');f.write_text(s,encoding='utf8')

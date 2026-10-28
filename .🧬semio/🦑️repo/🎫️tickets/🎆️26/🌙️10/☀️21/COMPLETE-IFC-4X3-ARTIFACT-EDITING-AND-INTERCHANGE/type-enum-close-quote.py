from pathlib import Path
import json,hashlib
root=Path('C:/git/semio');ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE';p=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details';before=json.loads((ticket/'🗑️generated/type-enum-source.json').read_text(encoding='utf8'));(ticket/'🗑️generated/type-enum-source-before-close-quote.json').write_text(json.dumps(before,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
f=p/'type_input_choices.rs';s=f.read_text(encoding='utf8');s=s.replace('copy_bytes: size_of::<StagedScalarChoice>(),', 'copy_bytes: 2 * size_of::<Self>() + size_of::<StagedScalarChoice>() + size_of::<usize>(),').replace('copy_bytes: size_of::<Vec<StagedScalarChoice>>(),','copy_bytes: 2 * size_of::<Self>() + size_of::<Option<Vec<StagedScalarChoice>>>() + size_of::<Vec<StagedScalarChoice>>(),');f.write_text(s,encoding='utf8')
f=p/'type_input_choices_tests.rs';s=f.read_text(encoding='utf8');a='''    assert_eq!(owner.as_ref().unwrap().rows().len(), 2);''';b=a+'''
    let quoted=owner.as_ref().unwrap().next_demand(4096).unwrap();
    assert!(quoted.copy_bytes>=2*size_of::<Choices>()+size_of::<StagedScalarChoice>()+size_of::<usize>());
    let mut denied=stock;denied.maximum_copy_bytes=quoted.copy_bytes-1;
    let before=(owner.as_ref().unwrap().rows().as_ptr(),owner.as_ref().unwrap().rows().len());
    assert!(matches!(owner.as_mut().unwrap().close_step(denied),RetirementStep::BudgetExhausted));
    assert_eq!(before,(owner.as_ref().unwrap().rows().as_ptr(),owner.as_ref().unwrap().rows().len()));
''';assert a in s;s=s.replace(a,b);f.write_text(s,encoding='utf8')
print('[DEBUG] Choice close quotes now include descriptor/Option/length envelopes; matching debit remains derived from next_demand.')

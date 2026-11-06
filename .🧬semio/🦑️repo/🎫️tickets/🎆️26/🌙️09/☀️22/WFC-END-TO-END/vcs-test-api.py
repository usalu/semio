from pathlib import Path

vcs = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs')
changes = {
 '🗃️history-ledger/replacements-tests.rs': [('owner.step(grant(d)).unwrap()', 'owner.step(grant(d)).result.unwrap()')],
 '🔗️read/🧪️tests/🦀️.rs': [('root.project(index+1,|ops|&ops[index],funded).unwrap()', 'root.project(index+1,|ops|&ops[index],funded).result.unwrap()')],
 '🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/tests.rs': [('owner.begin_granted(&mut input,denied).unwrap()', 'owner.begin_granted(&mut input,denied).result.unwrap()'), ('owner.begin_granted(&mut input,grant).unwrap()', 'owner.begin_granted(&mut input,grant).result.unwrap()'), ('owner.step_granted(grant).unwrap()', 'owner.step_granted(grant).result.unwrap()')],
 '🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/commit-facts-tests.rs': [('close.begin_granted(&mut original,grant(d)).unwrap()', 'close.begin_granted(&mut original,grant(d)).result.unwrap()'), ('close.step_granted(grant(d)).unwrap()', 'close.step_granted(grant(d)).result.unwrap()')],
}
for name, replacements in changes.items():
 path = vcs / name
 text = path.read_text()
 for old, new in replacements:
  assert old in text, (name, old)
  text = text.replace(old, new)
 path.write_text(text)
path = vcs / '🔗️read/🧪️tests/🦀️.rs'
text = path.read_text()
start = text.index('  let bytes:Vec<u8>=case["bytes"]', text.index('fn original_projected_source_requires_retirement_only_from_its_actual_authority'))
end = text.index('   let mut finished=false;', start)
replacement = '''  let bytes:Vec<u8>=case["bytes"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap() as u8).collect();let pointer=bytes.as_ptr();let authority=OriginalProjectedAuthority{original:OpaqueProjectedBytes{bytes}};
  let mut admission=semio_framework_value::retained_clone::RetainedCloneSourceAdmission::new(authority,(),|original:&OriginalProjectedAuthority|&original.original);let mut output=None;
  for _ in 0..100 {if admission.phase()=="ready"{break}let demand=admission.next_demand().unwrap();let funded=grant(demand);let mut denied=funded;denied.maximum_items=0;let(turn,allocated,released)=observed(||admission.advance(denied));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(admission.original_authority().unwrap().original.bytes.as_ptr(),pointer);let(turn,allocated,released)=observed(||admission.advance(funded));receipt(turn.result.unwrap().progress(),funded,allocated,released);}
  assert_eq!(admission.phase(),"ready");let funded=grant(RetirementDemand{depth:1,..Default::default()});let(turn,allocated,released)=observed(||admission.take_output(&mut output,funded));receipt(turn.result.unwrap().progress(),funded,allocated,released);let mut source=output.unwrap();assert_eq!(source.borrow().get().bytes.as_ptr(),pointer);assert_eq!(serde_json::to_value(&source.borrow().get().bytes).unwrap(),case["expected"]);
'''
text = text[:start] + replacement + text[end:]
text = text.replace(' use semio_framework_value::retained_clone::RetainedCloneSource;\n', '')
path.write_text(text)

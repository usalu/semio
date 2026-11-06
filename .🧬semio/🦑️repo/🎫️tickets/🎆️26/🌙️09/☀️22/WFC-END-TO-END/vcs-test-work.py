from pathlib import Path

vcs = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs')
read = vcs / '🔗️read/🧪️tests/🦀️.rs'
text = read.read_text().replace('root.project(index+1,|ops|&ops[index],funded).result.unwrap()', 'witness(root.project(index+1,|ops|&ops[index],funded)).unwrap()')
text = text.replace('let mut admission=semio_framework_value::retained_clone::RetainedCloneSourceAdmission::new', 'let mut admission_work=0;let mut admission=semio_framework_value::retained_clone::RetainedCloneSourceAdmission::new')
text = text.replace('assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(admission.original_authority()', 'assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(admission.original_authority()')
text = text.replace('observed(||admission.advance(funded));receipt(turn.result.unwrap().progress()', 'observed(||admission.advance(funded));admission_work+=turn.work;receipt(turn.result.unwrap().progress()')
text = text.replace('observed(||admission.take_output(&mut output,funded));receipt(turn.result.unwrap().progress()', 'observed(||admission.take_output(&mut output,funded));admission_work+=turn.work;assert!(admission_work>0);receipt(turn.result.unwrap().progress()')
read.write_text(text)
for name in ['🗃️history-ledger/replacements-tests.rs','🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/tests.rs','🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/commit-facts-tests.rs']:
 path = vcs / name
 text = path.read_text()
 import re
 text = re.sub(r'(owner\.step\(grant\(d\)\)|owner\.begin_granted\(&mut input,(?:denied|grant)\)|owner\.step_granted\(grant\)|close\.begin_granted\(&mut original,grant\(d\)\)|close\.step_granted\(grant\(d\)\))\.result\.unwrap\(\)', r'witness(\1).unwrap()', text)
 anchor = 'use super::*;'
 helper = '''use super::*;
fn witness<T>(turn:semio_framework_value::RetainedWorkOutcome<T>)->Result<T,semio_framework_value::ValueError>{TEST_WORK.fetch_add(turn.work,std::sync::atomic::Ordering::Relaxed);turn.result}
static TEST_WORK:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);'''
 assert anchor in text
 text = text.replace(anchor, helper, 1)
 path.write_text(text)

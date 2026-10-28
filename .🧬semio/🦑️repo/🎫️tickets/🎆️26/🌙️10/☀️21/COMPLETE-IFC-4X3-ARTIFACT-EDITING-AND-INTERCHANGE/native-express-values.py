from pathlib import Path
base=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation')
for file in ['evaluator.rs','population.rs']:
 p=base/file;s=p.read_text(encoding='utf-8').replace('use std::rc::Rc;','use std::sync::Arc;').replace('Rc<','Arc<').replace('Rc::','Arc::');p.write_text(s,encoding='utf-8',newline='\n')
p=base/'evaluator.rs';s=p.read_text(encoding='utf-8').replace('use std::cmp::Ordering;','use std::cmp::Ordering;\nuse std::borrow::Cow;').replace('pub fn scalar_text(&self) -> Option<String> {',"pub fn scalar_text(&self) -> Option<Cow<'_,str>> {")
s=s.replace('Some(value.to_string()),','Some(Cow::Owned(value.to_string())),').replace('Some(value.clone()),','Some(Cow::Borrowed(value.as_str())),')
p.write_text(s,encoding='utf-8',newline='\n')
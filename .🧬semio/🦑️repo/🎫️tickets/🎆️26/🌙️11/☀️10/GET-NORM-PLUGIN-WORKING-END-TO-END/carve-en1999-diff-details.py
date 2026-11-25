from pathlib import Path
import json
import re

family=Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999')
diff=family/'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff'
source=diff/'🦀️.rs'
text=source.read_text()
assert 'semio_s_artifact_norm_en1999_diff_rows_details' not in text
start=text.index('/// 🩹️ Sparse patch of the `vertices` row')
end=text.index('    /// 🔺️ Keyed sparse diff',start)
chunk=text[start:end]
keys=('SectionVertexKey','FatigueCategoryKey','PatchLoadKey','RingStiffenerKey','TransverseStiffenerKey')
key_parts=[]
for key in keys:
    pattern=r'/// 🔑️ Key extractor[^\n]*\n#\[derive\(Clone, Copy, Debug, PartialEq\)\]\npub struct '+key+r';\n\nimpl protocol::list_delta::KeyOf<[^\n]+> for '+key+r' \{\n.*?\n\}\n'
    matches=list(re.finditer(pattern,text,re.S))
    assert len(matches)==1,key
    key_parts.append(matches[0].group())
    text=text.replace(matches[0].group(),'')
assert text.count(chunk)==1
text=text.replace(chunk,'pub use semio_s_artifact_norm_en1999_diff_rows_details::*;\n\n')
out=diff/'🧱️rows/🔩️details/🦀️.rs'
assert not out.exists()
schema_path=diff/'🔣️.json'
schema_original=schema_path.read_text()
schema=json.loads(schema_original)
names=('En1999OutlineVerticesPatch','En1999FatigueCategoriesPatch','En1999PatchLoadsPatch','En1999RingStiffenersPatch','En1999TransverseStiffenersPatch')
lines=schema_original.splitlines(keepends=True)
for name in names:
    assert schema['$defs'][name]['x-semio-rust']['derive']
    indexes=[i for i,line in enumerate(lines) if line.lstrip().startswith(json.dumps(name)+':')]
    assert len(indexes)==1
    i=indexes[0]
    marker='"x-semio-rust":{"doc":'
    pos=lines[i].rfind(marker)
    assert pos>=0
    lines[i]=lines[i][:pos]+lines[i][pos:].replace(marker,'"x-semio-rust":{"file":"🧱️rows/🔩️details/🦀️.rs","doc":',1)
schema_updated=''.join(lines)
assert json.loads(schema_updated)
out.parent.mkdir(parents=True,exist_ok=True)
header='//! 🧱️ EN 1999 detail row diffs and keyed list deltas.\n\n#![allow(async_fn_in_trait)]\n\nextern crate semio_os_kernel as dsl;\nextern crate semio_os_kernel as protocol;\nextern crate semio_os_kernel as store;\nextern crate semio_framework_value_derive as value_derive;\n\n'
out.write_text(header+'\n'.join(key_parts)+'\n'+chunk)
assert source.read_text()!=text and schema_path.read_text()==schema_original
source.write_text(text)
schema_path.write_text(schema_updated)
print('moved bytes',len(chunk.encode())+sum(len(k.encode()) for k in key_parts))

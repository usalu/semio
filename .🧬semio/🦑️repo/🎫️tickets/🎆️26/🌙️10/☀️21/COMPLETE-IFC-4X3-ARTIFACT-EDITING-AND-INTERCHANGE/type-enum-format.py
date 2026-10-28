from pathlib import Path
import subprocess
root=Path('C:/git/semio');detail=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details';editor=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
files=[detail/name for name in ['constituent_roster.rs','domain_work.rs','type_input_kind.rs','type_input.rs','type-guidance.rs','type_input_choices.rs','type_input_choices_tests.rs','type_enum_input_tests.rs']]+[editor/'guided-type-enum-tests.rs']
for f in files:
 result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(f)],capture_output=True,encoding='utf8',check=True)
 content=result.stdout;content=content[content.index('//!'):].encode('utf8');old=f.stat().st_size
 with f.open('r+b')as target:target.write(content);target.write(b' '*max(0,old-len(content)))
print('[DEBUG] Formatted',len(files),'owned Rust files; parser only, no type or runtime evidence.')

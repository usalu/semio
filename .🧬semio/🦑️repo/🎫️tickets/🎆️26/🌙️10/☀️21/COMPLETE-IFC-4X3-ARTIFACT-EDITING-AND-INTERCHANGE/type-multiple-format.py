from pathlib import Path
import subprocess
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc')
for p in [root/'🔎️details/type-guidance.rs',root/'🔎️details/type-guidance-tests.rs',root/'🔎️details/type_multiple_input_tests.rs',root/'🔎️details/type_input_tests.rs',root/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-multiple-tests.rs']:
 data=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(p)],check=True,capture_output=True).stdout
 data=data[data.index(b'//!'):];size=p.stat().st_size
 with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,size-len(data)))

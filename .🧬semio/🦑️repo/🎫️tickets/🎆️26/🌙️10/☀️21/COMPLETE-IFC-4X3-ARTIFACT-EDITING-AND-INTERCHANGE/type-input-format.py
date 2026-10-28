from pathlib import Path
import subprocess
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
for name in ['type_input.rs','type-guidance.rs','type_input_tests.rs','type_remap.rs','constituent_roster.rs']:
    path=home/name
    data=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],check=True,capture_output=True).stdout
    data=data[data.index(b'//!'):]
    oldsize=path.stat().st_size
    with path.open('r+b')as output:
        output.write(data)
        if len(data)<oldsize:output.write(b' '*(oldsize-len(data)))

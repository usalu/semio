from pathlib import Path
import subprocess
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
for name in ['constituent_roster.rs','primary_remap_tests.rs']:
    path=home/name
    result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],check=True,capture_output=True).stdout
    result=result[result.index(b'//!'):]
    old_size=path.stat().st_size
    with path.open('r+b') as output:
        output.write(result)
        if len(result)<old_size:
            output.write(b' '*(old_size-len(result)))

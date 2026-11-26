"""📊 Connect the BIM schedule exporter to the RFC 4180 snapshot codec it uses."""
from pathlib import Path
import os,re
root=Path('✏️s/🔌️plugins/🏙️bim')
stdio=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv')
codec=next(root.rglob('📊️csv/🧱️codec/🦀️.rs'))
old='semio-s-artifact-stdio-csv'
new=old+'-snapshot'
snapshot=next(stdio.rglob('🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml')).parent
owner='semio-s-artifact-bim-model-examples'
def write(p,old,new):
 if old!=new:
  assert p.read_text()==old,p
  p.write_text(new)
s=codec.read_text()
write(codec,s,s.replace('semio_s_artifact_stdio_csv::','semio_s_artifact_stdio_csv_snapshot::').replace('csv_snapshot::{CsvField','csv_snapshot::standards::v_rfc4180::subsets::any::schema::snapshot::{CsvField'))
count=0
for p in root.rglob('Cargo.toml'):
 s=p.read_text()
 line=old+' = { workspace = true }\n'
 if line not in s:continue
 spec=new+' = { path = "'+Path(os.path.relpath(snapshot,p.parent)).as_posix()+'" }\n' if f'name = "{owner}"' in s else ''
 write(p,s,s.replace(line,spec))
 count+=1
print(count, 'manifests')
for p in [Path('🌎️hub/Cargo.lock'),root/'🏭️bridge/Cargo.lock',root/'🗿️artifacts/🏢️model/Cargo.lock',Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.lock')]:
 s=p.read_text();blocks=s.split('[[package]]')
 for i,b in enumerate(blocks):
  m=re.search('^name = "([^"]+)"',b,re.M)
  if m and m[1].startswith('semio-s-artifact-bim-model'):
   blocks[i]=b.replace(f' "{old}",\n',f' "{new}",\n' if m[1]==owner else '')
 write(p,s,'[[package]]'.join(blocks))

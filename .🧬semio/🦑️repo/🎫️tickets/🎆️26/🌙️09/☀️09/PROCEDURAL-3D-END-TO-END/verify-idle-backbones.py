"""🔍 Compile the actual instance registry and compare its sparse document iteration with SQLite."""
from pathlib import Path
import json,subprocess,sqlite3
base=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor')
data=(base/'🏃️runtime/📥️staging/♻️reactor-lifecycle/🗃️instance-registry/🦀️.rs').read_text()
source='\n'.join(s for s in data.splitlines() if 'const PLUGIN_RUNTIME_INSTANCE_' in s)
source+='\n'+data[data.index('pub struct RuntimeInstanceRegistry<T>'):data.index('pub struct RuntimeDocumentBackboneBindingV1')]
source+='''
fn main() {
    let mut registry = RuntimeInstanceRegistry::<u32>::new();
    for argument in std::env::args().skip(1) {
        let id: u32 = argument.parse().unwrap();
        registry.insert_admitted(id,id);
    }
    println!("{:?}",registry.entries().map(|(id, value)| {assert_eq!(id,*value);id}).collect::<Vec<_>>());
}
'''
out=Path(__file__).parent/'🗑️generated/bridge-current/idle-registry-iteration'
subprocess.run(['rustc','--edition=2021','-o',out,'-'],input=source,text=True,check=True)
fixture=json.loads((base/'🧪️tests/🧫️fixtures/🔣️.json').read_text())
db=sqlite3.connect(':memory:')
for case in fixture['idlePickerScans']:
    ids=case['instances']; expected=[r[0] for r in db.execute('SELECT value FROM json_each(?) ORDER BY value % 1024',[json.dumps(ids)])]
    result=json.loads(subprocess.check_output([out,*map(str,ids)]))
    assert result==expected==case['visits'],(result,expected)
    print('[DEBUG]',ids,result)

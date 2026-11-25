from pathlib import Path
import json
import re
import runpy

ticket=Path(__file__).parent
audit=runpy.run_path(str(ticket/'source-boundary-audit.py'))
family=Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999')
diff=family/'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff'
parent=diff/'🦀️.rs'
child=diff/'🧱️rows/🔩️details/🦀️.rs'
schema=json.loads((diff/'🔣️.json').read_text())
names=('OutlineVertices','FatigueCategories','PatchLoads','RingStiffeners','SheetProfiles','FireInsulations','TransverseStiffeners')
for name in names:
    token='En1999'+name+'Patch'
    assert re.search(r'pub struct '+token+r'\b',child.read_text())
    assert not re.search(r'pub struct '+token+r'\b',parent.read_text())
    if schema['$defs'][token].get('x-semio-rust',{}).get('derive'):
        assert schema['$defs'][token]['x-semio-rust']['file']=='🧱️rows/🔩️details/🦀️.rs'
workspace=(family/'Cargo.toml').read_text()
router=(family/'📦️packages/🦀️rust/📜️script.ts').read_text()
manifest=(child.parent/'📦️packages/🦀️rust/Cargo.toml').read_text()
assert 'semio-s-artifact-norm-en1999-diff-rows-details' in workspace and 'semio-s-artifact-norm-en1999-diff-rows-details' in router
assert 'semio-framework-dsl-record = ' in manifest and 'test-support' in manifest
for name,entry in [('diff',parent),('detail rows',child)]:
    files,size,issues=audit['closure'](str(entry),audit['unit_cfg']('macos',[],False),{})
    print(name,files,size,'issues',issues)
    assert not issues and files<=32 and size<=262144

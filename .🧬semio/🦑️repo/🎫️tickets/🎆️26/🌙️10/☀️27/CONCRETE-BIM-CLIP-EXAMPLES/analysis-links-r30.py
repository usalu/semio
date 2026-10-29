"""🔄 Derive all analysis snapshots against one explicitly captured source payload."""
import hashlib
import importlib
import json
import time
from pathlib import Path

prefix="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
importlib.reload(importlib.import_module(prefix+"🧱️model.🧮️analysis.🐍️"))
api=importlib.import_module(prefix+"🧱️model.🐍️")
importlib.reload(api)
api._source.cache_clear()
path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json')
raw=path.read_bytes();source=json.loads(raw);start=time.perf_counter()
for key in [link['id'] for link in source['links']]:source=api.resolve_link(source,key)
issues=api.consistency_issues(source)
assert not issues,issues
result={'source_hash':hashlib.sha256(raw).hexdigest(),'links':source['links'],'issues':issues,'seconds':time.perf_counter()-start}
Path(__file__).parent.joinpath('🗑️generated/analysis-links-r30.json').write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print(json.dumps({'source_hash':result['source_hash'],'links':[link['id'] for link in result['links']],'issues':issues,'seconds':result['seconds']}))

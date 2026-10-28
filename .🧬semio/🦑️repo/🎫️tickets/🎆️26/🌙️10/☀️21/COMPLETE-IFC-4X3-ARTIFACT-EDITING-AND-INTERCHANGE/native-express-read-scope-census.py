"""🔏 Record exact read-only native retained validation source observations."""
from pathlib import Path
import hashlib,json
p=Path(__file__).parent;repo=next(a for a in p.parents if (a/'AGENTS.md').exists());v=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation';names=['machine.rs','machine-statement.rs','machine-population.rs','machine-rules.rs','machine-domain.rs','machine-expression.rs','machine-operation.rs','retained.rs','receive.rs','population.rs'];paths=[v/n for n in names]+[v.parent/'📚️catalog/IFC4X3_ADD2.exp',v/'🧪️vector-arithmetic/isolation.json'];rows=[{'path':str(f),'bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in paths];(p/'🗑️generated/native-express-read-scope-source.json').write_text(json.dumps({'scope':'read-only census, not execution/type/funding proof','nativeExecuted':False,'paths':rows},indent=2),encoding='utf8');print('[DEBUG]',len(rows))


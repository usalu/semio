"""🔐 Freeze classification inputs and unchanged semantic production observations."""
import hashlib
import json
from pathlib import Path

ticket = Path(__file__).resolve().parent
repo = next(p for p in ticket.parents if (p / 'AGENTS.md').exists())
base = repo / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
validation = base / '🧬️schema/⚖️validation'
owned = sorted((validation / '🧪️classification').iterdir()) + [ticket / name for name in [
    'issued-express-classification-plan.md', 'issued-express-classification-author.py',
    'issued-express-classification-oracle.py', 'issued-express-classification-runtime.ts',
    'issued-express-classification-implementation.md', 'issued-express-classification-freeze.py']]
refs = [validation / name for name in [
    'index.ts', 'ast.ts', 'compiler.ts', 'parser.ts', 'evaluator.ts', 'population.ts',
    'read-scope.ts', 'control.ts', 'admission.ts', 'type-matching.ts',
    '🧪️numeric/fixture.ts', '🧪️coverage/cases.json', '🧪️numeric/cases.json',
    '🧪️vector/cases.json', '🧪️placement/cases.json', '🧪️aggregate/cases.json',
    '🧪️vector-arithmetic/cases.json', '🧪️projection/cases.json', '🧪️surface-curve/cases.json']]
refs += [base / name for name in [
    '🧬️schema/📚️catalog/IFC4X3_ADD2.exp', '🧬️schema/📚️catalog/source.json',
    '🧬️schema/📚️catalog/🟦️.ts', '🚪️io/step.ts', '🚪️io/control.ts',
    '🧬️schema/🔺️diff/🟦️.ts', '🧬️schema/📸️snapshot/🟦️.ts', '🧬️schema/🔣️.json']]

def observe(path):
    return dict(path=path.as_posix(), sha256=hashlib.sha256(path.read_bytes()).hexdigest().upper(),
                bytes=path.stat().st_size, utf16=len(str(path).encode('utf-16-le')) // 2)

rows = list(map(observe, owned))
references = list(map(observe, refs))
assert max(row['utf16'] for row in rows + references) <= 256
oracle = json.loads((ticket / '🗑️generated/issued-express-classification-oracle.json').read_text(encoding='utf-8'))
assert oracle['casesSha256'] == hashlib.sha256((validation / '🧪️classification/cases.json').read_bytes()).hexdigest()
result = dict(owned=rows, references=references, productionChanged=False,
              registrationCandidate=(validation / '🧪️classification/🧪️.test.ts').as_posix(),
              cases=154, tests=162, assertions=497, nativeExecuted=False, nativeTypechecked=False,
              semanticPublicationEnabled=False, capabilityEnabled=False)
(ticket / '🗑️generated/issued-express-classification-source.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
print('[DEBUG] ' + json.dumps(dict(owned=len(rows), references=len(references), maxUtf16=max(row['utf16'] for row in rows + references))))

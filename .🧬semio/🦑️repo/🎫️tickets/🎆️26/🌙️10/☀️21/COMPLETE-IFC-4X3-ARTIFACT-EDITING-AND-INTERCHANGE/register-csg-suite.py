"""🧱️ Register one independently audited suite while preserving all concurrent registry text."""
from pathlib import Path

ticket = Path(__file__).parent
path = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts'
text = path.read_text(encoding='utf-8')
entry = '  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/csg-block.test.ts",'
anchor = '  "🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/polygonal-neutral.test.ts",'
assert text.count(entry) <= 1
if entry not in text:
    assert text.count(anchor) == 1
    assert path.read_text(encoding='utf-8') == text
    path.write_text(text.replace(anchor, anchor + '\n' + entry, 1), encoding='utf-8', newline='\n')
assert path.read_text(encoding='utf-8').count(entry) == 1
print('[DEBUG] CSG suite registered exactly once in actual package script')

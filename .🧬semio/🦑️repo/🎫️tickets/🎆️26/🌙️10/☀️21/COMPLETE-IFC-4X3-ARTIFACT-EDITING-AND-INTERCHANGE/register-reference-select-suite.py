"""🧑‍🤝‍🧑️ Register one audited source-bound input suite without altering unrelated registry text."""
from pathlib import Path

ticket = Path(__file__).parent
path = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript/📜️script.ts'
text = path.read_text(encoding='utf-8')
entry = '  "🔎️details/type_reference_select_input.test.ts",'
anchor = '  "🔎️details/type_select_input.test.ts",'
assert text.count(entry) <= 1
if entry not in text:
    assert text.count(anchor) == 1
    assert path.read_text(encoding='utf-8') == text
    path.write_text(text.replace(anchor, anchor + '\n' + entry, 1), encoding='utf-8', newline='\n')
assert path.read_text(encoding='utf-8').count(entry) == 1
print('[DEBUG] Reference SELECT suite registered exactly once after127 reader terminal')

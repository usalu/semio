from pathlib import Path

source = Path(__file__).parents[7] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts'
text = source.read_text(encoding='utf-8')
replacements = {
    'new VertexIndex(weld.vertices,control)': 'drainSolidSteps(VertexIndex.buildSteps(weld.vertices),control)',
    'index.candidates(u,v)': 'drainSolidSteps(index.candidateSteps(u,v),control)',
    'index.candidates(weld.vertices[a],weld.vertices[b])': 'drainSolidSteps(index.candidateSteps(weld.vertices[a],weld.vertices[b]),control)',
}
for old, new in replacements.items():
    assert text.count(old) == 1, (old, text.count(old))
    text = text.replace(old, new)
source.write_text(text, encoding='utf-8', newline='\n')

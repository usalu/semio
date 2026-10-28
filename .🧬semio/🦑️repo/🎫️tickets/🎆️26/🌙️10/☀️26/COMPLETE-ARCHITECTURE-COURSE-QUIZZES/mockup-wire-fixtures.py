from pathlib import Path
root=Path('🧰️framework/🛍️products/🖥️server/🧪️tests')
path=root/'🧫️fixtures/🔌️wire/🔣️.json'
source=path.read_text(encoding='utf-8')
start=source.index('"name": "course-root-query-envelope"')
source=source[:start]+source[start:].replace('"version": 8','"version": 9',1)
path.write_text(source,encoding='utf-8')
path=root/'🔬️wire/🟦️.ts'
source=path.read_text(encoding='utf-8').replace('version: { const: 8 }','version: { const: 9 }')
path.write_text(source,encoding='utf-8')
path=Path('🧰️framework/🛍️products/📚️course/README.md')
source=path.read_text(encoding='utf-8').replace('\n# Workspace Previews\n','\n## Workspace Previews\n')
path.write_text(source,encoding='utf-8')

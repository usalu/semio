from pathlib import Path
path = Path('🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css')
source = path.read_text(encoding='utf-8').replace('/* 🖥️ ', '/* 🖥 ')
source = source.replace('var(--border)', 'var(--border-normal-color)').replace('var(--background)', 'var(--base)').replace('color-mix(in srgb, var(--primary) 18%, var(--base))', 'var(--active-base)')
source += '\n.course-graph-node h5 { height: 26px; box-sizing: border-box; line-height: 20px; padding: 3px 8px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }\n.course-graph-ports ul { padding-block: 2px; }\n.course-graph-ports li { box-sizing: border-box; line-height: 24px; }\n'
path.write_text(source, encoding='utf-8')

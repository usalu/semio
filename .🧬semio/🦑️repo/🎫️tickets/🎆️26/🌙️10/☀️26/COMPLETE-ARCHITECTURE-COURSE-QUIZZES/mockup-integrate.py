from pathlib import Path
import re, os
root = Path('🧰️framework/🛍️products/📚️course')
react = root / '🎯️targets/⚛️react'
def edit(path, transform):
    path.write_text(transform(path.read_text(encoding='utf-8')), encoding='utf-8')
for folder in ['🗂️classification', '↕️sorting', '🃏️matching']:
    path = react / '🔨️modules' / folder / '🟦️.tsx'
    source = path.read_text(encoding='utf-8')
    source = source.replace('import { localized', 'import { ItemMockup } from "../🖥️mockup/🟦️.tsx";\nimport { localized', 1)
    if folder == '🗂️classification':
        source = source.replace('        </select>\n      </li>', '        </select>\n        <ItemMockup item={item} locale={locale} />\n      </li>')
    elif folder == '↕️sorting':
        source = source.replace('            </li>', '              <ItemMockup item={task.items.find((item) => item.id === id)!} locale={locale} />\n            </li>')
    else:
        source = source.replace('                  </li>', '                    <ItemMockup item={item} locale={locale} />\n                  </li>')
    path.write_text(source, encoding='utf-8')
edit(react / '🎨️.css', lambda source: source + '@import "./🎨️styles/🖥️mockups/🎨️.css";\n')
edit(react / '🎨️styles/🟦️.ts', lambda source: source.replace('export const quizStylesheet', 'import s11 from "./🖥️mockups/🎨️.css?raw";\n\nexport const quizStylesheet').replace('s9, s10]', 's9, s10, s11]'))
edit(root / '🔨️modules/✅️validation/🖥️mockups/🦀️.rs', lambda source: source.replace('ExcelTarget, ', '').replace('                let _ = ExcelTarget::Cell;\n', ''))
skip = {'node_modules', 'target', 'dist', '.git', '.cache', '🗑️generated'}
changed = []
for parent, dirs, names in os.walk(root):
    dirs[:] = [name for name in dirs if name not in skip]
    for name in names:
        if not name.endswith('.rs'): continue
        path = Path(parent) / name
        source = path.read_text(encoding='utf-8')
        positions = []
        for match in re.finditer(r'\b(ClassificationItem|SortingItem|MatchingItem|SheetItem)\s*\{', source):
            if source[max(0, match.start()-8):match.start()].rstrip().endswith(('struct', '->')): continue
            start = match.end()
            depth = 1
            end = start
            while end < len(source) and depth:
                depth += (source[end] == '{') - (source[end] == '}')
                end += 1
            body = source[start:end-1]
            if not re.search(r'\bid\s*:', body) or 'mockup:' in body or re.search(r'\.\.[A-Za-z_]', body): continue
            positions.append(start)
        for position in reversed(positions): source = source[:position] + ' mockup: None,' + source[position:]
        if positions:
            path.write_text(source, encoding='utf-8')
            changed.append(str(path))
print('\n'.join(changed))

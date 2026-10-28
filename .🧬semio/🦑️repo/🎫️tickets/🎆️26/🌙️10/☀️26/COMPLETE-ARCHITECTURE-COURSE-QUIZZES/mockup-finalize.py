from pathlib import Path
import runpy, json
root = Path('🧰️framework/🛍️products/📚️course')
ticket = Path(__file__).parent
path = root / '🔨️modules/✅️validation/🟦️.ts'
source = path.read_text(encoding='utf-8')
source = source[:source.index('/** 🖥️ Strict, bounded workspace data')]
source += (ticket / 'mockup-semantic.ts.txt').read_text(encoding='utf-8')
source = source.replace('type Answer, type Command,', 'type Mockup, type Answer, type Command,')
source = source.replace('const presentationCheck =', 'const mockupCheck = contractCheck(CONTRACT_DEFINITIONS.Mockup!);\nconst presentationCheck =')
path.write_text(source, encoding='utf-8')
reference = runpy.run_path(str(root / '🧪️tests/🤝️wire-version/🐍️.py'))['reference']()
(root / '🧫️fixtures/🤝️wire-version/🔣️.json').write_text(json.dumps({'$comment': 'Printed by 🧪️tests/🤝️wire-version/🐍️.py run directly — never edit by hand.', **reference}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

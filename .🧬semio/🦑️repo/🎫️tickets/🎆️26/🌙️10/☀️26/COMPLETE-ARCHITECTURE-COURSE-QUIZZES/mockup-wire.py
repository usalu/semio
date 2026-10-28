from pathlib import Path
root = Path('🧰️framework/🛍️products/📚️course')
ticket = Path(__file__).parent
path = root / '🔨️modules/✅️validation/🟦️.ts'
source = path.read_text(encoding='utf-8')
source = source.replace('function itemHead(json: Json, path: string, report: Report): void {', 'function itemHead(json: Json, path: string, report: Report): void {\n  if (Object.hasOwn(json, "mockup")) mockup(json.mockup, at(path, "mockup"), report);')
source = source.replace('["short", "icon", "explanation"]', '["short", "icon", "explanation", "mockup"]').replace('["short", "icon", "familiar", "explanation"]', '["short", "icon", "familiar", "explanation", "mockup"]')
source += '\n' + (ticket / 'mockup-validation.ts.txt').read_text(encoding='utf-8')
path.write_text(source, encoding='utf-8')
path = root / '🔨️modules/🃏️sheet/🦀️.rs'
source = path.read_text(encoding='utf-8').replace('Challenge, Icon,', 'Challenge, Icon, Mockup,')
source = source.replace('task.items[index].icon.as_ref()))', 'task.items[index].icon.as_ref(), task.items[index].mockup.as_ref()))')
source = source.replace('icon: Option<&Icon>) -> SheetItem', 'icon: Option<&Icon>, mockup: Option<&Mockup>) -> SheetItem').replace('SheetItem { id: id.to_string()', 'SheetItem { mockup: mockup.cloned(), id: id.to_string()')
path.write_text(source, encoding='utf-8')

from pathlib import Path
import runpy
import re

ticket = Path(__file__).parent
audit = runpy.run_path(str(ticket / 'source-boundary-audit.py'))
family = Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999')
scene = family / '🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene'
main = scene / '🦀️.rs'
wrapper = scene / '🧪️visual-laws/🦀️.rs'
visual = scene / '🧪️tests/🎨️visual-pass/🦀️.rs'
manifest = scene / '🧪️visual-laws/📦️packages/🦀️rust/Cargo.toml'
workspace = (family / 'Cargo.toml').read_text()
router = (family / '📦️packages/🦀️rust/📜️script.ts').read_text()
assert '🎨️visual-pass' not in main.read_text()
assert (wrapper.parent / '../🧪️tests/🎨️visual-pass/🦀️.rs').resolve() == visual.resolve()
assert '🧪️visual-laws/📦️packages/🦀️rust' in workspace and 'semio-s-artifact-norm-en1999-scene-visual-laws' in router
assert (manifest.parent / '../../🦀️.rs').resolve() == wrapper.resolve()
assert (manifest.parent / '../../../../🧬️schema/📸️snapshot/🔣️.json').is_file()
assert 'facts, kit, layout' in wrapper.read_text()
assert 'semio-s-artifact-norm-en1999-mutations = { workspace = true }' in manifest.read_text()
assert 'layout::Layout' in visual.read_text() and 'semio_s_artifact_norm_en1999_mutations::EDIT_RULES' in visual.read_text()
print('visual tests:', len(re.findall(r'^#\[test\]', visual.read_text(), re.M)))
for name, entry in [('scene', main), ('visual-laws', wrapper)]:
    files, size, issues = audit['closure'](str(entry), audit['unit_cfg']('macos', [], True), {})
    print(name, files, size, 'issues', issues)
    assert not issues and files <= 32 and size <= 262144

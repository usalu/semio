import os
base = '/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results'
base = base.replace('/c/', 'C:/', 1)
cfg = base + '/🎚️config/📦️packages/🦀️rust'
t = 'C:/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️testing/📦️packages/🦀️rust'
rel = os.path.relpath(t, cfg).replace(chr(92), '/')
print(rel)
p = cfg + '/Cargo.toml'
lines = open(p, encoding='utf8').read().split('\n')
lines = [l for l in lines if 'clone-source-testing' not in l]
s = '\n'.join(lines).rstrip('\n') + '\nsemio-framework-value-clone-source-testing = { path = "' + rel + '" }\n'
open(p, 'w', encoding='utf8', newline='').write(s)
p = base + '/🫧️transient/🧪️tests/🔬️unit/🦀️.rs'
s = open(p, encoding='utf8').read()
old = 'let poll = semio_s_fem_receiving::testkit::granted(close.retained_grant(), |effect| publication.close_step(4096, effect));\n            let semio_framework_value::RetainedWorkPoll::Complete(turn) = poll else { panic!("a clock publication close turn stayed pending") };\n'
assert old in s
s = s.replace(old, 'semio_framework_value_clone_source_testing::drive::warm();\n            let turn = semio_framework_value_clone_source_testing::drive::drive(close.retained_grant(), 1 << 20, |effect| publication.close_step(4096, effect));\n')
open(p, 'w', encoding='utf8', newline='').write(s)

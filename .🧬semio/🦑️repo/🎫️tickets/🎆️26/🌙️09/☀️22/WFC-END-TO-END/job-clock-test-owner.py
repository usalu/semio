from pathlib import Path

job = Path('🧰️framework/🔨️modules/🧵️job')
law = job / '🧪️tests/🔬️clock-stride/🦀️.rs'
text = law.read_text()
assert 'use semio_framework_job::*;' in text
law.write_text(text.replace('use semio_framework_job::*;', 'use semio_framework_job_clock::ClockStride;', 1))
manifest = job / '📦️packages/🦀️rust/Cargo.toml'
text = manifest.read_text()
entry = '[[test]]\nname = "original-clock-stride"\npath = "../../🧪️tests/🔬️clock-stride/🦀️.rs"\n\n'
assert entry in text
manifest.write_text(text.replace(entry, '', 1))
clock = job / '🕰️clock-identity-and-budget/📦️packages/🦀️rust/Cargo.toml'
text = clock.read_text()
text += '\n[dev-dependencies]\nserde_json = { workspace = true }\n\n[[test]]\nname = "original-clock-stride"\npath = "../../../🧪️tests/🔬️clock-stride/🦀️.rs"\n'
clock.write_text(text)
workspace = Path('Cargo.toml')
text = workspace.read_text()
entry = '    "' + str(clock.parent) + '",' 
assert entry not in text
text = text.replace('members = [\n', 'members = [\n' + entry + '\n', 1)
workspace.write_text(text)

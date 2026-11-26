"""🔎 Point a standalone Cargo test manifest at the original reactor methods and native repository packages."""
from pathlib import Path
import re
root=Path.cwd(); ticket=Path(__file__).parent.resolve(); mirror=root/'.cache/g3d-native/s'
src=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/📦️packages/🦀️rust')
out=ticket/'🗑️generated/bridge-current/idle-registry-source'
out.mkdir(exist_ok=True)
manifest=(root/src/'Cargo.toml').read_text()
deps=manifest[manifest.index('[dependencies]'):]
deps=re.sub(r'path = "([^"]+)"',lambda m:'path = "'+str((mirror/src/m[1]).resolve())+'"',deps)
(out/'Cargo.toml').write_text('[workspace]\n[package]\nname="semio-plugin-reactor-surfaces-law"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="'+str((root/src/'../../🦀️.rs').resolve())+'"\n'+deps)
(out/'Cargo.lock').write_bytes((root/'Cargo.lock').read_bytes())

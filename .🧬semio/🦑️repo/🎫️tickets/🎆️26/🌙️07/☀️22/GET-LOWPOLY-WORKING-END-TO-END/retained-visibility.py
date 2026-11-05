from pathlib import Path

p = Path('✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor')
r = p / '🧵️retained-commands/🦀️.rs'
s = r.read_text(encoding='utf-8')
s = s.replace('enum LowpolyCommandDisposition', 'pub(super) enum LowpolyCommandDisposition')
u = p / '🧪️tests/🔬️unit/🦀️.rs'
t = u.read_text(encoding='utf-8')
start = t.index('fn retained_operation()')
end = t.index('//#region 🔖️CommandSurface')
retained = t[start:end].replace('authoring_seed: "authoring-seed-test".into()', 'authoring_seed: "authoring-seed-test".into(), retained: semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance')
fstart = t.index('pub(super) fn every_command()')
fend = t.index('//#endregion 🔖️CommandSurface', fstart)
fixture = t[fstart:fend]
(p / '🧪️tests/🧰️support/commands.rs').write_text('//! 🧪️ Command specimens shared by public surface and retained implementation tests.\n\nuse super::*;\n\n' + fixture, encoding='utf-8')
grantstart = t.index('/// 🎟️ Unbounded')
grantend = t.index('extern crate', grantstart)
grant = t[grantstart:grantend]
t = t[:start] + t[end:fstart] + t[fend:]
t = t[:grantstart] + t[grantend:]
t = t.replace('use semio_s_artifact_lowpoly_lowpoly_editor::*;', '''use semio_s_artifact_lowpoly_lowpoly_editor::*;
#[path = "../🧰️support/commands.rs"]
mod command_fixture;
use command_fixture::every_command;
use semio_s_artifact_lowpoly_lowpoly_modes::editor::lowpoly::modes::edit;
use semio_s_artifact_lowpoly_lowpoly_panels::{document::LOWPOLY_PLAY_BODY_ARTIFACT, catalogue::LOWPOLY_PLAY_BODY_CATALOGUE, inspection::LOWPOLY_PLAY_BODY_INSPECTION, layers::LOWPOLY_PLAY_BODY_LAYERS};''')
u.write_text(t, encoding='utf-8')
rt = p / '🧵️retained-commands/🧪️tests/🔬️unit/🦀️.rs'
rt.parent.mkdir(parents=True, exist_ok=True)
rt.write_text('''//! 🧵️ Private retained command partition, replay, freshness, close and turn laws.

use super::*;
use semio_framework_3d::mesh::MeshModelingOps;
#[path = "../../../../🧪️tests/🧰️support/commands.rs"]
mod command_fixture;
use command_fixture::every_command;

''' + grant + retained, encoding='utf-8')
r.write_text(s + '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n', encoding='utf-8')
print('Moved three retained tests; shared command specimens; parent-only disposition visibility.')

#!/usr/bin/env python3
"""bd10: restructures the raster editor root into session / modes / panels / commands crate roots plus a slim facade root (writes with --write)."""
import os, re, sys
R = '/Users/ueli/Documents/semio/'
E = R + '✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
write = '--write' in sys.argv
root = open(E + '🦀️.rs', encoding='utf-8').read()

def one(pattern, text, flags=0):
    m = re.search(pattern, text, flags)
    assert m, pattern
    return m

def drop(text, pattern, flags=0):
    m = one(pattern, text, flags)
    return text[:m.start()] + text[m.end():], m.group(0)

EXTERN = ('extern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\n'
          'extern crate semio_framework_os_kernel as store;\nextern crate semio_framework_schema as schema;\n')
ALLOW = '#![allow(clippy::result_large_err)]\n#![allow(unexpected_cfgs)]\n'
SESSION = 'semio_s_artifact_raster_raster_editor_session'
MODES = 'semio_s_artifact_raster_raster_editor_modes'
PANELS = 'semio_s_artifact_raster_raster_editor_panels'
COMMANDS = 'semio_s_artifact_raster_raster_editor_commands'

facade = root
# commands region and inline module
facade, commands_region = drop(facade, r'//#region 🔖️Commands\n.*?//#endregion 🔖️Commands\n', re.S)
facade, commands_mod = drop(facade, r'#\[path = "\."\]\npub mod commands \{\n(?:.*\n)*?\}\n')
facade, modes_mod = drop(facade, r'#\[path = "\."\]\npub mod modes \{\n(?:.*\n)*?\}\n')
facade, panels_mod = drop(facade, r'#\[path = "\."\]\npub mod panels \{\n(?:.*\n)*?\}\n')
for name, pattern in [
    ('config', r'#\[path = "🎚️config/🦀️\.rs"\]\npub mod config;\n'),
    ('presence', r'#\[path = "👥️presence/🦀️\.rs"\]\npub mod presence;\n'),
    ('selection', r'#\[path = "🎯️selection/🦀️\.rs"\]\npub mod selection;\n'),
    ('terminology', r'#\[path = "🗣️terminology/🦀️\.rs"\]\npub mod terminology;\n'),
    ('layer_selection', r'#\[path="🖱️selection/🦀️\.rs"\]\npub mod layer_selection;\n'),
    ('scene_helpers', r'#\[path = "🧰️scene-helpers/🦀️\.rs"\]\nmod scene_helpers;\npub use scene_helpers::\*;\n'),
]:
    facade, _ = drop(facade, pattern)
facade, _ = drop(facade, r'#\[cfg\(test\)\]\n#\[path="💾️document/🧪️tests/🦀️\.rs"\]\nmod document_tests;\n')
facade, tests_region = drop(facade, r'//#region 🧪️Tests\n.*?//#endregion 🧪️Tests\n', re.S)
print('tests region dropped:', tests_region.strip().replace('\n', ' | ')[:300])

strip_path = lambda mod, prefix: re.sub(r'#\[path = "' + re.escape(prefix), '#[path = "', mod)
commands_mod = strip_path(commands_mod, '🎮️commands/')
modes_mod = strip_path(modes_mod, '🎭️modes/')
panels_mod = strip_path(panels_mod, '📌️panels/')

commands_root = f'''//! 🎮️ Raster editor commands: the closed `RasterCommand` channel and every command payload behind it.
//!
//! 🔗 The session state and the edit modes arrive from their own crates; the retained work types name no concrete app, so this layer needs nothing from the app that dispatches it.

{ALLOW}
{EXTERN}
use {MODES}::modes;
use {SESSION}::config::{{RasterConfig, RasterConfigMutation}};
use {SESSION}::{{config, duplicate_layer_plan, layer_selection, merge_down_plan, raster_reset_document_effect, selection}};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;

//#region 🪢️TaxonomyMounts
{commands_mod.rstrip()}
//#endregion 🪢️TaxonomyMounts

{commands_region.rstrip()}
'''
modes_root = f'''//! 🎭️ Raster editor modes: the edit mode and its windows, composed over the session state.

{ALLOW}
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

use {SESSION}::{{config, raster_measure_action, raster_scene, terminology}};

//#region 🪢️TaxonomyMounts
{modes_mod.rstrip()}
//#endregion 🪢️TaxonomyMounts
'''
panels_root = f'''//! 📌️ Raster editor panels: the catalogue, document, inspection and mask panels over the session state.

{ALLOW}
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

use {SESSION}::{{config, terminology}};
use {SESSION}::{{duplicate_layer_plan, layer_row_id, mask_row_id, merge_down_plan, raster_action, ui_label, ui_value_list, ui_value_map, ui_value_text}};
use {SESSION}::{{RASTER_INTERACTION_DOMAIN, RASTER_INTERACTION_GRANULARITY, RASTER_PLAY_CONTROLLER_ID, RASTER_TREE_PREFIX}};

//#region 🪢️TaxonomyMounts
{panels_mod.rstrip()}
//#endregion 🪢️TaxonomyMounts
'''
session_root = f'''//! 🧠️ Raster editor session: the document-independent state every editing surface reads — session config, presence, selections, vocabulary and scene helpers.

{ALLOW}
{EXTERN}
//#region 🪢️TaxonomyMounts
#[path = "../🎚️config/🦀️.rs"]
pub mod config;
#[path = "../👥️presence/🦀️.rs"]
pub mod presence;
#[path = "../🎯️selection/🦀️.rs"]
pub mod selection;
#[path = "../🖱️selection/🦀️.rs"]
pub mod layer_selection;
#[path = "../🗣️terminology/🦀️.rs"]
pub mod terminology;
//#endregion 🪢️TaxonomyMounts

#[path = "../🧰️scene-helpers/🦀️.rs"]
mod scene_helpers;
pub use scene_helpers::*;
'''
reexports = f'''pub use {COMMANDS}::{{commands, RasterCommand}};
pub use {MODES}::modes;
pub use {PANELS}::panels;
pub use {SESSION}::*;
'''
anchor = 'extern crate semio_framework_schema as schema;\n'
assert facade.count(anchor) == 1
facade = facade.replace(anchor, anchor + '\n' + reexports)
facade = re.sub(r'\n{3,}', '\n\n', facade)

outputs = {
    '🎮️commands/🦀️.rs': commands_root,
    '🎭️modes/🦀️.rs': modes_root,
    '📌️panels/🦀️.rs': panels_root,
    '🧠️session/🦀️.rs': session_root,
    '🦀️.rs': facade,
}
for rel, text in outputs.items():
    print('==', rel, len(text.split('\n')), 'lines')
if write:
    for rel, text in outputs.items():
        os.makedirs(os.path.dirname(E + rel), exist_ok=True)
        open(E + rel, 'w', encoding='utf-8').write(text)
    print('written')

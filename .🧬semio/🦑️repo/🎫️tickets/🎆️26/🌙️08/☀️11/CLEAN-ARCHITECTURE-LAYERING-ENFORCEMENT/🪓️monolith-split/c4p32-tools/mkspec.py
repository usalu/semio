import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(T, 'specs'))
sys.path.insert(0, T)
from common import *

P = '✏️s/🔌️plugins'
SH = P + '/🎥️shooting/🗿️artifacts/🎥️shooting'
SHE = SH + '/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
FEM = P + '/🏗️fem/🗿️artifacts/🧊️3d'
FEME = FEM + '/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor'
BIM = P + '/🏙️bim/🗿️artifacts/🏢️model'
BIME = BIM + '/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
BIMP = BIM + '/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🧱️building-elements'


def spec(n, file, children, ext_dir, **kw):
    d = dict(orig='%d.rs.txt' % n, file=file, nest=False, children=children, external=rs_under(ext_dir, [file]))
    d.update(kw)
    return d


TABLE = {}

c = [
    ch('🧰', 'ui-helpers-and-io', 'ui_helpers_and_io', [(93, 230)], 'UI node builders and ports of the shooting editor: the action constructors, the scene value helpers, the tree item builder and the photo io ports.'),
    ch('🔀', 'args-bridge', 'args_bridge', [(303, 454)], 'Host-action bridge of the shooting editor: folds the shell `{action, args}` vocabulary into the closed `ShootingCommand` enum.', modbody=True, glob=False),
    ch('🧵', 'retained-commands', 'retained_commands', [(464, 627)], 'Retained commands of the shooting editor: the bounded tool ids and limits and the bounded command job factory.'),
    ch('🕹', 'play-app', 'play_app', [(458, 461), (630, 913)], 'Play app of the shooting editor: the `ShootingPlayApp` and its `ArtifactEditor` implementation.'),
    ch('📋', 'manifest', 'manifest', [(934, 1141)], 'Manifest of the shooting editor: the app manifest stitch of `create_shooting_app`.'),
]
TABLE['shootingeditor'] = spec(1, SHE + '/🦀️.rs', c, SH, wiring='each', prune_unused_reexports=True)

c = [
    ch('🪪', 'declaration', 'declaration', [(21, 176)], 'Declaration of the shooting artifact: the document schema and dialect, the artifact kind, the pilot languages and the definition.'),
    ch('🌍', 'scene-records', 'scene_records', [(180, 426)], 'Scene records of the shooting artifact: the cameras, assets, shots, lighting records with their defaults and the camera math.'),
    ch('🖼', 'emblem-codec', 'emblem_codec', [(461, 580)], 'Emblem codec of the shooting artifact: the composed image child slot, its base64 codec and the emblem accessors.'),
    ch('🩹', 'scene-patches', 'scene_patches', [(584, 739)], 'Sparse patches of the shooting artifact: the identified records, the assigned wrapper and the asset, shot, camera and scene patches.'),
]
TABLE['shootingroot'] = spec(2, SH + '/🦀️.rs', c, SH, wiring='each', prune_unused_reexports=True)

c = [
    ch('🗃', 'retained-commands', 'retained_commands', [(165, 462)], 'Retained commands of the FEM 3D editor: the tool ids, the publication contracts, the command work and the retained command job factory.'),
    ch('📬', 'store-preparation', 'store_preparation', [(466, 610)], 'Store preparation of the FEM 3D editor: the artifact mutation byte accounting and the one-item store preparation factory.'),
    ch('🔌', 'json-io-and-arg-helpers', 'json_io_and_arg_helpers', [(614, 733)], 'Result json, ports and argument helpers of the FEM 3D editor: the results map json, the io ports and the dof argument decoders.'),
    ch('🖥', 'play-app', 'play_app', [(737, 1307)], 'Play app of the FEM 3D editor: the `Fem3dPlayApp` and its `ArtifactEditor` implementation.'),
    ch('📋', 'manifest', 'manifest', [(1318, 1562)], 'Manifest of the FEM 3D editor: the inspector patch action and the app manifest stitch of `create_fem3d_app`.'),
]
TABLE['femeditor'] = spec(3, FEME + '/🦀️.rs', c, FEM, wiring='each', prune_unused_reexports=True, keep_empty=('UiHelpers',))

c = [
    ch('🧷', 'identity-and-recovery-slot', 'identity_and_recovery_slot', [(69, 179)], 'Identity and recovery slot of the mounted FEM 3D session: the app instance identity, the recovery publication and the recovery slot.'),
    ch('💼', 'mounted-state', 'mounted_state', [(181, 457)], 'Mounted state of the FEM 3D session: the per-app-instance state, its transitions and its drop retirement.'),
    ch('📸', 'snapshot-preflight', 'snapshot_preflight', [(459, 612)], 'Snapshot preflight of the FEM 3D session: the current publication, the preflight stages and the pending snapshot.'),
    ch('🗃', 'registry', 'registry', [(614, 710)], 'Registry of the FEM 3D session: the slot arrays of the mounted apps and the thread-local registry.', reexport_rank=1),
    ch('⚙', 'mounted-job', 'mounted_job', [(712, 869)], 'Mounted job of the FEM 3D session: the bounded job, its input codec and the admission helpers.'),
    ch('🚀', 'session-api', 'session_api', [(871, 1046)], 'Session API of the FEM 3D session: the initialization, snapshot read, reconcile and the live visual and solver publication.'),
    ch('🧹', 'recovery-and-close', 'recovery_and_close', [(1048, 1232)], 'Recovery and close of the FEM 3D session: the abandoned instance recovery, retirement, maintenance and the close steps.'),
]
TABLE['femsession'] = spec(4, FEME + '/🧵️session/🦀️.rs', c, FEM, wiring='each', prune_unused_reexports=True,
                           patch={'🗃️registry': [('    static MOUNTED:', '    pub(super) static MOUNTED:')]})

c = [
    ch('🔀', 'args-bridge', 'args_bridge', [(289, 516)], 'Host-action bridge of the BIM editor: folds the shell `{action, args}` vocabulary into the closed `BimCommand` enum.', modbody=True, glob=False),
    ch('🗃', 'io-and-retained-commands', 'io_and_retained_commands', [(520, 645)], 'Io and retained commands of the BIM editor: the io ports and the bounded command job factory.'),
    ch('📢', 'fault-notices', 'fault_notices', [(649, 760)], 'Fault notices of the BIM editor: the localized fault notice table.', keep_reexport=True),
    ch('🖥', 'play-app', 'play_app', [(764, 1074)], 'Play app of the BIM editor: the `BimModelApp` and its `ArtifactEditor` implementation.'),
    ch('📋', 'manifest', 'manifest', [(1078, 1186)], 'Manifest of the BIM editor: the action argument folding, the keybindings and the app manifest stitch of `create_bim_app`.'),
]
TABLE['bimeditor'] = spec(6, BIME + '/🦀️.rs', c, BIM, wiring='each', prune_unused_reexports=True)

c = [
    ch('🔤', 'field-codecs', 'field_codecs', [(54, 391)], 'Field codecs of the BIM entity table: the text and parse helpers of every field row kind.'),
    ch('🧮', 'field-tables', 'field_tables', [(518, 793)], 'Field tables of the BIM entity table: the authored parameter rows and inferred rows of every entity kind.'),
    ch('➕', 'create-rows', 'create_rows', [(797, 1001)], 'Create rows of the BIM entity table: the create mutation of every entity kind.'),
    ch('🗂', 'entity-table', 'entity_table', [(1005, 1095)], 'Entity table of the BIM entity table: the one static table of every entity kind.'),
    ch('🧾', 'properties', 'properties', [(1132, 1203)], 'Properties of the BIM entity table: the property types, texts, values and entries.'),
    ch('🔎', 'project-and-lookup', 'project_and_lookup', [(1099, 1128), (1207, 1232)], 'Project and lookup of the BIM entity table: the project fields, the classification text and the kind, holding, id and storey lookups.'),
]
TABLE['bimentities'] = spec(7, BIME + '/🧩️entities/🦀️.rs', c, BIM, wiring='each', prune_unused_reexports=True)

c = [
    ch('🏚', 'walls-and-curtain-walls', 'walls_and_curtain_walls', [(5, 228)], 'Wall patches of the building elements: the wall, curtain panel override and curtain wall sparse patches.'),
    ch('🏛', 'columns-beams-slabs', 'columns_beams_slabs', [(230, 453)], 'Structural patches of the building elements: the column, beam and slab sparse patches.'),
    ch('🔲', 'ceilings-roofs-openings', 'ceilings_roofs_openings', [(455, 690)], 'Envelope patches of the building elements: the ceiling, roof and opening sparse patches.'),
    ch('🪜', 'stairs-railings-ramps', 'stairs_railings_ramps', [(692, 1023)], 'Circulation patches of the building elements: the stair, railing and ramp sparse patches.'),
]
TABLE['bimpatches'] = spec(8, BIMP + '/🦀️.rs', c, BIM, wiring='each', prune_unused_reexports=True)

if __name__ == '__main__':
    for k, v in TABLE.items():
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(v, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)

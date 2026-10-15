F = '🧰️framework'
PL = '✏️s/🔌️plugins'
OS = F + '/🛍️products/💻️os'
EE = OS + '/🔨️modules/📺️renderer/🧑‍🎨engine'
EL = EE + '/🧱️elements'
INF = OS + '/🔨️modules/♾️infinite'
SUB = '🏅️standards/🔖️1/🪆️subsets/✳️any'
PRES = F + '/🛍️products/🎤️presentation'

SPECS = {}


def add(id, orig, parent, folders, exts, include=None, force_hollow=(), keep_stray=()):
    SPECS[id] = dict(id=id, orig=orig, parent=parent, folders=folders, exts=exts, include=include or [], force_hollow=force_hollow, keep_stray=keep_stray)


add('bim', PL + '/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs', PL + '/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs',
    [PL + '/🏙️bim/🗿️artifacts/🏢️model'], ('.rs',),
    include=[PL + '/🏙️bim/🗿️artifacts/🏢️model/' + SUB + '/🧬️schema/🧬️mutations/🦀️.rs'])
CADR = PL + '/📐️cad/🗿️artifacts/📐️cad/' + SUB + '/✏️editor/⚙️engine/📺️renderer'
add('cad', CADR + '/🟦️.tsx', CADR + '/🟦️.tsx', [CADR], ('.ts', '.tsx'))
P2D = PL + '/🧩️puzzle/🗿️artifacts/◻️2d/' + SUB + '/✏️editor'
add('p2d', P2D + '/🦀️.rs', P2D + '/🦀️.rs', [P2D], ('.rs',))
P5D = PL + '/🧩️puzzle/🗿️artifacts/🖐️5d/' + SUB + '/✏️editor'
add('p5d', P5D + '/🦀️.rs', P5D + '/🦀️.rs', [P5D], ('.rs',))
ANI = PL + '/🎞️animate/🗿️artifacts/🎬️presentation/' + SUB + '/✏️editor/📺️renderer/⚛️react'
add('md', ANI + '/🔨️modules/📝️markdown-html-compiler/🟦️.ts',
    PRES + '/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts', [], ('.ts',), force_hollow=('🧪️Tests',))
add('pres', PRES + '/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx',
    PRES + '/🎯️targets/⚛️react/🟦️.tsx', [PRES + '/🎯️targets/⚛️react'], ('.ts', '.tsx'))
add('pres2', ANI + '/🟦️.tsx',
    PRES + '/🎯️targets/⚛️react/🟦️.tsx', [PRES + '/🎯️targets/⚛️react'], ('.ts', '.tsx'))
W = INF + '/🌍️world'
add('world', W + '/🦀️.rs', W + '/🦀️.rs', [W], ('.rs',))
N = INF + '/🎲️board/🔌️ports/➡️directed/➕️normal'
add('normal', N + '/🦀️.rs', N + '/🦀️.rs', [N], ('.rs',))
D = INF + '/🎲️board/🔌️ports/➡️directed/🕸️dag'
add('dag', D + '/🦀️.rs', D + '/🦀️.rs', [D], ('.rs',))
WR = EE + '/🎯️targets/🧊️wgpu/🧊️renderer'
add('wgpurenderer', WR + '/🦀️.rs', WR + '/🦀️.rs', [WR], ('.rs',))
W3 = EL + '/🌐️World3dHost'
add('world3d', W3 + '/🟦️.tsx', W3 + '/🟦️.tsx', [W3], ('.ts', '.tsx'))
SC = EL + '/🎞️Scenes/🎯️targets/🧊️wgpu'
add('scenes', SC + '/🦀️.rs', SC + '/🦀️.rs', [SC], ('.rs',))
SH = EL + '/🏛️ShellHost'
add('shellhost', SH + '/🟦️.tsx', SH + '/🟦️.tsx', [SH], ('.ts', '.tsx'))
PR = EL + '/🔌️PluginRuntime'
add('pluginruntime', PR + '/🟦️.tsx', PR + '/🟦️.tsx', [PR], ('.ts', '.tsx'))
SHH = EL + '/🛠️ShellHelpers'
add('shellhelpers', SHH + '/🟦️.tsx', SHH + '/🟦️.tsx', [SHH], ('.ts', '.tsx'))
HO = OS + '/🖥️host'
add('oshost', HO + '/🦀️.rs', HO + '/🦀️.rs', [HO], ('.rs',))
add('osts', OS + '/🟦️.ts', OS + '/🟦️.ts', [], ('.ts',), include=[OS + '/🔨️modules/' + m + '/🟦️.ts' for m in ('🎁️packs', '📞️directory-client', '🧵️backbone', '🧾️app-channel', '🪢️wire-contracts')], keep_stray=('🔖️EnvelopeCodec',))
GQ = F + '/🛍️products/🦑️repo/🔨️modules/🔗️graphql'
add('graphql', GQ + '/📦️packages/🦀️rust/🦀️.rs', GQ + '/📦️packages/🦀️rust/🦀️.rs', [GQ], ('.rs',))
HUB = '🌎️hub'
add('hubboot', HUB + '/🏗️bootstrap/🦀️.rs', HUB + '/🏗️bootstrap/🦀️.rs', [HUB + '/🏗️bootstrap'], ('.rs',))
add('hubinf', HUB + '/💡️inference/🏃️runtime/🦀️.rs', HUB + '/💡️inference/🏃️runtime/🦀️.rs', [HUB + '/💡️inference/🏃️runtime'], ('.rs',))
add('hubdir', HUB + '/📇️directory/🦀️.rs', HUB + '/📇️directory/🦀️.rs', [HUB + '/📇️directory'], ('.rs',))
TM = F + '/🔨️modules/🗺️surface/🗺️tiled-map'
add('tiled', TM + '/🦀️.rs', TM + '/🦀️.rs', [TM], ('.rs',))
MF = F + '/🔨️modules/🛂️manifest'
add('manifest', MF + '/🦀️.rs', MF + '/🦀️.rs', [MF], ('.rs',))
SHL = EL + '/🐚️Shell/🎯️targets/🧊️wgpu'
add('shell', SHL + '/🦀️.rs', SHL + '/🦀️.rs', [SHL], ('.rs',))
PH = OS + '/🔨️modules/🔌️plugin/🖥️host'
add('plughost', PH + '/🦀️.rs', PH + '/🦀️.rs', [PH], ('.rs',))
CLI = F + '/🛍️products/🦑️repo/🔨️modules/⌨️cli'
add('cli', CLI + '/🦀️.rs', CLI + '/🦀️.rs', [CLI], ('.rs',))

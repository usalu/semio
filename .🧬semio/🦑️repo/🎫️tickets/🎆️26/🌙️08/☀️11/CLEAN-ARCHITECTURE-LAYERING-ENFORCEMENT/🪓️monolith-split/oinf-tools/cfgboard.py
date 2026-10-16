"""Board-lane plan configuration."""
from lane import *

def S(rel): return rel
B = '🎲️board/'
D = '🎲️board/🔌️ports/➡️directed/'
ORDER = [
 B+'🧬️schema/🦀️.rs',
 B+'🧬️schema/📐️layout/🦀️.rs',
 B+'🧬️schema/👁️visibility/🦀️.rs',
 B+'🧬️schema/🔺️edge-tip/🦀️.rs',
 B+'🧬️schema/🎨️palette/🦀️.rs',
 B+'🧬️schema/🎯️dag-input/🦀️.rs',
 B+'🧬️schema/🎯️dag-input/📊️progress/🦀️.rs',
 B+'🧬️schema/🎯️dag-input/📤️output/🦀️.rs',
 B+'🧬️schema/🎯️dag-input/♻️retirement/🦀️.rs',
 B+'🧬️schema/💡️inferences/📐️layout/🦀️.rs',
 B+'🧬️schema/💡️inferences/📐️layout/🕸️layered/🦀️.rs',
 B+'🧬️schema/💡️inferences/📐️layout/🌳️hierarchical/🦀️.rs',
 B+'🧬️schema/💡️inferences/📐️layout/⚛️force/🦀️.rs',
 B+'🧬️schema/💡️inferences/📐️layout/🔁️redraw/🦀️.rs',
 B+'♻️retirement/🦀️.rs',
 B+'🦀️.rs',
 B+'🔌️ports/🧬️schema/🦀️.rs',
 B+'🔌️ports/🦀️.rs',
 B+'🔌️ports/↔️undirected/🦀️.rs',
 D+'🦀️.rs',
 D+'🧬️schema/🦀️.rs',
 D+'🧬️schema/📸️snapshot/🦀️.rs',
 B+'🚪️io/🦀️.rs',
 B+'🚪️io/📝️text/🦀️.rs',
 B+'🚪️io/📝️text/🎨️palette/🦀️.rs',
 B+'🚪️io/📝️text/👁️visibility/🦀️.rs',
 B+'🚪️io/📝️text/📐️layout/🦀️.rs',
 B+'🚪️io/📝️text/📸️snapshot/🦀️.rs',
 B+'🚪️io/📝️text/📸️snapshot/📥️assembly/🦀️.rs',
 B+'🚪️io/📝️text/🎯️dag-input/🦀️.rs',
 B+'🚪️io/📝️text/🎯️dag-input/📊️progress/🦀️.rs',
 B+'🚪️io/📝️text/🎯️dag-input/📤️output/🦀️.rs',
 B+'🚪️io/📝️text/🎯️dag-input/📤️selection/🦀️.rs',
 B+'🚪️io/📝️text/🎯️dag-input/🧵️retained/🦀️.rs',
 B+'➕️normal/➡️directed/🦀️.rs',
 B+'➕️normal/↔️undirected/🦀️.rs',
 # dag
 D+'🕸️dag/🦀️.rs',
 D+'🕸️dag/🧹️retirement/🦀️.rs',
 D+'🕸️dag/🎛️input/🦀️.rs',
 D+'🕸️dag/🧭️host-selection/🦀️.rs',
 D+'🕸️dag/🧱️frame-minimap-align/🦀️.rs',
 D+'🕸️dag/🛠️mutate/🦀️.rs',
 D+'🕸️dag/🖌️paint/🦀️.rs',
 D+'🕸️dag/🌐️wasm-session/🦀️.rs',
 # normal
 D+'➕️normal/🦀️.rs',
 D+'➕️normal/🪣️fill-job/🦀️.rs',
 D+'➕️normal/🧭️host/🦀️.rs',
 D+'➕️normal/🎨️style/🦀️.rs',
 D+'➕️normal/🎛️selection-events/🦀️.rs',
 D+'➕️normal/🧹️scene-delete-links/🦀️.rs',
 D+'➕️normal/🖱️pointer-gestures/🦀️.rs',
]
def seed(n):
    if n.kind == 'struct' and n.name == 'BoardHost': return D+'➕️normal/🎨️style/🦀️.rs'
    if n.kind == 'struct' and n.name == 'DagHost': return D+'🕸️dag/🧭️host-selection/🦀️.rs'
    return n.mod
SPLIT = {'BoardHost', 'DagHost'}

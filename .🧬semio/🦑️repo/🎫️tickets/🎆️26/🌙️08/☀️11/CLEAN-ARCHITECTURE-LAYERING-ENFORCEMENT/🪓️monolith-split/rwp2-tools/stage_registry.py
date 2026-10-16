import re, sys
from model import *
import mvtool, imports
REG = EC + '/🗂️engine-surface-registry/🦀️.rs'
def rd(p): return open(p, encoding='utf8').read()
def wr(p, t): open(p, 'w', encoding='utf8').write(t)
def move(src, names, region):
    raw = rd(src)
    text, out = mvtool.cut(src, names, None, dry=True)
    need = imports.needed(text, raw)
    wr(src, out)
    d = rd(REG).rstrip('\n') + '\n\n//#region %s\n\n%s\n\n//#endregion %s\n' % (region, text, region)
    d = imports.add_uses(d, need)
    wr(REG, d)
    return [u['alias'] for u in need]
print(move(EC + '/✍️text-editor/🦀️.rs', ['TEXT_EDITOR_PENDING_ECHO_CAPACITY', 'TextEditorDeliverySnapshot', 'TextEditorTypingRun', 'TextEditorTypingEnd', 'text_editor_now_ms', 'TextEditorDeliveryInFlight', 'TextEditorDraftPublication', 'TextEditorDeliveryPublication', 'TextEditorDeliveryState'], '✍️TextEditorDelivery'))
print(move(EC + '/🕸️node-graph/🦀️.rs', ['engine_canvas_fault_log', 'engine_canvas_debug_log', 'PublishedGraphInteraction', 'published_graph_interaction'], '🪵️EngineCanvasDiagnostics'))
print(move(EC + '/🧩️engine-surface-attach/🦀️.rs', ['engine_camera_from_json'], '📷️EngineCamera'))

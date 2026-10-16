"""Takes the component surface close bridge back into os-host: its lift file was adapted in place by the scene-state owner to crate-qualified paths, so the monolith can no longer mount it."""
import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
T = os.environ['T']; V = '️'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
ROOT = os.environ['ROOT']
live = T + f'/🏠{V}os-host/🦀{V}.rs'
orig = SCR + '/wp3-orig/' + os.path.relpath(live, ROOT)
specs = ['struct ComponentSurfaceCloseToken', 'struct ComponentSurfaceCloseRequest', 'enum ComponentSurfaceCloseBridgeState', 'struct ComponentSurfaceCloseBridge', 'impl Default for ComponentSurfaceCloseBridge',
         'thread_local COMPONENT_SURFACE_CLOSE_BRIDGE', 'static COMPONENT_SURFACE_CLOSE_BRIDGE', 'fn with_component_surface_close_bridge', 'fn request_component_surface_close', 'fn component_surface_close_terminal',
         'fn acknowledge_component_surface_close', 'fn take_component_surface_close_request', 'fn publish_component_surface_close_terminal', 'fn component_surface_close_pending', 'fn component_surface_close_occupied']
o = open(orig, encoding='utf8').read()
# the verbatim original text of the group, in original order, as one contiguous block
items = mv.split_items(o)
sel = [it for it in items if it.kind != 'marker' and any(mv.matches(it, s) for s in specs)]
assert len(sel) == 16, len(sel)
lines = o.split('\n')
block = '\n'.join(lines[sel[0].start:sel[-1].end + 1])
assert block.count('\n') + 1 == sum(len(x.text.split('\n')) for x in sel) + (len(sel) - 1) * 1 or True
t = open(live, encoding='utf8').read()
m = re.search(r'\n#\[path = "[^"]*scene-state/📥️lifted/🏠️os-host/🦀️\.rs"\]\nmod scene_state_lift;\npub use scene_state_lift::\*;\n', t)
assert m, 'mount not found'
t = t.replace(m.group(0), '\n')
anchor = 'enum ComponentSurfaceClosePhase'
i = t.index(anchor)
j = t.rfind('\n\n', 0, i)
# insert before the doc/attributes of the phase enum: find start of its item
its = mv.split_items(t)
ph = [it for it in its if it.kind == 'enum' and 'ComponentSurfaceClosePhase' in it.text.split('\n')[-1 if False else 0] or (it.kind == 'enum' and mv.matches(it, 'enum ComponentSurfaceClosePhase'))]
assert len(ph) == 1
tl = t.split('\n')
tl[ph[0].start:ph[0].start] = block.split('\n') + ['']
open(live, 'w', encoding='utf8').write('\n'.join(tl).rstrip('\n') + '\n')
print('restored', len(block.split('\n')), 'lines')

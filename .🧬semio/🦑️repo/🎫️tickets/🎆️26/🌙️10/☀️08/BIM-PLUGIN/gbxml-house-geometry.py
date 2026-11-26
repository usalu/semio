exec(open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/gbxml-probe-audit.py').read().split('for name in (')[0])
m=o.Model((f/'🌿️gbxml/🏠️house/gbxml.xml').read_bytes())
for ident in ('space-sp-g1','space-sp-a1','space-sp-b1','space-sp-g4'):
 space=m.spaces[ident]; print('\n',ident,'declared',o.number_of(space,'Area'),o.number_of(space,'Volume'))
 for boundary in space.findall(o.q('SpaceBoundary')):
  p=o.loop_of(boundary.find(o.q('PlanarGeometry'))); v=o.area_vector(p)/2
  print(boundary.get('surfaceIdRef'),'z',round(min(x[2] for x in p),3),round(max(x[2] for x in p),3),'vec',v.round(3).tolist())

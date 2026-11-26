exec(open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/gbxml-probe-audit.py').read().split('for name in (')[0])
for name in ('🏘️zoning','🧱️stack'):
 m = o.Model((probe/name/'gbxml.xml').read_bytes())
 print(name)
 for id, space in m.spaces.items():
  print(id, 'declared',o.number_of(space,'Volume'))
  for boundary in space.findall(o.q('SpaceBoundary')):
   p=o.loop_of(boundary.find(o.q('PlanarGeometry'))); v=o.area_vector(p)/2
   print(' ',boundary.get('surfaceIdRef'), 'z',round(min(x[2] for x in p),3),round(max(x[2] for x in p),3),'area vector',v.round(3).tolist())

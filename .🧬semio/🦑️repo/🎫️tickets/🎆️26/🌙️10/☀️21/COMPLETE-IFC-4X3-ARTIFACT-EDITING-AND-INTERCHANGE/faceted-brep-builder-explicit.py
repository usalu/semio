from pathlib import Path
p=Path(__file__).resolve().parent/'faceted-brep-vectors-oracle.py';s=p.read_text(encoding='utf-8-sig')
s=s.replace("mode=row['mode'];scale=1000. if mode=='mm' else 1.;cache={}","mode=row['modifier']['kind'];scale=row['unit']['coordinateScale'];cache={}")
s=s.replace("Prefix='MILLI'if mode=='mm'else None","Prefix=row['unit']['prefix']")
a=s.index('        outer=[(0,0)');b=s.index('        loops=[];faces=[]',a)
s=s[:a]+"        geometry=row['geometry'];outer=geometry['outer'];holes=geometry['holes'];height=geometry['height']\n"+s[b:]
s=s.replace("(mode=='false')^reverse","row['orientation']['reversePolygon']^reverse").replace("Orientation=mode!='false'","Orientation=row['orientation']['bound']")
s=s.replace("shell=prism(outer,holes,0,height)","shell=prism(outer,holes,geometry['base'],height)")
a=s.index("        if mode=='voids':");b=s.index("        rep=f.create_entity",a)
s=s[:a]+'''        if geometry['voids']:
            cavities=tuple(prism(v['outer'],v['holes'],v['base'],v['height'],v['reverse'])for v in geometry['voids']);solid=f.create_entity('IfcFacetedBrepWithVoids',Outer=shell,Voids=cavities)
        else:solid=f.create_entity('IfcFacetedBrep',Outer=shell)
        modifier=row['modifier']
        if mode=='point-coordinates':
            selected=list(cache.values())[modifier['ordinal']];selected.Coordinates=tuple(modifier['coordinates'])if'coordinates'in modifier else selected.Coordinates[:modifier['dimensions']]
        if mode=='polygon-order':
            polygon=loops[modifier['ordinal']].Polygon;loops[modifier['ordinal']].Polygon=tuple(polygon[i]for i in modifier['order'])
        if mode=='two-outer':faces[0].Bounds=faces[0].Bounds+(f.create_entity('IfcFaceOuterBound',Bound=loops[0],Orientation=True),)
        if mode=='empty-bounds':faces[0].Bounds=()
'''+s[b:]
a=s.index("        if mode=='rigid':");b=s.index("        placement=f.create_entity",a)
s=s[:a]+"        pose=row['placement'];local=f.create_entity('IfcAxis2Placement3D',Location=f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x)for x in pose['origin'])),Axis=z,RefDirection=f.create_entity('IfcDirection',DirectionRatios=(math.cos(pose['rotation']),math.sin(pose['rotation']),0.)))\n"+s[b:]
p.write_text(s,encoding='utf-8')

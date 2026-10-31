"""🧮 Compare the actual owned flight cells before and after the single Float64 publication boundary."""
import ast,hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
from shapely.affinity import scale,translate
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
A=importlib.import_module(prefix+'🧱️model.🐍️');U=importlib.import_module(prefix+'🎨️visuals.📐️drafting.🧩️regions.🐍️');model=A.load_model();rows=[];start=time.monotonic()
for tag in sys.argv[1:] or ('RF-1B','RF-2B'):
    print('[DEBUG] '+tag+' original owned operands',flush=True);obj=A._find(model,tag);g=obj['geometry'];stock=A._ramp_reference_stock(model,obj);cutters=[A._ramp_reference_stock(model,A._find(model,key)) for key in g.get('flight_joint_owners',[])]+[part for key in g.get('interface_owners',[]) for part in A._interface_reference_parts(model,key)]
    cells=A._reference_material_parts(stock,cutters);actual=A.ramp_geometry(tag,model)['parts']
    def signatures(parts):
        """🪪 Match every public convex cell's complete vertex multiset without dropping positive material."""
        return sorted(tuple(sorted(tuple(float(x) for x in p) for p in A.part_vertices(part))) for part in parts)
    polygons=[]
    for cell in cells:
        vertices=A.part_vertices(cell)
        for face in A.part_faces(cell):
            ring=[tuple(vertices[i][:2]) for i in face]
            if U._area(ring)>0:polygons.append(ring)
    print('[DEBUG] '+tag+' exact upper-face union '+str(len(polygons)),flush=True);regions=U.polygon_union(polygons)
    body=unary_union([Polygon(row['outer'],row['holes']) for row in regions]);ink=unary_union([LineString(ring+[ring[0]]) for row in regions for ring in (row['outer'],*row['holes'])]);expected=unary_union([Polygon(A.part_footprint(part)) for part in actual])
    origin=[min(point[i] for ring in polygons for point in ring) for i in (0,1)]
    def chart(ring):
        """📏 Condition exact source coordinates before the independent oracle converts to binary64."""
        return [[float((point[i]-origin[i])*1000) for i in (0,1)] for point in ring]
    private=U._arrangement(tuple(tuple(tuple(p) for p in ring) for ring in polygons),lambda *args:None,lambda:None)
    oracle=unary_union([Polygon(chart(ring)) for ring in polygons]);chart_body=unary_union([Polygon(chart(row['outer']),[chart(ring) for ring in row['holes']]) for row in private]);chart_ink=unary_union([LineString(chart(ring+[ring[0]])) for row in private for ring in (row['outer'],*row['holes'])])
    chart_result={'area_difference_m2':chart_body.symmetric_difference(oracle).area/1e6,'boundary_difference_m':chart_ink.symmetric_difference(oracle.boundary).length/1000,'perimeter_m':chart_ink.length/1000,'oracle_perimeter_m':oracle.length/1000,'oracle_components':len(oracle.geoms) if hasattr(oracle,'geoms') else 1,'oracle_holes':sum(len(p.interiors) for p in oracle.geoms) if hasattr(oracle,'geoms') else len(oracle.interiors)}
    row={'affine_geos':chart_result,'flight':tag,'private_cells':len(cells),'actual_cells':len(actual),'every_cell_vertex_equal_after_publication':signatures(cells)==signatures(actual),'regions':len(regions),'holes':sum(len(row['holes']) for row in regions),'valid':body.is_valid,'area_difference':body.symmetric_difference(expected).area,'perimeter':ink.length,'expected_perimeter':expected.length,'boundary_difference':ink.symmetric_difference(expected.boundary).length,'geometry':regions};rows.append(row);print('[DEBUG] '+json.dumps({k:v for k,v in row.items() if k!='geometry'}),flush=True)
ns={'sys':sys};tree=ast.parse((ticket/'ramp-plan-union-r93.py').read_text(encoding='utf8'));exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef)and n.name=='peak_rss_bytes'],type_ignores=[]),str(ticket),'exec'),ns)
(ticket/'🗑️generated'/('ramp-private-projection-'+('-'.join(sys.argv[1:]))+'-r93.json' if sys.argv[1:] else 'ramp-private-projection-r93.json')).write_text(json.dumps({'rows':rows,'peak_rss_bytes':ns['peak_rss_bytes'](),'seconds':time.monotonic()-start},indent=2),encoding='utf8')

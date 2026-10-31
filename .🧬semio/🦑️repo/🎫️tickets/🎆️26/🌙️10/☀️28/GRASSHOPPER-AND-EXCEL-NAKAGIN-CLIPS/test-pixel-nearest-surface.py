"""🧪 Validate independent pixel rays against portable physical geometry and NumPy plane solutions."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest
import numpy as np

TICKET=Path(__file__).parent
spec=importlib.util.spec_from_file_location('pixel_nearest_surface',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
C=json.loads((TICKET/'pixel-ray-cases.json').read_text(encoding='utf8'))

class PixelSurfaces(unittest.TestCase):
    def test_positive_fragment_uses_independent_original_carrier(self):
        import copy
        case=json.loads((TICKET/'pixel-plane-lineage-cases.json').read_text(encoding='utf8'))
        original=np.asarray(case['parent']['vertices'],float)
        vertices=np.asarray(case['fragment']['vertices'],float)
        self.assertGreater(np.linalg.norm(np.cross(vertices[1]-vertices[0],vertices[2]-vertices[0])),0)
        item=dict(C['cases'][0]['items'][0],index=2,key='original-carrier',removed=False,normal=case['parent']['normal'],world_vertices=original.tolist(),pixel_polygons=[original[:,:2].tolist()])
        scene=M.SurfaceScene([*C['cases'][0]['items'],item],C['towards'])
        row={'stage':'culled','rank':0,'piece':0,'source':2,'polygon':0,'world_vertices':vertices.tolist(),'normal':case['fragment']['normal']}
        paint=M.FragmentPaint(scene,[row])
        self.assertEqual(len(paint.triangles),case['expected_triangles'])
        np.testing.assert_array_equal(paint.triangles[0],vertices)
        self.assertEqual(len(M.source_facets(vertices)[0]),1)
        point=vertices.mean(axis=0)[:2]
        actual=paint.intersections([point])[0]
        self.assertEqual(paint.contributors(actual,'culled')['visible_ids'],[2])
        _,_,vh=np.linalg.svd(original-original.mean(axis=0))
        normal=vh[-1]
        basepoint=(point-scene.offset)@scene.inverse.T
        expected=float(normal@(original[0]-basepoint)/(normal@scene.towards))
        self.assertLessEqual(abs(float(np.nanmax(actual))-expected),case['ulp_multiplier']*np.spacing(float(np.abs(original).max())))
        for mutation in ('wrong-normal','off-carrier'):
            bad=copy.deepcopy(row)
            if mutation=='wrong-normal':
                bad['normal']=[0,0,1]
            else:
                bad['world_vertices']=(vertices+[0,0,.001]).tolist()
            with self.assertRaises(ValueError):
                M.FragmentPaint(scene,[bad])
        scaled=copy.deepcopy(row)
        scaled['normal']=(3*np.asarray(row['normal'])).tolist()
        self.assertEqual(len(M.FragmentPaint(scene,[scaled]).triangles),1)

    def test_original_facet_facing_ignores_raw_normals_and_exported_flags(self):
        controls=json.loads((TICKET/'pixel-warped-mesh-cases.json').read_text(encoding='utf8'))
        for case in controls['facing_cases']:
            with self.subTest(case=case['id']):
                self.assertLessEqual(float(np.asarray(case['items'][0]['normal'])@C['towards']),0)
                self.assertFalse(case['items'][0]['raw_camera_facing'])
                scene=M.SurfaceScene(case['items'],C['towards'])
                physical=scene.contributors(scene.intersections([case['point']])[0])
                self.assertEqual(physical['nearest_ids'],case['nearest'])

    def test_physical_boundary_witness_has_no_original_paint_rank_dependency(self):
        case=C['fragment_cases'][0]
        self.assertTrue(all('full_rank' not in row and 'new_rank' not in row for row in case['items']))
        scene=M.SurfaceScene(case['items'],C['towards'])
        witness=M.boundary_witness(scene,0,[.5,.5])
        self.assertAlmostEqual(witness['projected_edge_distance_pixels'],.5)
        self.assertNotIn('full_rank',witness)
        self.assertNotIn('new_rank',witness)

    def test_verified_convex_fan_keeps_positive_tiny_fragment_material(self):
        from scipy.spatial import ConvexHull
        controls=json.loads((TICKET/'pixel-convex-fan-cases.json').read_text(encoding='utf8'))
        for case in controls['cases']:
            with self.subTest(case=case['id']):
                points=np.asarray(case['vertices'],float)
                actual=M.convex_fan(points)
                self.assertEqual(actual is not None,case['convex'])
                if actual is None:
                    continue
                local=points-points[0]
                cross=np.cross(local[:,None,:],local[None,:,:]).reshape(-1,3)
                normal=cross[np.argmax(np.linalg.norm(cross,axis=1))]
                drop=int(np.argmax(np.abs(normal)))
                projected=np.delete(local,drop,axis=1)
                normalized=projected/np.ptp(projected,axis=0)
                hull=ConvexHull(normalized)
                self.assertEqual(len(hull.vertices),len(points))
                area=sum(float(np.linalg.norm(np.cross(face[1]-face[0],face[2]-face[0]))/2) for face in actual)
                self.assertLess(abs(area-case['area']),1e-20)
                self.assertTrue(all(np.any(np.cross(face[1]-face[0],face[2]-face[0])!=0) for face in actual))
    def test_nonzero_finite_triangle_is_not_removed_by_absolute_area_cutoff(self):
        controls=json.loads((TICKET/'pixel-convex-fan-cases.json').read_text(encoding='utf8'))['tiny_triangle']
        back=C['cases'][0]['items'][1].copy()
        back.update(index=1,removed=False,normal=[0,0,1])
        tiny=dict(back,index=0,world_vertices=controls['vertices'],pixel_polygons=[[[p[0],p[1]] for p in controls['vertices']]])
        scene=M.SurfaceScene([tiny,back],C['towards'])
        physical=scene.contributors(scene.intersections([controls['point']])[0])
        self.assertEqual(physical['nearest_ids'],controls['nearest'])
        rows=[{'stage':'culled','rank':0,'piece':0,'source':0,'polygon':0,'world_vertices':controls['vertices'],'normal':[0,0,1]}]
        paint=M.FragmentPaint(scene,rows)
        self.assertEqual(paint.contributors(paint.intersections([controls['point']])[0],'culled')['visible_ids'],controls['nearest'])

    def test_declared_warped_quad_semantics_match_numpy_and_rhino(self):
        import rhino3dm
        controls=json.loads((TICKET/'pixel-warped-mesh-cases.json').read_text(encoding='utf8'))
        for case in controls['cases']:
            with self.subTest(case=case['id']):
                vertices=np.asarray(case['vertices'],float)
                actual,warped=M.source_facets(vertices)
                self.assertEqual(warped,case['warped'])
                np.testing.assert_array_equal(actual,vertices[np.asarray(case['triangles'])])
                if warped:
                    diagonal02=float(np.dot(vertices[2]-vertices[0],vertices[2]-vertices[0]))
                    diagonal13=float(np.dot(vertices[3]-vertices[1],vertices[3]-vertices[1]))
                    self.assertEqual(case['triangles'],[[0,1,2],[0,2,3]] if diagonal02<=diagonal13 else [[0,1,3],[1,2,3]])
                    mesh=rhino3dm.Mesh()
                    for point in vertices:
                        mesh.Vertices.Add(*point)
                    mesh.Faces.AddFace(0,1,2,3)
                    mesh.Faces.ConvertQuadsToTriangles()
                    self.assertEqual([list(mesh.Faces[index][:3]) for index in range(2)],case['triangles'])

    def test_fragment_ranks_match_independent_pillow_union_masks(self):
        from PIL import Image,ImageDraw
        for case in C['fragment_cases']:
            scene=M.SurfaceScene(case['items'],C['towards'])
            paint=M.FragmentPaint(scene,case['fragments'])
            ranks={}
            for row in case['fragments']:
                if row['stage']!='culled':
                    continue
                rank=ranks.setdefault(row['rank'],{'mask':Image.new('L',(192,192),0),'sources':set()})
                rank['sources'].add(row['source'])
                ImageDraw.Draw(rank['mask']).polygon([(16+48*x,16+48*y) for x,y,z in row['world_vertices']],fill=255)
            for probe in case['probes']:
                x,y=(int(16+48*value) for value in probe['point'])
                expected=[]
                for rank in sorted(ranks,reverse=True):
                    row=ranks[rank]
                    if not row['mask'].getpixel((x,y)):
                        continue
                    expected.extend(sorted(row['sources']))
                    if any(scene.items[source]['opacity']==1 for source in row['sources']):
                        break
                actual=paint.contributors(paint.intersections([probe['point']])[0],'culled')
                self.assertEqual(actual['visible_ids'],expected,case['name'])

    def test_fragment_painter_aliases_actual_source_and_compound_alpha(self):
        for case in C['fragment_cases']:
            with self.subTest(case=case['name']):
                scene=M.SurfaceScene(case['items'],C['towards'])
                paint=M.FragmentPaint(scene,case['fragments'])
                if 'expected_unique_triangles' in case:
                    self.assertEqual(len(paint.triangles),case['expected_unique_triangles'])
                for probe in case['probes']:
                    physical=scene.contributors(scene.intersections([probe['point']])[0])
                    actual=paint.contributors(paint.intersections([probe['point']])[0],'culled')
                    self.assertEqual(physical['visible_ids'],probe['physical'])
                    self.assertEqual(actual['visible_ids'],probe['painted'])
                    for stage,expected in probe.get('stages',{}).items():
                        self.assertEqual(paint.contributors(paint.intersections([probe['point']])[0],stage)['visible_ids'],expected)
                    if 'paint_layers' in probe:
                        self.assertEqual(len(actual['layers']),probe['paint_layers'])
                        self.assertEqual(actual['layers'][0]['opacity'],probe['front_opacity'])
                    if case['name']=='invented-gap-paint-is-detected':
                        self.assertNotEqual(actual['visible_ids'],physical['visible_ids'])
    def test_fragment_identity_validation_rejects_ambiguous_paint_rows(self):
        import copy
        case=C['fragment_cases'][0]
        scene=M.SurfaceScene(case['items'],C['towards'])
        for mutate in (lambda rows:rows.append(copy.deepcopy(rows[0])),lambda rows:rows[0].update(source=999),lambda rows:rows[0].update(polygon=999)):
            rows=copy.deepcopy(case['fragments'])
            mutate(rows)
            with self.assertRaises(ValueError):
                M.FragmentPaint(scene,rows)

    def test_actual_concave_disconnected_depth_and_transparent_cases(self):
        """🔬 Reproduce source visibility without fan-filling concave faces or hiding transparent rear layers."""
        for case in C['cases']:
            with self.subTest(case=case['name']):
                scene=M.SurfaceScene(case['items'],C['towards'],C['projection_tolerance'])
                hits=scene.intersections([case['point']])[0]
                self.assertEqual(scene.contributors(hits)['visible_ids'],case['before'])
                self.assertEqual(scene.contributors(hits,True)['visible_ids'],case['after'])
                np.testing.assert_allclose(scene.matrix,[[1,0,0],[0,1,0]],atol=1e-12)
                np.testing.assert_allclose(scene.offset,[0,0],atol=1e-12)

    def test_depth_crossing_agrees_with_independent_plane_linear_solve(self):
        """📏 Compare ray depth with NumPy's independent three-equation plane solve."""
        case=next(row for row in C['cases'] if row['name']=='depth-crossing-near')
        scene=M.SurfaceScene(case['items'],C['towards'])
        for point in ([.5,1.5],[2.5,1.5]):
            hits=scene.intersections([point])[0]
            visible=scene.contributors(hits)
            a=np.array([[1,0,0],[0,1,0],[-1,0,1]],float)
            xyz=np.linalg.solve(a,[*point,0])
            self.assertAlmostEqual(visible['nearest_depth'],max(1,xyz[2]))
            self.assertEqual(visible['nearest_ids'],[0] if xyz[2]>1 else [1])
        self.assertEqual(scene.painter(scene.intersections([[2.5,1.5]])[0],'full'),[1])

    def test_regular_subpixels_include_actual_centre(self):
        """🔎 Preserve fixed interior samples and exact centre without boundary jitter."""
        samples=M.sample_grid(8,9,C['grid'])
        self.assertEqual(len(samples),25)
        self.assertTrue(any(np.array_equal(point,[8.5,9.5]) for point in samples))
        self.assertTrue(np.all((samples>[8,9])&(samples<[9,10])))

    def test_projected_edge_witness_uses_actual_polygon_segments(self):
        """📍 Check exact inside and outside edge distances without a raster colour threshold."""
        case=C['cases'][0]
        scene=M.SurfaceScene(case['items'],C['towards'])
        self.assertAlmostEqual(M.boundary_witness(scene,0,[.5,.5])['projected_edge_distance_pixels'],.5)
        self.assertAlmostEqual(M.boundary_witness(scene,0,[-.1,1.5])['projected_edge_distance_pixels'],.1)
        self.assertAlmostEqual(M.boundary_witness(scene,0,[.5,.5])['extrapolated_plane_depth'],1)

if __name__=='__main__':
    unittest.main(verbosity=2)

"""🛋️ Validate the actual program furniture selected by the Tower B-to-C morph."""
import ast
import importlib
import json
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm as rh

HERE=Path(__file__).resolve().parent
ROOT=next(p for p in HERE.parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
CASE=json.loads((HERE/'program-morph.json').read_text(encoding='utf-8'))
MODEL=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')

class ProgramMorph(unittest.TestCase):
    def test_actual_scene_selects_study_sofa_without_reference_bed(self):
        tree=ast.parse((ROOT/CASE['scene']).read_text(encoding='utf-8'))
        selected={}
        for call in ast.walk(tree):
            if not isinstance(call,ast.Call) or getattr(call.func,'attr','')!='capsule_axon' or not call.args:
                continue
            argument=call.args[0]
            if not isinstance(argument,ast.Subscript) or not isinstance(argument.slice,ast.Name) or argument.slice.id not in CASE['programs']:
                continue
            mode=next((ast.literal_eval(kw.value) for kw in call.keywords if kw.arg=='interior'),'reference')
            selected[argument.slice.id]=mode
        self.assertEqual(selected,{key:CASE['interior'] for key in CASE['programs']})
        assembly=MODEL.capsule_assembly(CASE['programs']['new_key'],interior=selected['new_key'])
        roles={part.role for part in assembly.parts}
        expected=CASE['new_program']
        self.assertTrue(set(expected['required_roles'])<=roles)
        self.assertFalse(set(expected['forbidden_roles'])&roles)
        sofas=[part for part in assembly.parts if part.role=='sofa']
        self.assertGreaterEqual(len(sofas),expected['min_sofa_parts'])
        self.assertEqual(sum(part.solid.meta.get('primary_footprint',False) for part in sofas),expected['primary_sofas'])
        for part in sofas:
            mesh=rh.Mesh()
            for point in part.solid.vertices:
                mesh.Vertices.Add(*point)
            triangles=[]
            for face in part.solid.faces:
                for index in range(1,len(face)-1):
                    triangle=(face[0],face[index],face[index+1])
                    triangles.append(triangle)
                    mesh.Faces.AddFace(*triangle)
            self.assertTrue(mesh.IsClosed)
            self.assertTrue(mesh.IsValid)
            points=np.asarray(part.solid.vertices)
            tri=points[np.asarray(triangles)]
            volume=abs(float(np.einsum('ij,ij->i',tri[:,0],np.cross(tri[:,1],tri[:,2])).sum()/6))
            box=mesh.GetBoundingBox()
            extents=np.asarray([box.Max.X-box.Min.X,box.Max.Y-box.Min.Y,box.Max.Z-box.Min.Z])
            self.assertGreater(volume,0)
            self.assertAlmostEqual(volume,float(np.prod(extents)),places=6)

if __name__=='__main__':
    unittest.main(verbosity=2)

"""🧪 Verify canonical authoring imports without altering authored ordering."""
import ast
import importlib
import os
import sys
import hashlib
import json
from pathlib import Path
import unittest
TICKET=Path(__file__).resolve().parent
ROOT=Path('C:/git/semio')
CASES=json.loads((TICKET/'taxonomy-pipelines.tests.json').read_text(encoding='utf-8'))

sys.path.insert(0,str(ROOT))

class TaxonomyPipelineTests(unittest.TestCase):
    def test_trace_owners_are_distinct(self):
        modules=[importlib.import_module(path[:-3].replace("/",".")) for path in CASES["trace_owner_cases"]]
        self.assertNotEqual(modules[0].TRACE_MANIFEST,modules[1].TRACE_MANIFEST)
        for module in modules:
            owner=hashlib.sha256(module.BASE_DIR.relative_to(ROOT).as_posix().encode()).hexdigest()[:10]
            self.assertEqual(module.TRACE_MANIFEST,Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])/"trace"/owner)

    def test_canonical_sources(self):
        for row in CASES['modules']:
            with self.subTest(path=row['path']):
                text=(ROOT/row['path']).read_text(encoding='utf-8')
                self.assertIn(CASES['workspace_marker'],text)
                self.assertIn('import_module(',text)
                for token in CASES['forbidden']:
                    self.assertNotIn(token,text)

    def test_authored_order_and_narration_preserved(self):
        for row in CASES['modules']:
            tree=ast.parse((ROOT/row['path']).read_text(encoding='utf-8'))
            for label,expected in row['protected'].items():
                nodes=[node for node in tree.body if isinstance(node,ast.ClassDef) and node.name==label or isinstance(node,(ast.Assign,ast.AnnAssign)) and any(isinstance(target,ast.Name) and target.id==label for target in (node.targets if isinstance(node,ast.Assign) else [node.target]))]
                self.assertEqual(len(nodes),1,(row['path'],label))
                self.assertEqual(hashlib.sha256(ast.dump(nodes[0],include_attributes=False).encode()).hexdigest(),expected,(row['path'],label))

    def test_complete_module_inventory(self):
        for kind,count in CASES['counts'].items():
            self.assertEqual(sum(row['kind']==kind for row in CASES['modules']),count)

if __name__=='__main__':unittest.main()

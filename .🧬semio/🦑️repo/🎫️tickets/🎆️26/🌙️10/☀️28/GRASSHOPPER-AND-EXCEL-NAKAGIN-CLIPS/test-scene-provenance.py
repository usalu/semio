"""🧪 Validate scene-scoped provenance independently of native rendering."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import networkx as nx

HERE=Path(__file__).resolve().parent
(HERE/'🗑️generated').mkdir(parents=True,exist_ok=True)
CASE=json.loads((HERE/'scene-provenance.json').read_text(encoding='utf-8'))
SPEC=importlib.util.spec_from_file_location('scene_provenance',HERE/'scene-provenance.py')
P=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(P)

class Provenance(unittest.TestCase):
    def fixture(self,root):
        for name in (CASE['selected'],CASE['shared'],CASE['indirect'],CASE['unrelated']):
            imports=[target[:-3] for source,target in CASE['graph'] if source==name]
            (root/name).write_text(''.join('import '+name+'\n' for name in imports)+'value=1\n',encoding='utf-8')

    def test_declared_registry_inputs_seed_lazy_closure_before_execution(self):
        ast=__import__('ast')
        tree=ast.parse((HERE/'validate-scenes.py').read_text(encoding='utf-8'))
        helper=next((node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name=='registry_inputs'),None)
        self.assertIsNotNone(helper,'Native recorder must declare lazy registry inputs before baseline')
        namespace={'Path':Path,'json':json,'__file__':str(HERE/'validate-scenes.py')}
        root=next(path for path in HERE.parents if (path/'🧰️framework').is_dir())
        namespace['root']=root
        exec(compile(ast.Module(body=[helper],type_ignores=[]),'<registry-inputs>','exec'),namespace)
        case=CASE['registry_inputs']
        manifest=HERE/case['manifest']
        declared=namespace['registry_inputs'](case['series'])
        expected=[root/path for path in json.loads(manifest.read_text(encoding='utf-8'))]
        self.assertEqual(len(expected),case['count'])
        self.assertEqual(set(declared),{manifest,HERE/case['schema'],*expected})
        self.assertEqual(namespace['registry_inputs']('grasshopper'),())
        graph=nx.DiGraph((manifest,path) for path in expected)
        self.assertEqual(set(declared)-{manifest,HERE/case['schema']},nx.descendants(graph,manifest))
        function=next(node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name=='dependency_files')
        calls=[node for node in ast.walk(function) if isinstance(node,ast.Call) and isinstance(node.func,ast.Name) and node.func.id=='registry_inputs']
        self.assertEqual(len(calls),1)
        self.assertTrue(all(path.is_file() for path in declared))

    def test_recorder_compiles(self):
        ast=__import__('ast')
        candidate=HERE/'validate-scenes.py'
        ast.parse(candidate.read_text(encoding='utf-8'))

    def test_external_renderer_configuration_is_observed(self):
        with tempfile.TemporaryDirectory(dir=HERE/'🗑️generated') as directory:
            root=Path(directory)/'repo'
            root.mkdir()
            self.fixture(root)
            config=Path(directory)/CASE['external_config']['name']
            config.write_text(CASE['external_config']['before'],encoding='utf-8')
            paths=P.dependency_paths(root,[root/CASE['selected']],[config],[])
            start=P.fingerprints(root,paths)
            self.assertTrue(any(key.startswith('@external:') for key in start))
            config.write_text(CASE['external_config']['after'],encoding='utf-8')
            self.assertFalse(P.unchanged(start,P.fingerprints(root,paths)))

    def test_native_configuration_is_recorded(self):
        with tempfile.TemporaryDirectory(dir=HERE/'🗑️generated') as directory:
            path=Path(directory)/'harness.py'
            path.write_text('settings='+repr(CASE['render_settings'])+'\n',encoding='utf-8')
            self.assertEqual(P.render_configurations(path),[CASE['render_settings']])

    def test_closure_matches_networkx(self):
        with tempfile.TemporaryDirectory(dir=HERE/'🗑️generated') as directory:
            root=Path(directory)
            self.fixture(root)
            actual=P.dependency_paths(root,[root/CASE['selected']],[],[])
            graph=nx.DiGraph(CASE['graph'])
            expected={CASE['selected'],*nx.descendants(graph,CASE['selected'])}
            self.assertEqual({p.name for p in actual},expected)

    def test_unrelated_immunity_and_selected_shared_rejection(self):
        for mutation in CASE['mutations']:
            with self.subTest(file=mutation['file']),tempfile.TemporaryDirectory(dir=HERE/'🗑️generated') as directory:
                root=Path(directory)
                self.fixture(root)
                paths=P.dependency_paths(root,[root/CASE['selected']],[],[])
                start=P.fingerprints(root,paths)
                target=root/mutation['file']
                target.write_text(target.read_text(encoding='utf-8')+'changed=True\n',encoding='utf-8')
                end=P.fingerprints(root,paths)
                self.assertEqual(P.unchanged(start,end),mutation['accepted'])
                self.assertFalse(P.unchanged(start,{}))

    def test_manifest_binary_and_harness_inputs_are_observed(self):
        with tempfile.TemporaryDirectory(dir=HERE/'🗑️generated') as directory:
            root=Path(directory)
            self.fixture(root)
            asset=root/'palette.bin'
            asset.write_bytes(bytes([1,2,3]))
            harness=root/'harness.py'
            harness.write_text('enabled=True\n',encoding='utf-8')
            paths=P.dependency_paths(root,[root/CASE['selected']],[asset],[harness])
            start=P.fingerprints(root,paths)
            for target in (asset,harness):
                original=target.read_bytes()
                target.write_bytes(original+b' ')
                end=P.fingerprints(root,paths)
                self.assertEqual(P.unchanged(start,end),target==harness)
                target.write_bytes(original)
            harness.write_text('enabled=False\n',encoding='utf-8')
            self.assertFalse(P.unchanged(start,P.fingerprints(root,paths)))

if __name__=='__main__':
    unittest.main(verbosity=2)

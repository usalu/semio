"""🎞️ Check authored ordered-animation phase arguments before native execution."""
import ast
import json
from pathlib import Path
import unittest

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
CASE=json.loads(Path(__file__).with_name('animation-phases.json').read_text(encoding='utf-8'))

class AnimationPhases(unittest.TestCase):
    def test_authored_sequences(self):
        tree=ast.parse((ROOT/CASE['scene']).read_text(encoding='utf-8'))
        calls=[n for n in ast.walk(tree) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr==CASE['method']]
        self.assertGreater(len(calls),0)
        for call in calls:
            with self.subTest(line=call.lineno):
                self.assertIsInstance(call.args[0],(ast.Tuple,ast.List))
                for phase in call.args[CASE['subsequent_phases_start']:]:
                    self.assertIsInstance(phase,(ast.Tuple,ast.List))
                    self.assertEqual(len(phase.elts),len(CASE['phase_shape']))
                    self.assertIsInstance(phase.elts[0],(ast.Tuple,ast.List))

if __name__=='__main__':
    unittest.main(verbosity=2)

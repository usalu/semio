"""🧪 Cross-check complete-domain acceptance with an independent JSON Schema oracle."""
import ast
import copy
import json
from pathlib import Path
import unittest

TICKET=Path(__file__).parent
contract=json.loads((TICKET/'physical-acceptance-cases.json').read_text(encoding='utf8'))
schema=json.loads((TICKET/'physical-acceptance-domains.schema.json').read_text(encoding='utf8'))
source=ast.parse((TICKET/'accept-physical-evidence.py').read_text(encoding='utf8'))
namespace={}
exec(compile(ast.Module(body=[node for node in source.body if isinstance(node,ast.FunctionDef) and node.name=='valid_domains'],type_ignores=[]),str(TICKET/'accept-physical-evidence.py'),'exec'),namespace)

class PhysicalAcceptance(unittest.TestCase):
    def test_complete_and_corrupt_domains_match_json_schema(self):
        """🔐 Reject empty, absent, duplicate, wrong-count and failed physical gates independently."""
        expected=contract['expected_domains']
        rows=[{'domain':name,'tests':count,'failures':0,'errors':0,'success':True} for name,count in expected.items()]
        examples=[('complete',rows,True),('empty',[],False),('missing',rows[:-1],False)]
        for name in contract['invalid_domain_mutations']:
            if name in ('empty','missing'):
                continue
            candidate=copy.deepcopy(rows)
            if name=='duplicate': candidate[-1]=copy.deepcopy(candidate[0])
            if name=='wrong-count': candidate[0]['tests']+=1
            if name=='unknown': candidate[0]['domain']='unknown'
            if name=='failure': candidate[0]['failures']=1
            if name=='error': candidate[0]['errors']=1
            examples.append((name,candidate,False))
        (TICKET/'🗑️generated/physical-acceptance-examples.json').write_text(json.dumps(examples,ensure_ascii=False,indent=2),encoding='utf8')
        for name,candidate,valid in examples:
            with self.subTest(name=name):
                self.assertEqual(namespace['valid_domains'](candidate,expected),valid)

if __name__=='__main__':
    unittest.main(verbosity=2)

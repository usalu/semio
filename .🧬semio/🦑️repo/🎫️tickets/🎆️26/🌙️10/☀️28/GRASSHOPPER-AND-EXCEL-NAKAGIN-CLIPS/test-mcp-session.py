"""🧪 Exercise portable protocol deadlines and owned shutdown without launching a process."""
import ast
import json
from pathlib import Path
import queue
import subprocess
import tempfile
import time
from types import SimpleNamespace
import unittest

TICKET=Path(__file__).resolve().parent
C=json.loads((TICKET/'mcp-session-cases.json').read_text(encoding='utf-8'))

def function(name,namespace):
    """🔎 Load only a saved transport function, without executing its command entry point."""
    tree=ast.parse((TICKET/'repo-mcp-client.py').read_text(encoding='utf-8'))
    node=next((node for node in ast.walk(tree) if isinstance(node,ast.FunctionDef) and node.name==name),None)
    if node is None:
        raise AssertionError('Missing retained transport function: '+name)
    exec(compile(ast.Module(body=[node],type_ignores=[]),'<transport-function>','exec'),namespace)
    return namespace[name]

class Session(unittest.TestCase):
    def test_close_attempt_preserves_uncertainty_and_prevents_blind_retries(self):
        close=function('close_request',{'json':json,'time':time})
        for fixture in C['attempts']:
            with self.subTest(case=fixture['name']),tempfile.TemporaryDirectory(prefix='mcp-attempt-',dir=TICKET/'🗑️generated') as folder:
                calls=[]
                request={'name':'ticket_close','arguments':{'path':'test','summary':'Test only','files':[]}}
                attempt=Path(folder)/'attempt.json'
                def invoke(method,payload):
                    calls.append((method,payload))
                    if fixture['failure']:
                        raise TimeoutError('Reply was lost')
                    return {'content':[]}
                if fixture['failure']:
                    with self.assertRaises(TimeoutError):
                        close(request,attempt,invoke)
                else:
                    self.assertEqual(close(request,attempt,invoke),{'content':[]})
                record=json.loads(attempt.read_text(encoding='utf-8'))
                self.assertEqual(record['request'],request)
                self.assertEqual(record['outcome'],fixture['outcome'])
                if fixture['failure']:
                    self.assertEqual(record.get('failure',{}).get('type'),fixture['failure'])
                with self.assertRaisesRegex(RuntimeError,'inspect.*ticket state'):
                    close(request,attempt,invoke)
                self.assertEqual(len(calls),1)
        with tempfile.TemporaryDirectory(prefix='mcp-reject-',dir=TICKET/'🗑️generated') as folder:
            attempt=Path(folder)/'attempt.json'
            with self.assertRaises(ValueError):
                close({'name':C['prohibited_tool']},attempt,lambda *args:self.fail('Prohibited operation invoked'))
            self.assertFalse(attempt.exists())

    def test_notification_stream_cannot_extend_one_response_deadline(self):
        for fixture in C['deadlines']:
            with self.subTest(case=fixture['name']):
                clock=[0.0]
                pending=list(fixture['events'])
                def get(timeout):
                    delay,message=pending.pop(0)
                    if delay>timeout:
                        clock[0]+=timeout
                        raise queue.Empty()
                    clock[0]+=delay
                    return message
                namespace={'serial':0,'send':lambda *args:None,'incoming':SimpleNamespace(get=get),'time':SimpleNamespace(monotonic=lambda:clock[0]),'queue':queue}
                call=function('call',namespace)
                if fixture['outcome']=='timeout':
                    with self.assertRaises((queue.Empty,TimeoutError)):
                        call('tools/list',{})
                else:
                    self.assertEqual(call('tools/list',{}),{'ok':True})
                self.assertAlmostEqual(clock[0],fixture['elapsed'])

    def test_shutdown_reaps_only_the_owned_child_after_bounded_escalation(self):
        for fixture in C['shutdown']:
            with self.subTest(case=fixture['name']):
                events=[]
                remaining=[fixture['timeouts']]
                def wait(timeout):
                    events.append('wait:'+str(timeout))
                    if remaining[0]:
                        remaining[0]-=1
                        raise subprocess.TimeoutExpired('owned-test-child',timeout)
                    return 0
                child=SimpleNamespace(stdin=SimpleNamespace(close=lambda:events.append('close')),wait=wait,terminate=lambda:events.append('terminate'),kill=lambda:events.append('kill'))
                function('shutdown',{'subprocess':subprocess})(child)
                self.assertEqual(events,fixture['expected'])

if __name__=='__main__':
    unittest.main(verbosity=2)

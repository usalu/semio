"""🧪 Verify real worker/grandchild cancellation and source-drift cleanup through independent OS facts."""
import importlib.util,hashlib,json,os,subprocess,sys,time,unittest
from pathlib import Path
T=Path(__file__).resolve().parent;R=next(p for p in T.parents if (p/'🧰️framework').is_dir());O=T/'🗑️generated/capture-process-tests';O.mkdir(parents=True,exist_ok=True)
C=json.loads((T/'capture-process-cases.json').read_text(encoding='utf8'))
try:
    spec=importlib.util.spec_from_file_location('capture_worker_process',T/'capture-worker-process.py');M=importlib.util.module_from_spec(spec);spec.loader.exec_module(M)
except BaseException as exc:
    (O/'import-red.json').write_text(json.dumps({'success':False,'stage':'import','error':repr(exc)}));raise
class SourceDrift(RuntimeError):
    """🧬 A deliberate generated-input drift with no explicit stop request."""
def process_rows():
    """🔬 Independently query the existing portable repository doctor process port."""
    output=O/'independent-processes.json'
    command=['repo','tools','exec','--light','bun','--',str(T/'probe-capture-processes.ts'),str(output)]
    subprocess.run(command,cwd=R,check=True,timeout=30)
    return json.loads(output.read_text(encoding='utf8'))['rows']
class CaptureProcesses(unittest.TestCase):
    def test_success_and_silent_progress(self):
        case=C['cases'][0];events=[];record=O/'success-pid.json'
        code=M.run_worker(R,T,O,T/'capture-process-fixture.py','capture-test-success',events.append,arguments=['--mode','success','--record',str(record)],interval=.03)
        self.assertEqual(code,0);self.assertGreaterEqual(len(events),case['minimum_checkpoints']);self.assertEqual(events[-1]['phase'],'complete')
    def test_keyboard_and_source_drift_stop_real_owned_tree(self):
        for case in C['cases'][1:]:
            with self.subTest(case=case['id']):
                label='capture-test-'+case['id'];worker=O/(case['id']+'-worker.json');leaf=O/(case['id']+'-leaf.json');marker=O/(case['id']+'-source.txt');marker.write_text('before');before=hashlib.sha256(marker.read_bytes()).hexdigest();events=[]
                for stale in (worker,leaf):stale.unlink(missing_ok=True)
                refused=False
                def checkpoint(event):
                    nonlocal refused
                    events.append(event)
                    if leaf.exists() and not refused:
                        bad=O/(label+'-foreign.request.json');audit=O/(label+'-foreign.tree.json')
                        bad.write_text(json.dumps({'root_pid':event['pid'],'parent_pid':os.getpid()+1,'label':label,'started_at':time.time()-event['elapsed_seconds'],'record':str(audit)}),encoding='utf8')
                        result=subprocess.run(['repo','tools','exec','--light','bun','--',str(T/'stop-capture-worker.ts'),'--request',str(bad)],cwd=R,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=30)
                        self.assertNotEqual(result.returncode,0)
                        rejected=json.loads(audit.read_text(encoding='utf8'));self.assertFalse(rejected['success']);self.assertEqual(rejected['signalled'],[])
                        pids={json.loads(worker.read_text(encoding='utf8'))['pid'],json.loads(leaf.read_text(encoding='utf8'))['pid']}
                        self.assertTrue(pids.issubset({row['pid'] for row in process_rows()}));refused=True
                    if len(events)>=case['minimum_checkpoints'] and leaf.exists():
                        if case['id']=='keyboard':raise KeyboardInterrupt('fixture cancellation')
                        marker.write_text('after')
                        if hashlib.sha256(marker.read_bytes()).hexdigest()!=before:raise SourceDrift('fixture source changed')
                kind=KeyboardInterrupt if case['id']=='keyboard' else SourceDrift
                began=time.monotonic()
                with self.assertRaises(kind):
                    M.run_worker(R,T,O,T/'capture-process-fixture.py',label,checkpoint,arguments=['--mode','tree','--record',str(worker),'--leaf-record',str(leaf),'--leaf-label',label+'-leaf'],interval=.03)
                self.assertLess(time.monotonic()-began,45)
                self.assertGreaterEqual(len(events),case['minimum_checkpoints'])
                pids={json.loads(worker.read_text(encoding='utf8'))['pid'],json.loads(leaf.read_text(encoding='utf8'))['pid']}
                observed=process_rows();self.assertFalse(pids&{row['pid'] for row in observed})
                cleanup=json.loads((O/(label+'.tree.json')).read_text(encoding='utf8'));self.assertTrue(cleanup['success']);self.assertEqual(cleanup['remaining'],[]);self.assertGreaterEqual(len(cleanup['initial_descendants']),2)
    def test_exited_root_held_pipe_is_explicitly_unverified(self):
        from unittest.mock import patch
        import threading
        release=threading.Event()
        class Pipe:
            def __iter__(self):
                release.wait(3)
                return iter(())
            def close(self):pass
        class Exited:
            pid=987654321
            stdout=Pipe()
            def poll(self):return 0
        began=time.monotonic()
        with patch.object(M.subprocess,'Popen',return_value=Exited()) as launch:
            with self.assertRaisesRegex(M.CaptureWorkerError,'stdout held open') as caught:
                M.run_worker(R,T,O,T/'capture-process-fixture.py','capture-test-held-pipe',lambda event:None,interval=.03)
            self.assertEqual(launch.call_count,1)
        self.assertLess(time.monotonic()-began,6)
        audit=json.loads((O/'capture-test-held-pipe.tree.json').read_text(encoding='utf8'))
        self.assertFalse(audit['success']);self.assertFalse(audit['cleanup_verified']);self.assertIsNone(audit['remaining']);self.assertEqual(audit['reason'],'exited-root-held-pipe');self.assertTrue(caught.exception.__notes__)
    def test_input_scope_rejects_before_launch(self):
        from unittest.mock import patch
        for worker,output in [(R/'AGENTS.md',O),(T/'capture-process-fixture.py',T)]:
            with patch.object(M.subprocess,'Popen') as launch:
                with self.assertRaises(ValueError):M.run_worker(R,T,output,worker,'capture-test-invalid',lambda event:None)
                launch.assert_not_called()
if __name__=='__main__':
    result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(CaptureProcesses))
    (O/'result.json').write_text(json.dumps({'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors)}));raise SystemExit(0 if result.wasSuccessful() else 1)

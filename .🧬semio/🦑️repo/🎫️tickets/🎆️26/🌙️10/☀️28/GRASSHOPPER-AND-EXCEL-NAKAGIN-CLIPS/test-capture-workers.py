"""🧪️ Check fresh-worker cancellation and exact gallery composition with Pillow."""
from datetime import datetime,timedelta,timezone
import hashlib
import io
import json
from pathlib import Path
import runpy
import shutil
import tempfile
import unittest
from unittest.mock import patch
from PIL import Image

TICKET=Path(__file__).resolve().parent
PLAN=json.loads((TICKET/'capture-workers.json').read_text(encoding='utf-8'))
CASES=json.loads((TICKET/'capture-worker-cases.json').read_text(encoding='utf-8'))
RUN=runpy.run_path

class Workers(unittest.TestCase):
    def exercise(self,*,failure=0,drift=0,invalid_record=None):
        parent=TICKET/'🗑️generated'
        parent.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(dir=parent,prefix='capture-test-') as directory:
            output=Path(directory)
            record_directory=output/'records'
            record_directory.mkdir()
            calls=[]
            checkpoints=[]
            def checkpoint():
                checkpoints.append(len(checkpoints)+1)
                if len(checkpoints)==drift:
                    raise RuntimeError('Frozen capture inputs changed')
            class Child:
                def __init__(self,command,**options):
                    calls.append(command)
                    worker=PLAN['workers'][len(calls)-1]
                    self.code=1 if len(calls)==failure else 0
                    self.stdout=io.StringIO('[DEBUG] Fixture child\n')
                    self.pid=700000+len(calls)
                    colour=CASES['panel_colours'][int(worker['name'][-1])] if worker['name'].startswith('direction') else (0,0,0)
                    for name in worker['outputs']:
                        Image.new('RGB',(1920,1080),tuple(colour)).save(output/name)
                    label='fc-'+worker['name']
                    observed=json.loads(json.dumps(values['capture_runtime']))
                    observed['config'].update(media_dir=str(TICKET/'🗑️generated'/label/'media'),text_dir=str(TICKET/'🗑️generated'/label/'text'))
                    result={'success':True,'unchanged':True,'target':worker['script'],'arguments':worker['arguments'],'baseline_sha256':hashlib.sha256((record_directory/'start.json').read_bytes()).hexdigest(),'source_start':values['capture_source'],'source_end':values['capture_source'],'runtime_start':observed,'runtime_end':observed}
                    if invalid_record=='wrong_target':
                        result['target']='unplanned.py'
                    if invalid_record=='wrong_arguments':
                        result['arguments']=['--unplanned']
                    if invalid_record=='wrong_baseline':
                        result['baseline_sha256']='0'*64
                    if invalid_record=='changed_source':
                        result.update(source_start={'unexpected.py':'different'},source_end={'unexpected.py':'different'})
                    if invalid_record=='changed_runtime':
                        observed['packages']['fixture']='2'
                    if invalid_record=='wrong_cache':
                        observed['config']['text_dir']=str(TICKET/'🗑️generated'/'wrong-cache')
                    child_record=record_directory/('child-'+worker['name'])
                    child_record.mkdir()
                    (child_record/'result.json').write_text(json.dumps(result),encoding='utf-8')
                def __enter__(self):
                    return self
                def __exit__(self,*args):
                    return False
                def poll(self):
                    return self.code
                def wait(self,timeout=None):
                    return self.code
            runtime={'python':'fixture','executable':'fixture','platform':'fixture','packages':{'fixture':'1'},'config':{key:'fixture' for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate')}}
            values={'capture_checkpoint':checkpoint,'capture_generated':output,'capture_output':record_directory,'capture_started':(datetime.now(timezone.utc)-timedelta(seconds=1)).isoformat(),'capture_source':{'fixture':'immutable'},'capture_runtime':runtime}
            (record_directory/'start.json').write_text(json.dumps({'sources':values['capture_source'],'runtime':runtime}),encoding='utf-8')
            with patch('subprocess.Popen',Child),patch('runpy.run_path',return_value={}):
                if failure or drift or invalid_record:
                    with self.assertRaises(RuntimeError):
                        RUN(str(TICKET/'final-capture.py'),run_name='__main__',init_globals=values)
                else:
                    RUN(str(TICKET/'final-capture.py'),run_name='__main__',init_globals=values)
            if not failure and not drift and not invalid_record:
                self.assertEqual(len(calls),CASES['workers'])
                with Image.open(output/PLAN['gallery']['output']) as image:
                    for index,colour in enumerate(CASES['panel_colours']):
                        x=(PLAN['gallery']['boundaries'][index]+PLAN['gallery']['boundaries'][index+1])//2
                        self.assertEqual(image.getpixel((x,540)),tuple(colour))
                self.assertGreaterEqual(len(checkpoints),2*CASES['workers']+2)
                for command in calls:
                    self.assertEqual(command[:4],[shutil.which('repo'),'tools','exec','uv'])
            return len(calls)

    def test_all_fresh_workers_and_exact_panel_pixels(self):
        self.exercise()

    def test_failed_child_prevents_remaining_workers(self):
        self.assertEqual(self.exercise(failure=CASES['failure_worker']),CASES['failure_worker'])

    def test_changed_sources_prevent_the_next_child(self):
        self.assertEqual(self.exercise(drift=CASES['drift_checkpoint']),1)

    def test_actual_child_identity_must_match_before_any_next_worker(self):
        for invalid in CASES['invalid_records']:
            with self.subTest(invalid=invalid):
                self.assertEqual(self.exercise(invalid_record=invalid),1)

if __name__=='__main__':
    unittest.main(verbosity=2)

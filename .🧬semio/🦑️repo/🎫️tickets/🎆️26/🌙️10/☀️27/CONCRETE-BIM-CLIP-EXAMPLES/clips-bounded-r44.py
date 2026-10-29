"""🧪 Bounded native contract checks without final-source preparation."""
import hashlib,json,runpy,time,unittest
from pathlib import Path
ROOT=Path('/repo')
CLIP=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
TICKET=Path(__file__).parent
source=CLIP/'🧱️model/🔣️.json'
before=hashlib.sha256(source.read_bytes()).hexdigest()
suite=unittest.TestSuite()
for module in ('🪜️stair-parts','🕸️mesh-clipping'):
    namespace=runpy.run_path(str(CLIP/'🧱️model/🧪️tests'/f'{module}.py'))
    suite.addTests(unittest.FunctionTestCase(value) for name,value in namespace.items() if name.startswith('test_') and callable(value))
service=runpy.run_path(str(CLIP/'🧱️model/🧪️tests/🚰️services.py'))
for method in ('test_three_native_axes_preserve_hollow_material_and_exact_endpoints','test_every_representation_keeps_native_material_and_hollow_cross_sections','test_invalid_wall_and_nonorthogonal_axis_are_rejected','test_polyhedral_faces_reject_open_or_nonplanar_shells'):
    suite.addTest(service['ServiceProfileContract'](method))
bearing=runpy.run_path(str(CLIP/'🧱️model/🧪️tests/🏗️bearing.py'))
suite.addTest(bearing['RoofPostBearingContract']('test_a_structural_reserve_need_not_reach_the_separate_finish'))
progress=TICKET/'🗑️generated/clips-bounded-r44.json'
phases=[]
class Result(unittest.TestResult):
    """⏳ Publish progress and exact failures after every bounded native check."""
    def write(self,phase,test):
        progress.write_text(json.dumps({'debug':'[DEBUG] bounded native contracts','phase':phase,'test':str(test),'completed':len(phases),'planned':planned,'failures':[(str(t),m) for t,m in self.failures],'errors':[(str(t),m) for t,m in self.errors]}))
    def startTest(self,test):
        super().startTest(test)
        self.started=time.monotonic()
        self.write('started',test)
    def stopTest(self,test):
        phases.append((str(test),time.monotonic()-self.started))
        self.write('completed',test)
        super().stopTest(test)
planned=suite.countTestCases()
result=Result()
suite.run(result)
after=hashlib.sha256(source.read_bytes()).hexdigest()
print('BOUNDED',planned,result.testsRun,len(result.failures),len(result.errors),sum(s for _,s in phases),before,after)
for test,message in result.failures+result.errors:
    print(str(test),message)

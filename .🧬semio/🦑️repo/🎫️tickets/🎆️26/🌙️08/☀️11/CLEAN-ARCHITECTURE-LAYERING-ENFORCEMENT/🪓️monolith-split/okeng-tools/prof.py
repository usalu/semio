# ⏱️ Profiles the kernel-source rewrite of the store root for a fixed time (signal alarm) and prints the top cumulative functions.
import sys, os, cProfile, pstats, signal, io, builtins, time
sys.argv = ['a2x.py', 'b1', 'dump']
import a2x
from a2x import *
a2x.prepare()
crates = read_list(f'{OKD}/b1.crates')
reloc = [l.strip() for l in open(f'{OKD}/b1.reloc', encoding='utf8').read().split('\n') if l.strip()]
carve.P.Plan = a2lib.PredictPlan
b = KBatch(crates, relocate=sorted(reloc))
log('plan built')
pr = cProfile.Profile()
def stop(*a): raise KeyboardInterrupt
signal.signal(signal.SIGALRM, stop); signal.alarm(int(os.environ.get('SECS', '240')))
pr.enable()
try: b.build_kernel_source(ROOT)
except KeyboardInterrupt: log('interrupted')
pr.disable()
s = io.StringIO(); pstats.Stats(pr, stream=s).sort_stats('cumulative').print_stats(35); print(s.getvalue()[:6000])

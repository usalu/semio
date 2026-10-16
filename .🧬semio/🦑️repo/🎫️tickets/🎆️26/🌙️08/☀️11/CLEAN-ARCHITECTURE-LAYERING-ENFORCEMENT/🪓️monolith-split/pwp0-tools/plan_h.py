# 📋️ Regenerates the table of Appendix H of PLAN-PLUGIN-CRATES.md from pwp0-crates.json (idempotent; ranks keep their phase-0 values, so retired crates leave gaps).
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import crates as C, plan_appendix as A
crates = json.load(open(C.GEN + '/pwp0-crates.json'))
t = open(C.PLAN, encoding='utf-8').read()
a = t.index('## Appendix H.'); b = t.index('## Appendix I.')
blk = t[a:b]
i = blk.index('| rank | crate |')
new = blk[:i] + A.appendix_h() + '\n\n'
new = re.sub(r'\n80 crates\.', '\n%d crates (80 at phase 0; `tests-08` and `tests-09` retired and the dependency deltas of WP1, WP3, WP4 applied by the closing pass of 12.6).' % len(crates), new, count=1) if '\n80 crates.' in new else new
t = t[:a] + new + t[b:]
open(C.PLAN, 'w', encoding='utf-8').write(t)
print('Appendix H regenerated:', len(crates), 'crates')

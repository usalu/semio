"""🔍 Retain exact source endpoints and independent necessary-coverage regression results."""
import hashlib
import io
import json
from pathlib import Path
import runpy
import unittest

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
OUTPUT=TICKET/'🗑️generated/coverage-witness'
OUTPUT.mkdir(parents=True,exist_ok=True)
FILES=[BASE/'🎨️visuals/🐍️.py',BASE/'🧪️tests/👁️occlusion/🐍️.py',BASE/'🧪️cases/👁️occlusion/🔣️.json',BASE/'🧪️cases/👁️occlusion/🔣️.schema.json',Path(__file__)]
def fingerprint():
    """🪪 Record exact owned implementation, contract and harness bytes."""
    return {str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=fingerprint()
module=runpy.run_path(str(BASE/'🧪️tests/👁️occlusion/🐍️.py'))
names=['test_expanded_cover_bound_contains_independent_line_intersections','test_expanded_single_cover_bbox_skips_only_impossible_halfplanes','test_area_first_blockers_preserve_exact_union_identity','test_single_cover_matches_convex_numpy_and_pillow_oracles','test_single_cover_shortcut_preserves_strict_depth_and_opacity','test_coverage_witness_matches_expanded_numpy_halfplanes','test_coverage_witness_retains_holes_and_skips_only_impossible_clipping','test_opaque_union_coverage_preserves_real_holes_partial_faces_and_glass','test_opaque_coverage_does_not_change_actual_cairo_pixels','test_unknown_opacity_is_not_an_occluder']
stream=io.StringIO()
result=unittest.TextTestRunner(stream=stream,verbosity=2).run(unittest.TestSuite(module['Occlusion'](name) for name in names))
after=fingerprint()
(OUTPUT/'tests.log').write_text(stream.getvalue(),encoding='utf-8')
record={'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors),'before':before,'after':after,'unchanged':before==after}
(OUTPUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(stream.getvalue())
print('[DEBUG] Coverage witness source endpoints unchanged:',record['unchanged'])
raise SystemExit(0 if record['success'] and record['unchanged'] else 1)

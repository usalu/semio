"""usage: patch-silence-test.py <silence/tests file> — (1) scratch images of this test come from the per-run sandbox (testScratchRootV1), never from an inherited TMPDIR: a single-file run whose TMPDIR lies inside a managed native parent mounted `tmp/silence-*/seeded.dmg` there and killed a peer's build (course-f m1 01:32); (2) the host's fseventsd adds its own page files to the service directory of a freshly attached volume (252 vs 250 seeded): the seeded count is a lower bound with a small host allowance, silencing still has to leave 0."""
import sys
p=sys.argv[1];s=open(p).read()
def swap(a,b):
    global s
    assert s.count(a)==1,a
    s=s.replace(a,b)
swap('const scratchRoot=()=>realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"silence-")));','const scratchRoot=()=>realpathSync(mkdtempSync(join(testScratchRootV1("silence"),"silence-")));')
swap('import{tmpdir}from"node:os";','import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";')
swap('const before=oracle(join(mine,protocol.serviceDirectory));expect(before.files).toBe(250);','const before=oracle(join(mine,protocol.serviceDirectory));expect(before.files).toBeGreaterThanOrEqual(250);expect(before.files).toBeLessThan(266);')
open(p,'w').write(s)

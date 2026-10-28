"""⏱️ agents16: the estimates oracle and its law learn that a reservation backed by the class's own measured peaks takes no ramp slot (anchored, idempotent).
Usage: python3 -I oracle_ramp.py <root>"""
import os, sys
T = os.path.join(sys.argv[1], "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/")
FILES = {
 "🐍️oracle/🐍️.py": [
  ('def ramp_request(base, scaled, estimate):\n    shrunk = scaled["fixedMegabytes"] < base["fixedMegabytes"] or scaled["perJobMegabytes"] < base["perJobMegabytes"]\n    return {**scaled, "rampMilliseconds": estimate["rampMilliseconds"], "rampSlot": shrunk}',
   'def ramp_request(base, scaled, estimate, measured=False):\n    shrunk = scaled["fixedMegabytes"] < base["fixedMegabytes"] or scaled["perJobMegabytes"] < base["perJobMegabytes"]\n    return {**scaled, "rampMilliseconds": estimate["rampMilliseconds"], "rampSlot": shrunk and not measured}'),
  ('        ramps.append({"base": base, "scaled": scaled_case, "estimate": estimate_case, "expect": ramp_request(base, scaled_case, estimate_case)})\n',
   '        ramps.append({"base": base, "scaled": scaled_case, "estimate": estimate_case, "expect": ramp_request(base, scaled_case, estimate_case)})\n    ramps += [{**item, "measured": True, "expect": ramp_request(item["base"], item["scaled"], item["estimate"], True)} for item in ramps[:16]]\n'),
 ],
 "🟦️.ts": [
  ("for(const item of fixture.ramps)expect(rampHostRequestV1(item.base as HostRequestV1,item.scaled as HostRequestV1,item.estimate as HostEstimateV1),JSON.stringify(item.estimate)).toEqual(item.expect as HostRequestV1);",
   "for(const item of fixture.ramps)expect(rampHostRequestV1(item.base as HostRequestV1,item.scaled as HostRequestV1,item.estimate as HostEstimateV1,(item as {measured?:boolean}).measured??false),JSON.stringify(item.estimate)).toEqual(item.expect as HostRequestV1);"),
  ('expect(fixture.ramps.some(item=>item.expect.rampSlot)&&fixture.ramps.some(item=>!item.expect.rampSlot),"vectors reach both slot verdicts").toBe(true);',
   'expect(fixture.ramps.some(item=>item.expect.rampSlot)&&fixture.ramps.some(item=>!item.expect.rampSlot),"vectors reach both slot verdicts").toBe(true);\n expect(fixture.ramps.some(item=>(item as {measured?:boolean}).measured===true&&(item.scaled.fixedMegabytes<item.base.fixedMegabytes||item.scaled.perJobMegabytes<item.base.perJobMegabytes)&&!item.expect.rampSlot),"a shrunk reservation backed by the class\'s own measured peaks takes no ramp slot").toBe(true);'),
 ],
}
for rel, edits in FILES.items():
    path = T + rel; s = open(path, encoding="utf8").read()
    for old, new in edits:
        if new in s:
            continue
        if s.count(old) != 1:
            print("ANCHOR", s.count(old), rel, old[:70]); sys.exit(1)
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)
print("ramp oracle and law edited")

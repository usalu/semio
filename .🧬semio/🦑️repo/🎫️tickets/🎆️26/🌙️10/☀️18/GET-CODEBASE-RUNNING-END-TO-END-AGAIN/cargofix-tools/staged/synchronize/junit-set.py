"""🧮 Prints the sorted set of failing test names of one bun junit file (streamed): junit-set.py <junit.xml>"""
import sys, xml.etree.ElementTree as ET
passed=failed=0; names=[]
for _event, el in ET.iterparse(sys.argv[1], events=("end",)):
    if el.tag == "testcase":
        bad = any(child.tag in ("failure", "error") for child in el)
        if bad: failed += 1; names.append((el.get("classname") or "")[-60:] + " :: " + (el.get("name") or "")[:140])
        elif not any(child.tag == "skipped" for child in el): passed += 1
        el.clear()
print("pass", passed, "fail", failed)
for n in sorted(names): print("FAIL", n)

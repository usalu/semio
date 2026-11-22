"""🔍 Map sampled guest call sites to the exact rebuilt core and its embedded Rust locations."""
from collections import Counter
import importlib.util
import json
import mmap
from pathlib import Path
import re
import sys

source, binary, trace = map(Path, sys.argv[1:4])
module_spec = importlib.util.spec_from_file_location("pdf_wasm_locations", source)
assert module_spec is not None and module_spec.loader is not None
decoder = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(decoder)
text = trace.read_text(encoding="utf8")
samples = []
for value in text.split("[DEBUG] pdf-wasm-sample ")[1:]:
    stack = re.split(r"\n(?:\[|\d+ \|)", value, maxsplit=1)[0]
    frames = [(int(number),int(offset,16)) for number,offset in re.findall(r"wasm-function\[(\d+)\]:0x([0-9a-f]+)",stack)]
    if frames:
        samples.append(frames)
assert samples
weights = Counter(number for frames in samples for number in set(number for number, _ in frames))
with binary.open("rb") as file, mmap.mmap(file.fileno(),0,access=mmap.ACCESS_READ) as module:
    ranges = decoder.sections(module)
    data = decoder.memory_ranges(module,ranges[11])
    cursor, end = ranges[10]
    count, cursor = decoder.leb(module,cursor)
    desired = sorted(set(frame for frames in samples for frame in frames),key=lambda pair:pair[1])
    bodies = {}
    skips = []
    for ordinal in range(count):
        size,cursor=decoder.leb(module,cursor)
        stop=cursor+size
        while desired and desired[0][1]<stop:
            number,address=desired.pop(0)
            if address<cursor:
                skips.append((number,hex(address)))
            else:
                bodies[number,address]=(cursor,stop)
        cursor=stop
    assert cursor==end
    print("[DEBUG] Unmapped return addresses",skips,desired)
    locations = {}
    for (number, address),(begin,end) in bodies.items():
        if number not in locations:
            sites = decoder.candidate_sites(module,data,begin,end)
            locations[number] = [name for name in sites if "/registry/src/" not in name and "/rustc/" not in name]
    for number, count in weights.most_common():
        print("[DEBUG]", number, count, json.dumps(locations.get(number,[])[:15],ensure_ascii=False))
    for frames in samples:
        print("[DEBUG] Stack", " ".join(str(number) for number,_ in frames))

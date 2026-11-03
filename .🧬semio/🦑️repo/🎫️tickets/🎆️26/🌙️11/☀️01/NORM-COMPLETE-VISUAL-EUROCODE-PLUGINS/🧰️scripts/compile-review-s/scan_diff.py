"""🔎 Diff schema: patch field types vs. model row fields, KeyOf coverage, nested_patch! section order/duplicates, scalar-patch rows."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
MODEL = re.compile(r'semio_s_artifact_norm_en%s_model::' % code)
def norm(t): return re.sub(r'\s+', '', MODEL.sub("", t)).replace("semio_s_plugin_norm_compliance::", "").replace("std::string::", "")
def last(t):
    return re.sub(r'(\w+::)+', '', t)

out = []
CNT = [0]
alldiff = "".join(src for f, src in files.items() if "🔺️diff" in f)
ORDER = None
mm = re.search(r'macro_rules!\s+nested_patch\s*\{\s*\(.*?\$name:ident for \$row:ty \{(.*?)\}\)\s*=>', alldiff, re.S)
if mm: ORDER = re.findall(r'\b(set|within|optional|rows)\s*\{', mm.group(1))
keyed_all = set(re.findall(r'KeyOf<(?:[\w]+::)*(\w+)>', alldiff)) | set(re.findall(r'\bfor\s+(\w+);', alldiff))
for f, src in files.items():
    if "🔺️diff" not in f and "/diff" not in f: continue
    if "/🧪️tests/" in f: continue
    keyed = keyed_all

    def cmp(pname, row, fields, line):
        cands = [d for _, d in structs.get(row, [])]
        if not cands: return
        rf = max(cands, key=lambda d: len(set(d) & set(fields)))
        for fld, ty in fields.items():
            if fld not in rf: continue
            CNT[0] += 1
            ty = norm(re.sub(r'^Option<(.*)>$', r'\1', ty))
            if last(norm(rf[fld])) != last(ty) and norm(rf[fld]) != ty:
                out.append((f, line, f"{pname}.{fld}: patch type `{ty}` vs model `{norm(rf[fld])}`"))
    for m in re.finditer(r'\bscalar_patch!\s*\{', src):
        e = match_close(src, m.end() - 1)
        hm = re.search(r'(\w+)\s+for\s+((?:\w+::)*)(\w+)\s*\{', src[m.end():e])
        if hm:
            b = m.end() + hm.end() - 1
            cmp(hm.group(1), hm.group(3), fields_of(src[b + 1:match_close(src, b)]), src.count("\n", 0, m.start()) + 1)
    for m in re.finditer(r'RowPatch<(?:\w+::)*(\w+)>\s+for\s+(\w+)', src):
        row, pname = m.group(1), m.group(2)
        sm = re.search(r'\bstruct\s+%s\s*\{' % pname, src)
        if sm:
            cmp(pname, row, fields_of(src[sm.end():match_close(src, sm.end() - 1)]), src.count("\n", 0, sm.start()) + 1)
    for m in re.finditer(r'\b(row_patch|nested_patch)!\s*\{', src):
        e = match_close(src, m.end() - 1)
        body = src[m.end():e]
        hm = re.search(r'(\w+)\s+(?:of|for)\s+((?:\w+::)*(\w+))\s*\{', body)
        if not hm: continue
        pname, row = hm.group(1), hm.group(3)
        line = src.count("\n", 0, m.start()) + 1
        rowfields = [d for _, d in structs.get(row, [])]
        if not rowfields:
            out.append((f, line, f"{pname}: row type {row} not found in artifact")); continue
        rf = max(rowfields, key=lambda d: len(set(d) & set(re.findall(r'(\w+)\s*:', body))))
        inner = body[hm.end() - 1:]
        ie = match_close(inner, 0)
        sec = inner[1:ie]
        if m.group(1) == "nested_patch":
            names = re.findall(r'\b(set|within|optional|rows)\s*\{', sec)
            order = ORDER or ["set", "within", "optional", "rows"]
            if names != order: out.append((f, line, f"{pname}: nested_patch sections {names} != {order}"))
        for sm in re.finditer(r'\b(set|within|optional|rows)\s*\{', sec):
            se = match_close(sec, sm.end() - 1)
            sbody = sec[sm.end():se]
            for part in split_top(sbody):
                part = part.strip()
                if not part: continue
                if sm.group(1) == "set":
                    pm = re.match(r'(\w+)\s*:\s*(.+)$', part, re.S)
                    if not pm: continue
                    fld, ty = pm.group(1), norm(pm.group(2))
                    CNT[0] += 1
                    if fld not in rf: out.append((f, line, f"{pname}.{fld} not a field of {row}")); continue
                    if last(norm(rf[fld])) != last(ty) and norm(rf[fld]) != ty:
                        out.append((f, line, f"{pname}.{fld}: patch type `{ty}` vs model `{norm(rf[fld])}`"))
                elif sm.group(1) == "rows":
                    pm = re.match(r'(\w+)\s*:\s*(.+?)\s*=\s*"', part, re.S)
                    if pm and pm.group(1) not in rf: out.append((f, line, f"{pname} rows lane {pm.group(1)} not a field of {row}"))
    # scalar-patch lists: patch struct fields in derive style (generated row_patch: set {...} handled above)
    for m in re.finditer(r'list_delta!\s*\{', src):
        e = match_close(src, m.end() - 1)
        b = src[m.end():e]
        rm = re.search(r'row:\s*((?:\w+::)*)(\w+)\s*,\s*patch:\s*(\w+)', b)
        if rm and rm.group(2) not in keyed and not re.search(r'by\s+\w+', b):
            out.append((f, src.count("\n", 0, m.start()) + 1, f"list_delta row {rm.group(2)} lacks KeyOf"))
        elif rm and rm.group(2) not in keyed:
            out.append((f, src.count("\n", 0, m.start()) + 1, f"list_delta row {rm.group(2)}: no KeyOf<{rm.group(2)}> impl in this file (check `by` key)"))
    km = re.search(r'row_key!\s*\{(.*?)\n\}', src, re.S)
    if km:
        keys = re.findall(r'(\w+)\s+for\s+(\w+);', km.group(1))
        have = {r for _, r in keys}
        dup = [r for r in have if [x for _, x in keys].count(r) > 1]
        for r in dup: out.append((f, 0, f"row_key! duplicate {r}"))
        for m in re.finditer(r'row:\s*(?:\w+::)*(\w+)\s*,', src):
            if m.group(1) not in have and m.group(1) not in keyed:
                out.append((f, src.count("\n", 0, m.start()) + 1, f"row {m.group(1)} has no row_key!/KeyOf entry"))
for f, l, msg in sorted(set(out)):
    print(f"{rel(f)}:{l}: {msg}")
print(f"# {len(set(out))} findings, {CNT[0]} set fields compared, order={ORDER}", file=sys.stderr)

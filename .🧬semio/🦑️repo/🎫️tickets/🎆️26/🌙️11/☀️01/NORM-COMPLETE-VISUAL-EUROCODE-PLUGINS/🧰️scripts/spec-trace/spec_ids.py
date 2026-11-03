# 📚 Spec ids per EN code from the target specs (same extraction as the coverage audit), with the heading line of each id.
import re, os, json, sys, collections
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
SPEC = os.path.join(T, "📐️specs")
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"

def lines_of(name):
    return open(os.path.join(SPEC, name), encoding="utf-8").read().split("\n")

ID_START = re.compile(r"^\s*(?:\*\*|#{1,6}\s+|\|\s*|-\s+(?:\*\*)?|\*\s+(?:\*\*)?)?\s*(ID)(?![A-Za-z0-9])")
def ids_from(name, lo, hi, rx, heading_all=False):
    L = lines_of(name)
    out = collections.OrderedDict()
    for i in range(lo - 1, min(hi, len(L))):
        s = L[i]
        if heading_all and s.lstrip().startswith("#"):
            for m in re.finditer(rx, s):
                out.setdefault(m.group(0), i + 1)
            continue
        m = re.match(r"^\s*(?:\*\*|#{1,6}\s+|\|\s*|-\s+(?:\*\*)?|\*\s+(?:\*\*)?)?\s*(" + rx + r")(?![A-Za-z0-9])", s)
        if m:
            out.setdefault(m.group(1), i + 1)
    return out

def en1997_ids():
    L = lines_of("en1997-en1998-target.md")
    out = collections.OrderedDict()
    for i in range(87, 331):
        s = L[i]
        if not s.lstrip().startswith("#"):
            continue
        for m in re.finditer(r"\b([A-Z]+(?:-[A-Z]+)?)-(\d{2})\s*(?:…|\.\.\.|\-|–)\s*\1?-?(\d{2})\b", s):
            for k in range(int(m.group(2)), int(m.group(3)) + 1):
                out.setdefault(f"{m.group(1)}-{k:02d}", i + 1)
        for m in re.finditer(r"\b((?:G|GEO-F|EQU|UPL|HYD|SLS|PILE|RW|SLOPE|ANC|DEW)(?:-\d{2}|\d{2}))\b", s):
            out.setdefault(m.group(1), i + 1)
    return out

def en1997_fix(d):
    o = collections.OrderedDict()
    for k, v in d.items():
        o.setdefault(k, v)
    return o

SPECS = {
 "en1990": ("en1990-en1991-target.md", lambda: ids_from("en1990-en1991-target.md", 41, 168, r"E90\.\d{2}"), "⚖️en1990"),
 "en1991": ("en1990-en1991-target.md", lambda: ids_from("en1990-en1991-target.md", 169, 350, r"E1\d\.\d{2}"), "🏋️en1991"),
 "en1992": ("en1992-target.md", lambda: ids_from("en1992-target.md", 19, 431, r"[A-Z]{1,3}-\d{2,3}"), "🏛️en1992"),
 "en1993": ("en1993-target.md", lambda: ids_from("en1993-target.md", 27, 297, r"V-\d{3}[a-z]?"), "🔩️en1993"),
 "en1994": ("en1994-en1999-target.md", lambda: ids_from("en1994-en1999-target.md", 41, 277, r"C-\d{2}"), "🧩️en1994"),
 "en1999": ("en1994-en1999-target.md", lambda: ids_from("en1994-en1999-target.md", 278, 430, r"A-\d{2}"), "🪶️en1999"),
 "en1995": ("en1995-en1996-target.md", lambda: ids_from("en1995-en1996-target.md", 20, 489, r"T-\d{2,3}"), "🪵️en1995"),
 "en1996": ("en1995-en1996-target.md", lambda: ids_from("en1995-en1996-target.md", 490, 745, r"M-\d{2}"), "🪨️en1996"),
 "en1997": ("en1997-en1998-target.md", lambda: en1997_fix(ids_from("en1997-en1998-target.md", 88, 331, r"(?:G-\d{2}|(?:GEO-F|EQU-|UPL-|HYD-|SLS-|PILE-|RW-|SLOPE-|ANC-|DEW-)\d{2})", heading_all=True)), "🌍️en1997"),
 "en1998": ("en1997-en1998-target.md", lambda: ids_from("en1997-en1998-target.md", 332, 574, r"SEI-\d{2}"), "🫨️en1998"),
}


def spec_ids(code):
    return list(SPECS[code][1]().items())

def heading(code, line_no):
    return lines_of(SPECS[code][0])[line_no - 1]

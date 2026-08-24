#!/usr/bin/env python3
"""🩹 Give the derived R12 fixture's symbol tables the UNIQUE names a DXF symbol table must have.

Wave 7 derived `🖊️bus-shelter-r12.dxf` by writing it with the `dxf` 0.6 crate, whose `normalize()`
inserts its own `LAYER "0"`, `STYLE "STANDARD"`/`"ANNOTATIVE"` and `LTYPE
"BYBLOCK"`/`"BYLAYER"`/`"CONTINUOUS"` records — and the derivation then added the source drawing's
OWN `0`, `STANDARD` and `CONTINUOUS`.  The committed fixture therefore carries three exact-name
collisions (`LAYER ["0","0","DIMS"]`, `STYLE […,"STANDARD","STANDARD",…]`, two `CONTINUOUS`
linetypes), which its own feature description does not claim and which the DXF reference forbids:
a symbol table's names are its keys.  Every name-keyed layer/style/linetype mutation was refused
against it, and `set-snapshot` still is.

The collision is resolved by dropping the `normalize()`-added record and keeping the derivation's
own, which is the one carrying real content (`CONTINUOUS` description "Solid line", `STANDARD`
text height 2.5).  Nothing else in the file is touched.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14.
"""
PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf"

with open(PATH, "rb") as handle:
    raw = handle.read().decode("latin-1")
newline = "\r\n" if "\r\n" in raw else "\n"
lines = raw.split(newline)
stripped = [line.strip() for line in lines]

# 🗑️ (table, handle) of every record this repair drops — each the crate's own normalize() default,
# named identically to a record the derivation itself wrote with the real content.
DROP = {("LTYPE", "8"), ("LAYER", "7"), ("STYLE", "D")}

out, index, dropped = [], 0, []
while index < len(stripped):
    if stripped[index] == "0" and index + 3 < len(stripped) and stripped[index + 1] == "TABLE":
        table = stripped[index + 3]
        end = index
        while not (stripped[end] == "0" and stripped[end + 1] == "ENDTAB"):
            end += 1
        cursor = index
        while cursor < end:
            if stripped[cursor] == "0" and stripped[cursor + 1] == table and cursor > index:
                stop = cursor + 2
                while stop < end and not (stripped[stop] == "0" and stripped[stop + 1] == table):
                    stop += 1
                record = lines[cursor:stop]
                handle = next((stripped[k + 1] for k in range(cursor, stop) if stripped[k] == "5"), None)
                name = next((stripped[k + 1] for k in range(cursor, stop) if stripped[k] == "2"), None)
                if (table, handle) in DROP:
                    dropped.append((table, name, handle))
                else:
                    out.extend(record)
                cursor = stop
            else:
                out.append(lines[cursor])
                cursor += 1
        index = cursor
        continue
    out.append(lines[index])
    index += 1

assert len(dropped) == 3, dropped
with open(PATH, "wb") as handle:
    handle.write(newline.join(out).encode("latin-1"))
print("dropped", dropped)

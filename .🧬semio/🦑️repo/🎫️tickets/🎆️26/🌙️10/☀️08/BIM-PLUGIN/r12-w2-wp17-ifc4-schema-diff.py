import sys
import ifcopenshell.ifcopenshell_wrapper as w

old = w.schema_by_name("IFC2X3")
new = w.schema_by_name("IFC4")


def attrs(schema, name):
    try:
        return [(a.name(), a.optional()) for a in schema.declaration_by_name(name).as_entity().all_attributes()]
    except Exception:
        return None


for name in sys.argv[1:]:
    a, b = attrs(old, name), attrs(new, name)
    if a is None or b is None:
        print(name, "2x3" if a else "-", "4" if b else "-", a, b)
    elif a != b:
        print(name, "DIFF")
        print("  2x3", ["%d:%s%s" % (i, n, "?" if o else "") for i, (n, o) in enumerate(a)])
        print("  4  ", ["%d:%s%s" % (i, n, "?" if o else "") for i, (n, o) in enumerate(b)])
    else:
        print(name, "same", len(a))

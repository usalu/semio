import sys
import ifcopenshell.ifcopenshell_wrapper as w

schema = w.schema_by_name("IFC2X3")
for name in sys.argv[1:]:
    d = schema.declaration_by_name(name)
    attrs = [(a.name(), a.optional()) for a in d.as_entity().all_attributes()]
    print(name, len(attrs), ["%d:%s%s" % (i, a, "?" if o else "") for i, (a, o) in enumerate(attrs)])

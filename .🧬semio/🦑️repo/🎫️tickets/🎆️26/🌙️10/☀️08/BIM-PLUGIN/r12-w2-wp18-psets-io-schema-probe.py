import sys
import ifcopenshell.ifcopenshell_wrapper as w

schema = w.schema_by_name(sys.argv[1])
for name in sys.argv[2:]:
    d = schema.declaration_by_name(name)
    if hasattr(d, "enumeration_items"):
        print(name, list(d.enumeration_items()))
        continue
    entity = d.as_entity()
    print(name, "abstract" if entity.is_abstract() else "")
    for index, a in enumerate(entity.all_attributes()):
        print("  %d %s %s%s" % (index, a.name(), str(a.type_of_attribute())[:80], " ?" if a.optional() else ""))

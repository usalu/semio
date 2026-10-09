import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
old = "                    serializer_entry::<ModelSnapshot, export::csv::ModelIntoCsv>(BIM_MODEL_DIALECT),\n"
assert s.count(old) == 1
open(p + ".new", 'w', encoding='utf-8', newline='').write(s.replace(old, old + "                    serializer_entry::<ModelSnapshot, export::json::ModelIntoJson>(BIM_MODEL_DIALECT),\n"))

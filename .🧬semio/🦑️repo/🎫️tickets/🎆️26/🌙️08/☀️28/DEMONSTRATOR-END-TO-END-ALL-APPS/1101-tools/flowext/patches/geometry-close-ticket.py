G="✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs"
s=open(G).read()
a="semio_framework_value::geometry_close_ticket("
assert s.count(a)==1
open(G,"w").write(s.replace(a,"geometry_close_ticket("))

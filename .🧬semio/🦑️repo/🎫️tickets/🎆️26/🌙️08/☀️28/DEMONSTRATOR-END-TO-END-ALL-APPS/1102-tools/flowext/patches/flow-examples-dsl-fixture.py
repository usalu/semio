F="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📚️examples/🗣️.dsl.semio"
open(F,"w").write("""semio flow.flow.dsl v1
schema=flow.host_snapshot
camera {
  x=0 y=0 zoom=1
}
layout={ }
synapses [id:TEXT link:WIRE] {
  s1 slider@number->add@a
  s2 add@sum->preview
}
widgets {
  input-slider id=slider label=Number value=3 min=0 max=10 step=0.1
  neuron id=add neuron-kind=math.add preview=true input-ports=[ ] output-ports=[ ]
  output-preview id=preview expanded=[ ]
}
""")

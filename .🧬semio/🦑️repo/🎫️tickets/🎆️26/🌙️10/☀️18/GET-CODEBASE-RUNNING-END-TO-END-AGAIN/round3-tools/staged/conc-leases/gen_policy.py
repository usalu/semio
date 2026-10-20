import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.join(HERE,"staged","🧰️framework","🔨️modules","🏃️process","🔒️leases","🧭️order")
def cls(id,rank,scope,owner,en,de,*match,**extra):
    row={"id":id,"rank":rank,"scope":scope,"owner":owner,"title":{"en":en,"de":de},"match":[{"kind":m[0],"value":m[1]} for m in match]}
    row.update(extra);return row
P=lambda v:("prefix",v)
X=lambda v:("exact",v)
classes=[
 cls("compiler-capacity",1,"machine","compiler-capacity","Compiler capacity gate, picker and core slots","Compiler-Kapazitätstor, Auswahl und Kernplätze",X("semio.compiler.host-capacity/v1"),X("semio.compiler.cpu-picker/v1"),P("semio.compiler.cpu/v1/")),
 cls("tool-singleton",2,"checkout","build-tools","Tool activation, download and provisioning singletons","Singletons für Werkzeug-Aktivierung, -Download und -Bereitstellung",X("nx-tooling-activation"),X("nx-tool-download-cache"),X("nx-tooling-provision"),P("nx-tooling:"),P("nx-authorizer-"),P("nx-sdk-snapshot:"),X("bazel-metrics"),X("bazel-snapshot-restore"),P("bazel-"),P("bun-"),P("rustup:"),X("cache-prune"),X("tools"),P("tools:"),P("repository:selected-tooling:"),P("selected-rust-")),
 cls("cargo-coordination",2,"crate","cargo","Cargo registry, preparation, source admission and build leases","Cargo-Registry, -Vorbereitung, -Quellzulassung und -Build-Leases",P("cargo:"),P("cargo-"),P('["semio.cargo.')),
 cls("graph",3,"checkout","graph","Graph pools, writers, publications and daemons","Graph-Pools, -Schreiber, -Veröffentlichungen und -Daemons",P("nx:"),P("repository-graph-"),P("build-graph-"),P("runtime-graph:"),P("repository:graph-"),X("repository-build-reports")),
 cls("namespace",4,"project","namespaces","Per-project and per-crate namespaces, bindings and descriptors","Namespaces, Bindungen und Deskriptoren je Projekt und Crate",P("rust-code"),P("rust:"),P("rust-"),P("javascript-dependency-"),P("typescript-package-binding:"),P("styling-python:"),P("storybook-"),P("vite-"),P("native-snapshot"),P("native-artifact-")),
 cls("artifact",5,"path","artifacts","Artifact paths, scratch and capture","Artefaktpfade, Scratch und Erfassung",P("artifact:"),P("artifact-scratch:"),P("artifact-capture:")),
 cls("image",6,"generation","storage","Fixed images, native storage and persistent state","Feste Images, nativer Speicher und persistenter Zustand",P("native-fixed-image:"),P("native-storage:"),P("persistent-state:"),P("persistent-data:"),P("repository-cache-charge:"),P("repo:compiler-charge:")),
]
policy={
 "version":1,
 "rules":{"outOfOrder":"observe","holdAndWait":"enforce","selfDeadlock":"enforce","cycle":"enforce"},
 "holdAndWait":{"graceMilliseconds":15000,"pollMilliseconds":250},
 "cycle":{"intervalMilliseconds":500},
 "registry":{"flushMilliseconds":100,"maximumEntryBytes":65536,"maximumViolations":200},
 "fairness":{"queuedWithResourceLeases":True},
 "gate":{"releaseWhileWaiting":True,"pollInitialMilliseconds":20,"pollMaximumMilliseconds":250},
 "classes":classes,
 "allowed":[
  {"from":"compiler-capacity","to":"tool-singleton","reason":{"en":"A capacity slot is taken for the command that then activates tools.","de":"Ein Kapazitätsplatz wird für den Befehl genommen, der danach Werkzeuge aktiviert."}},
  {"from":"compiler-capacity","to":"graph","reason":{"en":"Capacity is taken before the graph is asked.","de":"Kapazität wird vor der Anfrage an den Graphen genommen."}}
 ]
}
os.makedirs(OUT,exist_ok=True)
open(os.path.join(OUT,"🔣️policy.json"),"w",encoding="utf8").write(json.dumps(policy,ensure_ascii=False,indent=2)+"\n")
print("policy written",len(classes))

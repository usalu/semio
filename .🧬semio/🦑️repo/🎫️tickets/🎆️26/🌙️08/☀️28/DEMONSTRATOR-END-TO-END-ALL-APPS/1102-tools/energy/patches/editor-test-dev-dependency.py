#!/usr/bin/env python3
"""Fail-closed patch (repo root): the energy editor lib-test target #[path]-includes the initialization tests, which use semio_framework_trace::observe_heap_allocations_on_this_thread and the HeapWitness global allocator; declare it as a dev-dependency (same path as the initialization crate). Manifest edit: tell the coordinator for `dependencies lock`."""
P="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml"
s=open(P).read()
a="[dev-dependencies]\nsemio-s-artifact-energy-model-import = { workspace = true }\n"
assert s.count(a)==1 and "semio-framework-trace" not in s
dep='semio-framework-trace = { path = "../../../../../../../../../../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust" }\n'
open(P,'w').write(s.replace(a,a+dep));print("applied")

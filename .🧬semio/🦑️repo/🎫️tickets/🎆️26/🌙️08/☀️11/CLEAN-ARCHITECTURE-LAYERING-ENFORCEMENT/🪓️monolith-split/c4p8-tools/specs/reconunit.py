SPEC = {
 "orig": "reconunit.rs.txt",
 "file": "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "📥️ingest-gate",
   "mod": "ingest_gate",
   "ranges": [
    [
     5,
     85
    ]
   ],
   "doc": "//! 📥️ Frame ingest tests of the staged reconstruction engine: the blur gate, the stride and frame cap and the video push counts with the flat and checker frame fixtures. See [`super`]."
  },
  {
   "folder": "🧊️synthetic-scene",
   "mod": "synthetic_scene",
   "ranges": [
    [
     89,
     313
    ]
   ],
   "doc": "//! 🧊️ Synthetic scene of the staged reconstruction engine tests: the vector helpers, the textured cube renderer, the orbiting frames and the tiny engine parameters. See [`super`]."
  },
  {
   "folder": "🔗️pipeline-contracts",
   "mod": "pipeline_contracts",
   "ranges": [
    [
     315,
     653
    ]
   ],
   "doc": "//! 🔗️ Pipeline contract tests of the staged reconstruction engine: the chunking invariance, the orbit registration, the preview and frame minimum, the end-to-end contract and the bounded step ceilings. See [`super`].",
   "reexport": False
  },
  {
   "folder": "🔎️orbit-diagnostics",
   "mod": "orbit_diagnostics",
   "ranges": [
    [
     655,
     1206
    ]
   ],
   "doc": "//! 🔎️ Orbit diagnostics of the staged reconstruction engine: the ignored stage trace, geometry audit and dense audit against the ground truth of the synthetic orbit. See [`super`].",
   "reexport": False
  },
  {
   "folder": "♻️dense-replay",
   "mod": "dense_replay",
   "ranges": [
    [
     1208,
     1477
    ]
   ],
   "doc": "//! ♻️ Dense replay diagnostic of the staged reconstruction engine: the dense dump codec and the replay of a dumped dense stage through fusion and meshing. See [`super`]."
  }
 ],
 "external": [],
 "patch": {
  "♻️dense-replay": [
   [
    "    pub(super) fn encode(",
    "    pub(in super::super) fn encode("
   ]
  ]
 }
}

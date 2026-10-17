SPEC = {
 "orig": "gen2d.rs.txt",
 "file": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "🛰️owner-and-io",
   "mod": "owner_and_io",
   "ranges": [
    [
     127,
     301
    ]
   ],
   "doc": "//! 🛰️ Instance operation owner and media I/O of the generation2d editor: the flow session close, the command categorisation, the owner with its retirement and maintenance steps and the typed media ports. See [`super`]."
  },
  {
   "folder": "🧵️retained-commands",
   "mod": "retained_commands",
   "ranges": [
    [
     341,
     763
    ]
   ],
   "doc": "//! 🧵️ Retained command route of the generation2d editor: the bounded, preview and flow-eval tool ids, the three command works and the bounded job factory. See [`super`]."
  },
  {
   "folder": "🎁️contributions-route",
   "mod": "contributions_route",
   "ranges": [
    [
     768,
     957
    ]
   ],
   "doc": "//! 🎁️ Contributions route of the generation2d editor: the contributions contract, its work and job factory and the factory proofs. See [`super`]."
  },
  {
   "folder": "📬️store-preparation",
   "mod": "store_preparation",
   "ranges": [
    [
     962,
     1293
    ]
   ],
   "doc": "//! 📬️ Store preparation of the generation2d editor: the artifact and config one-item publication factories with their admission and preparation. See [`super`]."
  },
  {
   "folder": "🎞️reserved-import",
   "mod": "reserved_import",
   "ranges": [
    [
     1298,
     1606
    ]
   ],
   "doc": "//! 🎞️ Reserved import job of the generation2d editor: the media import job with its `InteractiveJob` implementation. See [`super`]."
  },
  {
   "folder": "🕹️play-app-editor",
   "mod": "play_app_editor",
   "ranges": [
    [
     1610,
     2137
    ]
   ],
   "doc": "//! 🕹️ Play app of the generation2d editor: the play app type and its `ArtifactEditor` implementation. See [`super`]."
  },
  {
   "folder": "🖱️graph-interaction-and-render",
   "mod": "graph_interaction_and_render",
   "ranges": [
    [
     2345,
     2432
    ]
   ],
   "doc": "//! 🖱️ Graph interaction and window bodies of the generation2d editor: the resolved interaction marks and the single body-key render. See [`super`]."
  }
 ],
 "external": [
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️generation2d-config-preparation-laws/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs"
 ],
 "keep_empty": [
  "🧪️UnitTests"
 ],
 "patch_parent": [
  [
   "use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};\n#![allow(clippy::result_large_err)]\n",
   "#![allow(clippy::result_large_err)]\nuse semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};\n"
  ]
 ]
}

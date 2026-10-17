SPEC = {
 "orig": "7.rs.txt",
 "file": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "🧰️seismic-helpers",
   "mod": "seismic_helpers",
   "ranges": [
    [
     67,
     97
    ]
   ],
   "doc": "//! 🧰️ Helpers of the EN 1998 inference: the quantity and localisation constructors, the id selector and the annex resolution. See [`super`]."
  },
  {
   "folder": "🌍️full-seismic-check",
   "mod": "full_seismic_check",
   "ranges": [
    [
     99,
     1071
    ]
   ],
   "doc": "//! 🌍️ The full seismic check of the EN 1998 inference: the clause-by-clause evaluation across the claimed parts. See [`super`]."
  },
  {
   "folder": "⚖️part-evaluators",
   "mod": "part_evaluators",
   "ranges": [
    [
     1073,
     1454
    ]
   ],
   "doc": "//! ⚖️ Part evaluators of the EN 1998 inference: the integrity checks and the bridge, assessment, silo, foundation and tower evaluations. See [`super`]."
  }
 ],
 "external": [
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"
 ],
 "patch_parent": [
  [
   "rust: include_str!(\"🦀️.rs\"),",
   "rust: concat!(include_str!(\"🦀️.rs\"), include_str!(\"🧰️seismic-helpers/🦀️.rs\"), include_str!(\"🌍️full-seismic-check/🦀️.rs\"), include_str!(\"⚖️part-evaluators/🦀️.rs\")),"
  ]
 ]
}

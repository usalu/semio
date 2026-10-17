SPEC = {
 "orig": "2.rs.txt",
 "file": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "🏗️beam-rows",
   "mod": "beam_rows",
   "ranges": [
    [
     21,
     368
    ]
   ],
   "doc": "//! 🏗️ Beam sections of the EN 1994 diff: the beam action rows and the keyed beam row patches. See [`super`]."
  },
  {
   "folder": "🏛️column-rows",
   "mod": "column_rows",
   "ranges": [
    [
     370,
     661
    ]
   ],
   "doc": "//! 🏛️ Column sections of the EN 1994 diff: the column action rows and the keyed column row patches. See [`super`]."
  },
  {
   "folder": "🧱️slab-rows",
   "mod": "slab_rows",
   "ranges": [
    [
     663,
     954
    ]
   ],
   "doc": "//! 🧱️ Slab sections of the EN 1994 diff: the slab action rows and the keyed slab row patches. See [`super`]."
  },
  {
   "folder": "🔁️apply-and-inverse",
   "mod": "apply_and_inverse",
   "ranges": [
    [
     983,
     1077
    ]
   ],
   "doc": "//! 🔁️ Apply and inverse of the EN 1994 diff: the central applier hook and the algebra that returns the negative diff. See [`super`]."
  }
 ],
 "external": [],
 "patch_parent": [
  [
   "rust: include_str!(\"🦀️.rs\"),",
   "rust: concat!(include_str!(\"🦀️.rs\"), include_str!(\"🏗️beam-rows/🦀️.rs\"), include_str!(\"🏛️column-rows/🦀️.rs\"), include_str!(\"🧱️slab-rows/🦀️.rs\"), include_str!(\"🔁️apply-and-inverse/🦀️.rs\")),"
  ]
 ]
}

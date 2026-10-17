SPEC = {
 "orig": "1.rs.txt",
 "file": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "🩹️list-row-patches",
   "mod": "list_row_patches",
   "ranges": [
    [
     21,
     442
    ]
   ],
   "doc": "//! 🩹️ Positional list deltas of the ISO 16757 diff: the keyed sparse patches and row deltas of the catalogue lists (product groups, classes, series, products, indexes, property definitions, subjects). See [`super`]."
  },
  {
   "folder": "🗂️keyed-row-sections",
   "mod": "keyed_row_sections",
   "ranges": [
    [
     444,
     744
    ]
   ],
   "doc": "//! 🗂️ Keyed row sections of the ISO 16757 diff: the geometry objects, selection constraints and part number inputs rows and the script limits patch. See [`super`]."
  },
  {
   "folder": "🔁️apply-and-inverse",
   "mod": "apply_and_inverse",
   "ranges": [
    [
     746,
     807
    ],
    [
     852,
     1033
    ]
   ],
   "doc": "//! 🔁️ Apply and inverse of the ISO 16757 diff: the single id value patches, the central applier hook and the algebra that returns the negative diff. See [`super`]."
  }
 ],
 "external": [],
 "patch_parent": [
  [
   "rust: include_str!(\"🦀️.rs\"),",
   "rust: concat!(include_str!(\"🦀️.rs\"), include_str!(\"🩹️list-row-patches/🦀️.rs\"), include_str!(\"🗂️keyed-row-sections/🦀️.rs\"), include_str!(\"🔁️apply-and-inverse/🦀️.rs\")),"
  ]
 ]
}

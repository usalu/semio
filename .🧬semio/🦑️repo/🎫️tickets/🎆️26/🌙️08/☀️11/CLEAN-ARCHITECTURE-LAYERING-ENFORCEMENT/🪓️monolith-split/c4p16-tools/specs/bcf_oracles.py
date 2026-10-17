SPEC = {
 "orig": "bcf_oracles.rs.txt",
 "file": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles/🦀️.rs",
 "nest": False,
 "children": [],
 "external": [],
 "unwrap": [
  {
   "name": "oracles",
   "folder": "🧰️engine",
   "doc": "//! 🧰️ Independent BCF 2.1 oracle engine: the model, the XML tree, the zip archive, the markup and visualization XML, the JSON codec and the forward and inverse mutations. See [`super`].",
   "children": [
    {
     "folder": "📐️model-and-xml",
     "mod": "model_and_xml",
     "ranges": [
      [
       49,
       331
      ]
     ],
     "doc": "//! 📐️ Model and XML of the BCF oracle: the independent BCF document model, the XML tree, its parser and writer and the archive entries. See [`super`]."
    },
    {
     "folder": "💬️markup-and-viewpoint-xml",
     "mod": "markup_and_viewpoint_xml",
     "ranges": [
      [
       335,
       579
      ]
     ],
     "doc": "//! 💬️ Markup and viewpoint XML of the BCF oracle: the markup.bcf and visualization info parse and write. See [`super`]."
    },
    {
     "folder": "🧬️document-codec-and-json",
     "mod": "document_codec_and_json",
     "ranges": [
      [
       583,
       879
      ]
     ],
     "doc": "//! 🧬️ Document codec and JSON of the BCF oracle: the archive decode and encode and the JSON value mapping of the model. See [`super`]."
    },
    {
     "folder": "🦠️forward-and-inverse",
     "mod": "forward_and_inverse",
     "ranges": [
      [
       883,
       1159
      ]
     ],
     "doc": "//! 🦠️ Forward and inverse of the BCF oracle: the mutation kinds applied to the model and their computed inverse specs. See [`super`]."
    },
    {
     "folder": "🧭️routing-and-projection",
     "mod": "routing_and_projection",
     "ranges": [
      [
       1163,
       1247
      ]
     ],
     "doc": "//! 🧭️ Routing and projection of the BCF oracle: the mutation entry points, the round trip and the semantic projection. See [`super`]."
    }
   ]
  }
 ],
 "expect_missing": [
  "mod oracles {",
  "}"
 ]
}

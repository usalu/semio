SPEC = {
 "orig": "document.rs.txt",
 "file": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "📕️pdf-spec-and-oracles",
   "mod": "pdf_spec_and_oracles",
   "ranges": [
    [
     13,
     239
    ]
   ],
   "doc": "//! 📕️ PDF oracles of the document test oracle: the PDF spec, the creation and editing oracles and the independent projection. See [`super`]."
  }
 ],
 "external": [],
 "unwrap": [
  {
   "name": "ooxml",
   "folder": "🗂️ooxml",
   "doc": "//! 🗂️ The OOXML conformance-class engine of the document test oracle: the OPC container, the XML edits, the content types, the package facts, the wire form and the conformance mutations. See [`super`].",
   "children": [
    {
     "folder": "📦️container-and-xml-edits",
     "mod": "container_and_xml_edits",
     "ranges": [
      [
       272,
       562
      ]
     ],
     "doc": "//! 📦️ Container and XML edits of the OOXML engine: the independent zip read and write and the root element edits. See [`super`]."
    },
    {
     "folder": "🏷️content-types-and-package-facts",
     "mod": "content_types_and_package_facts",
     "ranges": [
      [
       567,
       805
      ]
     ],
     "doc": "//! 🏷️ Content types and package facts of the OOXML engine: the content-type map, the main part, the relationship types and the declared namespaces. See [`super`]."
    },
    {
     "folder": "🔌️package-wire-and-reading",
     "mod": "package_wire_and_reading",
     "ranges": [
      [
       807,
       1124
      ]
     ],
     "doc": "//! 🔌️ Package wire and reading of the OOXML engine: the logical XML parts, their arena and wire form and the independent reading. See [`super`]."
    },
    {
     "folder": "🏅️conformance-mutations-and-projection",
     "mod": "conformance_mutations_and_projection",
     "ranges": [
      [
       1128,
       1408
      ]
     ],
     "doc": "//! 🏅️ Conformance mutations and projection of the OOXML engine: the profile, the namespace and relationship rewrites, the arrange and inverse specs and the projection. See [`super`]."
    }
   ]
  },
  {
   "name": "pdf_conformance",
   "folder": "🧷️pdf-conformance",
   "doc": "//! 🧷️ The PDF conformance-class engine of the document test oracle: the profile, the object-graph scans and builders, the forward mutations and the arrange, inverse and projection halves. See [`super`].",
   "children": [
    {
     "folder": "🔎️profile-container-and-scans",
     "mod": "profile_container_and_scans",
     "ranges": [
      [
       1443,
       1795
      ]
     ],
     "doc": "//! 🔎️ Profile, container and scans of the PDF engine: the conformance profile, the lopdf loading, the object-graph scans and the object builders. See [`super`]."
    },
    {
     "folder": "⚙️forward-mutation",
     "mod": "forward_mutation",
     "ranges": [
      [
       1799,
       2186
      ]
     ],
     "doc": "//! ⚙️ Forward mutation of the PDF engine: the conformance mutation entry and its in-place application. See [`super`]."
    },
    {
     "folder": "🔁️arrange-inverse-and-projection",
     "mod": "arrange_inverse_and_projection",
     "ranges": [
      [
       2191,
       2419
      ]
     ],
     "doc": "//! 🔁️ Arrange, inverse and projection of the PDF engine: the arrange spec, the inverse spec and the semantic projection. See [`super`]."
    }
   ]
  }
 ],
 "expect_missing": [
  "pub mod ooxml {",
  "pub mod pdf_conformance {",
  "}",
  "}"
 ]
}

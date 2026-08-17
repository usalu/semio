#!/usr/bin/env python3
"""Mechanical sweep: rewrite cc1-cc6 subset analyzer/builder/composer files that consumed
StepSnapshot.document directly, now that StepSnapshot has typed header/entities fields instead.
Uses StepSnapshot::to_part21_document()/from_part21_document() as the conversion boundary.
Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, F4 step agent.
"""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets")
SUBSETS = ["✳️cc1", "✳️cc2", "✳️cc3", "✳️cc4", "✳️cc5", "✳️cc6"]

changed = []

for n in SUBSETS:
    # --- analyzer ---
    f = ROOT / n / "🧐️analyzer" / "🦀️component.rs"
    text = f.read_text()
    orig = text
    text = text.replace(
        "    let doc = &snapshot.document;\n",
        "    let doc = snapshot.to_part21_document();\n",
    )
    text = text.replace("file_schema_contains(doc,", "file_schema_contains(&doc,")
    text = text.replace("ladder_violations(doc,", "ladder_violations(&doc,")
    text = text.replace("has_product_definition_chain(doc)", "has_product_definition_chain(&doc)")
    text = text.replace(
        "StepSnapshot { document: base_doc(), ..StepSnapshot::default() }",
        "StepSnapshot::from_part21_document(base_doc())",
    )
    text = text.replace(
        "StepSnapshot { document: doc, ..StepSnapshot::default() }",
        "StepSnapshot::from_part21_document(doc)",
    )
    text = text.replace(
        "retained lossless `Part21Document` graph (`StepSnapshot.document`)",
        "retained lossless `Part21Document` graph (`StepSnapshot::to_part21_document()`)",
    )
    text = text.replace(
        "checks against the retained lossless `Part21Document` graph (`StepSnapshot.document`). CC1 is",
        "checks against the retained lossless `Part21Document` graph (`StepSnapshot::to_part21_document()`). CC1 is",
    )
    if text != orig:
        f.write_text(text)
        changed.append(str(f))

    # --- composer ---
    f = ROOT / n / "🎹️composer" / "🦀️component.rs"
    text = f.read_text()
    orig = text
    text = text.replace(
        "        let mut snapshot = inner.snapshot;\n        ensure_file_schema(&mut snapshot.document, \"AUTOMOTIVE_DESIGN\");\n",
        "        let mut snapshot = inner.snapshot;\n        let mut doc = snapshot.to_part21_document();\n        ensure_file_schema(&mut doc, \"AUTOMOTIVE_DESIGN\");\n        snapshot = StepSnapshot::from_part21_document(doc);\n",
    )
    text = text.replace(
        "<StepSnapshot as store::ArtifactPack>::encode_pack(&StepSnapshot { document: doc, ..StepSnapshot::default() })",
        "<StepSnapshot as store::ArtifactPack>::encode_pack(&StepSnapshot::from_part21_document(doc))",
    )
    text = text.replace(
        "file_schema_contains(&composed.snapshot.document, \"AUTOMOTIVE_DESIGN\")",
        "file_schema_contains(&composed.snapshot.to_part21_document(), \"AUTOMOTIVE_DESIGN\")",
    )
    if text != orig:
        f.write_text(text)
        changed.append(str(f))

    # --- builder ---
    f = ROOT / n / "🏗️builder" / "🦀️component.rs"
    text = f.read_text()
    orig = text
    text = text.replace(
        "        StepSnapshot {\n            document: Part21Document {",
        "        StepSnapshot::from_part21_document(Part21Document {",
    )
    text = text.replace(
        "            },\n            ..StepSnapshot::default()\n        }\n    }\n",
        "            },\n        })\n    }\n",
    )
    text = text.replace(
        "has_product_definition_chain(&snapshot.document));",
        "has_product_definition_chain(&snapshot.to_part21_document()));",
    )
    text = text.replace(
        "        let mut snapshot = conforming_snapshot();\n        snapshot.document.instances.push(",
        "        let mut snapshot = conforming_snapshot();\n        let mut doc = snapshot.to_part21_document();\n        doc.instances.push(",
    )
    # Close the push(...) call and reassign snapshot from the mutated doc, then let the
    # existing `let (mutated, _diff) = ...` line follow unchanged.
    text = text.replace(
        "            entities: vec![(\"ADVANCED_BREP_SHAPE_REPRESENTATION\".into(), vec![])],\n        });\n        let (mutated, _diff) =",
        "            entities: vec![(\"ADVANCED_BREP_SHAPE_REPRESENTATION\".into(), vec![])],\n        });\n        snapshot = StepSnapshot::from_part21_document(doc);\n        let (mutated, _diff) =",
    )
    if text != orig:
        f.write_text(text)
        changed.append(str(f))

print(f"Changed {len(changed)} files:")
for c in changed:
    print(" ", c)

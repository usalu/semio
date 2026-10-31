# Program Study Narration

The English and German Program hook now explicitly applies four proposed A–D room-program studies to the 140 reconstructed capsules and distinguishes them from the six historical physical capsule families. Only the two hook strings and their matching transcript paragraphs changed; animation and geometry remain unchanged.

A portable localized fixture with exact JSON Schema constants was authored before the production correction. The new presentation regression extracts the actual narration with Python AST, compares both localized strings independently with NumPy, and checks both real transcript paragraphs. The baseline failed against the historical four-type claim. The corrected regression passed one test in 1.101 s. Initial Python invocation lacked jsonschema; the final test uses the existing NumPy oracle without adding a runtime dependency.

Files: Excel Program scene, English/German Excel transcripts, presentation fixture and presentation test module. Native lifecycle replay must use the new Program dependency fingerprint after the final roof-frame freeze.

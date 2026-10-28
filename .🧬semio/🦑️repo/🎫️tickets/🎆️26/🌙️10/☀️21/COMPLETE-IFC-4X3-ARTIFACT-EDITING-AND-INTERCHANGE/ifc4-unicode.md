# STEP Unicode Correctness

The shared native Part21 writer previously emitted supplementary Unicode scalars inside four-digit X2 groups, producing invalid group widths. It now emits X4 eight-digit scalar runs whenever a run needs the full coding space, preserving existing BMP X2 spelling. The native lexer now decodes X2 UTF16 surrogate pairs as one scalar and rejects unpaired surrogates. X4 continues to reject surrogate codepoints and out-of-range scalars.

Native regression tests were authored first in shared Part21 unit tests; neutral vectors are stored in shared Part21/🧪️tests/🧫️fixtures/🔡️unicode.json. An IFC4 permanent TypeScript suite consumes those same neutral vectors with the real STEP adapter and compares positive decoding with existing test-only iconv-lite 0.7.3 UTF16BE/UTF32BE codecs. It passes four valid vectors, five malformed sequences, exact graph roundtrips and strict TypeScript compilation. Native execution remains blocked before compile by repository build storage.

Source guidance: [buildingSMART string encoding](https://technical.buildingsmart.org/resources/ifcimplementationguidance/string-encoding/), [buildingSMART implementation guide](https://standards.buildingsmart.org/documents/Implementation/IFC2x_Model_Implementation_Guide_V2-0b.pdf). X4 is the canonical emitted spelling for characters outside the BMP.

Changed files: shared Part21/✍️writer/🦀️.rs; shared Part21/🔤️lexer-and-parse/🦀️.rs; shared Part21/🧪️tests/🔬️unit/🦀️.rs; shared neutral Unicode JSON; IFC4/🧪️tests/🔬️step-unicode/🟦️.test.ts; existing IFC TypeScript suite router. Reusable input harness: ifc4-unicode-check.mjs.

Final audit also found the TypeScript shifted-character directive accepted a non-ASCII operand that native parsing refused. The shared TS parser now rejects it; the neutral/native suites include that sixth malformed vector. Executed Unicode source suite still passes with zero type diagnostics.

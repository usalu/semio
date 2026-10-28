# Official Layout Interchange Fixtures

Retrieved unmodified accepted SVG inputs from the W3C SVG 1.1 Second Edition 2011-08-16 suite. The [accepted-test index](https://www.w3.org/Graphics/SVG/Test/20110816/harness/htmlObjectApproved/index.html) lists coordinate transforms, fill rules, and text inputs. Original embedded notices are retained; SHA256 and byte counts are in the domain fixture provenance.json. These are additional to the rectangle fixture acquired by the interchange agent.

These files establish authentic upstream provenance. They do not yet prove full renderer conformance; some suite files reference external fonts and harness resources. Exact fixture tests must declare the supported import/render behavior.

## PDF and STEP Inputs

Added three unmodified veraPDF Consortium corpus PDFs pinned to revision bb75f4f0073d9350dfd058c0162a367e6fadf25e, with SHA256, original paths and CC BY4.0 attribution in the PDF provenance manifest. Independent pdfjs-dist5.7.284 parsing produced pdfjs-oracle.json with all page boxes, text and transforms. Both PDF/A1b positive witnesses have one page and empty extracted text. The ISO32000-1 negative text-state witness extracts Hello World; it is a negative rule witness, not a PDF compliance positive. Native import tests remain unrun.

Added NIST PMI Validation and Conformance Testing Project geometry-only sample nist-ftc11-ap203.stp,7634bytes, from the official NIST sample archive. Its declared FILE_SCHEMA is CONFIG_CONTROL_DESIGN (AP203), so it is not AP214 proof. Manifest preserves exact archive member, source and SHA256. The layout projection uses generic Part21 parsing; faceted geometry acceptance will be verified in native tests.

Primary sources: [veraPDF corpus](https://github.com/veraPDF/veraPDF-corpus), [PDF Association corpus description](https://pdfa.org/resource/verapdf-test-suite/), [NIST STEP analyzer and sample archive](https://www.nist.gov/services-resources/software/step-file-analyzer-and-viewer), [STEP Tools AP214 samples](https://steptools.com/docs/stpfiles/ap214/index.html). CAx-IF library returned HTTP403; no official AP214 validation claim is made. Adobe cookbook link returned404; no official IDML suite was acquired.

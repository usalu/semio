# XLSX document input and output

The XLSX analyzer recognized physical workbooks but sent binary input to the internal snapshot Pack decoder. Builders and subset validators used the same Pack-only path. XLSX input now selects the decoder from the framework envelope signature. The registered Strict and Transitional composers accept sources labeled with their own dialect and use the shared package codec.

The Strict and Transitional physical-file entry points apply the profile checks on both load and save. The base writer validates OPC authority before serializing XML parts and retains the imported content-type declarations, including defaults that otherwise acquired an override on export.

The base codec tests compare the original and exported two-sheet workbook with Calamine, exercise both physical and packed ZIP input, and verify that an opaque custom XML part continues to use the package's default content type. A language-neutral scenario compares native analysis and XLSX export with the existing Calamine and rust_xlsxwriter reference implementation. Its test manifest declares the Rust subject packages independently of the oracle dependencies. The Strict and Transitional suites exercise direct load and save, composition, builder admission, and cross-profile refusal.

The native writer regenerates ZIP and XML syntax from logical authority. The semantic comparison covers the contents and coordinates of the sheets, with a focused package-level assertion for content types and custom XML. The existing profile checkers cover their declared axes. The workbook codec still expects UTF-8 OPC/XML parts.

The artifact diff passes the repository's whitespace check. The repository Gherkin parser resolved the native-document differential scenario, its long test level, the XLSX oracle, and the semantic spreadsheet comparison profile. All four subject host entries resolve to local Cargo manifests.

Direct invocation of the differential runner failed before test dispatch because it lacked the original caller capabilities. The registered parity target reached project graph admission and the resident service closed with exit code 1 and reference `6ca7d949`, including with the isolated owner and service-off options. No differential scenario ran during those attempts.

The unfiltered suite passed the native codec profile tests and exposed a Transitional inverse-law failure. Strict construction updated only the workbook XML root, so its initial workbook contained Strict and Transitional namespace declarations and Transitional relationship types. A whole-package namespace change then discarded the distinction. Both profile constructors now set declarations and relationship types consistently across the package. The existing inverse law checks the three worksheet roots and package relationship base before testing each inverse and the complete round trip. The combined suite will verify the revised construction.

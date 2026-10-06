# Current Rust Chart Mutation Verification

The current registered route uses pinned Bun1.3.14 and `@semio-tech/print-rs:test --args="--lib tests:: -- --skip tests::sqlite_snapshot_tests"`. CARGO_TARGET_DIR, CARGO_BUILD_BUILD_DIR, SEMIO_TEST_ARTIFACT_DIR and PRINT_NATIVE_CHART_TIKZ are ticket-contained; SEMIO_COVERAGE=0. No Rust or shared infrastructure source was edited by this lane.

First actual route30686 exited1 in1m47s before test execution. MSVC LNK1104 could not create a proc-macro import library under the original263-character output path. Same-route retry12924 used generated/r (corresponding output226characters), passed that linker boundary, and reached actual current print Rust compilation. It exited1 in2m51s (target2m32s;critical2m50s), reporting43 compiler errors in semio-framework-print, including owned DSL trait/API failures. Therefore no current11-test Rust PASS is claimed. Both complete authoritative logs, nextest artifacts, and all generated target outputs are retained in generated/native-current-rust-chart-tests.log, native-current-rust-chart-tests-short-path.log, generated/ra and generated/r.

This is a real current Rust compile failure after removing the ticket-path infrastructure obstruction. Parent coordinates required source repair with concurrent owners; no repeated unchanged test dispatch is justified here.

## Required current framework contract repair

The43 compiler errors group into: private DslValue/FromValue/ToValue imported through the OS barrel (which also prevents required value derives/manual impls); obsolete pack::json and unresolved semio_framework_pack paths (the installed owned Pack library is named pack and owned JSON is a separate first-party package with explicit member policy); removed ArtifactInferrer marker; checked Result signatures now required by Inference::infer and Mutation::inverse; and the private SQLite transfer grow helper consumed by the existing chart reconstruction.

Chosen changes before editing: import owned value traits/macros directly in existing print leaves; use existing pack::record for native/operation codecs; depend on the existing first-party owned JSON package and require JsonMemberPolicy::Reject for authored documents; remove the unused obsolete ChartBuilder marker; implement current checked trait returns while preserving semantic-invalid inference diagnostics and mutation behavior; expose the existing paid/cancellable grow helper needed by chart reconstruction with a native emoji docstring, without duplicating allocation logic. Existing native chart mutation tests will explicitly unwrap checked successful calls. No external dependency, default language, restored old API, new runtime module or planner is introduced. The actual12924 compile failure is the pre-repair RED; existing neutral mutation/schema/paint/native-grammar fixtures and serde_json test oracle remain the controls.

Exact before bytes of every touched existing owner are retained in authored-inputs/native-rust-contract-before. Parent authorized this current Rust build repair after the real registered gate failure; unrelated source changes are preserved by fresh reads.

| Existing Rust owner | Before SHA256 | Candidate SHA256 |
| --- | --- | --- |
| 🧰️framework/🛍️products/📓️print/🦀️.rs | 593D6C623B9457CC996D8EFFDFD4058047F654E9C5BF1524A474D8F763093FC3 | BBF3B61365C2C07BFD9CA6AC4DA7C0597691D5BEFD0484030B866E73630A1219 |
| 🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/Cargo.toml | C5BBB2AF1D7EE2A3B8253A2FCB050733FAAC5D3E379FF3C679E409E572420D34 | 912E6FBDA8523F9DA8E455444736B36FA12ACCF6B79D65751743A84EE2E9170C |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🦀️.rs | FD5B9AA5E68A81B3CB18AD16C494AECBD755DDB1194D424C83E78D902FD65852 | 35607135095C76C3DCA0BEE2F1A943E16E3E8CF29F865FB4B383C671BBDB1D22 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs | 7E9AF1C04A436BD79EE22AC311DD9D0A594B8EE62495228909A437B9CBD87A1D | F10F20707D7A9AE226034577104282FAF383DA41DDC06B6DD8BF9F9B7B494E19 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🚦️native/🦀️.rs | DA5B4250495F0F36D59EDB9A515775100F686DEE2823973AC242660161BD6057 | 5AB4598B34C31D9DC7166EB069A0FA6DC8788051B1F4FDD4A16DCCF32C419E00 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs | 4055DB0B82DEAC0D78F5C939D056BDAFD534AF454B6D6E09E2974453A305923F | ABD7D76477A72D1D1112BEC79AA93CAE4C73CB7C3050A5D00F841C73E0019D21 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs | FBEAA44AF4EC31B3E6D22315AC56BD022678D7EFB5DF32D2F7B35999CCFCA65C | 3D92B2CDEEF6E533D6B3EBAB4D0C3537F6EA23C7680A9B9D36C32A380B94BBF6 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/📦️codec/🦀️.rs | 27DC435DFC21F4F19C61DCEA3FAFBFB848E01EA23097607B5EC622D643271299 | 35A0C4B82A271FA15F56078C265766D514B759CFA4FF16F36BFA37A57074BA74 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs | 7B183CB88EC8333263F2AE80A0FDB8CB62F65ABBBB57B84E0EF7184D37480DAC | 761FBA4D3F9D8347EA3A73D6C6D94FEB4AA948DE1D63C1A3871CC1B4AAAFD8EF |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🦀️.rs | F0D28DF8D5BCAD2EE65D3C64AE715480638C90D21522760FB02D4C14F3A0EA0A | 8F86DD4944B688B812031A5EFAA63C9B9A1204C91BC1664744D42EBDCB3A65D1 |
| 🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs | 63FC22470579E0AD8FF339678E56F5EAE66C192816B07188BDBF0865F978ADE6 | BCCC942CEB94687B733B42ED18E339A04FAD14A92EF8A9565F1146F479782949 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs | 6AC2F9E9364F76D45D6FBAE709637FEE6E41DC55FF0955AA83B8EFE49753CC09 | B1504C9A1D530E66B83C642A15B36706E16B33F9598BA8A70691897D04FC77C6 |

Actual contract candidate32961 exited1/1m55s,7 remaining compile errors: duplicate macro imports because the owned value trait exports also expose derive macros;2 Pack option type-identity errors from calling the separate Pack crate against OS-owned ArtifactPack options; and a now-fallible capability claim chain. Corrected existing imports, propagated the claim error with?, and selected the existing protocol::record codec that owns the ArtifactPack options. No option adapter or unchecked conversion was added. The now-unused direct Pack dependency was removed from print; the existing first-party owned JSON dependency remains. This precise current codec type correction supersedes the initial pack::record choice; the old attempt is retained as actual failed evidence.

Actual candidate2(58897) exited1/38.1s, remaining8errors: capability chain final claim also requires propagation; the snapshot trait's Pack options specifically belong to the OS pack_rt value codec, whereas protocol::record reexports the separate owned crate; and the nested SQLite test module (compiled even when runtime tests skipped) still imported now-private value traits/Number through protocol. Corrected these existing caller/import boundaries; no type adapter added. Nested SQLite test before bytes retained separately. Snapshot option-preserving calls now use the already-existing exact pack_rt owner. Full tests remain pending.

Two in-place source-write attempts were refused by Windows because the Rust sources were mapped; immediate audit confirmed both original nonzero files intact and unchanged (root10454bytes/native5178bytes). The intended exact hunks were subsequently applied by fresh-read/hash-guarded replacement from ticket-contained temporary files, preserving all other current bytes. The last observed candidate3 output contained the bootstrap dispatch; its actual compiler log, source fingerprints and terminal will determine the verified scope.

## Actual candidate3 terminal and current ownership

Registered candidate3 session96499 exited0: actual Rust compiler succeeded, 11 tests passed, 0 failed, 8 SQLite tests filtered. Nx run48.7s, critical47.7s, print target28.5s. The test named native_chart_source_can_be_compiled_by_the_print_toolchain parses the neutral native grammar, infers source, asserts no diagnostics and writes PRINT_NATIVE_CHART_TIKZ; it does not invoke TeX. Current emitted source is retained at generated/native-current-rust-chart.tex. TeX stage equivalence or current compilation must be separately established.

Current14 touched owners below comprise12 initial Rust owners plus one compiled nested SQLite test and the separate registered TypeScript browser controller. Ownership differs from the complete first-party Rust context closure.

| Path | Current SHA256 |
| --- | --- |
| 🧰️framework/🛍️products/📓️print/🦀️.rs | 860C8BB0536D60685919E206D550A8CC0D4C9EAA3EA8DE17BE361C26E53FE458 |
| 🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/Cargo.toml | AEF4112ADBE0FF1055E1A7D2E3E53AD3770D0C266DE2E9457FF35DFABAED1C7E |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🦀️.rs | 0A82ACBAC71BF905110337C15980FA5A8D33B607A23B9D9114EFF131CC37F113 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs | F10F20707D7A9AE226034577104282FAF383DA41DDC06B6DD8BF9F9B7B494E19 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🚦️native/🦀️.rs | 95B691135396ED3031194829616FF1E2B7CA22DDF78097B290623857C19D9766 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs | ABD7D76477A72D1D1112BEC79AA93CAE4C73CB7C3050A5D00F841C73E0019D21 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs | 3D92B2CDEEF6E533D6B3EBAB4D0C3537F6EA23C7680A9B9D36C32A380B94BBF6 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/📦️codec/🦀️.rs | 77B0E572E27B6C46A08B8856D931513330C2C9378963FEC1CB2BBBB01E350CBE |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs | A18A9F453686CCE7A268722B456E8F803A494A7892E084DD03ADF230F659565A |
| 🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🦀️.rs | 8F86DD4944B688B812031A5EFAA63C9B9A1204C91BC1664744D42EBDCB3A65D1 |
| 🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs | BCCC942CEB94687B733B42ED18E339A04FAD14A92EF8A9565F1146F479782949 |
| 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🦀️.rs | B1504C9A1D530E66B83C642A15B36706E16B33F9598BA8A70691897D04FC77C6 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs | 0F7BE74C73D73992C477785718538681D693EA3BF844B5079F0E99FAEF4816EC |
| 🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🟦️.ts | CA3A51DA3F8F5F8B3255F9B14DC896705163809FCA149AA562677B937F0CF654 |

## Actual full19 current Rust RED

Registered26921 exited1 in45.4s/critical44.7s. Actual compiler succeeded;16 tests passed and3 failed, including2 native public-I/O/control failures because shared borrowed Pack preflight lacks intrinsic-value visitors, and the9-variant numeric law because Display-format subnormal/extreme Float text exceeds the32-byte exact companion. No19-test PASS is claimed. The pre-repair numeric fixture/control includes UInt64 max, Int64 min, signed positive7, negative zero, integral Float7, subnormal, minimum normal and both signed maximum finite values.

Planned narrow numerical correction before source editing: retain the existing32-byte bounded ASCII companion and encode Float with Rust shortest scientific formatting; owned integer formatting stays decimal. Scientific strings parse directly back to the same finite binary64 bits. The actual same-owner test will read explicit language-neutral numeric variants and independently query actual SQLite number_kind/number_exact with Bun SQLite, compare parsed values to serde_json number formatting, and continue testing controlled native endpoints after the separate shared intrinsic visitor is repaired. No larger arbitrary allocation or fallback codec is introduced.

The complete read-only Cargo metadata audit produced49 first-party dependency packages and813 hashed inputs from54 emitted .d files plus manifests/root Cargo controls. Initial capture is generated/native-current-rust-firstparty-input-closure.json; refinement will bind actual active target src_path/build-script membership. The first nx exec audit was accidentally dispatched across projects and repeated only read-only metadata/hash work; verified owned Nx69836 tree was stopped and its identities retained. Rust test26921 was unaffected. This audit dispatch is not a test PASS.

Current Print native19 control route69347 actual terminal exit0, pinnedBun1.3.14. All19 tests passed (eleven chartmutation/inference + eight SQLite/native codec controls); zero failed/filtered. Actual compiler/test executable used registered cache/cargo/build, test runtime1.97s, Nx2m53s/critical2m51s, Printtarget2m14s. This proves current NumericText32 nine-variant exact bit/SQLite companion checks and both Pack/Text borrowed intrinsic codec/preflight callers. Shared neutral Text semantic test95658 separately actual0/five controls. Emitted Rust native-current-rust-chart.tex is output only; no TeX compiler is inferred from its test name. The required canonical override/data customization compiled proof remains pending.

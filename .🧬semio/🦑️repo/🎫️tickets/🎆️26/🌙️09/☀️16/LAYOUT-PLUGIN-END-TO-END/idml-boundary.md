# IDML Source Ownership Boundary

## Evidence And Design

The failed integrated Layout selection genuinely depends on IDML: export-unit-tests enables engine-export testkit, whose normal dependency is Layout IO, whose normal dependency is IDML. The original normal closure was 37 files / 162,402 bytes / 1,336 active lines; it exceeded the 32-file limit. The failing retained-part file was 7,081 bytes with 126,026 bytes still available. This was a source ownership problem, not a large-file or unrelated-selection failure. The original own-test closure was 58 / 303,982 / 1,713, independently over files and bytes. The independent read-only evidence is idml-source-boundary.md.

The existing schema descriptor already defines a cohesive independent domain: four language facets for artifact, snapshot, diff, and mutations. It only requires the first-party schema registry. The new semio-s-artifact-stdio-idml-schema package owns this descriptor and the same twenty include assets at their existing paths. The IDML schema module imports this canonical descriptor owner. No facet was deleted, copied, rewritten, or made conditional.

The IDML root keeps all representation, validation, snapshot, mutation, diff, issuer, and native lifetime implementation. Its existing private retained mutation/overlay/apply tests remain owned by the library. The existing public container and admission laws now belong to semio-s-artifact-stdio-idml-tests. The root container law still mounts its existing native.rs laws; the companion additionally mounts the original admission, retained version, sparse-change, and retained-part laws. Test source and neutral fixtures stay at their existing paths. Only test imports and test ownership mounts changed. No test body, budget assertion, cancellation cut, original source/decoder protocol, format, or fixture was removed.

No admission crate depending back on IDML was introduced; that would create a dependency cycle. No limit, exclusion, wildcard ownership rule, or runtime feature bypass was added.

## Exact Packages

- semio-s-artifact-stdio-idml-schema: root ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/📇️descriptor; manifest root/📦️packages/🦀️rust/Cargo.toml.
- semio-s-artifact-stdio-idml-tests: root ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests; manifest root/📦️packages/🦀️rust/Cargo.toml.

Both use the existing IDML Cargo workspace. Rust owns its additive membership edits; root owns build-owner registrations. New absolute manifest paths have 119 and 105 characters. The test companion declares direct dev dependencies used by all six existing source files, including pack and stdio-contract from native.rs; no transitive dependency assumption is used.

## Static Validation

Actual managed idml-boundary-inventory.ts uses the canonical source-closure reader with native macOS/unix cfg and separately measures normal/test sources:

| Owner | Mode | Files | Bytes | Active Lines |
| --- | --- | ---: | ---: | ---: |
| IDML | normal | 21 | 151848 | 1294 |
| IDML | own tests | 31 | 196045 | 1409 |
| Schema descriptor | normal and tests | 21 | 29526 | 36 |
| Public law companion | tests | 12 | 103195 | 271 |

All fit 32 files / 262144 bytes / 4000 active lines. Narrow managed git diff --check for IDML passed. Source-lifetime independently reviewed the six public test sources and found no remaining private API or direct dependency gap; see idml-boundary-assist.md.

The companion preserves thirteen actual #[test] cases: one root container law, six existing native representation/declaration/receipt laws, three admission/lookup laws, one retained version law, one sparse-change law, and one retained-part law. The initial coordination count of seven omitted the existing nested native.rs six cases; thirteen is the correct native expected count. The original private library tests remain separately mounted.

## Independent Existing Behavior Witnesses

The existing owned TypeScript test interfaces were executed using fast-json-patch for RFC6902 outputs, fflate for actual ZIP member round trips, and Python's system ElementTree for XML syntax and decoded root attributes. The ElementTree port evaluates actual fixture inputs and refuses missing input records; it does not substitute expected validity. Its 347 fixture scalar/byte inputs are generated under 🗑️generated/idml-xml-oracle.json. XML findings come from this system parser; no fast-xml-parser execution is claimed (that optional package was unavailable). No runtime dependency was installed.

Actual managed oracle result:
- admission: 12 cases, 2051-member case, four cancellation cuts, four replacement cases, three retained version fields;
- sparse changes: eight cases, four field cases, four work grants, four cancellation cuts;
- algebra: nine cases, eight refusals;
- mutation/package oracle: nine cases, nine ZIP archives, ten XML parts.

These results execute existing neutral fixture semantics and independent libraries. They do not claim native Rust execution or physical caller-budget proof. Native assertions retain that separate responsibility.

Retained command inputs:
- repo --service=off tools exec --light python -- '<ticket>/idml-xml-oracle.py'
- repo --service=off tools exec --light bun -- '<ticket>/idml-boundary-oracle.ts'
- repo --service=off tools exec --light bun -- '<ticket>/idml-boundary-inventory.ts'

All commands used explicit cwd /Users/ueli/Documents/semio. No compiler or Cargo job was started by this packet. Rust was given the two member paths, the whole companion package name, and expected thirteen native tests without a filter. Native validation and the integrated Layout retry remain pending its sole sequenced lane.

## Actual Native Selection Started

The Rust lane started the first real IDML native request using this artifact's owning Cargo.toml with both semio-s-artifact-stdio-idml and semio-s-artifact-stdio-idml-tests selected, --lib --nocapture. This preserves execution of the original private root tests and the thirteen relocated public cases. Full output: generated/idml-native-1.log. The actual result is pending; the new typing EDIT/fold work remains unmounted until this owned request terminates.

## First IDML Native Refusal

The first public/private native request ended before compiler or tests: Cargo's unit graph rejected profile.prototype.trim-paths without the owning workspace's cargo-features opt-in. Added exactly the same trim-paths header already used by the Layout workspace, preserving IDML members and package settings. The repo-owned lock refresh is included in working-file attribution. No test pass is claimed. Both EDIT owners and coordinator were notified on authoritative terminal before another snapshot.

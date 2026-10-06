# TypeScript Codec Verification

## Ownership

Physical Typescript implementations moved for Forms JSON/value/condition/definition/response, Forms dictionary and response CSV/JSON export; Program and Layout JSON snapshot/diff; Semio Drawing native JSON; Semio Graph JSON; PDF native JSON (nine modules); Block 3D JSON; Remodeling snapshot/diff/mutation/artifact JSON; TXT protobuf reader and six physical field decoders.

Canonical schema dependencies remain one-way: codecs import semantic types and validators. Schema no longer imports codecs for targeted PDF/TXT/Layout. Remodeling owned validators no longer accept a transport switch; TXT semantic refusal codes no longer include protobuf errors. PDF canonical schema defines explicit owned binary64/i64/u64 scalar roles; its native JSON schema is an IO artifact.

## Tests Run

- `bun nx run @semio-tech/block-3d:check`: passed.
- `bun nx run @semio-tech/stdio-txt:check`: passed after final transport error ownership refinement.
- `bun nx run @semio-tech/forms-js:check`: passed after repairing artifact entry's stale SQLite reexports to direct IO ownership.
- `bun nx run @semio-tech/architect-js:test`: passed; two example laws, zero failures.
- `bun nx run @semio-tech/stdio-pdf:check`: initially exposed bodyless invalid-fixture rejection loops in PDF14 and PDF17 SQLite tests. Meaningful independent Ajv fixture rejection bodies were restored, including the correct neutralGraph oracle binding; final run passed (exit 0).
- `bun nx run @semio-tech/layout-js:test`: two example laws passed; renderer-contract failed before tests at Vite `Denied ID` for the framework hub-source service-worker module with `?worker&url`.
- New eight-law regression suite and neutral fixture registered by root into existing `@semio-tech/repo-lib:test-artifact-io-ownership` Nx route; first runtime RED exposed a strict-canonical Remodeling scalar leak and a malformed Block fixture. The fixture is now complete, canonical Remodeling/Program guards use owned parsers, and the complete Program corpus has a 60 second per-law timeout. Final GREEN is handed off to root; this agent does not claim it has passed. The suite covers exact graph IEEE words, PDF owned-versus-native scalar admission, TXT Unicode/truncation, Forms JSON and response export corpus, Remodeling canonical defaults and JSON words, Block 3D canonical word preservation, Program 532-snapshot/260-diff oracle corpus, and Layout 90-snapshot/39-diff Drawing/dictionary corpus. Third-party Ajv and existing Forms oracle tests plus independent Buffer scalar decoding validate outcomes.

The inventory in `typescript-extraction-files.md` records implementation and reference changes; restored incidental import quote normalization is excluded from final attribution where contents return to their original spelling.

## Canonical Scalar Closure

`typescript-canonical-scalar-files.md` lists twenty additional canonical schema sources whose scalar parsers still performed native transport conversion. They now import owned `parseBinary32`/`parseBinary64` from Value. Program and Remodeling canonical exact-record guards were fixed separately. The framework physical scalar transport extraction is owned by root.

`typescript-import-spelling.md` records restoration of incidental same-target import spelling normalization. No modifying Git command was used.

## Final Handoff

Root owns final combined architecture/physical-codecs GREEN execution and removal of the temporary physical-codec `afterAll` DEBUG logger once successful runtime evidence is captured. The last source-boundary run before the final canonical fixes had 12 passing and 3 failing tests: root Bun type diagnostic, Remodeling strict hex-word admission, and Program per-test timeout. All three causes were addressed by the respective owners before handoff, but a successful fresh run is still required.

Additional final corpus checking may surface former direct native-number callers now that twenty canonical schemas require owned IEEE words; those callers must use an existing physical codec or explicitly construct semantic words. No compatibility wrappers were added.

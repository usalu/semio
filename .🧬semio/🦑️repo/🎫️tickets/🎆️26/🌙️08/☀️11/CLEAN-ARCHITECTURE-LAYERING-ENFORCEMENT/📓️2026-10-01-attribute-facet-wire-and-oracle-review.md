# Attribute Facet Wire And Oracle Review

Read-only review of existing Rust source direction attribute facet; no compiler/test executed. Files are under Repo library's schema/fixtures/tests `🧱️rust-source-direction/🧾️attributes`.

Nine authored cases independently describe nativeInputs and scanner result: active outer/inner doc, dormant outer/inner doc, outer/inner opaque attribute, opaque stringify documentation and outer/inner string decoys. Native success expected for every initial source is consistent by Rust source reasoning: opaque unknown attributes are inactive and stringify does not expand its token argument. Dormant deletion remains native success while gate reports missing input; active deletion is native failure. These expectations are meaningfully independent.

JSON schema is closed at corpus/case/result/reference/removal objects. oneOf resolved/unsupported state prevents references on refused rows. Files only permit flat lowercase .txt keys, matching actual materialization without recursive mkdir. IDs are checked unique in the schema law. nativeInputs/removal paths accept arbitrary nonempty strings while file keys are restricted; add portable input path constraints and assert removals are actual authored file keys. A future removal on an unsupported row would silently be ignored by the current harness; close that combination with schema conditional or execute an explicit refusal assertion. Current rows all satisfy that intended relationship.

## Native Dependency Portability

Test line36 selects a dep-info rule by raw `binary + ':'`, then splits whitespace at38. Makefile dep-info escapes spaces and other characters in the absolute target path. A caller-owned artifact directory containing spaces therefore breaks the match; Windows path spelling/drive separators may differ too. pathe normalize applied after whitespace splitting cannot restore escaped token boundaries. Existing parent source-direction dependencies helper at124–135 already handles escaped dependency tokens and relative path normalization; reuse a shared owned test parser, or emit a relative binary name and still parse make tokens correctly. Preserve caller-controlled artifact storage, do not ban spaces to hide this defect. Add portable dep-info parser vectors for spaces, escaped # and Windows drive/backslash spelling, independently matching actual native output where applicable.

## Physical Deletion And Byte Witnesses

The harness writes the source and fixture bytes, compiles and runs the real binary, resolves scanner facts, deletes files with rmSync, recompiles, checks native outcome, confirms unchanged source bytes and scanner facts, and checks typed missing inputs. These are genuine deletion comparisons. It currently does not reread fixture bytes before deletion, assert deleted files physically absent, or witness nonremoved fixture bytes. Add explicit witnesses rather than relying on writeFileSync intent. Existing missing expectation maps every reference, which is valid for today's one-reference removal rows but must filter by actual removed target for future multi-input cases.

Original line provenance is consistently1 in all current attribute rows. Ordinary RustSourceReference exposes line, not a UTF16 source offset; this facet therefore does not independently assert exact token offsets. Do not claim offset coverage from line1 alone. If offset witnesses are required, retain closed source span expectations separately or extend canonical API/schema coherently.

## Inner Guard And Coverage

The current inner opaque row is the necessary negative boundary: outer-only rustAttributes leaves inner attributes to generic recursion and bypasses owned doc/opaque validation. Require a genuine prepatch RED for this row and preserve it through the broader scope correction. The active/dormant/opaque/string-decoy matrix is appropriately separated from main core uniform dep-info expectations.

Still useful additions: concat doc matching the two actual UI builder inputs, nested cfg_attr doc, comment decoy, raw builtin attribute spelling, and doc wrapper with a local/qualified concat name that consumes tokens. Unknown wrapper refusal must preserve independent native no-input evidence. All current initial binaries print the exact DEBUG witness; deletion builds do not execute stale binaries after expected compilation failure.

No native success/fullpass claim is made by this audit.

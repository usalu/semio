# Lossless Nullable Image Profiles

## Problem and Result

ImageLink.color_profile is an optional canonical string, including a valid present empty string. The old ImageLinkPatch used an empty-string clear sentinel. Updating a Link could erase present-empty data and its stored inverse could not restore it. The owned setter also discarded outer setter presence when a displaced original value was absent.

The schema now has an explicit nested optional Native field: missing preserves the current profile, present null clears it, and every string including empty sets that exact value. Original displaced values remain in the patch owner for funded retirement. No sentinel compatibility path remains.

## Schema-First Evidence

Twelve neutral profile ownership matrix cases were written before the implementation change. The old implementation failed image-profile-null-clear, establishing the red witness. The fixed portable owned setter passes16total cases against an independent RFC6902 implementation behind the existing owned oracle interface. All12profile combinations preserve unrelated object identity and displaced optional presence.

Six additional neutral central diff cases cover clear, set empty, restore original empty, missing preserving empty, clearing empty, and Unicode. The actual central applier/inverse/replay/composition suite passes46portable,45golden,500generated and10refusal cases. Native cold and controlled/serde round-trip cases use the same corpus; their execution remains unrun.

Four neutral JSON wire cases separately pass actual parseImageLinkPatch, AJV primary field schema validation, ProtobufJS encode/decode of the real full dependency schema, and GraphQL parse/build/validate/query over the real schema facets plus their first-party dependency bindings. The combined GraphQL graph has136definitions,15LayoutDifffields,14LayoutArtifactfields and0validationerrors. The helper's query only observes the profile setter; this is facet interoperability evidence, not an implemented whole-document GraphQL/Protobuf transport claim.

Portable feature closure60real implementation sources/0strictdiagnostics. Seven changed Native sources parse with0grammarerrors; this is not compilation or Native allocation evidence.

## Changes

- Native ImageLinkPatch double-option FromValue/ToValue, controlled decode, serde and retirement follow the established nullable framework contracts.
- Cold patch/inverse, UpdateLink forward and retained SimpleDiffOutput preserve optional profile exactly.
- Retained central OwnedPatch uses an admitted header swap; it does not allocate, free or reinterpret empty text during commit.
- Portable central applier uses supplied-field presence. Portable evaluator and owned setter preserve clear and empty separately.
- JSON schema explains the three states; Protobuf uses existing NullableString oneof; GraphQL uses a nullable outer NullableStringSetter with nullable inner value.
- Independent GraphQL validation exposed36identical duplicate declarations in the existing diff facet; these were removed. A failed local file truncation was caught and the complete15/14multiline field blocks were restored and independently checked. The diff facet also now describes genuine handle-only LayoutDrawingMember and s.draw.drawing child identity.
- Actual Protobuf loading exposed a stale framework/schema/artifact-reference.proto import. The generic ArtifactLink facet now imports its real canonical first-party source path directly.
- The canonical Layout runner mounts the four profile laws. Interchange separately added two UpdateLink mutation/inverse witnesses for authored empty and restoring original empty; reported43kinds128commoncases/5016undergrants/496cuts, Nativeunrun.

## Ownership

Root owns the field/facet/central setter changes and related neutral reports/tests. Interchange owns removal of unsupported Link metadata caps and common mutation fixture mounts. Core owns genuine Drawing canonical fixtures and original producer infrastructure; Validation owns prepared Drawing providers/vector consumers. The full goal and ticket remain open.

Fresh canonical integration after removal of obsolete Layout SetDrawingText:65 schemas,58 transport/semantic variants,122 portable semantic cases+51 goldens+40IEEEwords; GraphQL135definitions,14diff and14artifact fields,0errors. All nullable-profile setters/algebra remain passing.

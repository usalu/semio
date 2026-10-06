# Energy Outer Borrowed Record Roles

Read the actual borrowed first-party fields; do not allocate ArtifactChild, ArtifactRef, LinkPin or ArtifactLink to forecast SQL costs.

| Owner | Required borrowed native roles | SQL contribution |
|---|---|---|
| ArtifactChild | Record exactly IDs0 child_id Text,1 target Record | identity8 + child_id UTF8 + target four texts |
| ArtifactRef | Record exactly IDs0 artifact-id Text,1 artifact-kind Text,2 standard Text,3 subset Text | four literal text lengths; dialect is flattened, not a nested record |
| ArtifactLink | Record exactly IDs0 target Record,1 pin Record,2 role Text | identity8 + target four texts + role + pin tag + branch payload |
| Head pin | ID0 Enum(0); unused1..4 absent or missing | tag `head`4; five SQL payload cells Null0 |
| Checkpoint pin | ID0 Enum(1),ID1 checkpoint_id Text; unused2..4 absent or missing | tag `checkpoint`10 + checkpoint UTF8 |
| Snapshot pin | ID0 Enum(2),ID2 blob_hash Text,ID3 blob_size UInt,ID4 blob_media_type Text; unused1 absent or missing | tag `snapshot`8 + hash UTF8 + highword8 + lowword8 + media UTF8 |

The ArtifactRef flat layout is authoritative in [Io reference specification](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️reference/🦀️.rs:6); its exact four-field validation is line9. [Store ArtifactChild controlled constructor](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3974) requires exactly two fields and nested reference. [LinkPin specification](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4063) defines field ordinals and optional shapes; [branch validator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4157) allows missing or Absent unused fields while refusing active fields missing/Absent and any present unrelated payload. Reject IDs>4 and enum ordinals>2. Validate active fields' exact Text/UInt category before counting. Do not require all unused optional IDs physically present: the canonical literal may omit them.

[ArtifactLink specification](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4289) uses target0/pin1/role2. The controlled encoders bound nested construction with scoped_depth64 and paid stages; the borrowed visitor should retain the original decoder's depth/backing control and step per role, without introducing recursive unbounded traversal or copied strings. Empty/NUL/Unicode identity text is literal and accepted. Unsigned64 blob size is not narrowed; SQL stores exact high/low unsigned32 integer values.

Energy outer fields0 schemaText,1 modelValue,2 structureRecord,3 zonesRecord,4 referencedModel optionalRecord,5 weatherLink optionalRecord remain separate. Missing required child is a refusal; optional link FieldValue::Absent means no SQL row. These roles complete the earlier69-table model blueprint. No gate or production modification was performed.

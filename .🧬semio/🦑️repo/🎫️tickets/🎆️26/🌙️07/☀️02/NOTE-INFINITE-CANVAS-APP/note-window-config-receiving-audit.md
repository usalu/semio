# Note window configuration receiving audit

Attempt 23 (driver `w53iP91`) reached the Note component. Its window configuration owner failed the required `OpBinaryReceiving` bound because the local camera update encoded its JSON text as binary bytes.

The configuration mutation now uses the ordinal Pack operation codec used by the Note document mutation and the FEM model window configuration. The derived variant binds the `set-camera` payload to the existing three-field camera record. The original receiving macro constructs the same paged operation cursor that reads the ordinal Pack format. Its derived receiving selector opens a retained cursor for the camera and wraps the payload after the three fields have been received. The native cursor uses the caller's original work authority and physical grant.

The JSON text representation and the transient mutation's codecs continue to use their original implementations. The existing language-neutral window fixture now exercises the original camera operation receiving law with poses independently decoded through serde_json. The JS ownership oracle passed its Ajv validation, foreign-field refusal, JSON Patch check, and exact-window isolation checks. That oracle validates window state semantics. The Note staging run and native fixture are the Rust checks.

## Changed source

- Add the typed variant and ordinal Pack codecs. [Window configuration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:37)
- Derive the retained receiver for the existing camera record. [Note camera schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:497)
- Run the original receiving law on camera mutations obtained from the neutral fixture. [Window unit fixture](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:79)

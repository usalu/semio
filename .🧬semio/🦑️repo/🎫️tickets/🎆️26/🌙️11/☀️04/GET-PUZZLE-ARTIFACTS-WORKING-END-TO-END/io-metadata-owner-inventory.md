# IO Metadata Owner and Constructor Inventory

Read-only scoped source inventory; no implementation/executor.

## Complete metadata already retained

Actual IoEntry in OS io/io-mechanism retains from/into, fidelity, direction, sniff function, run and owned_serializer. serializer_entry constructors use T::FIDELITY and no sniff; deserializer_entry uses T::FIDELITY and the actual deserializer_sniff function (including when the implementation returns no confidence). io_register stores references to these original static entries after atomic proposed/preflight validation; same_io_entry compares all properties. No metadata is lost at typed declarations or registration.

Existing descriptor_of(entry) line142 already projects full framework IO-schema IoEntryDescriptor {from,into,fidelity,sniffs:entry.sniff.is_some()}. io_entries() line857 publishes these plus synthesized native↔SQLite descriptors. Those synthesized routes are legitimate but are explicitly not the same as registered-only native export/import contributions; blindly replacing the latter with io_entries() changes roster semantics. Existing io_native_routes() line881 visits registered entries only, preserving native/foreign/direction but dropping fidelity/sniffs at that projection.

## Actual publication and consumption

Plugin describe consumes io_native_routes into manifest IoEntryDescriptor {owner,counterpart,direction}; it also inserts composer-import rows into that list, while retaining those same composer declarations separately in composer_entries. MCP catalog uses the manifest view; generic typed host IoRouter requires the full IO-schema view. OS Run currently supplies an empty typed slice. Full properties can be projected directly from actual registry rows without guessed defaults. Composer-only rows have no demonstrated full IoEntry fidelity/sniff source, so extending the shared manifest row with invented values for that branch would be unsound. Retain actual composer metadata in its existing composer channel; schema-first distinguish genuine executable registered-route rows if the owning contract changes.

## Constructor and schema edit frontier

If extending the native-side route publication: IoNativeRoute declaration and its two actual constructor arms (io-mechanism888–889) are the complete scoped Rust constructor frontier. Preserve actual entry.fidelity and sniff presence at origin.

If extending/changing the manifest row: actual definition is framework/modules/manifest/plugin-manifest431; production literals are plugin/describe141 (composer-derived) and153 (native-derived). The current unit expected literals are plugin/describe/tests/unit31–32. The manifest TypeScript schema projection string is schema/projection/rows-t-z706; it currently prints only owner/counterpart/direction. MCP capability-builders implements ContributionRow for that manifest type, and catalog-compile consumes it. Existing committed JSON/pack descriptors and independent neutral fixtures must be qualified through normal ownership, not blanket compatibility defaults.

The distinct full IO-schema row is framework/modules/io/schema191 with TS interface/parser schema TS51/62. Its actual constructors already provide all four current fields: descriptor_of143, snapshot_descriptor232, host router resolved-hop106 and the host/transfer/Sequence carrier test literals. They are not missing metadata and need not be churned merely because the manifest publication is strengthened. Avoid conflating two same-named structs.

General declaration serializers and native route registry are the single source of fidelity/sniff truth. Minimal responsibility work is to preserve that truth across the actual semantic descriptor publication contract, then consume it in OS Run registration and canonical export discovery. Do not hand-maintain20 Puzzle rows, assign assumed Exact/false, or claim metadata is lost at registration itself. Source caps, strict pairs and actual native/browser gates remain unchanged.

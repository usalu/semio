# Retained Buffer Raw Array Audit

The Source-only inventory in `drawing-retained-buffer-callers.json` contains 25 direct helper call sites across the neutral owner, Font producers, Drawing text and its controller test. No frozen TypeScript primitive inputs were edited.

The generic fallback accepting every `Array` is not established ownership authority. Most live payloads are defining RetainedWorkBuffer owners despite array-shaped public types: font tables, selected kerning indices, outline frame ends/flags/points, Unicode outputs, mapping glyphs and shaped run outputs. Empty `[]` defaults can reach archival cleanup before a producer output exists. These need explicit empty-state treatment rather than blanket ownership approval.

A concrete owned raw-array case is FontContextMatchJob offsets/counts/classDefs initialized as `[0,0,0]` and passed to retireRetainedWorkBuffers. They are mutable fixed control metadata owned by the job, not externally retained borrowed data. Their layout should become explicit fixed scalar header fields/records and the close path should retire that header with actual item/depth/CPU, without pretending to release a retained backing. This requires coordinated producer edits; the current fallback cannot justify them.

The nested helper retireRetainedWorkMatcher also forwards any array or records array, so its actual call chain needs explicit ownership narrowing. RetainedWorkChildren and archive callbacks must retain real child output identities until their defining owned buffers close. No whole Font/Drawing custody readiness is claimed.

A malformed outline retirement callback parameter from the earlier authoring pass was repaired by removing a stray `type RetainedWorkTurn` token from the parameter list. Only that callback signature was changed; producer methods and frozen core/counter/bytes inputs were preserved.

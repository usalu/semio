# IFC2x3 Typed Details Plan

## Existing Own Types and Authority

IFC2x3 uses its own Ifc2x3Snapshot with document.instances, each containing a positive-u64 identity and one or more Part21 entity constituents. Its values retain arbitrary decimal coefficient/scale/exponent, integers, references, lists and typed values. This is a different first-party model from IFC4; no snapshot conversion or IFC4 value alias will be introduced.

The current editor exposes generic Details. Its existing mutation authority is UpsertInstance, RemoveInstance and SetHeader. Existing EditRules map edits below a complete instance to UpsertInstance; identity renames use the current explicit upsert/remove pair. Complex constituents already exist in the owned model as instance.entities.

## Bounded Implementation

1. Reuse schema-neutral catalog metadata queries already shared by all versions; IFC2x3 controls select the pinned 653-entity TC1 catalog. Extract declaration-only control construction so providers can share metadata without sharing value identity.
2. Add an IFC2x3-owned provider walking /document/instances/row/entities/constituent/1-or-arguments positions according to the actual native/TypeScript serialization shape. Simple instances use inherited slots; complex constituents use direct slots and effective inherited derived redeclarations.
3. Keep exact decimal reference fields synthetic only at the provider boundary; normalize them to own Part21Value::Ref and emit the existing UpsertInstance primitive. Verify target types against all target constituents, optional clearing and SET/UNIQUE/cardinality constraints.
4. Supply schema-native enum/select/create/unset/type controls. Entity type changes preserve instance identity and compatible arguments, then emit one own UpsertInstance. Exact decimal values remain exact throughout. No independent publisher.
5. Thread the original caller through expensive scans/planning and retained copying where first-party source APIs support it; refuse bounded unsupported publication sizes rather than silently truncate.

## Required Evidence

Neutral vectors must distinguish IFC2x3 IfcWall’s eight inherited attributes/no PredefinedType, IfcProject inheriting IfcObject, unbounded IfcLabel STRING, own closed enums/selects, exact u64 references and decimal retention. Independent existing AJV can validate declared control domains; authored IfcOpenShell metadata tests remain unexecuted where unavailable. Native syntax checks and authored actual editor/central inverse assertions must be labeled separately from native execution under the current cache gate.

## Selection Extension

Proceed with own topology domain and ifc-ID identity matching geometry. A selected-instance provider delegates to own typed Details by translating virtual /ifc-ID paths into /document/instances/current-row paths under the initiating snapshot. Original-context stable-ID lookup must precede the existing own mutation planner, and a viewer must render the same provider as read only. This is a native first-party surface over the own model; no IFC4 graph adapter.

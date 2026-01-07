# Previously

The C# codebase was out of date compared to the JavaScript/TypeScript implementation. Schema mismatches existed in diff classes, and tests were not passing.

# Plan

1. Fix schema mismatches between C# and JS for all diff classes
2. Fix `Design.Flatten` method bugs (inconsistent dictionary keys)
3. Fix Expression tests for cross-framework compatibility (net7.0/net48)
4. Ensure all 62 tests pass on both .NET 7.0 and .NET 4.8

# Changes

## Schema Fixes (Semio.cs)

1. **DiffUpdate<T> Wrapper Class**: Created generic wrapper for `{ Id, Diff }` structure used in `*sDiff.Updated` properties

2. **TypeDiff Changes**:
   - Changed `Models` from `List<Model>` to `ModelsDiff?`
   - Changed `Connectors` from `List<Connector>` to `ConnectorsDiff?`
   - Added `MergeDiff` methods to `ModelsDiff` and `ConnectorsDiff`
   - Added helper methods `ApplyModelsDiff`, `ApplyConnectorsDiff` in Type class

3. **KitDiff Changes**:
   - Changed `Attributes` from `List<Attribute>` to `AttributesDiff?`
   - Changed `Authors` from `List<Author>` to `AuthorsDiff?`
   - Changed `Interfaces` to use proper `InterfacesDiff?`
   - Updated `Kit.ApplyDiff` and implicit operators

4. **New Diff Classes**:
   - `AttributeDiff` with Guid property
   - `AttributesDiff` with Added, Removed, Updated
   - `AuthorDiff` for author diffs
   - `AuthorsDiff` for multiple author diffs

5. **Added Guid to Attribute class**

## Bug Fixes (Semio.cs)

1. **Design.Flatten Dictionary Key Mismatch**:
   - Fixed `connectors` dictionary - was built with `type.Guid` as key but accessed with `type.Name`
   - Changed `connectors[connectedType.Name]` to `connectors[connectedType.Guid]` and similar

2. **Expression.Calculate Return Type**:
   - Fixed to return `UnitValue` instead of raw `float` when `targetUnit` is specified
   - Ensures proper locale-independent string formatting

3. **UnitValue.ToString() Precision**:
   - Changed from default `ToString()` to `ToString("G9", CultureInfo.InvariantCulture)`
   - Ensures consistent precision across .NET 7.0 and .NET Framework 4.8

## Test Fixes (Tests.cs)

1. **Added AssertUnitValueEqual Helper**:
   - Tolerance-based comparison for unit value strings
   - Parses numeric and unit parts, compares numbers with 0.01% tolerance
   - Falls back to exact comparison for non-unit values

2. **Updated Expression Tests**:
   - All Expression tests now use `AssertUnitValueEqual` instead of `Assert.Equal`
   - Works correctly on both .NET 7.0 and .NET Framework 4.8

## Test Results

All 62 tests pass on both frameworks:

- .NET 7.0: 62/62 passed
- .NET 4.8: 62/62 passed

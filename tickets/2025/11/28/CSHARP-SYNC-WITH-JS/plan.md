# Previously

C# codebase was out of date with JS schema. Key issues:

- `Type.Parent` and `Design.Parent` were `string?` but JSON uses `{ guid: "..." }` objects
- `Connection.X/Y` should be `U/V` (lowercase, nullable float)
- `Connector.CompatibleInterfaces` was obsolete (only valid on `Interface` class)
- JS unit tests in `semio.test.ts` use fixtures from `assets/semio/` folder

# Plan

1. ✅ Analyze JS test structure in `js/semio/semio.test.ts`
2. ✅ Identify JSON fixtures in `assets/semio/`
3. ✅ Fix C# schema: Parent fields → TypeId?/DesignId?
4. ✅ Fix Connection/ConnectionDiff: X/Y → U/V (float?)
5. ✅ Remove obsolete CompatibleInterfaces from Connector/ConnectorDiff
6. ✅ Add HashCode polyfill for .NET Framework 4.8 compatibility
7. ✅ Rewrite C# Tests.cs matching JS test structure
8. ✅ Verify all tests pass on net7.0
9. ✅ Verify both net7.0 and net48 build successfully

# Changes

## `net/Semio/Semio.cs`

- Changed `Type.Parent` from `string?` to `TypeId?`
- Changed `TypeDiff.Parent` from `string?` to `TypeId?`
- Changed `Design.Parent` from `string?` to `DesignId?`
- Changed `DesignDiff.Parent` from `string?` to `DesignId?`
- Renamed `Connection.X` → `Connection.U`, `Connection.Y` → `Connection.V` (changed type from `float` to `float?`)
- Renamed `ConnectionDiff.X` → `ConnectionDiff.U`, `ConnectionDiff.Y` → `ConnectionDiff.V`
- Removed `CompatibleInterfaces` property from `Connector` class
- Removed `CompatibleInterfaces` property from `ConnectorDiff` class
- Added `HashCode` polyfill struct (conditional compilation for NET48)

## `net/Semio/Semio.csproj`

- Added `<DefineConstants Condition="'$(TargetFramework)' == 'net48'">$(DefineConstants);NET48</DefineConstants>`

## `net/Semio.Tests/Tests.cs`

- Complete rewrite with 14 tests matching JS semio.test.ts structure:
  - `KitTests`: DeserializeKitMetabolism, SerializeKitMetabolism, DiffKitMetabolism, ValidateKitInvalid
  - `FlattenDesignTests`: 10 tests for Nakagin variants and Capsule Dream designs
- Uses JSON fixtures from `assets/semio/` folder

## Test Results

- All 14 tests passing on net7.0
- Both net7.0 and net48 targets build successfully

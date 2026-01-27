# Plan: Fix Policy Violation Kinds Not Showing in Tree

## Problem Analysis

When policies are fetched via GraphQL, the `violationKinds` field is returning empty arrays. This causes no violation kinds to appear as children when expanding a policy in the VS Code extension tree view.

**Root Cause**: In `go/repo/main.go`, the `repoContext.GetPolicies()` function at line 11315 creates `Policy` objects but does NOT populate the `ViolationKinds` field:

```go
result[i] = &Policy{
    ID:          policies[i].ID,
    Name:        policies[i].Name,
    Description: descPtr,
    Scopes:      policies[i].Scopes,
    // ViolationKinds is missing!
}
```

The `PolicyDef` struct has a `Kinds []ViolationKind` field that contains the violation kinds, but this is not being converted to `[]*ViolationKindMeta` for the Policy.

## Solution

Update `repoContext.GetPolicies()` to build the `ViolationKinds` field by converting each `ViolationKind` in the `PolicyDef.Kinds` to a `ViolationKindMeta` using the `kind.Info()` method.

## Implementation

1. In `GetPolicies()`, iterate over `policies[i].Kinds`
2. For each kind, call `kind.Info()` to get the `ViolationKindMeta`
3. Set `PolicyID` on each meta
4. Append to a slice and assign to `result[i].ViolationKinds`

## Files to Modify

- `go/repo/main.go` - Update `GetPolicies()` function around line 11324

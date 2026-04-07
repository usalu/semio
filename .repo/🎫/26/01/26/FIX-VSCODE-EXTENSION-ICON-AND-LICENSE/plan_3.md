# Plan: Fix Policy Breach Kinds Not Showing in Tree

## Problem Analysis

When policies are fetched via GraphQL, the `statutes` field is returning empty arrays. This causes no statutes to appear as children when expanding a policy in the VS Code extension tree view.

**Root Cause**: In `./repo/cli/main.go`, the `repoContext.GetPolicies()` function at line 11315 creates `Policy` objects but does NOT populate the `Statutes` field:

```go
result[i] = &Policy{
    ID:          policies[i].ID,
    Name:        policies[i].Name,
    Description: descPtr,
    Scopes:      policies[i].Scopes,
    // Statutes is missing!
}
```

The `PolicyDef` struct has a `Kinds []Statute` field that contains the statutes, but this is not being converted to `[]*StatuteMeta` for the Policy.

## Solution

Update `repoContext.GetPolicies()` to build the `Statutes` field by converting each `Statute` in the `PolicyDef.Kinds` to a `StatuteMeta` using the `kind.Info()` method.

## Implementation

1. In `GetPolicies()`, iterate over `policies[i].Kinds`
2. For each kind, call `kind.Info()` to get the `StatuteMeta`
3. Set `PolicyID` on each meta
4. Append to a slice and assign to `result[i].Statutes`

## Files to Modify

- `./repo/cli/main.go` - Update `GetPolicies()` function around line 11324

# Refactor plan: remove *Meta* types and keep a single source of truth (Go)

This plan removes `*Meta` structs entirely and leaves **one type per concept**:
- `Policy` (metadata + runnable implementation)
- `ViolationKind` (enum) with a method-based “descriptor” API (no `ViolationKindMeta`)

It also removes duplicated registries (`policyMetas`, `policyFuncs`, `violationKindMetas`) so nothing can drift.

---

## Goals

1. **Delete all `*Meta` types** (`PolicyMeta`, `ViolationKindMeta`, etc.).
2. **Single source of truth**:
   - Policies are defined once (metadata + `Run`).
   - Violation kind “metadata” is obtained from `ViolationKind.Info()` rather than an exported meta type.
3. **No legacy API compatibility** — rename freely, simplify call sites.

---

## Overview of the new shapes

### Policy

One struct holds everything needed to run and to describe.

```go
type Policy struct {
    ID          string
    Name        string
    Description string
    Scopes      []string
    Priority    ViolationPriority
    Kinds       []ViolationKind

    Run PolicyFunc // function implementation
}
```

### ViolationKind

Keep the enum type (string) for readability/serialization, but replace `ViolationKindMeta`
with a descriptor method returning a “single concept” struct.

```go
type ViolationKind string

type ViolationKindInfo struct {
    Kind        ViolationKind
    Priority    ViolationPriority
    Reason      string
    Solution    string
    Autofixable bool
}

func (k ViolationKind) Info() ViolationKindInfo
```

---

## Step-by-step changes

### 1) Remove Policy meta plumbing and registries

**Delete:**
- `type PolicyMeta ...`
- `type RegisteredPolicy ...`
- `var policyMetas ...`
- `var policyFuncs ...`
- any “join” logic that matches IDs between meta and funcs

**Create:**
- `type Policy struct { ... }` (as above)
- `var policies = []Policy{ ... }` as the single registry

**Migration instructions:**
Replace each former pair:
- `policyMetas` entry for `"code"`
- `policyFuncs["code"] = codePolicy`

with a single `Policy` entry:

```go
var policies = []Policy{
    {
        ID: "code",
        Name: "Code",
        Description: "...",
        Scopes: []string{ ... },
        Priority: PriorityLow,
        Kinds: []ViolationKind{
            // ...
        },
        Run: codePolicy,
    },
    // ...
}
```

**Checklist:**
- Ensure every former `policyMetas` entry becomes exactly one `Policy{...}`.
- Remove any `panic("missing policy func")`-style guards that existed only due to split registries.

---

### 2) Simplify policy selection & execution

Update any “get policy by id” logic from map lookups on `policyFuncs` to either:

**Option A: iterate `policies`** (simplest, often fine)

```go
func FindPolicy(id string) (Policy, bool) {
    for _, p := range policies {
        if p.ID == id {
            return p, true
        }
    }
    return Policy{}, false
}
```

**Option B: build a map once** (if called frequently)

```go
var policiesByID = func() map[string]Policy {
    m := make(map[string]Policy, len(policies))
    for _, p := range policies {
        m[p.ID] = p
    }
    return m
}()
```

Then selection:

```go
p, ok := policiesByID[id]
if !ok { /* unknown policy */ }
```

Update `CheckPolicies` (or equivalent):
- Replace `RegisteredPolicy`/`PolicyMeta` usage with `Policy`.
- Call `p.Run(ctx)` directly.

---

### 3) Remove ViolationKindMeta and move “meta” behind a method

**Delete:**
- `type ViolationKindMeta ...`
- `GetViolationKindMeta(...)` (if it returns that meta)
- `violationKindMetas map[ViolationKind]ViolationKindMeta`

**Create:**
- `type ViolationKindInfo struct { ... }`
- `func (k ViolationKind) Info() ViolationKindInfo`
- `violationKindInfoTable` as the single registry for kind descriptors

Implementation pattern:

```go
type ViolationKindInfo struct {
    Kind        ViolationKind
    Priority    ViolationPriority
    Reason      string
    Solution    string
    Autofixable bool
}

var violationKindInfoTable = map[ViolationKind]ViolationKindInfo{
    ViolationCodeHeaderMissingRegion: {
        Kind: ViolationCodeHeaderMissingRegion,
        Priority: PriorityLow,
        Reason: "...",
        Solution: "...",
        Autofixable: false,
    },
    // ...
}

func (k ViolationKind) Info() ViolationKindInfo {
    if info, ok := violationKindInfoTable[k]; ok {
        if info.Kind == "" { // if you decide to omit Kind in entries later
            info.Kind = k
        }
        return info
    }
    return ViolationKindInfo{
        Kind: k,
        Priority: PriorityLow,
        Reason: "Unknown violation kind",
        Solution: "Fix the violation",
        Autofixable: false,
    }
}
```

> Optional: To avoid repeating `Kind:` in each entry, store a `map[ViolationKind]struct{ ... }`
and fill `Kind` at read time.

---

### 4) Update all call sites to use `.Info()`

Mechanical replacements:

- `GetViolationKindMeta(v.Kind)` → `v.Kind.Info()`
- `GetViolationKindMeta(kind)` → `kind.Info()`

Then update field references:

- `meta.Priority` → `info.Priority`
- `meta.Reason` → `info.Reason`
- `meta.Solution` → `info.Solution`
- `meta.Autofixable` → `info.Autofixable`

Tip: in loops, compute once:

```go
info := v.Kind.Info()
```

---

### 5) JSON and external output structs (if any)

If you previously returned `PolicyMeta` via JSON, just return `Policy` now, and exclude `Run`:

```go
type Policy struct {
    ID          string            `json:"id"`
    Name        string            `json:"name"`
    Description string            `json:"description"`
    Scopes      []string          `json:"scopes"`
    Priority    ViolationPriority `json:"priority"`
    Kinds       []ViolationKind   `json:"kinds"`
    Run         PolicyFunc        `json:"-"`
}
```

If your UI needs kind descriptions, return `ViolationKindInfo` values (e.g., as part of a report).

---

### 6) Enforce “single registry” invariants with tests

Add tests to prevent drift:

1) **All policy kinds are described**
- Iterate all policies and their `Kinds`.
- Assert `kind.Info().Reason != "Unknown violation kind"` (or check presence in table).

2) **No unused kind descriptors**
- Optionally build `usedKinds` set from policies.
- Ensure every entry in `violationKindInfoTable` appears in `usedKinds`.

---

## Execution order (safe sequence)

1. Introduce `Policy`, `ViolationKindInfo`, and `ViolationKind.Info()` while leaving old code in place.
2. Add `policies []Policy`, migrate policy definitions into it.
3. Update policy runner/selection to use `policies`.
4. Update violations/reporting to use `kind.Info()`.
5. Delete old `*Meta` types and old registries.
6. Run `gofmt ./...`, `go test ./...`, `go vet ./...`.

---

## Mechanical “find and delete” list

- `type PolicyMeta`
- `type RegisteredPolicy`
- `policyMetas`
- `policyFuncs`
- any `getRegisteredPolicies` / “join metas with funcs” helper
- `type ViolationKindMeta`
- `GetViolationKindMeta`
- `violationKindMetas` (replace with `violationKindInfoTable`)

---

## End state checklist

- ✅ One policy registry: `var policies []Policy`
- ✅ Policy carries its own runnable implementation: `Policy.Run`
- ✅ ViolationKind has a single descriptor API: `ViolationKind.Info()`
- ✅ No `*Meta` types remain
- ✅ No duplicated “meta vs func” registries remain

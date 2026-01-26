# Log: Extend Dependabot with Go and Rust

## 2026-01-14

### Task
Extend dependabot configuration to include Go and Rust package ecosystems.

### Investigation
- Found existing dependabot.yml at `.github/dependabot.yml`
- Identified Go modules in: `/go/cli`, `/go/mcp`, `/go/repo`, `/go/semio`
- Identified Rust crate in: `/rs/semio`

### Implementation
Updated `.github/dependabot.yml` to add:
- 4 gomod entries for Go modules
- 1 cargo entry for Rust crate
- All using weekly schedule consistent with existing entries

### Changes Made
- Modified `.github/dependabot.yml`

---

## 2026-01-14 (Follow-up)

### Issue
User still receiving dependency update PRs for:
- System.Collections.Immutable from 7.0.0 to 10.0.2
- Grasshopper from 8.10.24226.13001 to 8.26.25349.19001
- System.Drawing.Common from 7.0.0 to 10.0.2
- System.Resources.Extensions from 7.0.0 to 10.0.2

### Investigation
Checked current csproj files:
- `Semio.csproj` has `System.Collections.Immutable` at 9.0.10 (already past 7.0.0)
- `Semio.Grasshopper.csproj` has the other three packages

Ignore rules exist in dependabot.yml for `/net/Semio` and `/net/Semio.Grasshopper`.

### Root Cause
The PRs were likely created BEFORE the ignore rules were added to the config. Dependabot doesn't automatically close existing PRs when ignore rules are added - they remain open until manually closed or merged.

### Resolution
User needs to manually close the stale PRs. The ignore rules are correctly configured and future updates for these packages will be ignored.

---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Complete inventory of all cobra commands with validation logic
## Findings

### Complete Command Inventory (root → subcommands)

Root command: `repo` (line 313). All subcommands registered at lines 340-363.

---

### 1. `section delete` (line 3503)

- **Use:** `"delete"`
- **Args:** `cobra.MaximumNArgs(2)`
- **Flags:** `--file`, `--name`
- **Positional fallback:** `args[0]` → file, `args[1]` → name
- **Validation:** `if file == "" || name == "" { return fmt.Errorf("missing file or name") }`
- **Error:** `"missing file or name"`

### 2. `section extract` (line 3691)

- **Use:** `"extract"`
- **Args:** `cobra.MaximumNArgs(3)`
- **Flags:** `--source-file`, `--source-section`, `--target-file`
- **Positional fallback:** `args[0]` → sourceFile, `args[1]` → sourceSection, `args[2]` → targetFile
- **Validation:** `if sourceFile == "" || sourceSection == "" || targetFile == "" { return fmt.Errorf("missing source file, source section, or target file") }`
- **Error:** `"missing source file, source section, or target file"`

### 3. `section integrate` (line 3650)

- **Use:** `"integrate"`
- **Args:** `cobra.MaximumNArgs(4)`
- **Flags:** `--source`, `--target-section`, `--target-file`, `--target-parent`
- **Positional fallback:** `args[0]` → source, `args[1]` → targetSection, `args[2]` → targetFile, `args[3]` → targetParent
- **Validation:** `if source == "" || targetSection == "" || targetFile == "" { return fmt.Errorf("missing source, target section, or target file") }`
- **Error:** `"missing source, target section, or target file"`

### 4. Top-level `move` (line 3784)

- **Use:** `"move <source> <target>"`
- **Args:** `cobra.ExactArgs(2)` (cobra enforces exactly 2 args)
- **No flags** (uses ArtifactRef parsing)
- **Validation errors within RunE:**
  - `if len(source.SectionParts) == 0 || len(target.SectionParts) == 0 { return fmt.Errorf("missing section path") }` (section→section same-file)
  - `if len(source.SectionParts) == 0 { return fmt.Errorf("missing section path") }` (section→file)
  - `return fmt.Errorf("unsupported move: %s → %s", source.Kind, target.Kind)` (default case)

### 5. Top-level `extract` (line 3914)

- **Use:** `"extract [source] [target]"`
- **Args:** `cobra.MaximumNArgs(2)`
- **Flags:** `--file`, `--section`, `--parent-section`, `--target-file`
- **Two modes:**
  - **ArtifactRef mode (2 args):** Parses `args[0]` as source, `args[1]` as target. If `source.Kind == "section" && target.Kind == "file"`, validates: `if len(source.SectionParts) == 0 { return fmt.Errorf("missing section path") }`
  - **Flag mode:** `if file == "" || section == "" || targetFile == "" { return fmt.Errorf("missing file, section, or target-file") }`
- **Errors:** `"missing section path"` or `"missing file, section, or target-file"`

### 6. Top-level `integrate` (line 3850)

- **Use:** `"integrate [source] [target]"`
- **Args:** `cobra.MaximumNArgs(2)`
- **Flags:** `--file`, `--target-file`, `--target-section`, `--parent-section`
- **Two modes:**
  - **ArtifactRef mode (2 args):** Parses `args[0]` as source, `args[1]` as target. If `source.Kind == "file" && target.Kind == "section"`, runs directly (no additional validation beyond ToolIntegrate result).
  - **Flag mode:** `if file == "" || targetFile == "" || targetSection == "" { return fmt.Errorf("missing file, target-file, or target-section") }`
- **Error:** `"missing file, target-file, or target-section"`

### 7. `goal change` (line 1920)

- **Use:** `"change <SLUG>"`
- **Args:** `cobra.ExactArgs(1)` (cobra enforces exactly 1 arg)
- **Flags:** `--title`, `--description`, `--due-date`, `--parent`, `--no-github`
- **No additional validation** beyond cobra's ExactArgs. All flags are optional; only changed flags are included in GraphQL input via `cmd.Flags().Changed()`.

### 8. `bundle list` (line 2553) / `bundle tree` (line 2601)

- **Use:** `"list"` / `"tree"`
- **Args:** None specified (default: any number)
- **No argument validation.** Uses stream flags + optional `--status` filter. No `return fmt.Errorf(...)`.

---

### All Other Commands with Validation

#### `section create` (line 3444)
- **Use:** `"create"`, **Args:** `MaximumNArgs(3)`
- **Flags:** `--file`, `--name`, `--parent`
- **Error:** `"missing file or name"`

#### `section move` (line 3475)
- **Use:** `"move"`, **Args:** `MaximumNArgs(3)`
- **Flags:** `--file`, `--old`, `--new`
- **Error:** `"missing file or names"`

#### `section list` (line 3526)
- **Use:** `"list [file]"`, **Args:** `MaximumNArgs(1)`
- **Error:** `"missing file"`

#### `graphql` (line 448)
- **Use:** `"graphql [query]"`, **Args:** `MaximumNArgs(1)`
- **Flags:** `--query`, `--vars`
- **Errors:** `"missing query"`, `"invalid variables JSON: %w"`

#### `mcp` (line 415)
- **Use:** `"mcp"`, supports `--dry-run`
- **No validation**

#### `policy check` (line 842)
- **Use:** `"check"`, **Args:** `MaximumNArgs(2)`
- **Flags:** `--id`, `--scope`
- **Error:** `"missing policy id"`

#### `draft create` (line 1052)
- **Use:** `"create [slug]"`, **No Args constraint**
- **Error:** `"missing slug"` (via `len(args) < 1`)
- **NOTE:** Draft command is **defined but never registered** to root (not wired via AddCommand)

#### `draft delete` (line 1079)
- **Use:** `"delete [slug]"`, **No Args constraint**
- **Error:** `"missing slug"` (via `len(args) < 1`)
- **NOTE:** Not wired

#### `todo create` (line 1130)
- Validation via GraphQL, no explicit arg validation in cobra

#### `folder create` (line 2668)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing path"`

#### `folder move` (line 2686)
- **Args:** `MaximumNArgs(2)`, **Error:** `"missing source or target"`

#### `folder delete` (line 2711)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing path"`

#### `file create` (line 3211)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing path"`

#### `file move` (line 3229)
- **Args:** `MaximumNArgs(2)`, **Error:** `"missing source or target"`

#### `file delete` (line 3254)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing path"`

#### `definition list` (line 3739, aliases: `tree`)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing file"`

#### `contributor add` (line 2461)
- **Args:** `MaximumNArgs(3)`, **Error:** `"missing github"`

#### `contributor remove` (line 2498)
- **Args:** `MaximumNArgs(1)`, **Error:** `"missing github"`

#### `export` (line 783)
- **Args:** `MaximumNArgs(1)`, **No validation** (output path defaults)

#### `preflight` (line 26887)
- **Args:** none, **Error:** `"unknown command: %s"` for invalid subcommand

---

### Commands With No Argument Validation

These take no required args or have no `fmt.Errorf` validation:

| Command | Line | Subcommands |
|---|---|---|
| `sync github` | 403 | No args |
| `analyze` | 489 | Optional scope |
| `fix` | 524 | Optional scope |
| `tree` | 552 | Optional query |
| `policy list` | 814 | Stream only |
| `policy tree` | 890 | Stream only |
| `todo create/change/delete/search` | 1130+ | GraphQL-delegated |
| `ticket open/close/reopen/change` | 1262+ | Complex flag handling, GraphQL-delegated |
| `goal open/close/reopen` | 1968+ | GraphQL-delegated |
| `goal list/tree` | 1880+ | Stream only |
| `violationKind list/tree` | 2322+ | Stream only |
| `commit list` | 2396 | Stream + limit flag |
| `contributor list` | 2433 | Stream only |
| `project list/tree` | 2528+ | Stream only |
| `bundle list/tree` | 2553+ | Stream only |
| `folder list/tree` | 2730+ | Optional scope |
| `file list/tree` | 3273+ | Optional scope |
| `section tree` | 3566 | Optional file |
| `benchmark` | 26695 | `--dry-run` |
| `update` | 26968 | `--dry-run`, `--apply` |

## Changes

## Log

## Todos

## Plan

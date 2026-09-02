# Artifact Directory Name → Module Path Mapping

## 1. Rust Module Wiring: Mapping Directory Names to Module Paths

### Core Mechanism: Explicit `#[path]` Attributes
The artifact directory structure maps to Rust modules through explicit `#[path]` attributes in the lib.rs file. **No build.rs or codegen script exists** — the mapping is entirely hand-written.

**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/🦀️.rs`

**Example Mapping (Lines 31-38):**
```rust
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod present {
        #[path = "../../🗿️artifacts/🎬️present/🦀️.rs"]
        mod component;
        pub use component::*;
```

**Transformation Rules:**
1. Directory emoji prefix is **stripped**: `🎬️present` → module name `present`
2. Trailing slash notation (`#[path = "."]`) groups nested modules without splicing the module name into the directory path
3. The file reference uses full relative path from the lib.rs location to the artifact file: `../../🗿️artifacts/🎬️present/🦀️.rs`
4. Module path becomes: `crate::artifacts::present`

### Deeper Nesting Example (Lines 43-46):
```rust
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🦀️.rs"]
        mod component;
```
- Directory: `🏅️standards/🔖️1/` → Module path: `::standards::v1::`
- Emoji stripped: `🔖️1` → `v1`

### Module Declaration Chain for `artifacts::present::standards::v1::subsets::any::schema`:
**Line 51-58:**
```rust
pub mod subsets {
    #[path = "."]
    pub mod any {
        #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
        mod component;
        pub use component::*;
```

**File Path:** `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`

**Module Path:** `crate::artifacts::present::standards::v1::subsets::any::schema`

**Emoji Stripping:**
- `🏅️standards` → `standards`
- `🔖️1` → `v1`
- `🪆️subsets` → `subsets`
- `✳️any` → `any`
- `🧬️schema` → `schema`

### Cargo.toml Configuration
**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml`

**Line 30-32:**
```toml
[lib]
crate-type = ["cdylib", "rlib"]
path = "🦀️.rs"
```

Points to the lib file that contains all `mod` declarations with `#[path]` attributes.

---

## 2. TypeScript Module Wiring: Barrel/Index Pattern

### Direct Export with Emoji Stripping
**File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️.ts`

**Lines 1-8:**
```typescript
/** 📦️ animate facet WASM facades — mirrors the declaration-tree taxonomy */
export * as present_schema from "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts";
export * as present_io from "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts";
```

**Transformation Rules:**
1. Directory emoji prefix is **stripped**: `🎬️present` → `present`
2. Sub-directory emoji prefixes are **stripped**: `🧬️schema` → `schema`, `🚪️io` → `io`
3. Module name pattern: `<artifact_name>_<facet>` (kebab-case directory names become snake_case)
4. No index generation: exports are hand-written re-exports pointing directly to artifact TypeScript files (`🟦️.ts`)

### TypeScript Artifact File Pattern
**File:** `🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`

**Content (Lines 1-13):**
```typescript
export interface PresentArtifact {
  schema: string;
  source: FigureTileSource;
  tiles: FigureTileDraft[];
  selectedIds: string[];
  engagementInput: string;
  locale: string;
}
```

**Access Pattern:**
```typescript
import { PresentArtifact } from "@semio-tech/animate-js";
// Resolves to the re-exported `present_schema` namespace
const artifact: PresentArtifact = { ... };
```

---

## 3. Emoji Prefix Stripping Implementation

### Core Logic
**File:** `/Users/ueli/Documents/semio/📜️script.ts`

**Lines 22710-22712:**
```typescript
function policyStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}
```

**Function:** Removes all non-ASCII codepoints (emojis + variation selectors)

**Examples:**
- `"📐️cad"` → `"cad"`
- `"🗣️dsl"` → `"dsl"`
- `"🔖️1"` → `"1"`
- `"🎬️present"` → `"present"`

### Leading Emoji Extraction
**Lines 27889-27894:**
```typescript
function policyLeadingEmojiPrefix(name: string): string {
  const ascii = policyStripEmoji(name);
  if (!ascii) return name;
  const idx = name.indexOf(ascii);
  return idx > 0 ? name.slice(0, idx) : "";
}
```

Extracts the emoji prefix portion before the ASCII remainder.

---

## 4. Registry of Allowed File/Directory/Artifact Names

### Configuration Files (No Registry Found)
Searched:
- `.🧬semio/🦑️repo/📋️config.toml` — Contains only logging settings
- `.🧬semio/🦑️repo/🛂️manifest/*.cypher` — Neo4j database initialization files (no name registry)

### Validation Rules (In Code)
**File:** `/Users/ueli/Documents/semio/📜️script.ts`

**Line 22115:**
```typescript
if (!emoji || parts.length < 2 || parts.some((part) => !/^[a-z][a-z0-9]*$/u.test(part)))
  throw new Error(`new mutation: "${name}" must be an emoji-prefixed semantic verb-noun kebab name.`);
```

**Allowed Patterns:**
- Emoji-prefixed directory names (e.g., `🎬️present`, `🔖️1`)
- ASCII remainder: lowercase alphanumeric kebab-case (e.g., `present`, `resize-source-frame`)
- Reserved infrastructure directories: `📚️examples`, `💾️binary`, `📝️text`

**Comment (Line 22336-22341):**
> "emoji-tolerant way `new surface` resolves them; the final, NEW segment is taken literally (it must already carry the right emoji prefix and, for standard/subset, the taxonomy's dir prefix)"

---

## 5. Build/Codegen Strategy Summary

| Language | Mechanism | Location | Approach |
|----------|-----------|----------|----------|
| **Rust** | Explicit `#[path]` attributes | `📦️packages/🦀️rust/🦀️.rs` (lib.rs) | Hand-written module declarations |
| **TypeScript** | Barrel exports (re-exports) | `📦️packages/🟦️typescript/🟦️.ts` | Hand-written export aliases |
| **No Codegen** | N/A | N/A | Emoji stripping is policy-enforced at lint/compile time, not runtime generation |

**Key Insight:** The taxonomy is **declaratively specified** in source code (`#[path]` for Rust, barrel exports for TS), not **procedurally generated**. Directory names are automatically validated/stripped during linting.


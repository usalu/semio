# 🖨️ Registrar Handoff — Print Product De-Sandwich

De-sandwiched `🧰️framework/🛍️products/📓️print/` into Shape V2: `@semio-tech/print` at
`📦️packages/🟦️typescript/` (`semio.role = "framework"`, `semio.id = "print"`); LaTeX `.cls` / `.sty`
live as manifest-less components at `📓️print/🖋️latex/` (no LaTeX packages). The
`⚡️implementations` sandwich under print is fully deleted.

**OUTSIDE print tree — intentionally not edited** (per exclusive scope).

## 1. Root `package.json` — workspaces

Replace:

```
    "🧰️framework/🛍️products/📓️print/⚡️implementations/🟦️typescript",
```

with:

```
    "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript",
```

(line ~105 in current root `package.json`).

## 2. `bun.lock`

Regenerate after workspace path change (`bun install`).

## 3. Repo-wide consumers (none found in print tree)

`rg "📓️print/⚡️implementations"` outside ticket snapshots should be zero post-registrar.
`dev:print` / `build:print` in root `package.json` use nx project name `@semio-tech/print` — no path
change needed once workspace + lockfile are updated.

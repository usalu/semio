# Remaining Shared Chip Levels

Badge and StatusChip now paint glass at the level above their owner, publish that level and glass to descendants, and retain semantic ink and status dots. Neutral badges no longer paint a muted background. TabsList publishes the same next-level glass context to its tab descendants. IconShotFrame's dimension chip already painted at the next level; it now also publishes that scope. The remaining explicit chips in the React target were reviewed; named chrome overlay levels are intentional.

Changed paths:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🚦️status-surface/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🔌️host-ports/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🧩️component/boundary.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/surface-levels.feature`

Verification: the owned ui-react Vitest configuration passed both matching boundary files, 11 tests total, with the new level, descendant scope, neutral fill and semantic ink assertions. The first `repo test-file` invocation selected plain Bun and failed because that runner had no DOM; the owned jsdom configuration was then run successfully. The graph-based project test was cancelled while awaiting its shared graph lease. The existing semantic-ink contrast suite independently checks the corresponding ink tokens on chip glass in both appearances; it was inspected, not rerun here.

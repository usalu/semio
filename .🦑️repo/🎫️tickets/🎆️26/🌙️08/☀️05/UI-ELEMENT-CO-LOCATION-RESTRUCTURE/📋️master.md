# UI Element Co-location Restructure — master

Goal: `🎯aioptimizedrepo` · Ticket: `2026/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE`

## Wave status

| Wave | Status | Notes |
|------|--------|-------|
| W0 mechanisms + baselines | DONE | taxonomy/discovery/registry/storyGlobs/SFR exemption/barrel lint/dep-cruiser |
| W1 package moves | DONE | react/wgpu/tui under 📦️packages/…/🎯️targets |
| W2 Select pilot + schema core | DONE | TEMPLATE-UI.md written |
| W3 react/wgpu/tui element extraction | DONE (elements) / PARTIAL (react core still in barrel) | 55 TS leaves; 10 wgpu widget leaves; 12 tui leaves; barrel still ~25k lines of shared core |
| W4 renderer engine | DONE (packages + element splits) | react barrel ~1.2k; wgpu lib ~1.2k wiring-ish; W4-interim remains |
| W5 styling + assets | DONE (styling ×4) / PARTIAL (ui assets shape) | styling under 🎨️styling/📦️packages; 🖱️ui/🖼️assets flat (not packages/); framework-level 🖼️assets still sandwich |
| W6 activation + teardown | IN PROGRESS | blocked on W3/W4-interim=0 + barrels→wiring-only + areas flip |
| W7 story co-location | MOSTLY DONE | 51/75 moved; remainder need element homes or out-of-scope |

## Active claims (2026-08-06)

| Claim | Owner | File lock |
|-------|-------|-----------|
| StyleClasses core extract | subagent StyleClasses | ui-react 📦️index.tsx (shared — atomic) |
| Label/Surface/ElementProps core | subagent LabelSurface | ui-react 📦️index.tsx (shared — atomic) |
| Icons/Button/ContextMenu extract | subagent IconsButton | ui-react 📾️index.tsx (shared — atomic) |
| W3-interim resolvable rewire | done | 50 files / 178 symbols rewired to leaves |

## Gates before W6 close

- [ ] `rg W3-interim` = 0 under 🖱️ui
- [ ] `rg W4-interim` = 0 under renderer engine
- [ ] ui-react + renderer-react barrels ≤ wiring budget (or warn→hard justified)
- [ ] ui-wgpu lib.rs engine mods split to 🦀️<name>.rs + wiring-only entry
- [ ] taxonomy `areas` flip for ui + renderer-engine → `"taxonomy"`
- [ ] barrel/leaf validations warn→hard
- [ ] old path strings = 0; export-snapshot diff empty
- [ ] verify gate + launch.json regen no-op

## W6 update (2026-08-06)
- ui-wgpu engine split: lib.rs 17881 → 232 wiring; 24× 🦀️<name>.rs. cargo check --features wgpu green; full engine check pending external workspace churn.
- StyleClasses extraction attempted then REVERTED (see 🧪️w6-core-styleclasses.txt).
- Icons/Button/ContextMenu: inventory done; extraction resumed with barrel lock.

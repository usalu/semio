# Luna Fleet Scope Report

**Fleet:** 64 plugin crates + 1 extension
**Analysis Date:** 2026-08-20

## Executive Summary

- **Total fleet crates:** 64 (64 plugins + 1 extension)
- **Total diagnostic errors:** 51,851
- **Own errors (this crate):** 43,055
- **Inherited errors (🧰️framework/*):** 8,796
- **Await-eligible fraction:** 74.3%

### Key Findings

1. **Most plugins have zero own errors** — all ~100-200 errors per plugin are inherited from framework-plugin-host
2. **Exception: semio-s-plugin-stdio** — 43,055 own errors (73% await-eligible), dominates the repair cost
3. **Exception: semio-s-plugin-note** — 0 own errors, all 123 inherited from plugin-host
4. **Error distribution:**
   - E0277 (mismatched types): ~30%
   - E0308 (type mismatch): ~29%
   - E0599 (no method): ~24%
   - Other codes: ~17%

## Per-Crate Breakdown

| Crate | Total | Own | Inherited | Status | Await% |
|-------|-------|-----|-----------|--------|--------|
| semio-s-plugin-stdio | 44,102 | 43,055 | 1,047 | ok | 74% |
| semio-s-plugin-note | 123 | 0 | 123 | ok | 0% |
| semio-s-imperative-extension-sdk | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-animate | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-architect | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-block | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-cad | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-cad-aec-building | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-cad-aec-building-energy | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-cad-aec-building-structure | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-cad-spatial-shape | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-dag | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-demonstrator | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-draw | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-draw-fsm | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-draw-fsm-macros | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-energy | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-fem | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-bim | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-brep | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-dictionary | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-draw | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-list | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-logic | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-math | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-primitive | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-flow-extension-text | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-forms | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-gis | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative-control | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative-effect | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative-logic | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative-math | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-imperative-text | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-layout | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-lowpoly | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-mathematical | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-norm | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-playbook | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-playbook-procedural | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-procedural | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-process | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-process-concrete | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-process-metal | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-process-robotic | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-process-wood | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-puzzle | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-raster | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-reasoning-mindmap | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-remodel | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-sequence | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-shooting | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-sourcing | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-sourcing-beams | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-sourcing-slabs | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-sourcing-windows | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-space | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-trinity | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-trinity-jack-lsp | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-trinity-jack-shell | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-vcs | 123 | 0 | 123 | inherited | 0% |
| semio-s-plugin-writer | 123 | 0 | 123 | inherited | 0% |

## Proposed Batch Partition (≤8 batches)

Cost is weighted by repair difficulty: 100% await = 10% cost multiplier (mechanical), 0% await = 100% multiplier (design-heavy).

### Batch 1

**Repair Cost Estimate: 14264 units | Own Errors: 43,055 | Design-Heavy: 0 crates**

```
semio-s-plugin-stdio                          own=43,055 await= 74.3% 🟢 Low
```

### Batch 2

**Repair Cost Estimate: 0 units | Own Errors: 0 | Design-Heavy: 63 crates**

```
semio-s-plugin-note                           own=     0 await=  0.0% 🔴 High
semio-s-imperative-extension-sdk              own=     0 await=  0.0% 🔴 High
semio-s-plugin-animate                        own=     0 await=  0.0% 🔴 High
semio-s-plugin-architect                      own=     0 await=  0.0% 🔴 High
semio-s-plugin-block                          own=     0 await=  0.0% 🔴 High
semio-s-plugin-cad                            own=     0 await=  0.0% 🔴 High
semio-s-plugin-cad-aec-building               own=     0 await=  0.0% 🔴 High
semio-s-plugin-cad-aec-building-energy        own=     0 await=  0.0% 🔴 High
semio-s-plugin-cad-aec-building-structure     own=     0 await=  0.0% 🔴 High
semio-s-plugin-cad-spatial-shape              own=     0 await=  0.0% 🔴 High
semio-s-plugin-dag                            own=     0 await=  0.0% 🔴 High
semio-s-plugin-demonstrator                   own=     0 await=  0.0% 🔴 High
semio-s-plugin-draw                           own=     0 await=  0.0% 🔴 High
semio-s-plugin-draw-fsm                       own=     0 await=  0.0% 🔴 High
semio-s-plugin-draw-fsm-macros                own=     0 await=  0.0% 🔴 High
semio-s-plugin-energy                         own=     0 await=  0.0% 🔴 High
semio-s-plugin-fem                            own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow                           own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-bim             own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-brep            own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-dictionary      own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-draw            own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-list            own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-logic           own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-math            own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-primitive       own=     0 await=  0.0% 🔴 High
semio-s-plugin-flow-extension-text            own=     0 await=  0.0% 🔴 High
semio-s-plugin-forms                          own=     0 await=  0.0% 🔴 High
semio-s-plugin-gis                            own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative                     own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative-control             own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative-effect              own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative-logic               own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative-math                own=     0 await=  0.0% 🔴 High
semio-s-plugin-imperative-text                own=     0 await=  0.0% 🔴 High
semio-s-plugin-layout                         own=     0 await=  0.0% 🔴 High
semio-s-plugin-lowpoly                        own=     0 await=  0.0% 🔴 High
semio-s-plugin-mathematical                   own=     0 await=  0.0% 🔴 High
semio-s-plugin-norm                           own=     0 await=  0.0% 🔴 High
semio-s-plugin-playbook                       own=     0 await=  0.0% 🔴 High
semio-s-plugin-playbook-procedural            own=     0 await=  0.0% 🔴 High
semio-s-plugin-procedural                     own=     0 await=  0.0% 🔴 High
semio-s-plugin-process                        own=     0 await=  0.0% 🔴 High
semio-s-plugin-process-concrete               own=     0 await=  0.0% 🔴 High
semio-s-plugin-process-metal                  own=     0 await=  0.0% 🔴 High
semio-s-plugin-process-robotic                own=     0 await=  0.0% 🔴 High
semio-s-plugin-process-wood                   own=     0 await=  0.0% 🔴 High
semio-s-plugin-puzzle                         own=     0 await=  0.0% 🔴 High
semio-s-plugin-raster                         own=     0 await=  0.0% 🔴 High
semio-s-plugin-reasoning-mindmap              own=     0 await=  0.0% 🔴 High
semio-s-plugin-remodel                        own=     0 await=  0.0% 🔴 High
semio-s-plugin-sequence                       own=     0 await=  0.0% 🔴 High
semio-s-plugin-shooting                       own=     0 await=  0.0% 🔴 High
semio-s-plugin-sourcing                       own=     0 await=  0.0% 🔴 High
semio-s-plugin-sourcing-beams                 own=     0 await=  0.0% 🔴 High
semio-s-plugin-sourcing-slabs                 own=     0 await=  0.0% 🔴 High
semio-s-plugin-sourcing-windows               own=     0 await=  0.0% 🔴 High
semio-s-plugin-space                          own=     0 await=  0.0% 🔴 High
semio-s-plugin-trinity                        own=     0 await=  0.0% 🔴 High
semio-s-plugin-trinity-jack-lsp               own=     0 await=  0.0% 🔴 High
semio-s-plugin-trinity-jack-shell             own=     0 await=  0.0% 🔴 High
semio-s-plugin-vcs                            own=     0 await=  0.0% 🔴 High
semio-s-plugin-writer                         own=     0 await=  0.0% 🔴 High
```


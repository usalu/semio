# UI Accordion Zero-Consumer Audit

## Scope

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`
- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Semantic owner: framework UI React element `Accordion`
- Active scope excludes `compose`, `🌎️hub`, `♻️mit-bestand`, repository tickets/history, dependency caches, generated outputs, and taxonomy legacy/exempt areas.

## Evidence

| Path | Baseline SHA-256 | State |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🟦️component.tsx` | `b1be5b04a682aa3f40eb245bc07a36a721e9c19cdf844b3fc98496157ae55d81` | clean |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪗️Accordion/🧪️story.tsx` | `3d8b211775aa46966203aca0725b3b963d3e92c3b12aade2896f2e1a16a615af` | clean |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` | `01005e76dbc844cbaa2e9c8b2e6b7727bfd3d575f7ef887e62c3f1ce249c4a52` | dirty only from accepted serialized UI registrar removals |

The active-source import and identifier closure contains only:

1. The component implementation itself.
2. Its Storybook story, which is test/example evidence and does not count as a production consumer.
3. The mechanical React package import/re-export region.
4. An otherwise-unused `@radix-ui/react-accordion` namespace import in that same mechanical package surface.

No active production component, runtime mount, registry, direct source import, JSX consumer, or independent package consumer exists. The story and barrel do not increase the production-consumer count.

## Decision

`Accordion` has zero independent active production consumers. Delete its component and exclusive story, then remove its exact React import/re-export region and its now-dead package-level Radix namespace import. Do not create a module, compatibility export, alias, or replacement wrapper.

The shared React index remains coordinator-owned and must be edited only after rehashing the accepted serialized baseline. Validation requires active stale-reference scans, scoped ordinary/cached diff checks, UI React lint, typecheck, test-quick, and build; unrelated existing failures must be classified without broad repair.

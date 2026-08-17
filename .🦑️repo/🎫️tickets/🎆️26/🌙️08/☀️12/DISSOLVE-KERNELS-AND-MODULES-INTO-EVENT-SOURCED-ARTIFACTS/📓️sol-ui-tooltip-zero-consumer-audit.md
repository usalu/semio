# UI Tooltip Zero-Consumer Audit

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Component SHA-256: `991a8aef87f1f42236f0ff9deac758fdd5a8b86456342c293912b3e231fad5d7`, clean.
- Story SHA-256: `559ce277a52d8363821e1512cda8b719b977b23e1bae4e0ae42e5c1add3ff8e8`, clean.
- React index at audit time: `2b46ce80be9578c93625d27e26cca398761bac8b20861f24375dff0363ce239a`.

The React Tooltip family has zero active production consumers. The semantic component and its exclusive story feed a mechanical import/type-export region. The shared React index also owns unused `ComposeTooltip` and `IdComposeTooltip` wrappers plus a family export line; none is called by active production, test, or story code. Rust WGPU/OS tooltip overlays are separate native implementations and do not consume the React component. Generic translation text and comments are not consumers.

Decision: delete the component/story and remove the exact React import/type region, unused wrapper definitions/contracts, family export, and otherwise-unused package-level Radix namespace import. Queue the now-dead direct `@radix-ui/react-tooltip` dependency with Accordion/HoverCard for atomic Bun regeneration. No module, alias, replacement, or compatibility export is allowed.

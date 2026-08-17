# UI Orb Ring Inline Terra Packet

## Lease

- Baseline HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Orb component SHA-256 after ClassNames split: `ef40a085ec84797203ca379615c5d042888325511bfc289f7e31bf182c2f475f`
- Orb story SHA-256: `a6df96142c0e9057a05b4e3ed713730381566474320b6db16e8d2188ea54ca32`
- Ring component SHA-256 after ClassNames split: `b7028e41c7e3ac71efabcd37ceebd295caba06db224bdc16eec132768b65fb1e`
- Protected React barrel SHA-256: `9e24d693c415feaf14804482df1f24c76e33fbcec13d958630496453fb419838`
- Read applicable AGENTS, use `apply_patch`, and run no modifying Git command.

## Writable Source Scope

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🟦️component.tsx`
- delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔮️Orb/🟦️component.tsx`
- delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔮️Orb/🧪️story.tsx`
- unique ticket acceptance Markdown only

Do not edit the React barrel, Storybook spec, package manifests, locks, generated sources, or any other file.

## Implementation

Move Orb's contract and implementation into Ring as private code in the appropriate Ring region/subregion. Preserve exact geometry, pointer behavior, data attributes, classes, and disabled behavior. Remove Ring's direct Orb import. Delete the standalone component/story identity and add no compatibility export or module.

Stop after source edits and send final Ring hash plus confirmation that the protected barrel remains at the supplied hash. The coordinator will remove the explicit Orb/OrbProps barrel registration and return a new hash.

## Final Validation

- Zero active Orb path, import/export, public `Orb`/`OrbProps`, or standalone story references outside ticket history.
- Ring's private marker implementation and representative Ring behavior retained.
- Empty Orb directory removed only after confirming it has no dependency/cache files.
- Scoped ordinary/cached `git diff --check`.
- Run UI React lint, typecheck, test-quick, and build once through Nx; classify unrelated failures without repairing them.
- Record exact file inventory, hashes, commands, outcomes, and blockers in unique acceptance Markdown.

# Terra Packet: Navbar Example Selector Split

## Objective

Move the playground/example-selection responsibility out of the mixed Navbar component into its own maximally specific `NavbarExampleSelect` component. Preserve the package API and protected ShellHost consumer through mechanical registrar assembly; add no compatibility wrapper.

## Baseline

- Navbar: `b5f7e2b1c71cbd255e0f40aa462b41d18ee1de15422fad880d09e483de1e039b`.
- protected React barrel: `4e916cf18ad6c1a44961405f6adddb20b0a7383e3283af306f5c756e016ca52d`.
- Icons: `820f11dcb3ae80a618efbc7ed31593d996901bbe068c57bfd6947c8ca6159f82`.

## Writable Source Lease

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️component.tsx`
- new `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️component.tsx`
- one unique acceptance Markdown

Do not edit the protected barrel or product paths. Stop at registrar handshake.

## Implementation Contract

1. Move private sentinel/normalizer, `NavbarExampleOption`, `NavbarExampleSelectProps`, and `NavbarExampleSelect` intact to the new component.
2. Give the new file the repository header, direct adapters, and regions. Import Select wrappers directly, `cn` directly, `reactHostPort` directly, `UiLabel` directly, Label/useLabel directly, and `Icon`/`IconName` directly from Icons. Never route through the React barrel.
3. Preserve exact labels, filtering, sentinel normalization, option order, icon, ids, classes, include-no-example behavior, and callback values.
4. Remove selector-only imports from Navbar only after local usage proof. Keep Navbar, branding, item layout, and filler behavior byte-semantically unchanged.
5. Add no forwarding re-export from Navbar source and do not edit stories/products/tests.

## Coordinator Registrar

The coordinator will remove selector symbols from the Navbar import/export block and add a separate explicit `NavbarExampleSelect` component import/export block. The package names stay unchanged for ShellHost and inline tests.

## Gates

After registrar signal: old Navbar source has zero selector symbols/imports; new component owns all selector behavior; no new reverse barrel edge; protected product refs still resolve through unchanged package names; scoped ordinary/cached diff checks pass; run UI lint, typecheck, test-quick, build once and record exact outcomes/hashes.

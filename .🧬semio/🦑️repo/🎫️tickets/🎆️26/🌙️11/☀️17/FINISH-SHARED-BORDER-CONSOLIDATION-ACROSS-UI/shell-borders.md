# Navbar and footer borders

Navbar and Footer register their inward edges for shared border composition. The declared edge follows hover across the band and its nested toggles. The framework resolves the band and toggle edges together wherever they coincide.

The positioned startup edge is one CSS pixel high. It disappears once the shared border renderer attaches, and the band's bounds stay the same across hover and attachment.

## Verification

- The two React regression cases use the real shell bands and Toggle. Both failed before the change and passed after the shared renderer landed. They check that the band and the child both respond to hover, that the band retains its centerline, and that its color state resets on pointer exit.
- Playwright with Chrome checked both startup edges at device scale factors 1, 1.25, and 2. All six checks verified a one-pixel positioned edge, normal and hover colors, zero layout border, stable bounds, and automatic removal on attachment.
- The live OS browser harness checked both bands at 1440×900 and 375×812. All four lines changed color on hover and retained their screen coordinates and one-pixel stroke. The desktop footer handed its normal stroke from an adjacent owner to the hovered footer at the same coordinate.
- The demonstrator generator used the OS branded footer at 1440×900, 768×1024, and 375×812. In each viewport the footer row's scroll width matched its client width. The tablet and phone each showed one accessible credits button whose popover held all three logo links.
- The settled OS rich two-window view at 768×1024 returned zero neighboring-border gaps, including the previously reported near-40-pixel pair. The cap-row opening is part of a continuous silhouette.
- The repository test target waited behind a shared graph refresh. The focused tests completed through `repo tools exec vitest` using the existing ticket-local UI test configuration.

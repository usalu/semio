# UI Navbar Private Support Registrar Integration

Terra verified Navbar SHA-256 `85f2e37dd539498d05e193673a0eb7388d67e68083c327b618429a18fdba9099` and protected barrel SHA-256 `b4b1622b05d3bdbf50e7ef5f1edfd4cda00e35963a1b07c28efba2ca37cfd9c5`, then removed only the public modifiers from `navbarFillClassName`, `NAVBAR_NO_EXAMPLE_ID`, and `normalizePlaygroundExampleId`. Their bodies and internal Navbar usage are unchanged; Navbar final SHA-256 is `b5f7e2b1c71cbd255e0f40aa462b41d18ee1de15422fad880d09e483de1e039b`.

The coordinator removed the three now-private support symbols from the mechanical Navbar barrel import/export lists. `navbarFillItem` and `NavbarExampleSelect` remain the public semantic surfaces. The protected barrel final SHA-256 is `48d2d0da1eacb16553b4c9924b45b34ddc7dc70e33af4b9ac115513768a66076`; no barrel reference remains, all three declarations are private, and scoped `git diff --check` passed.

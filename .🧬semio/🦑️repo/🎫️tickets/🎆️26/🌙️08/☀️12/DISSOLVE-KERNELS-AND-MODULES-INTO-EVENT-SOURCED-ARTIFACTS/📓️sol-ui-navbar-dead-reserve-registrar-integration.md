# UI Navbar Dead Reserve Registrar Integration

Terra verified Navbar SHA-256 `2918372bf6dcee1d211db0a0082db4f7cd596db2c06ba6fb91d641d426ce024e` and protected barrel SHA-256 `537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d`, then deleted only the zero-consumer `shellNavbarTrailingEndReserveCss` definition and docstring. Its final Navbar SHA-256 is `85f2e37dd539498d05e193673a0eb7388d67e68083c327b618429a18fdba9099`.

The coordinator removed the symbol from the mechanical Navbar import and export lists. Measured trailing-end width behavior, fullscreen slot, Panel reserve behavior, and all other Navbar exports remain unchanged. The active UI reference scan is zero, scoped `git diff --check` passed, and the protected barrel final SHA-256 is `b4b1622b05d3bdbf50e7ef5f1edfd4cda00e35963a1b07c28efba2ca37cfd9c5`.

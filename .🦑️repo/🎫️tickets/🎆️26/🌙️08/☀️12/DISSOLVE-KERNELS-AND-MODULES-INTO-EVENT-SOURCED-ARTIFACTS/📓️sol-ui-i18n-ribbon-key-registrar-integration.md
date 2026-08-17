# UI I18n Ribbon Key Registrar Integration

Terra verified I18n SHA-256 `15122bb408ca449538f80a6d973bae23db4f85e92942336ea241edd7ea099891` and protected barrel SHA-256 `48d2d0da1eacb16553b4c9924b45b34ddc7dc70e33af4b9ac115513768a66076`, then deleted only the production-zero-consumer `UiRibbonParentKey` declaration and docstring. I18n final SHA-256 is `ac87da2a670949bf7755af2ef85c0bebc2dcbeb3b655784f1e3336a5ce4152d2`.

The coordinator removed the old type from the barrel's mechanical I18n import/export and changed its sole test-only cast to the existing repository-owned `UiTranslationKey`. Translation resources, compile-time coverage contracts, and runtime I18n behavior are unchanged. The exact old-type reference scan is zero, scoped `git diff --check` passed, and the protected barrel final SHA-256 is `4e916cf18ad6c1a44961405f6adddb20b0a7383e3283af306f5c756e016ca52d`.

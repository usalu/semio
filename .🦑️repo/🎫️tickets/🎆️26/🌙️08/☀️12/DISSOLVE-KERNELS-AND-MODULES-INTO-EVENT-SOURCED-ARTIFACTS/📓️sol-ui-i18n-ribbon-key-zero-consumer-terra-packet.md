# Terra Packet: I18n Ribbon Key Zero-Consumer Type

## Objective

Delete the production-zero-consumer `UiRibbonParentKey` type. Its only active source use outside the defining file is an inline React-barrel test; tests cannot retain a production contract.

## Baseline

- I18n SHA-256: `15122bb408ca449538f80a6d973bae23db4f85e92942336ea241edd7ea099891`.
- Protected barrel after Navbar private-support registrar: `48d2d0da1eacb16553b4c9924b45b34ddc7dc70e33af4b9ac115513768a66076`.

## Terra Lease

Edit I18n only: delete exactly the exported `UiRibbonParentKey` declaration and its attached docstring if any. Preserve every other I18n contract and runtime value. Do not edit the barrel. Stop at registrar handshake.

The coordinator will remove the type from the barrel import/export and change the one test-only assertion cast to the existing broader `UiTranslationKey`. After signal, prove zero old-type references, run scoped diff checks and UI lint/test-quick once, then write a unique acceptance record. Do not alter bundles, translations, products, manifests, generated output, or locks.

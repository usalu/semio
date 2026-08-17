# UI Element Identity Registrar Integration

## Baseline

- Released protected React barrel SHA-256: `de3c18afdb4a6cb03ef35814457c139547b268d7ba960748ff5bc4c652a52f99`.
- Terra source handshake confirmed the old element source absent, a single repository-owned `ElementProps` contract in `🆔️element-identity`, nine direct component consumers, and zero non-barrel legacy references.

## Registrar Change

- Removed the old `Transaction`, `TransactionProvider`, `useTransaction`, `ElementBaseProps`, and `ElementProps` import/export from `🐹️ElementProps`.
- Added a mechanical type-only import/export of `ElementProps` from `🆔️element-identity`.
- Added no compatibility alias or runtime behavior to the list root.

## Validation Handoff

The protected barrel final SHA-256 is `a9a764971875336ed637b8be0ec1dae23150dfce09985ddf7cd5d69cafc774f6`. The coordinator confirmed zero old transaction/path references, exactly nine direct source identity imports plus the barrel, and a clean scoped `git diff --check` before returning the semantic lease to Terra for the registered UI Nx matrix.

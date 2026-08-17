# UI Navbar Example Selector Registrar Integration

## Source Handshake

- Navbar baseline/final SHA-256: `b5f7e2b1c71cbd255e0f40aa462b41d18ee1de15422fad880d09e483de1e039b` / `f8518236c01a8795097fb347d11e0950cfa2338b93917574d91376ad819e92d4`.
- New specific component SHA-256: `a09f1b454e1f6011a2c5116696ac6d6b826a1b87d6b43420a3eba6f00f2e7db6`.
- Protected barrel baseline SHA-256: `4e916cf18ad6c1a44961405f6adddb20b0a7383e3283af306f5c756e016ca52d`.
- Old Navbar has no selector responsibility or forwarder; new component imports every dependency directly and has no barrel edge.

## Registrar Change

The coordinator removed the selector component/types from the Navbar assembly block and added a separate explicit `NavbarExampleSelect` component import/export region. Package API names remain unchanged for protected OS consumers and inline tests. Old Navbar selector scan is zero, scoped `git diff --check` passed, and the protected barrel final SHA-256 is `fe035d5241f8ab88f9a8c31348faa6aa50368e0a1fd8e1b2bb6add71b9e10bf6`.

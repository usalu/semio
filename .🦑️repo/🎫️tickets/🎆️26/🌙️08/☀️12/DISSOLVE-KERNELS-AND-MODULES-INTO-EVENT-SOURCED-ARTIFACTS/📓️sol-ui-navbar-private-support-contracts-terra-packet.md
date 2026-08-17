# Terra Packet: Navbar Private Support Contracts

## Objective

Privatize three implementation details with zero active consumers outside Navbar: `navbarFillClassName`, `NAVBAR_NO_EXAMPLE_ID`, and `normalizePlaygroundExampleId`. Preserve `navbarFillItem`, `NavbarExampleSelect`, and every runtime behavior exactly.

## Baseline

- Navbar after dead reserve removal: `85f2e37dd539498d05e193673a0eb7388d67e68083c327b618429a18fdba9099`.
- Protected React barrel: `b4b1622b05d3bdbf50e7ef5f1edfd4cda00e35963a1b07c28efba2ca37cfd9c5`.

## Terra Lease

Edit Navbar only: remove `export` from the constant/function declarations without changing bodies, names, types, docs, or internal calls. Do not edit the barrel. Stop at registrar handshake. After coordinator removes the three mechanical import/export entries, prove all three are private-only, run scoped diff checks plus UI lint/test-quick once, and write a unique acceptance record.

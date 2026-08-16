# Neural DAG Integration Rerun After stdio HEAD Advance

## State

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Neural DAG dissolution source state remained unchanged from `📓️terra-neural-dag-flow-host-dissolution-acceptance.md`.
- Two read-only stability samples of the active stdio frontier each reported 70 dirty paths.
- stdio Rust package glue SHA-256: `1f560f11b58cf77df851b803fc670b12cce40330bfa4861f200664e0e600d2e1`
- stdio Cargo manifest SHA-256: `39855d37736f5edc86684f2cc680ae7c3313c4031a07a835aa33179b5e61f1a2`

## Validation

Command:

```text
bun nx run semio-framework-os-flow-core:test-quick --skip-nx-cache
```

Result: exit 1. The build advanced beyond the previous ten stdio unresolved-import errors, but the `semio-s-plugin-stdio` dependency still failed with 56 compiler errors and 735 warnings. The retained output tail showed `E0609`; output truncation did not preserve every error site. No Flow host or deleted neural adapter error appeared.

## Disposition

N-01 remains source-complete and integration-blocked by the moving stdio owner. No clean integration claim is made. Do not rerun until the stdio error frontier is repaired and stable again.

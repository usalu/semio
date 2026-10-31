# Course and Pets Typecheck and Unit Audit

2026-10-30. `repo://goals` was read. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open under the teaching-architecture deploy work. Commands used `repo tools exec` so they did not wait on a full workspace graph.

## Typecheck

`repo tools exec tsc -- --noEmit --pretty false -p <tsconfig>`. Each exited 0 with no diagnostics.

| Package | Project |
| --- | --- |
| `@semio-tech/course-react` | `framework/products/course/targets/react/packages/typescript/tsconfig.json` |
| `@semio-tech/pets` | `framework/products/pets/packages/typescript/tsconfig.json` |
| `@semio-tech/pets-react` | `framework/products/pets/targets/react/packages/typescript/tsconfig.json` |
| `@teaching/architecture-course` | `teaching/architecture/course/packages/typescript/tsconfig.json` |

## Unit tests

`repo tools exec vitest -- run --config <config> --configLoader native --reporter=dot`. Vitest 4.1.12.

| Package | Config | Result |
| --- | --- | --- |
| `@semio-tech/course` | `framework/products/course/tests/config` | 19 files, 595 tests, passed in 6.21s |
| `@semio-tech/pets` | `framework/products/pets/tests/config` | 34 files, 1164 tests, passed in 6.42s |
| `@semio-tech/pets-react` | `framework/products/pets/targets/react/tests/config` | 9 files, 218 tests, passed in 4.44s |

## Outcome

No diagnostic or test failure remained. No source change was required.

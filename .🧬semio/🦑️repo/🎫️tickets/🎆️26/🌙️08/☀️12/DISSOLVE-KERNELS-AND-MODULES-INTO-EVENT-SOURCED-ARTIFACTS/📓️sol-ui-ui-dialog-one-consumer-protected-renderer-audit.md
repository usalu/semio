# UI Dialog One-Consumer Protected-Renderer Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- UIDialog component SHA-256: `98cdd441e04311be689cc509d01942e7fa10ecef745ea31c379067685e7aa544`, clean.
- UIDialog story SHA-256: `a11bf46c9eeb0758ad6848290fc5612df4f66f31de0b67746268de7c7a779917`, clean.
- React index SHA-256: `64eb6dcf68e5c20a02409cedf789a96010f040d4144793b7de069f982795a10f`, accepted serialized UI changes only.

## Consumer Closure

Protected OS renderer `ShellHost` is the sole active production terminal that imports and renders `UIDialog`. The OS renderer package index and framework UI barrel are glue. The exclusive UIDialog story and Storybook smoke inventory are test/example evidence. An owner-local documentation comparison in the React package does not create a consumer.

## Decision

UIDialog has one production terminal and is therefore an inline/collapse candidate below the two-consumer minimum, not a shared-module candidate. The correct lowest owner is protected OS renderer `ShellHost`. Do not delete or move it in an isolated UI lease: its implementation, props contract, renderer import, story disposition, package barrels, tests, and runtime host validation must move atomically only after the protected renderer owner is released and rehashed. No source edit is authorized now.

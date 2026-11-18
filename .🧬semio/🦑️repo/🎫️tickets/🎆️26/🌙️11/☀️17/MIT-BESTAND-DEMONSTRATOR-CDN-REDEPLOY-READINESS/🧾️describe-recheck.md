# Describe recheck against the live worktree

## Root cause
`withDescriptorEmitterTaskV1` (`🖨️describe/🛠️emitter/🟦️.ts`) built `recheck()` from every compilation index row with `custody==="current"`: it re-digested `row.path` in the LIVE worktree and compared with the digest recorded at compile time. Current-custody rows are the compiler's source inputs, which are not copied into the sealed store. In the shared, continuously edited worktree any edit in the descriptor tool's source closure during a 20-40 minute component build therefore failed the last `tool.recheck()` (fresh-component, descriptor-emission). The refusal did not name path or digests.

## Intended guarantee
Provenance: the binary used is exactly the product of the recorded inputs. That is proven by the sealed artifact, not by the live tree: binary sha256 equals a receipt output, the receipt equals the index receipt digest, every retained blob equals its indexed digest (all checked at task start), and the artifact reader custody. The build itself (`buildDescriptorEmitterV1`) still rechecks the capture against the live inputs while compiling, so a mutation during the compile is refused there.

## Fix
- `recheck()` now verifies only sealed content: the executable and every retained compilation file against their own recorded digests (`assertSealedFilesV1`), then physical and artifact custody. A newer live tree keeps the sealed binary valid; it is deterministic and never fails at the end.
- Refusals name every drifted path with expected and actual digest (`absent` if missing).
- New `🛠️emitter/🔏️sealed/🟦️.ts`; test `🛠️emitter/🔏️sealed/🧪️tests/🟦️.ts` (live change tolerated, tamper and absence named).

## Residual
A build whose post-stage `compilation.recheck()` fails leaves its published binary in the store; the next `describe-build` overwrites it.

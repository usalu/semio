# Plugin R6 Joint Source Release

## Release

The mutation-owned repair for the 17 diagnostics attributed to this lane in the retained R6 inventory is source-coherent for a fresh native Plugin inventory. It is not native acceptance. The runtime lane owns the sole compiler and the other two R6 diagnostics; no Cargo command was started here.

The four final source gates independently captured the same Plugin component SHA-256: `49e052ef1f38628104ace5c03b047b895556dee14108c8ca91b36c21f976b103`. This lane freezes its Plugin, Store and derive source while the runtime lane captures and compiles R7. Unrelated runtime changes remain separately attributed.

## Actual Repairs Reviewed

- The shared test document/config roots own `TestSnapshot`, `TestDiff`, and `TestConfig`, their unchanged codecs and the existing clone/encoding probes. The builder imports those real canonical types instead of sibling-private definitions. See [shared-state review](📓️plugin-r6-shared-state-50.md).
- The dummy, transaction and surface fixtures physically own their snapshots/diffs, commands, apps, helpers and 23 existing native test functions. Their five mutation leaves remain descendants of the private count fields. An explicit `#[path = "."]` rebases the inline `app` module; its canonical `mutation_fixture` child preserves access to existing app-private test helpers. No count visibility widening, proxy or compatibility reexport was added. See [private-count review](📓️plugin-private-count-fixture-roots-50.md).
- The declaration fixture uses the existing public `store::ArtifactPack` trait rather than a private import.
- The five KeyedTestApp no-state hooks return exact typed constructors from the app-local no-state fixture child. The transient disposer transfers the entire old unit-state store into the existing owned-value retirement factory and witnesses the replacement root/generation; it does not retire a cloned root and certify the original owner empty. The authored native law includes zero/short grants and owner drift after completion. See [no-state review and superseded attempts](📓️plugin-r6-no-state-fixture-49.md).

The root review rejected earlier `None` hooks, cloned-root disposal, sibling-private factory access and the initial top-level fixture mount. Those failed/intermediate records remain retained and are not acceptance evidence.

## Executed Source Gates

All four commands ran through Bun and Nx with cache bypass and exited zero:

| Packet | Actual result | Retained output |
| --- | --- | --- |
| Shared document and direct leaves | 44/44 | [run-boqQev](../🧪️test-mutation-direct-leaves/🧫️run-boqQev/🔣️result.json) |
| Shared config and nullable selection | 37/37 | [run-SOkMbY](../🧪️test-config-selection/🧫️run-SOkMbY/🔣️results.json) |
| Private-count owner roots | 10/10 | [run-mtc0f9hs](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0f9hs/🔣️result.json) |
| Exact no-state hooks and lifecycle source | 25/25 | [run-22tPgF](../🧪️plugin-r6-no-state-fixture-49/🧫️run-22tPgF/🔣️result.json) |

These are schema/reference, source-structure and captured-input checks. The private-count gate additionally ran rustfmt in stdout-only parse mode for its three owner roots. The no-state block check is structural, not a Rust typecheck. None executed the native fixture laws. Local no-follow checks and first/final hashes are scoped input checks, not a proof against arbitrary concurrent ancestor swaps.

## Key Source Hashes

- Parent fixture root: `3bd467944a404b12fd61bc62af6c9b016bd139fb618f66c2e30ba0f9b60a03f1`.
- No-state child: `c1a5f0ef698a57d5cb11e045fb3181824f1bff928276afb687f53a2863b6c80f`.
- Declaration fixture: `401a2ecb4cdfccfc7aa95932d35366e388b0654d326f8a9bb9eb0ded2bc0e91d`.
- Dummy owner: `0ddd5d7b88e026cdcd55b4def9beb4fb30e6d63fd1ecf450b6130c720af53b35`.
- Transaction owner: `c5fbfbd08edd772be5676cb3ead8502f6823b893567629a84b1a6ce301da98b8`.
- Surface owner: `5f1187463355727519cf507e53d08fbf25204b9142bd1f100e56328319d8f643`.

The per-gate results retain the remaining leaf, descriptor, schema and controller hashes. The new Store poison/rejected-page native prototypes remain ticket-only and are not mounted during R7. No Plugin, Flow, GIS or whole-monorepo publication readiness follows from this release.

## R7 Native Boundary And R8 Source Release

The runtime lane actually ran R7 Plugin `--lib --no-run`. The root read all eleven diagnostic groups in the retained [raw native log](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-native-inventory-r7-2026-08-27.txt:7603). The compiler no longer reported the earlier 17 diagnostics; it stopped with eleven E0599 errors in the moved transaction owner because the implemented `PluginApp` trait was not imported. No native test ran.

The root added the exact import regression to the existing private-count controller and executed source RED [run-mtc0my0w](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0my0w/🔣️result.json): 10/11, only `transactionImportsItsNativeAppTrait` false. The only Rust edit then added `PluginApp` to that owner's existing `crate::app` import. All transaction bodies, private fields, mutation leaves and shared implementation methods were preserved.

The actual source GREEN is [run-mtc0nj5w](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0nj5w/🔣️result.json): 11/11, twenty stable first/final inputs and the same three stdout-only Rust parser checks. Transaction owner SHA-256 is now `2eb46ec3079249d8522a63b67cba5c60861ae3f2e160eeef637e9eb0097ec157`; Plugin main remains `49e052ef1f38628104ace5c03b047b895556dee14108c8ca91b36c21f976b103`. This is the narrow source release for runtime-owned R8, not a native GREEN claim. This lane again freezes Plugin/Store/derive during that capture and compile.

## R8 Compile And Behavioral Follow-Through

R8 actually compiled the Plugin test crate in 44.02 seconds, exit zero. The root read the terminal result and relevant warnings in the retained [R8 raw log](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-native-inventory-r8-2026-08-27.txt). That establishes native compilation of this repair, not execution of its fixture laws.

The subsequent peer-run combined selection contained 24 tests (dummy 5, surface 8, transaction 10, keyed no-state 1). It reported three passes before dummy convergence stack-overflow/SIGABRT. It is not a full 24-test outcome. The runtime lane is isolating the tests against the unchanged retained binary; no stack or budget increase was requested here.

Root review also found the surface viewer test discarded the Future returned by `assert_viewer_never_mutates`. Any old-binary pass for that test cannot certify the viewer law. After the peer explicitly released source for this narrow correction, the root executed source RED [run-mtc0wpic](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0wpic/🔣️result.json), 11/12, then added the missing `.await` inside the existing async test. Source GREEN [run-mtc0xio2](../🧪️plugin-private-count-roots-50/🧫️runs/run-mtc0xio2/🔣️result.json) is 12/12 with twenty stable first/final captures. The corrected surface SHA-256 is `937a2de33bd227d5ba3553c55fc1e0da8ae2f405cba7c05d2a3e4f24da26880f`.

Both latest source runs captured main `e285bdc23387698da65be78588b10643d4ba0bd1e92a045123c7a501edb501ef`, after the runtime lane's independently announced construction correction. This lane did not edit or restore that main boundary. The corrected viewer test still requires fresh native compilation and execution; old-binary isolation remains separately attributed.

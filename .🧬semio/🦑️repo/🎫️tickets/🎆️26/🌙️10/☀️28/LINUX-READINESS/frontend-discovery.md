# Per-user Native Frontend Discovery

Source pointer .🧬semio/🦑️repo/native-repo-path is a cross-environment defect: container installation writes its executable path into a bind-mounted checkout shared with the host. It is redundant with per-user installed.json and PATH publication. Remove the Source artifact and all its reads/writes, without compatibility fallback.

## Exact Owners

- MCP 📜️script.ts InstallScript.run53–67: per-user receipt creation56–57, PATH publication59–66, Source pointer publication67. Remove only pointer publication; retain installed.json provenance.
- library/🎛️commands/📝️commit/🪝️hooks/🟦️.ts MICRO_COMMIT_RESOLVE_REPO_CLI_SH48–71: explicit override50–53, pointer54–61, per-user launcher62–66, PATH67–70. Remove pointer branch.
- library/🧪️testing/📦️installed-frontend/🟦️.ts captureInstalledFrontendProofV1 60–78: derives per-user receipt65–66; validates receipt/executable/source67–72; redundant Source pin73–74. Remove pin from provenance structure and assertSameInstalledFrontendProofV1 key set82; keep receipt/profile/executable identity and digest checks.
- Tests MCP/📥️installation/🧪️tests/🟦️.ts45 assert pointer/order. Update to per-user discovery and assert no repository pointer.
- Existing portable hook workspace law under library/🧪️tests/🔬️workspace-contract/🔬️micro-commit/🟡️installmicrocommitgithooks-writes-portab/🟦️.ts25 asserts pointer. Update to canonical per-user route.

## Shortest Clean Design

Extend installation schema Protocol with receipt filename and Unix receipt-root fragments (currently hard-coded .local/share/semio/repo), plus a closed receipt definition matching version/platform/digest/executable/directory/source/mode. Share this schema with installer and proof. Existing Windows directory fragments and Unix executable directory fragments already belong to this protocol.

For hooks, explicit authorized override followed by per-user launcher and command -v repo already provides executable discovery without JSON parsing or shared checkout state. Installation publishes user PATH on all supported hosts. Prefer this small deletion over adding a second receipt parser in shell. If exact installed frontend rather than launcher is a strict hook requirement, generate one per-user discovery contract from the schema, with a real JSON parser under the platform interface; do not parse receipt JSON using sed. The receipt remains authoritative for provenance tests and installation ownership.

Regression fixture: one shared workspace, separate host/container homes, distinct per-user installed frontend paths. Installing container frontend must write only container home and leave host receipt/executable discovery unchanged. Test both independent homes and hook PATH isolation with an existing third-party path/schema oracle. No source changes made by this audit.

## Implemented Per-user Discovery

Removed shared Source pointer publication and hook/provenance reads. User installation receipt location and closed Receipt shape are schema-owned; provenance still authenticates per-user receipt, built native profile and installed executable bytes and ancestry. Hooks retain explicit override, per-user launcher and PATH discovery. Deleted the machine-specific tracked pointer and added its ignore rule. Neutral platform/home receipt corpus agrees with independent SQLite path oracle and Ajv/owned schema validation, including invalid receipts; both TypeScript and esbuild actual-function extraction run. Red reproduced cross-environment pointer writer; green2 tests134assertions. Full authenticated installer suite pending shared dependency setup.

## Independent Implementation Review

Reviewed final installer/hook/provenance changes read-only against tracked baseline. Installation plan still selects path.win32 for Windows fixture inputs and path.posix for Unix; native receipt writer/proof use host-native join correctly. unixReceiptDirectory and receiptFile now come from installation schema. Receipt is closed with all seven fields required, concrete platform/mode enums, digest format and bounded nonempty paths.

Removing redundant Source pin does not weaken executable provenance: proof retains exact per-user root/directory/executable/source/profile/mode checks, bounded physical file/ancestry/descriptor identity, both executable digests and native binary headers. Before/after proof still compares receipt/profile/executable and full record. Hook removes only shared-checkout pointer branch and preserves explicit override, per-user launcher and PATH. Product scan found no remaining pointer consumers or stale .pin proof field references; only negative regression assertions mention the removed pointer. No product documentation references found.

One minor consistency issue was reported to root: installer plan path-budget calculation still spells installed.json.stage- rather than composing schema receiptFile plus stage suffix. Current schema filename matches, so no immediate behavior defect, but schema changes should remain authoritative. No source edits by reviewer.

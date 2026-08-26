# S-TICKET-IMPORTANT-GO-OWNER

## Outcome

The Go ticket owner now treats `📌️important/📝️.md` as the only important-document location. Ticket status is mandatory and must be exactly `open` or `closed`; deserialization, persistence, finish, and reopen reject missing or unknown values without inference or fallback.

Create, read, rename, finish, reopen, and bulk finish are covered with isolated filesystem fixtures. Create and rename refuse occupied destination ticket roots. Reopen preserves any existing regular important document byte-for-byte and exclusively creates a zero-byte document only when absent. Finish, including bulk finish, requires an existing zero-byte regular document in an otherwise empty `📌️important` directory, then removes the exact leaf and directory. Whitespace is content and blocks finish.

Failed finish persistence restores status, summary, interactions, and the zero-byte important document without overwriting a concurrently present node. Failed reopen persistence restores goal, parent, status, interactions, sessions, and removes only the document/directory created by that reopen. Invalid status is rejected before manifest or lifecycle mutation.

## TDD Evidence

The first routed red attempt used the router's documented separator form:

```text
bun ./📜️script.ts test -- -run='Test(...)'
[budget] go test ... -short -- -run=Test(...) exceeded 15000ms — killed
```

The separator reached `go test` and disabled selection. A narrower red run still executed the package but recorded the intended failures: old `📌️important.md`, missing/empty/unknown status accepted, bulk bypassing the important document, whitespace/nonempty/missing/nonregular documents accepted, reopen not recreating the document, and failed Save mutating status/interactions/sessions. A later isolated red test proved `CreateTicket` overwrote a pre-existing nonempty important document.

The supported direct router form is `test -run=...`, and Nx forwards the same no-space/equal-form selector.

## Green Verification

Independent root verification on 2026-08-26 first tried to combine exact selectors with a parenthesized regular expression through Nx. The Nx shell route removed the protective quoting, so `/bin/sh` rejected the command before Go tests ran. This is recorded as a routing failure, not test evidence. The supported simple selectors below were then rerun independently and passed:

```text
bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=Important -count=1
ok  github.com/usalu/semio/repo/client  0.809s
NX Successfully ran target test for project @semio-tech/repo-client

bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=TestTicketUnmarshalRequiresExplicitValidStatus -count=1
ok  github.com/usalu/semio/repo/client  0.435s
NX Successfully ran target test for project @semio-tech/repo-client

bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=TestTicketLifecycleRejectsInvalidStatusWithoutMutation -count=1
ok  github.com/usalu/semio/repo/client  0.312s
NX Successfully ran target test for project @semio-tech/repo-client
```

Uncached direct Go owner suite, including the pre-existing oversized-artifact finish regression:

```text
go test -short -count=1 -run='Test(TicketImportantPathCanonical|TicketUnmarshalRequiresExplicitValidStatus|CreateReadAndRenameTicketUseCanonicalImportantPath|CreateTicketNeverOverwritesExistingImportantDocument|TicketLifecycleRejectsInvalidStatusWithoutMutation|FinishTicketImportantLifecycle|ReopenTicketImportantLifecycle|FinishTicketPurgesOversizedArtifacts|PurgeAllOversizedTicketArtifacts)$' .
ok  github.com/usalu/semio/repo/client  1.065s
```

Uncached Bun/Nx-owned routes:

```text
bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=Important -count=1
ok  github.com/usalu/semio/repo/client  0.985s
NX Successfully ran target test for project @semio-tech/repo-client

bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=TestTicketUnmarshalRequiresExplicitValidStatus -count=1
ok  github.com/usalu/semio/repo/client  0.345s
NX Successfully ran target test for project @semio-tech/repo-client

bun nx run @semio-tech/repo-client:test --skip-nx-cache -- -run=TestTicketLifecycleRejectsInvalidStatusWithoutMutation -count=1
ok  github.com/usalu/semio/repo/client  0.378s
NX Successfully ran target test for project @semio-tech/repo-client
```

Formatting validation:

```text
gofmt -w 🐹️component.go 🧪️component_test.go
git diff --check -- 🐹️component.go 🧪️component_test.go
exit 0
```

## Touched Owner Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`
- This report.

No taxonomy, discovery, normalization, Compose, manifests, scripts, or Git state were modified.

## Acceptance Notes

- Canonical storage is expressed as a language-neutral filesystem/JSON contract and exercised through real isolated nodes.
- There is no legacy `📌️important.md` lookup, status inference, missing-status default, alias, or compatibility path.
- No runtime dependency was added.
- Package-wide short tests contain unrelated pre-existing repository failures, so acceptance uses exact owner selectors and records the successful Nx-owned route rather than masking those failures.

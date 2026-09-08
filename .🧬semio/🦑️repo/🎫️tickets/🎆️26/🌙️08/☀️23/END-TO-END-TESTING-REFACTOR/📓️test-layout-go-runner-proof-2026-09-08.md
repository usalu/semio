# Canonical Go Test Discovery Proof

Executed `GOWORK=off go test -overlay=overlay.json -v .` on an isolated local package. Its only authored test is `🧪️tests/addition/🐹️.go`; the overlay registers it as a virtual Go test file in the implementation package. No `_test.go` file was authored. The test accesses a private function in its original package.

Observed runtime output: `TestAddition` passed; package returned `ok example.com/layout-proof 0.563s` and exit 0.

This supports canonical authored filenames with standard Go discovery using an ephemeral overlay manifest. Input fixture source is retained in `🔬️go-overlay-proof`; generated proof output remains in `🗑️generated` for cleanup.

The production runner now derives `🧪️tests` and `🐹️.go` from the taxonomy, scans each semantic owner, maps each source to a deterministic virtual package-local `zz_semio_<hash>_test.go`, and invokes only the owning Go packages. This preserves package-private access and prevents `go test ./...` from treating case directories as production packages. Overlay manifests are created in the operating-system temporary directory and removed on success, failure, and process exit.

A language-neutral JSON vector and Bun test compare the plan against the standard Go toolchain. The test passed with 1 test, 6 expectations, including `TestPrivateAddition` and `TestPrivateUppercase`. Public Nx evidence also passed:

- `bun nx run @semio-tech/repo-mcp-go:test --skip-nx-cache`: `github.com/usalu/semio/repo/mcp` passed.
- `bun nx run @semio-tech/repo-client:test-quick --skip-nx-cache -- -run '^TestG1InvalidFlagFixture$'`: the root package passed and both internal packages compiled through their canonical overlays.

The unfocused client quick target reached its existing 30-second cold-build budget and was killed. The focused public Nx rerun completed in 12.1 seconds. No authored legacy Go filenames remain.

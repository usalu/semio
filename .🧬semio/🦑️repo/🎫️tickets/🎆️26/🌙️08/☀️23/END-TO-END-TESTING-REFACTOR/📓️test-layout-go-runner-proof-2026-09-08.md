# Canonical Go Test Discovery Proof

Executed `GOWORK=off go test -overlay=overlay.json -v .` on an isolated local package. Its only authored test is `🧪️tests/addition/🐹️.go`; the overlay registers it as a virtual Go test file in the implementation package. No `_test.go` file was authored. The test accesses a private function in its original package.

Observed runtime output: `TestAddition` passed; package returned `ok example.com/layout-proof 0.563s` and exit 0.

This supports canonical authored filenames with standard Go discovery using an ephemeral overlay manifest. The permanent runner can create and remove that manifest in the test output folder. Input fixture source is retained in `🔬️go-overlay-proof`; generated overlay output remains in `🗑️generated` for cleanup.

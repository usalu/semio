"""🧫 [isolate] manual edits of batch `tools` (after the codemod): the tooling store, headroom and generation laws name the private repository as their workspace; the headroom child script adopts its parent's private repository."""
T = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"
MANUAL = {
    T + "/🔏️authorizers/🧪️tests/🟦️.ts": [("const workspace=process.cwd(),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;", "const workspace=repository.root,artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;", 1)],
    T + "/🧹️retention/📏️headroom/🧪️tests/📜️script.ts": [("const workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original", "const workspace=repository.root,control=scriptCompilerAdmissionControlV1(original", 1)],
    T + "/🧹️retention/📦️generation/🧪️tests/🟦️.ts": [(",workspace=process.cwd(),control=scriptCompilerAdmissionControlV1(original,\"tooling:copied-retirement-law\"", ",workspace=repository.root,control=scriptCompilerAdmissionControlV1(original,\"tooling:copied-retirement-law\"", 1)],
}
HELPERS = {T + "/🧹️retention/📏️headroom/🧪️tests/🟦️.ts": ("privateRepositoryWorkspaceV1",)}

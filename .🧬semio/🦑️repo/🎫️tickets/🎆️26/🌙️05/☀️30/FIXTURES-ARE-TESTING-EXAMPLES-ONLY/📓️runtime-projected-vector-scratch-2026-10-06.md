# Projected vector scratch ownership

The projected storage law previously allocated a synthetic workspace under os.tmpdir. On macOS its /var ancestry is a symlink, correctly rejected by production workspaceAdmission. The test now uses the existing first-party repoTestArtifactEnvironment with the projected-vector-storage route, honors SEMIO_TEST_ARTIFACT_DIR, and creates a unique projected-nx scratch child. The existing finally removes only that unique child. Workspace admission and every executable-vs-storage discovery assertion remain unchanged.

Only the test module import and its one scratch allocation changed; other independent temporary test paths and production security behavior were not edited. Exact registered repo-test-long filtered check is pending.

"""🧫 [isolate] manual edits of batch `budget` (applied after the codemod): the prepared-worker children take the store from the parent's private repository; the graph partition laws name the private repository as their workspace."""
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget"
W = B + "/📥️opening/📤️publication/🧪️tests/🧵️prepared-worker"
MANUAL = {
    W + "/👷️asset-publication/📜️script.ts": [("repositoryCacheBudgetV1(root)", "repositoryCacheBudgetV1(repository.root)", 3)],
    W + "/👷️graph-purpose/📜️script.ts": [("repositoryCacheBudgetV1(root)", "repositoryCacheBudgetV1(repository.root)", 2), ("openRepositoryCachePartitionV1(root,\"leases\",control)", "openRepositoryCachePartitionV1(repository.root,\"leases\",control)", 1)],
    B + "/🕸️graph/🧪️tests/🟦️.ts": [("const database=new Database(\":memory:\"),workspace=process.cwd();", "const database=new Database(\":memory:\"),workspace=repository.root;", 1), ("const workspace=process.cwd(),manifest=fixture.writerManifest", "const workspace=repository.root,manifest=fixture.writerManifest", 1)],
    B + "/🧪️tests/🟦️.ts": [("const database=new Database(\":memory:\"),root=process.cwd(),protectedPaths=cleanProtectedPrefixes(root)", "const database=new Database(\":memory:\"),root=repository.root,protectedPaths=cleanProtectedPrefixes(root)", 1)],
}
HELPERS = {W + "/🟦️.ts": ("privateRepositoryWorkspaceV1",)}

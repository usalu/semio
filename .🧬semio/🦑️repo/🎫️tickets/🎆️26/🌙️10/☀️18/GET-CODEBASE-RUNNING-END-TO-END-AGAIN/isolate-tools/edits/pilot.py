"""🧫 [isolate] pilot: caching artifacts publication tests use the private repository. usage: pilot.py <root>"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from conv import edit, LIB
root = sys.argv[1]
edit(root, LIB + "/⚡️caching/📦️artifacts/🧪️tests/🟦️.ts", [
    ("leaseDirectory=join(process.cwd(),fixture.publicationStore)", "leaseDirectory=join(repository.root,fixture.publicationStore)", 1),
    ("const workspace=process.cwd(),root=mkdtempSync(", "const workspace=repository.root,root=mkdtempSync(", 1),
    ("withRepositoryCacheStoreV1(process.cwd(),\"products\",", "withRepositoryCacheStoreV1(repository.root,\"products\",", 2),
    ("\"test:artifact-repository-ownership\",30000),workspace=process.cwd(),", "\"test:artifact-repository-ownership\",30000),workspace=repository.root,", 1),
    ("withRepositoryArtifactDestinationV1({workspace:process.cwd(),", "withRepositoryArtifactDestinationV1({workspace:repository.root,", 1),
])

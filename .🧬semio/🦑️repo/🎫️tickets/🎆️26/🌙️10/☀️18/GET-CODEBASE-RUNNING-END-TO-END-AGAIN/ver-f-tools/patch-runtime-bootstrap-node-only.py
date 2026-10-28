"""usage: patch-runtime-bootstrap-node-only.py — the image no longer installs Bun (the `repo` launcher provisions the pinned JavaScript runtime itself: bootstrap script header, Dockerfile since Oct 22/23); the runtime-acquisition law is node-only: no BUN_VERSION, one checksum-verified archive, one timeout, node-only architecture rows. package.json `packageManager` still pins bun@fixture.bun."""
import json,sys
root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/"
t=root+"🚀️runtime-bootstrap/🟦️.ts";s=open(t).read()
def swap(a,b):
    global s
    assert s.count(a)==1,a[:80]
    s=s.replace(a,b)
swap('  assert.ok(dockerfile.includes(`ARG BUN_VERSION=${fixture.bun}\\n`));\n','')
swap('bun_archive="([^"]+)"; bun_sha="([a-f0-9]{64})"; node_archive','node_archive')
swap('.map(([, architecture, bunArchive, bunSha256, nodeArchive, nodeSha256]) => ({ architecture, bunArchive, bunSha256, nodeArchive, nodeSha256 }))','.map(([, architecture, nodeArchive, nodeSha256]) => ({ architecture, nodeArchive, nodeSha256 }))')
swap('for (const tool of ["bun", "node"]) {','for (const tool of ["node"]) {')
swap('runtime.indexOf(tool === "bun" ? "unzip -p" : "tar -xJf")','runtime.indexOf("tar -xJf")')
swap('(runtime.match(/sha256sum -c -/g) ?? []).length, 2','(runtime.match(/sha256sum -c -/g) ?? []).length, 1')
swap('(runtime.match(/--max-time 300/g) ?? []).length, 2','(runtime.match(/--max-time 300/g) ?? []).length, 1')
swap('  assert.ok(runtime.includes("https://github.com/oven-sh/bun/releases/download/bun-v$BUN_VERSION/"));\n','  assert.ok(!runtime.includes("oven-sh/bun"), "The image installs no Bun: the repo launcher provisions the pinned runtime");\n')
swap('printf \'%s|%s|%s|%s\' "$bun_archive" "$bun_sha" "$node_archive" "$node_sha"','printf \'%s|%s\' "$node_archive" "$node_sha"')
swap('[row.bunArchive, row.bunSha256, row.nodeArchive, row.nodeSha256].join("|")','[row.nodeArchive, row.nodeSha256].join("|")')
open(t,'w').write(s)
f=root+"🧫️fixtures/🚀️runtime-bootstrap/🔣️.json";raw=open(f).read();d=json.loads(raw)
for p in d["platforms"]:
    p.pop("bunArchive");p.pop("bunSha256")
open(f,"w").write(json.dumps(d,ensure_ascii=False,indent=2)+("\n" if raw.endswith("\n") else ""))

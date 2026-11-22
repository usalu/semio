F="⚡️caching/🩹️patches/nx@23.2.0.patch"
def plan(p,L):
    f=L+F
    p.sub(f,"+                const receipt = read(directory, '.semio-nx-tooling.json'), marker = JSON.parse(receipt.bytes), authorizers = marker.authorizers;\n","+                const receipt = read(directory, '.semio-nx-tooling.json'), marker = JSON.parse(receipt.bytes);\n")
    t=p.load(f)
    i=t.index("+                if (marker.version !== 1 || !/^[a-f0-9]{64}$/u.test(marker.digest) || path.basename(directory) !== marker.digest || !authorizers || ")
    j=t.index("\n",i)+1
    old=t[i:j]
    tail=old[old.index("|| !authorizers || ")+len("|| "):]
    p.sub(f,old,"+                if (marker.version !== 1 || !/^[a-f0-9]{64}$/u.test(marker.digest) || path.basename(directory) !== marker.digest || marker.authorizers !== undefined) throw Error('Immutable dependency generation marker required');\n")
    anchor="+                for (const [relative, expected] of Object.entries(patch.files)) if (!/^[a-f0-9]{64}$/u.test(expected) || digest(read(sdk, relative).bytes) !== expected) throw Error('Actual loaded Nx patch bytes changed');\n"
    p.sub(f,anchor,anchor+
"+                const bundleRoot = process.env.SEMIO_NX_AUTHORIZER_ROOT;\n"
"+                if (!bundleRoot || !path.isAbsolute(bundleRoot) || path.resolve(bundleRoot) !== bundleRoot || fs.realpathSync(bundleRoot) !== bundleRoot || path.dirname(bundleRoot) !== path.join(path.dirname(path.dirname(directory)), 'nx-authorizers')) throw Error('Immutable authorizer bundle beside the dependency generation required');\n"
"+                const bundleReceipt = read(bundleRoot, '.semio-nx-authorizers.json'), bundleMarker = JSON.parse(bundleReceipt.bytes), authorizers = bundleMarker.authorizers;\n"
"+                if (Object.keys(bundleMarker).sort().join(',') !== 'authorizers,deps,digest,version' || bundleMarker.version !== 1 || !/^[a-f0-9]{64}$/u.test(bundleMarker.digest) || path.basename(bundleRoot) !== bundleMarker.digest || bundleMarker.deps !== marker.digest || "+tail)
    p.sub(f,"+                const source = read(directory, '.semio-authorizer-source.json'), manifest","+                const source = read(bundleRoot, '.semio-authorizer-source.json'), manifest")
    p.sub(f,"+                const bundle = read(directory, output.path, 33554432n, true);","+                const bundle = read(bundleRoot, output.path, 33554432n, true);")
    p.sub(f,"+                const after = read(directory, '.semio-nx-tooling.json'); if (!stable(receipt.status, after.status) || digest(receipt.bytes) !== digest(after.bytes)) throw Error('Immutable toolchain marker changed');","+                const after = read(directory, '.semio-nx-tooling.json'), bundleAfter = read(bundleRoot, '.semio-nx-authorizers.json'); if (!stable(receipt.status, after.status) || digest(receipt.bytes) !== digest(after.bytes) || !stable(bundleReceipt.status, bundleAfter.status) || digest(bundleReceipt.bytes) !== digest(bundleAfter.bytes)) throw Error('Immutable toolchain marker changed');")
    p.sub(f,"markerSha256: digest(receipt.bytes), outputSha256: output.sha256 };","markerSha256: digest(Buffer.concat([receipt.bytes, bundleReceipt.bytes])), outputSha256: output.sha256 };")
    p.sub(f,"@@ -1,3 +1,59 @@\n \"use strict\";\n Object.defineProperty(exports, \"__esModule\", { value: true });\n exports.handleImport = handleImport;","@@ -1,3 +1,63 @@\n \"use strict\";\n Object.defineProperty(exports, \"__esModule\", { value: true });\n exports.handleImport = handleImport;")

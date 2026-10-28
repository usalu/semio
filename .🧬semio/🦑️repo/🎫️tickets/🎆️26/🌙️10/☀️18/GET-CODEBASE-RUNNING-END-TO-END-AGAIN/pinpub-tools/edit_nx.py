"""🪶 [pinpub] the Nx graph pool admits sparse image backings (store-g made new macOS images sparse); usage: edit_nx.py <root>  (anchored edit of the Nx graph store module; exit 1 = anchor lost)."""
import sys
ROOT = sys.argv[1]
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"
path = f"{ROOT}/{REL}"
s = open(path, encoding="utf8").read()
EDITS = (
    ('import {requireNativeStorageReservationV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";\n',
     'import {requireNativeStorageReservationV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";\n'
     'import {nativeImageBackingAllocatedBytesV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🪶️extent/🟦️.ts";\n'),
    ('/** 💾️ Funds the native N-API writer pool in its own top-level nx partition without expanding any other envelope. */\n',
     '/** 🪶️ Admits the backing file of the Nx graph pool or of one graph child image: one regular file with one link whose declared extent covers it (`nativeImageBackingAllocatedBytesV1`); a sparse image that grows with its content is admitted like a preallocated one, as every other native image since images are created sparse. */\n'
     'export function nxGraphBackingAdmittedV1(row:Readonly<{isFile():boolean;isSymbolicLink():boolean;nlink:number|bigint;size:number|bigint}>):boolean{return row.isFile()&&!row.isSymbolicLink()&&Number(row.nlink)===1&&Number(nativeImageBackingAllocatedBytesV1(row))>=Number(row.size);}\n\n'
     '/** 💾️ Funds the native N-API writer pool in its own top-level nx partition without expanding any other envelope. */\n'),
    ('const backing=lstatSync(base.image.image);if(!backing.isFile()||backing.isSymbolicLink()||backing.nlink!==1||backing.blocks*512<backing.size)throw Error("Fully allocated Nx graph pool backing required");',
     'if(!nxGraphBackingAdmittedV1(lstatSync(base.image.image)))throw Error("Original single-link Nx graph pool backing required");'),
    ('try{const backing=lstatSync(child.image.image);if(backing.blocks*512<backing.size)throw Error("Fully allocated native graph backing required");',
     'try{if(!nxGraphBackingAdmittedV1(lstatSync(child.image.image)))throw Error("Original single-link native graph backing required");'),

    ('native=child.image;const backing=lstatSync(native.image);if(backing.blocks*512<backing.size)throw Error("Fully allocated native graph backing required");check(store);',
     'native=child.image;if(!nxGraphBackingAdmittedV1(lstatSync(native.image)))throw Error("Original single-link native graph backing required");check(store);'),
    ('if(existsSync(shard)){const allocated=lstatSync(shard).blocks*512;',
     'if(existsSync(shard)){const allocated=Number(nativeImageBackingAllocatedBytesV1(lstatSync(shard)));'),
)
for old, new in EDITS:
    if s.count(old) != 1: sys.exit(f"anchor lost ({s.count(old)}): {old[:100]}")
    s = s.replace(old, new)
open(path, "w", encoding="utf8").write(s)
print("edited", REL[-50:])

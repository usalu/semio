#!/bin/sh
# 👯 Red-on-base check of the multiwriter laws that need no new module: installs the store-level race laws (`lost swap` tests) into a MIRROR in BASE state with the two new-module imports replaced by an inline pseudo-owner predicate, runs them, removes the file. Expect failures on base (no merge: 3 computations, `merged` undefined).
# usage: sh redbase.sh <mirror root in base state>
set -u
here=$(cd "$(dirname "$0")" && pwd); root=${1:?mirror root}
[ -d "$root/.git" ] && { echo "refusing the live checkout"; exit 2; }
G="$root/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
[ -e "$G/🔀️merge/🟨️.mjs" ] && { echo "root is not in base state"; exit 2; }
mkdir -p "$G/🧪️tests/👯️multiwriter"; target="$G/🧪️tests/👯️multiwriter/🟦️.ts"
sed -e '/🔐️custody\/🟨️.mjs/d' -e 's|^import {isPseudoOwnerV1} from "../../🔀️merge/🟨️.mjs";|const isPseudoOwnerV1=(name:string)=>name==="@env"\|\|name.startsWith("@derived:");type SliceClaimV1=any;const createSliceCustodyV1:any=undefined,renderCustodyMessageV1:any=undefined;|' "$here/src/🧪️tests/👯️multiwriter/🟦️.ts" > "$target"
cd "$root" && bun test "./${target#$root/}" -t "lost swap" 2>&1 | grep -E '^\(pass\)|^\(fail\)|Expected|Received|^ *[0-9]+ (pass|fail)' | head -30
rm -f "$target"; rmdir "$G/🧪️tests/👯️multiwriter" 2>/dev/null; true

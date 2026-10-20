#!/bin/sh
# 🧪 Builds the pre/post overlays under /private/tmp/gran-value, runs the oracle on both and the seeded regressions, then the differentials.
# usage: sh overlay.sh   (read-only on the checkout; one heavy process at a time; ~6 minutes)
set -eu
R=/Users/ueli/Documents/semio
H="$(cd "$(dirname "$0")" && pwd)"
O=/private/tmp/gran-value
V="🧰️framework/🔨️modules/🌱️value"
mkdir -p "$O/overlay/pre" "$O/fam"
cd "$R"
rsync -a --exclude node_modules --exclude dist --exclude target --exclude .cache --exclude Cargo.lock --include='*/' --include='Cargo.toml' --include='*.rs' --include='*.json' --include='*.ts' --include='*.feature' --exclude='*' --relative "$V/" "🧰️framework/🛍️products/📓️print/" "$O/overlay/pre/"
cp Cargo.toml "$O/overlay/pre/Cargo.toml"
cp "🧰️framework/Cargo.toml" "$O/overlay/pre/🧰️framework/Cargo.toml"
rm -rf "$O/overlay/post"
cp -R "$O/overlay/pre" "$O/overlay/post"
python3 "$H/apply.py" --root "$O/overlay/post" --write --ledger "$O/ledger-post"
python3 "$H/verify_manifests.py" "$O/overlay/pre" "$O/overlay/post"
python3 - <<'PY'
import json, os, re
root = "/private/tmp/gran-value/overlay/pre"
ex = {"tests", "tests-2", "native-decoding-tests", "clone-source-testing", "testing-allocator", "testing-presence-retirement", "resident", "snapshot-read", "dictionary", "retirement-owner"}
names = []
for dp, dn, fn in os.walk(root):
    if "Cargo.toml" in fn:
        t = open(os.path.join(dp, "Cargo.toml"), encoding="utf-8").read()
        m = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
        if m and m.group(1).startswith("semio-framework-value") and "proc-macro = true" not in t and m.group(1)[len("semio-framework-value-"):] not in ex:
            names.append(m.group(1))
json.dump({"name": "valuecore", "crates": sorted(names)}, open("/private/tmp/gran-value/fam/valuecore.json", "w"))
PY
python3 "$H/oracle.py" run "$O/overlay/pre" "$O/fam/valuecore.json" pre --cfgs lib > "$O/oracle-pre.log" 2>&1
python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/valuecore.json" post --cfgs lib > "$O/oracle-post.log" 2>&1
python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/valuecore.json" seedpriv --cfgs lib --mutate "semio-framework-value-ordered::$V/🗂️ordered/🚦️native/🦀️.rs@@pub(super) fn encode<V:ToValue>@@fn encode<V:ToValue>" > "$O/oracle-seedpriv.log" 2>&1
python3 "$H/oracle.py" run "$O/overlay/post" "$O/fam/valuecore.json" seedimp --cfgs lib --mutate "semio-framework-value-ordered::$V/🗂️ordered/📦️packages/🦀️rust/🦀️.rs@@use crate::__m_semio_framework_value_intrinsic::{DecodedValue,DslValue,FromValue,NativeEncodeControl,ToValue};@@use crate::__m_semio_framework_value_intrinsic::{DecodedValue,DslValue,NativeEncodeControl,ToValue};" > "$O/oracle-seedimp.log" 2>&1
G="$H/../../🗑️generated/gran-value/oracle"
for l in post seedpriv seedimp; do echo "== pre vs $l"; python3 "$H/oracle_diff.py" "$G/gv-valuecore-pre-cur-lib-final.err" "$G/gv-valuecore-$l-cur-lib-final.err" | head -4 || true; done

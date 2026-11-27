#!/bin/zsh
# usage: exp.sh <root>  (builds chain under <root>)
set -e
ROOT=$1; M=/Users/ueli/Documents/semio/🧰️framework/🔨️modules
RC=~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/rustc; export RUSTC_BOOTSTRAP=1
REG=$(ls -d ~/.cargo/registry/src/*/ | head -1)
rm -rf $ROOT; mkdir -p $ROOT/out $ROOT/semio-compiler-resources $ROOT/pkg/app
export SEMIO_COMPILER_RESOURCE_ROOT=$ROOT/semio-compiler-resources
D=$ROOT/out
cp -R /private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H15/pre0/. $D/ 2>/dev/null
b(){ local name=$1 src=$2 ct=$3; shift 3; (cd $ROOT/pkg; CARGO_CRATE_NAME=$name nice -n 10 $RC --crate-name $name --edition 2021 --crate-type $ct -Z threads=0 -C metadata=m$name -C extra-filename=-$name --emit=dep-info,metadata,link --out-dir $D -L dependency=$D --remap-path-prefix=$ROOT=/r --remap-path-prefix=$REG=/reg "$@" $src 2>&1 | grep -E "^error" -A6 | head -20); }
CARGO_MANIFEST_DIR=$M/🔏️hash/📦️packages/🦀️rust b semio_framework_hash $M/🔏️hash/🦀️.rs lib
CARGO_MANIFEST_DIR=$M/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/📦️packages/🦀️rust b semio_framework_native_build_compiler_resources $M/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs lib --extern semio_framework_hash=$D/libsemio_framework_hash-semio_framework_hash.rlib
cat > $ROOT/pkg/value.rs <<'V'
#[derive(Debug,Clone)] pub enum DslValue{Null,Bool(bool),String(String),Array(Vec<DslValue>),Object(Vec<(String,DslValue)>),Int(i64),Uint(u64),Float(f64)}
impl DslValue{pub fn float(v:f64)->Self{Self::Float(v)} pub fn int(v:i64)->Self{Self::Int(v)} pub fn uint(v:u64)->Self{Self::Uint(v)}}
V
b semio_framework_value $ROOT/pkg/value.rs lib
SY=$D/libsyn-syn.rlib
CARGO_MANIFEST_DIR=$M/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust b semio_framework_value_derive_resources $M/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/🦀️.rs proc-macro -Clink-arg=-Wl,-install_name,@rpath/libsemio_framework_value_derive_resources-semio_framework_value_derive_resources.dylib -Clink-arg=-Wl,-oso_prefix,$D/ --extern proc_macro2=$D/libproc_macro2-proc_macro2.rlib --extern quote=$D/libquote-quote.rlib --extern syn=$D/libsyn-syn.rlib --extern semio_framework_native_build_compiler_resources=$D/libsemio_framework_native_build_compiler_resources-semio_framework_native_build_compiler_resources.rlib --extern proc_macro
echo '{"a":[1,2,{"b":"x"}],"c":null}' > $ROOT/pkg/app/data.json
cat > $ROOT/pkg/app/lib.rs <<'A'
#![allow(dead_code)]
pub fn v()->semio_framework_value::DslValue{ semio_framework_value_derive_resources::owned_json_file!("data.json") }
A
CARGO_MANIFEST_DIR=$ROOT/pkg/app b semio_app $ROOT/pkg/app/lib.rs lib --extern semio_framework_value=$D/libsemio_framework_value-semio_framework_value.rlib --extern semio_framework_value_derive_resources=$D/libsemio_framework_value_derive_resources-semio_framework_value_derive_resources.dylib
ls $D | grep -E "app|derive" 

#!/bin/zsh
# ⚖️ LW1 → T6 row 6b (U6 stdio wire drift): the 47 stdio lib suites exactly as U6's overlay2 proof ran them (one `cargo test --lib`
# over `wp-u6/t4/stdio-crates.txt`, default features).
R=/Users/ueli/Documents/semio
pkgs=(${(f)"$(cat $R/.tmp-ticket/wp-u6/t4/stdio-crates.txt)"}); args=(); for p in $pkgs; do args+=(-p $p); done
cargo test --offline --lib --no-fail-fast $args; echo "LW1-STEP stdio-47 rc=$?"

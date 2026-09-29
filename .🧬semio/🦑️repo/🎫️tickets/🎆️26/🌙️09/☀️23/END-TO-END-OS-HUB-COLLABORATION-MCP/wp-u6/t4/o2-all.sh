#!/bin/zsh
# 🧪️ Overlay2: the 47 stdio lib suites exactly as the live red count runs them (same package set → same feature unification).
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
pkgs=(${(f)"$(cat /Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4/stdio-crates.txt)"}); args=(); for pkg in $pkgs; do args+=(-p $pkg); done
cargo test --offline --lib --no-fail-fast $args 2>&1 | tee $L/$1.txt | /usr/bin/grep -E "^test .* FAILED$|^error|^test result: FAILED" | head -60
echo "DONE $(date +%T)"
echo "E1 $(date +%T)"
cd /Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay && CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-obuild CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-otarget cargo test --offline --lib --no-fail-fast -p semio-s-artifact-stdio-docx --features component-app-assembly 2>&1 | tee $L/e1-docx-2.txt | /usr/bin/grep -E "^test .*(FAILED|five_megabyte.*)$|^error|^test result" | head -40
cd /Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay && CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-obuild CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-otarget cargo test --offline --lib --no-fail-fast -p semio-framework-os-kernel store 2>&1 | tee $L/e1-kernel-2.txt | /usr/bin/grep -E "^test .* FAILED$|^error|^test result" | head -40
echo "E1-DONE $(date +%T)"

#!/bin/zsh
# 🧪️ E1 overlay proof (run inside `ocargo.sh`): docx/contract/semio lib suites, then the kernel store suite; reds re-run alone.
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
echo "O2-STDIO $(date +%T)"
( cd /Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay2 && pkgs=(${(f)"$(cat /Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4/stdio-crates.txt)"}) && args=() && for pkg in $pkgs; do args+=(-p $pkg); done && CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-o2build CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-o2target cargo test --offline --lib --no-fail-fast $args 2>&1 | tee $L/o2-stdio-all.txt | /usr/bin/grep -E "^test .* FAILED$|^error|^test result: FAILED" | head -60 )
echo "BUILD $(date +%T)"
cargo test --offline --lib --no-run --message-format short -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-semio -p semio-framework-os-kernel 2>&1 | tee $L/e1-build.txt | /usr/bin/grep -E "^(error|warning: unused)" | head -40
cargo test --offline --lib --no-fail-fast -p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-semio 2>&1 | tee $L/e1-stdio.txt | /usr/bin/grep -E "^test .* FAILED$|^test result|^error" | head -40
cargo test --offline --lib --no-fail-fast -p semio-framework-os-kernel store 2>&1 | tee $L/e1-kernel.txt | /usr/bin/grep -E "^test .* FAILED$|^test result|^error" | head -40
echo "DONE $(date +%T)"

#!/bin/zsh
BASE=/private/tmp/bzlrust
BZ=$(echo /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/*/bazel)
cd $BASE/bw 2>/dev/null && $BZ --output_base=$BASE/ob --install_base=$BASE/install --nosystem_rc --nohome_rc --noworkspace_rc shutdown >/dev/null 2>&1
chmod -R u+w $BASE/ob 2>/dev/null; rm -rf $BASE/ob $BASE/disk $BASE/spawns.log $BASE/spawns.before $BASE/exec-*.json $BASE/out-*.txt
echo reset

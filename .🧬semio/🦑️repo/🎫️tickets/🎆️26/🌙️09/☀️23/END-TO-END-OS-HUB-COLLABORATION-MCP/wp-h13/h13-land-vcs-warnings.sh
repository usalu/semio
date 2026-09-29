#!/bin/zsh
# 🧹️ H13 (session 14c): the warning cleanup the landed kernel-db `vcs` removal left behind (the now-unused engine
# `to_core_actor_id` + its `ActorId` import, the hash-codec test imports, and the path qualifications the removed glob
# re-export had made necessary), in one hub + native lane window: apply, kernel-db compile (default + all features) with a
# zero-new-warning census on the touched files, the touched laws, semio-hub check; any red reverses the patch.
# usage: zsh fleet-mutex.sh hub h13 -- zsh fleet-mutex.sh native h13 -- zsh h13-land-vcs-warnings.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/vcs-removal"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/vcs-removal-warnings.patch"
LABEL=$1
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (hub + native lanes held)"

restore() {
  echo "=== RED $(date +%T): $1 — reversing"
  patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
patch -p1 -N -s < "$P" || restore "patch apply"
while IFS= read -r f; do cmp -s "$R/$f" "$W/edit2/$f" || echo "APPLIED-DIFFERS-FROM-EDIT2 (peer edit merged) $f"; done < "$W/warnings-files.txt"

zsh $H/h13-cargo.sh "$LABEL-db-default" check -p semio-framework-os-kernel-db --locked --lib --tests || restore "kernel-db default check"
zsh $H/h13-cargo.sh "$LABEL-db-all" check -p semio-framework-os-kernel-db --locked --all-features --lib --tests || restore "kernel-db all-features check"
for log in "$L/$LABEL-db-default.txt" "$L/$LABEL-db-all.txt"; do
  left=$(/usr/bin/grep -a -E 'to_core_actor_id|`OpBinary`|`store::ArtifactPack`|std::future::Future<Output = Result<\(\), DbError>>|std::sync::Mutex<Vec<EmitEvent>>|std::sync::Arc<HashProjection>' "$log" | head -3)
  [ -n "$left" ] && restore "warnings remain in ${log:t}: $left"
  /usr/bin/grep -a -E '^warning: `semio-framework-os-kernel-db`' "$log"
done
zsh $H/h13-cargo.sh "$LABEL-db-laws" test -p semio-framework-os-kernel-db --locked --features sqlite --lib --no-fail-fast -- db_observe:: db_artifact::tests::committed_durable_group_recovery db_engine::tests::database_shutdown || restore "touched laws"
zsh $H/h13-cargo.sh "$LABEL-hub-check" check -p semio-hub --locked --all-features --lib --bins --tests || restore "semio-hub check"
echo "=== LANDED $(date +%T) $LABEL"

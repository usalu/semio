# Canonical Receipt Ctime-Only Change Audit

Read-only current Source review. Parent's actual diagnostics93931 reports stampEqualfalse/bytesEqualtrue, unchanged dev/inode/mode/nlink/size/mtime/SHA and ctime1792215726265421945→1792215728950326324 (2684904379ns). No settled stage; successfalse/ownedCustodyClosedfalse. No live protected metadata/file, Native/store operation or experiment executed by reviewer.

## Source Findings

No chmod/chown/utimes/xattr/chflags operation appears in production Native-storage TypeScript subtree when tests are excluded. Publication owner captures receipt recordStamp after rename, host-directory metadata sync and actual bounded read. There is no subsequent Source write/rename/link/unlink targeting image.json before finish.

Subsequent macOS body actions are hdiutil attach -nobrowse -noautoopen -mountpoint ROOT FIXED.DMG; hdiutil info -plist; mounted payload witness; read-only backing/root/ancestor checks. Their explicit argv do not target image.json. Witness traverses mounted payload ROOT, not host base receipt. syncNativeDirectoryMetadataV1 opens the declared directory O_RDONLY, checks its FD/ancestry and handle.sync; it does not chmod or write the receipt. Candidate intent unlink/namespace rmdir occur only after successful final equality and therefore cannot explain this earlier refusal.

This does not prove system commands/other actors had no effect; it establishes no explicit Source metadata mutation explaining ctime. Equal nanosecond mtime and changed ctime are insufficient to identify chmod, ownership, ACL/extended-attribute or delayed-filesystem behavior. Do not remove ctime or round timestamp precision from this evidence.

## Small Generated Experiment Design

Use existing ticket Source-cost or candidate-IO owner, genuine bounded Original/control and only a fresh Generated host directory on the same host volume. Schema-first cases: original writer/fsync→candidate rename→actual directory sync→actual bounded reader; repeat reader/ancestor stats at bounded real checkpoints over about3seconds; repeated directory sync; separate no-op chmod(currentMode) as explicit positive control. Keep no-op chmod confined to its own diagnostic fixture and retain bytes/mtime/ctime results rather than assuming its behavior.

At each existing operation boundary collect stage, monotonic elapsed, seven BigInt stamp fields as decimal strings, bytes length/SHA and sync-lstat/async-FD-stat observations. Compare actual Bun observations independently with Node lstat bigints and Python os.stat nanoseconds/sha. Capture full operation schedule and exact host dev without claiming APFS unless existing filesystem identification confirms it. Observe whether time-only changes follow a specific writer/rename/sync/read/metadata control, whether Bun sync/async differ, or whether all independent ports agree. No Native attach, mounted payload or protected read is needed.

If the inert Generated fixture reproduces a ctime-only delta, isolate the earliest boundary using same schedule before proposing a fix. If not, add diagnostic snapshots of already-read expected/current stamps at postpublish and postattach/postverify/after-witness boundaries under a separately approved canonical diagnostic; do not replace existing admission. Current failed receipt demonstrates real delta but not its cause.
